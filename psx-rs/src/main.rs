use std::path::{Path, PathBuf};
use std::io::{stdout, stderr};
use std::io::Write;
use std::panic;
use log::{error, info};
use sdl2::video::GLProfile;
use opengl_sys::*;
use openal_sys::*;
use libpsx_rs::{Core, Config};
use libpsx_rs::backends::context::*;
use libpsx_rs::backends::video::*;
use libpsx_rs::backends::audio::*;
use libpsx_rs::debug::DEBUG_CORE_EXIT;

fn main() {
    // Signal handlers
    setup_signal_handler();
    
    // Working directory / workspace
    let workspace_path = PathBuf::from(r"./workspace/");
    println!("Working directory: {}, workspace directory: {}", std::env::current_dir().unwrap().to_str().unwrap(), workspace_path.to_str().unwrap());

    // Setup logging
    let logs_path = workspace_path.join(r"logs/");
    let log_file_path = setup_log_file(&logs_path);
    setup_logger(&log_file_path);
    info!("Logging initialized");

    // Initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    info!("SDL initialized");

    // Initialize window & video backend
    let video_subsystem = sdl_context.video().unwrap();
    setup_gl_context(&video_subsystem);
    let window = video_subsystem.window("psx-rs", 1024, 512).position_centered().opengl().build().unwrap();
    let opengl_context = window.gl_create_context().unwrap();
    let opengl_acquire_context = || { window.gl_make_current(&opengl_context).unwrap(); &opengl_context };
    let opengl_release_context = || { window.subsystem().gl_release_current_context().unwrap(); };
    opengl_acquire_context();
    let opengl_vendor_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_VENDOR as GLenum) as *const i8).to_string_lossy().into_owned() };
    let opengl_version_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_VERSION as GLenum) as *const i8).to_string_lossy().into_owned() };
    let opengl_renderer_string = unsafe { std::ffi::CStr::from_ptr(glGetString(GL_RENDERER as GLenum) as *const i8).to_string_lossy().into_owned() };
    info!("Video initialized: {}, {}, {}", opengl_vendor_string, opengl_version_string, opengl_renderer_string);
    unsafe { glClearColor(0.0, 0.0, 0.0, 1.0); }
    unsafe { glClear(GL_COLOR_BUFFER_BIT); }
    opengl_release_context();

    // Initialize audio
    let openal_device = unsafe { alcOpenDevice(std::ptr::null()) };
    let openal_context = unsafe { alcCreateContext(openal_device, std::ptr::null()) };
    let openal_acquire_context = || { unsafe { alcMakeContextCurrent(openal_context); &openal_context } };
    let openal_release_context = || { unsafe { alcMakeContextCurrent(std::ptr::null_mut()); } };
    openal_acquire_context();
    unsafe { alListener3f(AL_POSITION as ALenum, 0.0, 0.0, 0.0) };
    unsafe { alListener3f(AL_VELOCITY as ALenum, 0.0, 0.0, 0.0) };
    unsafe { alListenerfv(AL_ORIENTATION as ALenum, [0.0, 0.0, -1.0, 0.0, 1.0, 0.0].as_ptr()) };
    let openal_vendor_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_VENDOR as ALenum)).to_string_lossy().into_owned() };
    let openal_version_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_VERSION as ALenum)).to_string_lossy().into_owned() };
    let openal_renderer_string = unsafe { std::ffi::CStr::from_ptr(alGetString(AL_RENDERER as ALenum)).to_string_lossy().into_owned() };
    info!("Audio initialized: {}, {}, {}", openal_vendor_string, openal_version_string, openal_renderer_string);
    openal_release_context();

    // Initialize psx_rs core
    let config = Config {
        workspace_path: PathBuf::from(r"./workspace/"),
        bios_filename: "scph5501.bin".to_owned(),
        video_backend: VideoBackend::OpenGl(
            open_gl::BackendParams {
                context: BackendContext::new(&opengl_acquire_context, &opengl_release_context),
            }
        ),
        audio_backend: AudioBackend::OpenAl(
            open_al::BackendParams {
                context: BackendContext::new(&openal_acquire_context, &openal_release_context),
            }
        )
    };
    let mut core = Core::new(config);
    info!("Core initialized");

    // Do event loop
    'main: while unsafe { !DEBUG_CORE_EXIT } {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        if run_core(&mut core).is_err() {
            error!("Panic occurred, exiting");
            break;
        }
    }

    // Post mortem
    core.debug_analysis();
    stdout().flush().unwrap();
    stderr().flush().unwrap();
}

fn run_core(core: &mut Core) -> Result<(), ()> {
    match panic::catch_unwind(panic::AssertUnwindSafe(|| { core.run(); })) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

fn setup_gl_context(video_subsystem: &sdl2::VideoSubsystem) {
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(false);
    gl_attr.set_context_flags().debug().set();
}

fn setup_log_file(logs_path: &Path) -> PathBuf {
    std::fs::create_dir_all(&logs_path).unwrap();
    let file_name = format!("{}.log", chrono::Local::now().format("%Y%m%d_%H%M%S"));
    logs_path.join(file_name)
}

fn setup_logger(log_file_path: &Path) {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file_path).unwrap())
        .apply()
        .unwrap();
}

fn setup_signal_handler() {
    let ctrl_c_handler = || { unsafe { DEBUG_CORE_EXIT = true; } };
    ctrlc::set_handler(ctrl_c_handler).unwrap();
}
