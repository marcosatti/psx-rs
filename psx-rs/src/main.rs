mod backend;

use std::path::{Path, PathBuf};
use std::panic;
use std::env::args;
use std::time::Duration;
use std::sync::atomic::{Ordering, AtomicBool};
use std::time::Instant;
use sdl2::EventPump;
use sdl2::video::GLProfile;
use libpsx_rs::{Core, Config};
use libpsx_rs::controllers::r3000::debug::{ENABLE_INTERRUPT_TRACING, ENABLE_STATE_TRACING, ENABLE_MEMORY_SPIN_LOOP_DETECTION_READ, ENABLE_MEMORY_SPIN_LOOP_DETECTION_WRITE, ENABLE_REGISTER_TRACING};
use libpsx_rs::controllers::gpu::debug::{ENABLE_GP0_COMMAND_TRACING, ENABLE_GP0_RENDER_PER_CALL};
use libpsx_rs::debug::DEBUG_CORE_EXIT;
use libpsx_rs::debug::analysis as debug_analysis;

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
    let window = video_subsystem.window("psx-rs", 1024, 512).position_centered().opengl().build().unwrap();
    log::info!("SDL initialized");

    // Initialize video
    let video_backend = backend::initialize_video_backend(&window);

    // Initialize audio
    let audio_backend = backend::initialize_audio_backend();

    // Initialize CDROM
    let cdrom_backend = backend::initialize_cdrom_backend();

    // Initialize psx_rs core
    let time_delta_us = args().nth(2).map_or(25, |v| v.parse::<usize>().unwrap());
    let worker_threads = args().nth(3).map_or(2, |v| v.parse::<usize>().unwrap());
    let config = Config {
        workspace_path: PathBuf::from(r"./workspace/"),
        bios_filename: "scph5501.bin".to_owned(),
        video_backend: video_backend,
        audio_backend: audio_backend,
        cdrom_backend: cdrom_backend,
        time_delta: Duration::from_micros(time_delta_us as u64),
        worker_threads: worker_threads,
    };
    
    main_inner(&mut event_pump, config);

    // CDROM teardown
    backend::terminate_cdrom_backend();

    // Audio teardown
    backend::terminate_audio_backend();

    // Video teardown
    backend::terminate_video_backend();
}

fn setup_log_file(logs_path: &Path) -> PathBuf {
    std::fs::create_dir_all(&logs_path).unwrap();
    let file_name = format!("{}.log", chrono::Local::now().format("%Y%m%d_%H%M%S"));
    logs_path.join(file_name)
}

fn setup_logger(log_file_path: &Path) {
    let now = Instant::now();
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}ms][{}][{}] {}",
                now.elapsed().as_millis(),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file_path).unwrap())
        .apply()
        .unwrap();
}

fn main_inner(event_pump: &mut EventPump, config: Config) {
    let mut core = Core::new(config);
    log::info!("Core initialized");

    match args().nth(1) {
        Some(disc_path_raw) => {
            let disc_path = Path::new(&disc_path_raw);
            core.change_disc(disc_path);
            log::info!("Changed disc to {}", disc_path.display());
        },
        None => {},
    }

    // Do event loop
    let result = panic::catch_unwind(
        panic::AssertUnwindSafe(|| {
            'event_loop: while !DEBUG_CORE_EXIT.load(Ordering::Acquire) {
                for event in event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => break 'event_loop,
                        sdl2::event::Event::KeyDown { keycode, .. } => {
                            if let Some(key) = keycode {
                                handle_keycode(key);
                            }
                        },
                        _ => {},
                    }
                }

                core.step();
            }
        })
    );

    if result.is_err() {
        log::error!("Panic occurred, exiting");
    }

    // Post mortem
    debug_analysis(&mut core);
}

fn setup_signal_handler() {
    let ctrl_c_handler = || { 
        DEBUG_CORE_EXIT.store(true, Ordering::Release);
    };
    
    ctrlc::set_handler(ctrl_c_handler).unwrap();
}

fn handle_keycode(keycode: sdl2::keyboard::Keycode) {
    use sdl2::keyboard::Keycode;

    match keycode {
        Keycode::F1 => { toggle_debug_option(&ENABLE_REGISTER_TRACING, "R3000 register output"); },
        Keycode::F2 => { toggle_debug_option(&ENABLE_STATE_TRACING, "R3000 state tracing"); },
        Keycode::F3 => { toggle_debug_option(&ENABLE_MEMORY_SPIN_LOOP_DETECTION_READ, "spin loop detection (read)"); },
        Keycode::F4 => { toggle_debug_option(&ENABLE_MEMORY_SPIN_LOOP_DETECTION_WRITE, "spin loop detection (write)"); },
        Keycode::F5 => { toggle_debug_option(&ENABLE_INTERRUPT_TRACING, "interrupt tracing"); },
        Keycode::F6 => { toggle_debug_option(&ENABLE_GP0_RENDER_PER_CALL, "GPU rendering per draw call"); },
        Keycode::F7 => { toggle_debug_option(&ENABLE_GP0_COMMAND_TRACING, "GPU GP0 command tracing"); },
        _ => {},
    }
}

fn toggle_debug_option(flag: &'static AtomicBool, identifier: &str) {
    let old_value = flag.fetch_xor(true, Ordering::AcqRel);
    log::debug!("Toggled {} from {} to {}", identifier, old_value, !old_value);
}
