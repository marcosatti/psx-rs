#![feature(const_fn)]
#![feature(nll)]
#![feature(duration_float)]
#![feature(box_syntax)]

mod constants;
mod types;
mod utilities;
mod resources;
mod controllers;
pub mod debug;
pub mod backends;

use std::ops::DerefMut;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use opengl_sys::*;
use openal_sys::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use log::{debug, info};
use crate::debug::BenchmarkDebug;
use crate::debug::debug_opengl_trace;
use crate::backends::video::VideoBackend;
use crate::backends::video::opengl;
use crate::backends::audio::AudioBackend;
use crate::backends::audio::openal;
use crate::resources::Resources;
use crate::controllers::Event;
use crate::controllers::r3000::run as run_r3000;
use crate::controllers::gpu::crtc::run as run_gpu_crtc;
use crate::controllers::intc::run as run_intc;
use crate::controllers::gpu::run as run_gpu;
use crate::controllers::dmac::run as run_dmac;
use crate::controllers::spu::run as run_spu;
use crate::controllers::spu::dac::run as run_spu_dac;

pub struct State<'b, 'a: 'b> {
    pub resources: *mut Resources,
    pub video_backend: &'b VideoBackend<'a>,
    pub audio_backend: &'b AudioBackend<'a>,
}

unsafe impl<'b, 'a> Sync for State<'b, 'a> {}

pub struct Config<'a> {
    pub workspace_path: PathBuf,
    pub bios_filename: String,
    pub video_backend: VideoBackend<'a>,
    pub audio_backend: AudioBackend<'a>,
    pub time_delta_us: u64,
    pub worker_threads: usize,
}

pub struct Core<'a> {
    pub resources: Box<Resources>,
    task_executor: ThreadPool,
    config: Config<'a>,
}

impl<'a> Core<'a> {
    pub fn new(config: Config) -> Core {
        info!("Initializing libpsx-rs with {} time delta (us) and {} worker threads", config.time_delta_us, config.worker_threads);
        info!("Main thread ID: {}", thread_id::get());

        let mut resources = Resources::new();
        let task_executor = ThreadPoolBuilder::new()
            .num_threads(config.worker_threads)
            .thread_name(|id| format!("libpsx-rs:{}:{}", thread_id::get(), id))
            .start_handler(|_| info!("Worker thread ID: {:?}", thread_id::get()))
            .build()
            .unwrap();

        let bios_path = config.workspace_path.join(r"bios/").join(&config.bios_filename);
        load_bios(&bios_path, &mut resources);

        video_setup(&config.video_backend);
        audio_setup(&config.audio_backend);

        Core {
            resources: resources,
            task_executor: task_executor,
            config: config,
        }
    }

    pub fn run(&mut self) {
        let state = State {
            resources: self.resources.deref_mut() as *mut Resources,
            video_backend: &self.config.video_backend,
            audio_backend: &self.config.audio_backend,
        };

        let benchmark_debug = BenchmarkDebug::empty();

        let time = Duration::from_micros(self.config.time_delta_us);

        self.task_executor.scope(|scope| {
            scope.spawn(|_|{
                let timer = Instant::now();
                run_r3000(&state, Event::Time(time));
                unsafe { *benchmark_debug.r3000_benchmark.get() = timer.elapsed(); }
            });
            scope.spawn(|_| {
                let timer = Instant::now();
                run_dmac(&state, Event::Time(time));
                unsafe { *benchmark_debug.dmac_benchmark.get() = timer.elapsed(); }
            });
            scope.spawn(|_| {
                let timer = Instant::now();
                run_gpu(&state, Event::Time(time));
                unsafe { *benchmark_debug.gpu_benchmark.get() = timer.elapsed(); }
            });
            scope.spawn(|_| {
                let timer = Instant::now();
                run_spu(&state, Event::Time(time));
                unsafe { *benchmark_debug.spu_benchmark.get() = timer.elapsed(); }
            });
            scope.spawn(|_| {
                let timer = Instant::now();
                run_gpu_crtc(&state, Event::Time(time));
                unsafe { *benchmark_debug.gpu_crtc_benchmark.get() = timer.elapsed(); }
            });
            scope.spawn(|_| {
                let timer = Instant::now();
                run_spu_dac(&state, Event::Time(time));
                unsafe { *benchmark_debug.spu_dac_benchmark.get() = timer.elapsed(); }
            });
            scope.spawn(|_| { 
                let timer = Instant::now();
                run_intc(&state, Event::Time(time));
                unsafe { *benchmark_debug.intc_benchmark.get() = timer.elapsed(); }
            });
        });

        debug::trace_performance(&time, &benchmark_debug);
    }

    pub fn debug_analysis(&self) {
        debug::analysis(self);
    }
}

fn load_bios(path: &PathBuf, resources: &mut Resources) {
    debug!("Loading BIOS from {}", path.to_str().unwrap());
    let mut f = File::open(path).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    resources.bios.write_raw(&buffer, 0);
}

fn video_setup(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::Opengl(ref params) => video_setup_opengl(params),
    }
}

fn video_setup_opengl(backend_params: &opengl::BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        glDebugMessageControlARB(GL_DONT_CARE, GL_DONT_CARE, GL_DONT_CARE, 0, std::ptr::null(), GL_TRUE as GLboolean);
        glDebugMessageCallbackARB(Some(debug_opengl_trace), std::ptr::null());

        let mut window_fbo = 0;
        glGetIntegerv(GL_FRAMEBUFFER_BINDING, &mut window_fbo);
        opengl::rendering::WINDOW_FBO = window_fbo as GLuint;

        let mut fbo = 0;
        glGenFramebuffers(1, &mut fbo);
        glBindFramebuffer(GL_DRAW_FRAMEBUFFER, fbo);

        let mut texture = 0;
        glGenTextures(1, &mut texture);
        glBindTexture(GL_TEXTURE_2D, texture);
        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB as GLint, 1024, 512, 0, GL_RGB, GL_UNSIGNED_BYTE, std::ptr::null());
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as GLint);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);  

        let mut rbo = 0;
        glGenRenderbuffers(1, &mut rbo);
        glBindRenderbuffer(GL_RENDERBUFFER, rbo);
        glRenderbufferStorage(GL_RENDERBUFFER, GL_DEPTH24_STENCIL8, 1024, 512);
        
        glFramebufferTexture2D(GL_DRAW_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, texture, 0);
        glFramebufferRenderbuffer(GL_DRAW_FRAMEBUFFER, GL_DEPTH_STENCIL_ATTACHMENT, GL_RENDERBUFFER, rbo); 

        glClearColor(0.0, 0.0, 0.0, 1.0);
        glClear(GL_COLOR_BUFFER_BIT);

        if glGetError() != GL_NO_ERROR {
            panic!("Error initializing OpenGL video backend");
        }
    }
}

fn audio_setup(audio_backend: &AudioBackend) {
    match audio_backend {
        AudioBackend::Openal(ref params) => audio_setup_openal(params),
    }
}

fn audio_setup_openal(backend_params: &openal::BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        alGenSources(openal::rendering::SOURCES.len() as ALsizei, openal::rendering::SOURCES.as_mut_ptr());
        alGenBuffers(openal::rendering::BUFFERS.len() as ALsizei, openal::rendering::BUFFERS.as_mut_ptr());

        if alGetError() != AL_NO_ERROR as ALenum {
            panic!("Error initializing OpenAL audio backend");
        }
    }
}
