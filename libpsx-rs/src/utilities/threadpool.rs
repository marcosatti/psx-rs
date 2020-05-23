use crossbeam::{
    queue::ArrayQueue,
    utils::Backoff,
};
use std::{
    any::Any,
    panic::{
        catch_unwind,
        AssertUnwindSafe,
    },
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
    },
    thread,
};

pub(crate) trait Thunk {
    fn call_once(self);
}

pub(crate) struct ThreadPool<F>
where F: Thunk + Send
{
    pool: Vec<thread::JoinHandle<()>>,
    data: Arc<Data<F>>,
}

impl<'a: 'b, 'b, F> ThreadPool<F>
where F: Thunk + Send + 'a
{
    pub(crate) fn new(pool_size: usize, queue_size: usize, thread_name_prefix: &'static str) -> ThreadPool<F> {
        let mut pool = Vec::new();
        let data = Arc::new(Data::new(queue_size));

        for i in 0..pool_size {
            let data = data.clone();
            let name = format!("{}-{}", thread_name_prefix, i);
            let handle = unsafe { thread::Builder::new().name(name).spawn_unchecked::<'a, _, ()>(move || thread_main(data)).unwrap() };
            pool.push(handle);
        }

        ThreadPool {
            pool,
            data,
        }
    }

    pub(crate) fn scope<F2, F3, S>(&self, this_thread_fn: Option<F2>, scope_fn: S)
    where
        F2: Thunk + 'b,
        F3: Thunk + Send + 'b,
        S: FnOnce(&mut Scope<'_, F3>),
    {
        // Transmute send queue to be of type F2.
        let send_queue = unsafe { std::mem::transmute(&self.data.send_queue) };

        let mut scope = Scope::new(send_queue);
        let scope_result = catch_unwind(AssertUnwindSafe(|| scope_fn(&mut scope)));
        if scope_result.is_err() {
            panic!("Panic occurred while invoking the scope closure: {}", any_to_string(scope_result.unwrap_err().as_ref()));
        }

        if this_thread_fn.is_some() {
            this_thread_fn.unwrap().call_once();
        }

        let backoff = Backoff::new();
        let target_count = scope.consume();
        let mut count = 0;
        while count < target_count {
            match self.data.recv_queue.pop() {
                Ok(r) => {
                    r.unwrap();
                    count += 1;
                },
                Err(_) => {
                    backoff.spin();
                },
            }
        }
    }
}

impl<F> Drop for ThreadPool<F>
where F: Thunk + Send
{
    fn drop(&mut self) {
        self.data.stop.store(true, Ordering::SeqCst);
        self.pool.drain(..).for_each(|h| h.join().unwrap());
    }
}

struct Data<F>
where F: Thunk + Send
{
    send_queue: ArrayQueue<F>,
    recv_queue: ArrayQueue<Result<(), String>>,
    stop: AtomicBool,
}

impl<F> Data<F>
where F: Thunk + Send
{
    fn new(queue_size: usize) -> Data<F> {
        Data {
            send_queue: ArrayQueue::new(queue_size),
            recv_queue: ArrayQueue::new(queue_size),
            stop: AtomicBool::new(false),
        }
    }
}

fn thread_main<F>(data: Arc<Data<F>>)
where F: Thunk + Send {
    let mut backoff = Backoff::new();

    while !data.stop.load(Ordering::Relaxed) {
        match data.send_queue.pop() {
            Ok(thunk) => {
                let thunk = AssertUnwindSafe(move || thunk.call_once());
                let result = catch_unwind(thunk).map_err(|e| any_to_string(e.as_ref()));

                let push_result = data.recv_queue.push(result);
                if push_result.is_err() {
                    break;
                }

                std::mem::swap(&mut backoff, &mut Backoff::new());
            },
            Err(_) => {
                backoff.spin();
            },
        }
    }
}

fn any_to_string(any: &dyn Any) -> String {
    match any.downcast_ref::<String>() {
        Some(s) => s.clone(),
        None => String::from("Unknown panic"),
    }
}

pub(crate) struct Scope<'a, F>
where F: Thunk + Send
{
    counter: usize,
    send_queue: &'a ArrayQueue<F>,
}

impl<'a, F> Scope<'a, F>
where F: Thunk + Send
{
    fn new(send_queue: &'a ArrayQueue<F>) -> Scope<'a, F> {
        Scope {
            counter: 0,
            send_queue,
        }
    }

    pub(crate) fn spawn(&mut self, thunk: F) {
        self.send_queue.push(thunk).unwrap();
        self.counter += 1;
    }

    fn consume(self) -> usize {
        self.counter
    }
}
