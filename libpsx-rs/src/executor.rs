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
use parking_lot::{
    Condvar,
    Mutex,
};
use std::{
    cell::UnsafeCell,
    ptr::NonNull,
    sync::Arc,
    thread::{
        Builder,
        JoinHandle,
    },
};
use thread_priority::*;

const CONTROLLER_COUNT: usize = 9;
const CONTROLLER_HANDLERS: [ControllerHandler; CONTROLLER_COUNT] = [run_r3000, run_intc, run_dmac, run_gpu, run_spu, run_timers, run_cdrom, run_padmc, run_gpu_crtc];
const CONTROLLER_NAMES: [&'static str; CONTROLLER_COUNT] = ["r3000", "intc", "dmac", "gpu", "spu", "timers", "cdrom", "padmc", "gpu_crtc"];

#[derive(Copy, Clone, Debug, PartialEq)]
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

struct ThreadStatus {
    exited: bool,
    task_status: [TaskStatus; CONTROLLER_COUNT],
    errors: Vec<String>,
}

impl ThreadStatus {
    fn new() -> ThreadStatus {
        ThreadStatus {
            exited: false,
            task_status: [TaskStatus::Finished; CONTROLLER_COUNT],
            errors: Vec::new(),
        }
    }
}

struct ThreadState {
    status: Mutex<ThreadStatus>,
    pending_cvar: Condvar,
    finished_cvar: Condvar,
    context: UnsafeCell<ThreadContext>,
}

impl ThreadState {
    fn new() -> ThreadState {
        ThreadState {
            status: Mutex::new(ThreadStatus::new()),
            pending_cvar: Condvar::new(),
            finished_cvar: Condvar::new(),
            context: UnsafeCell::new(ThreadContext::new()),
        }
    }
}

unsafe impl Sync for ThreadState {
}

unsafe impl Send for ThreadState {
}

fn thread_main(thread_state: Arc<ThreadState>) {
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

pub(crate) struct ThreadedExecutor {
    thread_state: Arc<ThreadState>,
    thread_pool: Vec<JoinHandle<()>>,
}

impl Drop for ThreadedExecutor {
    fn drop(&mut self) {
        self.thread_state.status.lock().exited = true;
        self.thread_state.pending_cvar.notify_all();
        self.thread_pool.drain(..).for_each(|h| h.join().unwrap());
    }
}

pub(crate) struct Executor {
    threaded: Option<ThreadedExecutor>,
}

impl Executor {
    pub(crate) fn new(thread_count: Option<usize>) -> Executor {
        set_current_thread_priority(ThreadPriority::Max).unwrap();

        match thread_count {
            None => Executor::new_unthreaded(),
            Some(c) => Executor::new_threaded(c),
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
            Some(ref t) => Executor::run_threaded(t, context, &events),
            None => Executor::run_unthreaded(context, &events),
        }
    }

    fn new_unthreaded() -> Executor {
        Executor {
            threaded: None,
        }
    }

    fn new_threaded(thread_count: usize) -> Executor {
        assert!(thread_count > 0);
        let thread_state = Arc::new(ThreadState::new());

        let mut thread_pool = Vec::new();
        for i in 0..thread_count {
            let thread_state_clone = Arc::clone(&thread_state);
            let name = format!("worker-{}", i);
            let handle = Builder::new().name(name).spawn(move || thread_main(thread_state_clone)).unwrap();
            thread_pool.push(handle);
        }

        Executor {
            threaded: Some(ThreadedExecutor {
                thread_state,
                thread_pool,
            }),
        }
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

    fn run_threaded(executor: &ThreadedExecutor, context: &ControllerContext, events: &[Event; CONTROLLER_COUNT]) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Set the context.
        unsafe {
            let thread_context = executor.thread_state.context.get().as_mut().unwrap();
            thread_context.controller_context = NonNull::new_unchecked(std::mem::transmute(context));
            (0..CONTROLLER_COUNT).for_each(|i| thread_context.events[i] = events[i]);
        };

        // Start the tasks.
        {
            let mut thread_status = executor.thread_state.status.lock();
            (0..CONTROLLER_COUNT).for_each(|i| thread_status.task_status[i] = TaskStatus::Pending);
        }
        executor.thread_state.pending_cvar.notify_one();

        // Wait for all tasks to be finished.
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
}
