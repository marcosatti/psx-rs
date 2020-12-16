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
use parking_lot::Mutex;
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
        sleep,
        Builder,
        JoinHandle,
    },
    time::Duration,
};

const CONTROLLER_COUNT: usize = 9;
const CONTROLLER_HANDLERS: [ControllerHandler; CONTROLLER_COUNT] = [run_r3000, run_intc, run_dmac, run_gpu, run_spu, run_timers, run_cdrom, run_padmc, run_gpu_crtc];
const CONTROLLER_NAMES: [&'static str; CONTROLLER_COUNT] = ["r3000", "intc", "dmac", "gpu", "spu", "timers", "cdrom", "padmc", "gpu_crtc"];

#[atomic_enum]
#[derive(PartialEq)]
enum TaskStatus {
    Finished,
    Pending,
    Running,
    Error,
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

struct ThreadState {
    exited: AtomicBool,
    task_status: [AtomicTaskStatus; CONTROLLER_COUNT],
    context: UnsafeCell<ThreadContext>,
    errors: Mutex<[String; CONTROLLER_COUNT]>,
}

impl ThreadState {
    fn new() -> ThreadState {
        ThreadState {
            exited: AtomicBool::new(false),
            task_status: array![AtomicTaskStatus::new(TaskStatus::Finished); CONTROLLER_COUNT],
            context: UnsafeCell::new(ThreadContext::new()),
            errors: Mutex::new(array![String::new(); CONTROLLER_COUNT]),
        }
    }
}

unsafe impl Sync for ThreadState {
}

unsafe impl Send for ThreadState {
}

fn thread_main(thread_state: Arc<ThreadState>, partition_index: usize) {
    const UNSUCCESSFUL_LOOPS_THRESHOLD: usize = 10000;

    let mut unsuccessful_loops = UNSUCCESSFUL_LOOPS_THRESHOLD + 1;

    loop {
        if unsuccessful_loops > UNSUCCESSFUL_LOOPS_THRESHOLD {
            if thread_state.exited.load(Ordering::Acquire) {
                break;
            }

            sleep(Duration::from_millis(5));
        }

        for offset in 0..CONTROLLER_COUNT {
            let controller_index = (partition_index + offset) % CONTROLLER_COUNT;
            let status = &thread_state.task_status[controller_index];

            // Test if we can "own" this task, otherwise we will try another one.
            if status.compare_and_swap(TaskStatus::Pending, TaskStatus::Running, Ordering::AcqRel) == TaskStatus::Pending {
                // We now own this task; execute it.
                let result = unsafe {
                    let handler = CONTROLLER_HANDLERS[controller_index];
                    let thread_context = thread_state.context.get().as_ref().unwrap();
                    let controller_context = thread_context.controller_context.as_ref();
                    let event = thread_context.events[controller_index];
                    handler(controller_context, event)
                };

                let new_status = match result {
                    Ok(()) => TaskStatus::Finished,
                    Err(s) => {
                        let name = CONTROLLER_NAMES[controller_index];
                        thread_state.errors.lock()[controller_index] = format!("{}: {}", name, &s);
                        TaskStatus::Error
                    },
                };
                status.store(new_status, Ordering::Release);

                unsuccessful_loops = 0;
            } else {
                unsuccessful_loops += 1;
                spin_loop();
            }
        }
    }
}

pub(crate) struct Executor {
    thread_state: Arc<ThreadState>,
    thread_pool: Vec<JoinHandle<()>>,
}

impl Executor {
    pub(crate) fn new(pool_size: usize) -> Executor {
        let thread_state = Arc::new(ThreadState::new());

        let mut thread_pool = Vec::new();
        for i in 0..pool_size {
            let thread_state_clone = Arc::clone(&thread_state);
            let partition_index = (CONTROLLER_COUNT / pool_size) * i;
            let name = format!("worker-{}", i);
            let handle = Builder::new().name(name).spawn(move || thread_main(thread_state_clone, partition_index)).unwrap();
            thread_pool.push(handle);
        }

        Executor {
            thread_state,
            thread_pool,
        }
    }

    pub(crate) fn run(&self, config: &Config, context: &ControllerContext) -> Result<(), Vec<String>> {
        let time_delta = config.time_delta * config.global_bias;
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

        // Set the context.
        unsafe {
            let thread_context = self.thread_state.context.get().as_mut().unwrap();
            thread_context.controller_context = NonNull::new_unchecked(std::mem::transmute(context));
            (0..CONTROLLER_COUNT).for_each(|i| thread_context.events[i] = Event::Time(time_delta * biases[i]));
        };

        // Start the tasks.
        for i in 0..CONTROLLER_COUNT {
            self.thread_state.task_status[i].store(TaskStatus::Pending, Ordering::Release);
        }

        let mut errors = Vec::new();

        // Wait for all tasks to be either finished or error'd.
        for i in 0..CONTROLLER_COUNT {
            loop {
                match self.thread_state.task_status[i].load(Ordering::Acquire) {
                    TaskStatus::Error => {
                        errors.push(self.thread_state.errors.lock()[i].clone());
                        break;
                    },
                    TaskStatus::Finished => break,
                    _ => {},
                }
            }
        }

        if errors.len() == 0 {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        self.thread_state.exited.store(true, Ordering::Release);
        self.thread_pool.drain(..).for_each(|h| h.join().unwrap());
    }
}
