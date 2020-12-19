use crate::{
    system::{
        cdrom::controllers::run as run_cdrom,
        dmac::controllers::run as run_dmac,
        gpu::{
            controllers::run as run_gpu,
            crtc::controllers::run as run_gpu_crtc,
        },
        intc::controllers::run as run_intc,
        padmc::controllers::run as run_padmc,
        r3000::controllers::run as run_r3000,
        spu::controllers::run as run_spu,
        timers::controllers::run as run_timers,
        types::{
            ControllerContext,
            ControllerHandler,
            Event,
        },
    },
    Config,
};
use array_macro::*;
use atomic_enum::*;
use parking_lot::{
    Condvar,
    Mutex,
};
use rand::{
    Rng,
    SeedableRng,
};
use rand_xorshift::XorShiftRng;
use std::{
    cell::UnsafeCell,
    hint::spin_loop,
    ptr::NonNull,
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
    },
    thread::{
        Builder,
        JoinHandle,
    },
};
use thread_priority::*;

const CONTROLLER_COUNT: usize = 9;
const CONTROLLER_HANDLERS: [ControllerHandler; CONTROLLER_COUNT] = [run_r3000, run_intc, run_dmac, run_gpu, run_spu, run_timers, run_cdrom, run_padmc, run_gpu_crtc];
const CONTROLLER_NAMES: [&'static str; CONTROLLER_COUNT] = ["r3000", "intc", "dmac", "gpu", "spu", "timers", "cdrom", "padmc", "gpu_crtc"];

#[atomic_enum]
#[derive(PartialEq)]
enum TaskStatus {
    Finished,
    Pending,
    Running,
}

struct ThreadContext {
    controller_context: NonNull<ControllerContext<'static, 'static>>,
    events: [Event; CONTROLLER_COUNT],
}

impl ThreadContext {
    fn new() -> ThreadContext {
        ThreadContext {
            controller_context: NonNull::dangling(),
            events: [Event::Time(0.0); CONTROLLER_COUNT],
        }
    }
}

struct MutexThreadStatus {
    exited: bool,
    task_status: [TaskStatus; CONTROLLER_COUNT],
    errors: Vec<String>,
}

impl MutexThreadStatus {
    fn new() -> MutexThreadStatus {
        MutexThreadStatus {
            exited: false,
            task_status: [TaskStatus::Finished; CONTROLLER_COUNT],
            errors: Vec::new(),
        }
    }
}

struct MutexThreadState {
    status: Mutex<MutexThreadStatus>,
    pending_cvar: Condvar,
    finished_cvar: Condvar,
    context: UnsafeCell<ThreadContext>,
}

impl MutexThreadState {
    fn new() -> MutexThreadState {
        MutexThreadState {
            status: Mutex::new(MutexThreadStatus::new()),
            pending_cvar: Condvar::new(),
            finished_cvar: Condvar::new(),
            context: UnsafeCell::new(ThreadContext::new()),
        }
    }
}

unsafe impl Sync for MutexThreadState {
}

unsafe impl Send for MutexThreadState {
}

struct SpinlockThreadState {
    exited: AtomicBool,
    status: [AtomicTaskStatus; CONTROLLER_COUNT],
    errors: Mutex<Vec<String>>,
    context: UnsafeCell<ThreadContext>,
}

impl SpinlockThreadState {
    fn new() -> SpinlockThreadState {
        SpinlockThreadState {
            exited: AtomicBool::new(false),
            status: array![AtomicTaskStatus::new(TaskStatus::Finished); CONTROLLER_COUNT],
            errors: Mutex::new(Vec::new()),
            context: UnsafeCell::new(ThreadContext::new()),
        }
    }
}

unsafe impl Sync for SpinlockThreadState {
}

unsafe impl Send for SpinlockThreadState {
}

fn thread_main_mutex(thread_state: Arc<MutexThreadState>) {
    set_current_thread_priority(ThreadPriority::Max).unwrap();

    let this_thread = std::thread::current();
    log::info!("{} thread spawned", this_thread.name().unwrap_or("worker"));

    'main: loop {
        // Wait for pending status or exit.
        let worker_index;
        let notify_worker;
        {
            let mut thread_status = thread_state.status.lock();

            loop {
                if thread_status.exited {
                    break 'main;
                }

                if let Some(index) = thread_status.task_status.iter().position(|s| *s == TaskStatus::Pending) {
                    thread_status.task_status[index] = TaskStatus::Running;
                    worker_index = index;
                    break;
                } else {
                    thread_state.pending_cvar.wait(&mut thread_status);
                }
            }

            notify_worker = thread_status.task_status.iter().any(|s| *s == TaskStatus::Pending);
        }

        if notify_worker {
            thread_state.pending_cvar.notify_one();
        }

        // Run the controller.
        let result = unsafe {
            let handler = CONTROLLER_HANDLERS[worker_index];
            let thread_context = thread_state.context.get().as_ref().unwrap();
            let controller_context = thread_context.controller_context.as_ref();
            let event = thread_context.events[worker_index];
            handler(controller_context, event)
        };

        // Notify main thread & propagate errors.
        {
            let all_finished;

            {
                let mut thread_status = thread_state.status.lock();
                thread_status.task_status[worker_index] = TaskStatus::Finished;
                result.unwrap_or_else(|s| thread_status.errors.push(format!("{}: {}", CONTROLLER_NAMES[worker_index], &s)));
                all_finished = thread_status.task_status.iter().all(|s| *s == TaskStatus::Finished);
            }

            if all_finished {
                thread_state.finished_cvar.notify_one();
            }
        }
    }
}

