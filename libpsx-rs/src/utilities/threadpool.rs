use crossbeam::utils::Backoff;
use crossbeam::queue::ArrayQueue;
use std::thread;
use std::sync::Arc;
use std::panic::{self, AssertUnwindSafe};
use std::any::Any;

pub(crate) trait Thunk {
    fn call_once(self);
}

pub(crate) struct ThreadPool<F> 
where
    F: Thunk + Send,
{
    pool: Vec<thread::JoinHandle<()>>,
    data: Arc<Data<F>>,
}

impl<F> ThreadPool<F> 
where
    F: Thunk + Send,
{
    pub(crate) fn new(pool_size: usize, queue_size: usize) -> ThreadPool<F> {
        let mut pool = Vec::new();
        let data = Arc::new(Data::new(queue_size));

        for i in 0..pool_size {
            let data = data.clone();
            let name = format!("libpsx-rs-{}", i);
            let handle = thread::Builder::new().name(name).spawn_unchecked(move || thread_main(data)).unwrap();
            pool.push(handle);
        }

        ThreadPool {
            pool,
            data,
        }
    }
}

struct Data<F> 
where
    F: Thunk + Send,
{
    send_queue: ArrayQueue<F>,
    recv_queue: ArrayQueue<Result<(), String>>,
}

impl<F> Data<F>
where
    F: Thunk + Send,
{
    fn new(queue_size: usize) -> Data<F> {
        Data {
            send_queue: ArrayQueue::new(queue_size),
            recv_queue: ArrayQueue::new(queue_size),
        }
    }
}

fn thread_main<F>(data: Arc<Data<F>>) 
where
    F: Thunk + Send,
{
    let mut backoff = Backoff::new();

    loop {
        match data.send_queue.pop() {
            Ok(func) => {
                let func = AssertUnwindSafe(move || func.call_once());
                let result = panic::catch_unwind(func).map_err(|e| err_to_string(e));
                data.recv_queue.push(result).unwrap();
                std::mem::swap(&mut backoff, &mut Backoff::new());
            },
            Err(_) => {
                backoff.snooze();
            },
        }
    }
}

fn err_to_string(e: Box<dyn Any + Send>) -> String {
    e.downcast::<String>().map(|s| (*s).clone()).unwrap_or_else(|_| "Unknown panic".to_owned())
}

pub(crate) fn scope<'scope: 'pusher, 'pusher, S: FnOnce(&'pusher mut Pusher<'scope, F>) + 'pusher>(&'scope self, scope_fn: S) {
    let mut pusher = Pusher::new(&self.data.send_queue);
    scope_fn(&mut pusher);

    let spawn_count = pusher.consume();
    let mut done_counter = 0;
    let backoff = Backoff::new();
    while done_counter < spawn_count {
        match self.data.recv_queue.pop() {
            Ok(r) => {
                r.unwrap();
                done_counter += 1;
            }
            Err(_) => backoff.snooze(),
        }
    }
}

pub(crate) struct Pusher<'a, F> 
where
    F: Thunk<'a> + Send + 'a,
{
    counter: usize,
    send_queue: &'a ArrayQueue<F>,
}

impl<'a, F> Pusher<'a, F>
where
    F: Thunk<'a> + Send + 'a,
{
    fn new(send_queue: &'a ArrayQueue<F>) -> Pusher<'a, F> {
        Pusher {
            counter: 0,
            send_queue,
        }
    } 

    pub(crate) fn spawn<'c, F2>(&mut self, func: F2) 
    where
        F2: Thunk<'c> + Send, 
    {
        unsafe { self.send_queue.push(std::mem::transmute(func)); }
        self.counter += 1;
    }

    fn consume(self) -> usize {
        self.counter
    }
}
