mod backend;
mod config;
mod state;

use libpsx_rs::Config as CoreConfig;
use sdl2::video::GLProfile;
use std::{
    path::{
        Path,
        PathBuf,
    },
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
    time::Instant,
};

pub(crate) static EXIT: AtomicBool = AtomicBool::new(false);

fn main() {
    // Signal handlers
    setup_signal_handler();

    // Working directory / workspace
    let workspace_path = PathBuf::from(r"./workspace/");
    println!("Working directory: {}, workspace directory: {}", std::env::current_dir().unwrap().to_str().unwrap(), workspace_path.to_str().unwrap());

    // Read config
    let config = config::load(&workspace_path);

    // Setup logging
    let logs_path = workspace_path.join(r"logs/");
    let log_file_path = setup_log_file(&logs_path);
    setup_logger(&log_file_path);
    log::info!("Logging initialized");

    // Initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(false);
    gl_attr.set_context_flags().debug().set();
    let window = video_subsystem.window("psx-rs", 1024, 512).position_centered().resizable().allow_highdpi().opengl().build().unwrap();
    log::info!("SDL initialized");

    // Initialize video.
    let video_backend = backend::initialize_video_backend(config.video_backend_kind, &window);

    // Initialize audio.
    let audio_backend = backend::initialize_audio_backend(config.audio_backend_kind);

    // Initialize CDROM.
    let cdrom_backend = backend::initialize_cdrom_backend(config.cdrom_backend_kind);

    // Initialize psx-rs core.
    let core_config = CoreConfig {
        workspace_path: PathBuf::from(r"./workspace/"),
        bios_filename: "scph5501.bin".into(),
        video_backend,
        audio_backend,
        cdrom_backend,
        time_delta: config.time_delta_secs,
        worker_threads: config.worker_threads,
        internal_scale_factor: config.internal_scale_factor,
        global_bias: config.global_bias,
        r3000_bias: config.r3000_bias,
        gpu_bias: config.gpu_bias,
        gpu_crtc_bias: config.gpu_crtc_bias,
        dmac_bias: config.dmac_bias,
        spu_bias: config.spu_bias,
        timers_bias: config.timers_bias,
        cdrom_bias: config.cdrom_bias,
        padmc_bias: config.padmc_bias,
        intc_bias: config.intc_bias,
    };

    state::main_inner(&window, &mut event_pump, config, core_config);

    // CDROM teardown.
    backend::terminate_cdrom_backend(config.cdrom_backend_kind);

    // Audio teardown.
    backend::terminate_audio_backend(config.audio_backend_kind);

    // Video teardown.
    backend::terminate_video_backend(config.video_backend_kind);
}

fn setup_log_file(logs_path: &Path) -> PathBuf {
    std::fs::create_dir_all(&logs_path).unwrap();
    let file_name = format!("{}.log", chrono::Local::now().format("%Y%m%d_%H%M%S"));
    logs_path.join(file_name)
}

fn setup_logger(log_file_path: &Path) {
    let now = Instant::now();
    fern::Dispatch::new()
        .format(move |out, message, record| out.finish(format_args!("[{}ms][{}][{}] {}", now.elapsed().as_millis(), record.target(), record.level(), message)))
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file_path).unwrap())
        .apply()
        .unwrap();
}

fn setup_signal_handler() {
    let ctrl_c_handler = || {
        EXIT.store(true, Ordering::Relaxed);
    };

    ctrlc::set_handler(ctrl_c_handler).unwrap();
}