fn thread_main_spinlock(thread_state: Arc<SpinlockThreadState>) {
    set_current_thread_priority(ThreadPriority::Max).unwrap();

    let this_thread = std::thread::current();
    log::info!("{} thread spawned (using spinlock)", this_thread.name().unwrap_or("worker"));

    let mut rng = XorShiftRng::from_rng(rand::thread_rng()).unwrap();

    'main: loop {
        // Wait for pending status or exit.
        let worker_index;

        'work: loop {
            if thread_state.exited.load(Ordering::Relaxed) {
                break 'main;
            }

            let start_index: usize = rng.gen();
            for index in 0..CONTROLLER_COUNT {
                let index = (start_index + index) % CONTROLLER_COUNT;
                if thread_state.status[index].compare_and_swap(TaskStatus::Pending, TaskStatus::Running, Ordering::AcqRel) == TaskStatus::Pending {
                    worker_index = index;
                    break 'work;
                }

                spin_loop();
            }

            spin_loop();
        }

        // Run the controller.
        let result = unsafe {
            let handler = CONTROLLER_HANDLERS[worker_index];
            let thread_context = thread_state.context.get().as_ref().unwrap();
            let controller_context = thread_context.controller_context.as_ref();
            let event = thread_context.events[worker_index];
            handler(controller_context, event)
        };

        // Propagate errors and signal finished.
        result.unwrap_or_else(|s| thread_state.errors.lock().push(format!("{}: {}", CONTROLLER_NAMES[worker_index], &s)));

        thread_state.status[worker_index].store(TaskStatus::Finished, Ordering::Release);
    }
}

pub(crate) struct MutexThreadedExecutor {
    thread_state: Arc<MutexThreadState>,
    thread_pool: Vec<JoinHandle<()>>,
}

impl Drop for MutexThreadedExecutor {
    fn drop(&mut self) {
        self.thread_state.status.lock().exited = true;
        self.thread_state.pending_cvar.notify_all();
        self.thread_pool.drain(..).for_each(|h| h.join().unwrap());
    }
}

pub(crate) struct SpinlockThreadedExecutor {
    thread_state: Arc<SpinlockThreadState>,
    thread_pool: Vec<JoinHandle<()>>,
}

impl Drop for SpinlockThreadedExecutor {
    fn drop(&mut self) {
        self.thread_state.exited.store(true, Ordering::Release);
        self.thread_pool.drain(..).for_each(|h| h.join().unwrap());
    }
}

enum ThreadedExecutorKind {
    None,
    Mutex(MutexThreadedExecutor),
    Spinlock(SpinlockThreadedExecutor),
}

#[derive(Debug, Copy, Clone)]
pub enum ThreadingKind {
    None,
    Mutex(usize),
    Spinlock(usize),
}

pub(crate) struct Executor {
    threaded: ThreadedExecutorKind,
}

impl Executor {
    pub(crate) fn new(threading: ThreadingKind) -> Executor {
        set_current_thread_priority(ThreadPriority::Max).unwrap();

        match threading {
            ThreadingKind::None => Executor::new_unthreaded(),
            ThreadingKind::Mutex(c) => Executor::new_threaded_mutex(c),
            ThreadingKind::Spinlock(c) => Executor::new_threaded_spinlock(c),
        }
    }

    pub(crate) fn run(&self, config: &Config, context: &ControllerContext) -> Result<(), Vec<String>> {
        let biases = [
            config.r3000_bias,
            config.intc_bias,
            config.dmac_bias,
            config.gpu_bias,
            config.spu_bias,
            config.timers_bias,
            config.cdrom_bias,
            config.padmc_bias,
            config.gpu_crtc_bias,
        ];

        let time_delta = config.time_delta * config.global_bias;
        let events = array![|i| Event::Time(time_delta * biases[i]); CONTROLLER_COUNT];

        match self.threaded {
            ThreadedExecutorKind::None => Executor::run_unthreaded(context, &events),
            ThreadedExecutorKind::Mutex(ref t) => Executor::run_threaded_mutex(t, context, &events),
            ThreadedExecutorKind::Spinlock(ref t) => Executor::run_threaded_spinlock(t, context, &events),
        }
    }

    fn new_unthreaded() -> Executor {
        Executor {
            threaded: ThreadedExecutorKind::None,
        }
    }

    fn new_threaded_mutex(thread_count: usize) -> Executor {
        assert!(thread_count > 0);
        let thread_state = Arc::new(MutexThreadState::new());
        let thread_pool = Executor::new_thread_pool(&thread_state, thread_main_mutex, thread_count);

        Executor {
            threaded: ThreadedExecutorKind::Mutex(MutexThreadedExecutor {
                thread_state,
                thread_pool,
            }),
        }
    }

    fn new_threaded_spinlock(thread_count: usize) -> Executor {
        assert!(thread_count > 0);
        let thread_state = Arc::new(SpinlockThreadState::new());
        let thread_pool = Executor::new_thread_pool(&thread_state, thread_main_spinlock, thread_count);

        Executor {
            threaded: ThreadedExecutorKind::Spinlock(SpinlockThreadedExecutor {
                thread_state,
                thread_pool,
            }),
        }
    }

    fn new_thread_pool<S: Send + Sync + 'static>(thread_state: &Arc<S>, thread_fn: fn(Arc<S>), thread_count: usize) -> Vec<JoinHandle<()>> {
        let mut thread_pool = Vec::new();

        for i in 0..thread_count {
            let thread_state_clone = Arc::clone(thread_state);
            let name = format!("worker-{}", i);
            let handle = Builder::new().name(name).spawn(move || thread_fn(thread_state_clone)).unwrap();
            thread_pool.push(handle);
        }

        thread_pool
    }

    fn run_unthreaded(context: &ControllerContext, events: &[Event; CONTROLLER_COUNT]) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for i in 0..CONTROLLER_COUNT {
            CONTROLLER_HANDLERS[i](context, events[i]).unwrap_or_else(|s| errors.push(format!("{}: {}", CONTROLLER_NAMES[i], &s)));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn run_threaded_mutex(executor: &MutexThreadedExecutor, context: &ControllerContext, events: &[Event; CONTROLLER_COUNT]) -> Result<(), Vec<String>> {
        // Start the tasks.
        {
            let mut thread_status = executor.thread_state.status.lock();
            (0..CONTROLLER_COUNT).for_each(|i| thread_status.task_status[i] = TaskStatus::Pending);
        }

        // Set the context.
        unsafe {
            let thread_context = executor.thread_state.context.get().as_mut().unwrap();
            thread_context.controller_context = NonNull::new_unchecked(std::mem::transmute(context));
            (0..CONTROLLER_COUNT).for_each(|i| thread_context.events[i] = events[i]);
        };

        executor.thread_state.pending_cvar.notify_one();

        // Wait for all tasks to be finished.
        let mut errors = Vec::new();

        {
            let mut thread_status = executor.thread_state.status.lock();

            while !thread_status.task_status.iter().all(|s| *s == TaskStatus::Finished) {
                executor.thread_state.finished_cvar.wait(&mut thread_status);
            }

            errors.extend(thread_status.errors.drain(..));
        };

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn run_threaded_spinlock(executor: &SpinlockThreadedExecutor, context: &ControllerContext, events: &[Event; CONTROLLER_COUNT]) -> Result<(), Vec<String>> {
        // Set the context.
        unsafe {
            let thread_context = executor.thread_state.context.get().as_mut().unwrap();
            thread_context.controller_context = NonNull::new_unchecked(std::mem::transmute(context));
            (0..CONTROLLER_COUNT).for_each(|i| thread_context.events[i] = events[i]);
        };

        // Start the tasks.
        (0..CONTROLLER_COUNT).for_each(|i| executor.thread_state.status[i].store(TaskStatus::Pending, Ordering::Release));

        // Wait for all tasks to be finished.
        while !executor.thread_state.status.iter().all(|s| s.load(Ordering::Acquire) == TaskStatus::Finished) {
            spin_loop();
        }

        let mut errors = Vec::new();
        errors.extend(executor.thread_state.errors.lock().drain(..));

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
