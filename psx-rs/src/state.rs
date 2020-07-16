use crate::config::Config;
use libpsx_rs::{
    Config as CoreConfig,
    Core,
};
use sdl2::{
    keyboard::Keycode,
    video::Window,
    EventPump,
};
use std::{
    cell::Cell,
    env::args,
    panic,
    path::Path,
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
    thread::sleep,
    time::Duration,
};

#[derive(Debug, Copy, Clone)]
enum State {
    Paused,
    Running,
    Exception,
    Quit,
}

pub(crate) fn main_inner(_window: &Window, event_pump: &mut EventPump, config: Config, core_config: CoreConfig) {
    let mut core = Core::new(core_config);
    handle_disc(&mut core);
    log::info!("Core initialized");

    let state = Cell::new(if config.pause_on_start {
        State::Paused
    } else {
        State::Running
    });
    
    log::info!("{:?}", state.get());

    let quit_fn = || {
        log::info!("Quit");
        state.set(State::Quit);
    };

    loop {
        match state.get() {
            State::Running => {
                let keydown_fn = |key| {
                    match key {
                        Keycode::F1 => {
                            state.set(State::Paused);
                            log::info!("Paused");
                        },
                        Keycode::F2 => {
                            core.reset();
                            log::info!("Reset");
                        },
                        Keycode::F3 => {
                            quit_fn();
                        },
                        Keycode::F11 => {
                            core.save_state(None).unwrap();
                            log::info!("Saved state ok");
                        },
                        Keycode::F12 => {
                            core.load_state(None).unwrap();
                            log::info!("Loaded state ok");
                        },
                        _ => return false,
                    }
                    true
                };

                if !handle_events(event_pump, quit_fn, keydown_fn) {
                    if let Err(()) = handle_core_step(&mut core) {
                        if config.quit_on_exception {
                            log::error!("Panic occurred; quitting");
                            state.set(State::Quit);
                        } else {
                            log::error!("Panic occurred; exception");
                            state.set(State::Exception);
                        }
                    }
                }
            },
            State::Paused => {
                let keydown_fn = |key| {
                    match key {
                        Keycode::F1 => {
                            state.set(State::Running);
                            log::info!("Resumed");
                        },
                        Keycode::F2 => {
                            core.reset();
                            log::info!("Reset");
                        },
                        Keycode::F3 => {
                            quit_fn();
                        },
                        Keycode::F11 => {
                            core.save_state(None).unwrap();
                            log::info!("Saved state ok");
                        },
                        Keycode::F12 => {
                            core.load_state(None).unwrap();
                            log::info!("Loaded state ok");
                        },
                        _ => return false,
                    }
                    true
                };

                handle_events(event_pump, quit_fn, keydown_fn);

                sleep(Duration::from_millis(16));
            },
            State::Exception => {
                let keydown_fn = |key| {
                    match key {
                        Keycode::F1 => {
                            log::info!("Cannot resume from exception state");
                        },
                        Keycode::F2 => {
                            log::info!("Reset not supported yet");
                            // core.reset();
                            // state.set(State::Paused);
                            // log::info!("Reset; paused");
                        },
                        Keycode::F3 => {
                            quit_fn();
                        },
                        Keycode::F11 => {
                            log::info!("Cannot save state from exception state");
                        },
                        Keycode::F12 => {
                            log::info!("Load state not supported yet");
                            // core.load_state(None).unwrap();
                            // state.set(State::Paused);
                            // log::info!("Loaded state ok; paused");
                        },
                        _ => return false,
                    }
                    true
                };

                handle_events(event_pump, quit_fn, keydown_fn);

                sleep(Duration::from_millis(16));
            },
            State::Quit => {
                break;
            },
        }
    }

    // Post mortem
    core.analyze();
}

fn handle_events<F1, F2>(event_pump: &mut EventPump, mut quit_fn: F1, mut keydown_fn: F2) -> bool
where
    F1: FnMut(),
    F2: FnMut(Keycode) -> bool,
{
    let mut handled_event = false;

    for event in event_pump.poll_iter() {
        if let sdl2::event::Event::Quit {
            ..
        } = event
        {
            quit_fn();
            handled_event = true
        } else if let sdl2::event::Event::KeyDown {
            keycode,
            ..
        } = event
        {
            if let Some(key) = keycode {
                handled_event |= keydown_fn(key);
            }
        }
    }

    handled_event
}

fn handle_core_step(core: &mut Core) -> Result<(), ()> {
    panic::catch_unwind(panic::AssertUnwindSafe(|| {
        core.step();
    }))
    .map_err(|_| ())
}

fn handle_disc(core: &mut Core) {
    match args().nth(1) {
        Some(disc_path_raw) => {
            let disc_path = Path::new(&disc_path_raw);
            core.change_disc(disc_path);
            log::info!("Changed disc to {}", disc_path.display());
        },
        None => {},
    }
}

fn _toggle_debug_option(flag: &'static AtomicBool, identifier: &str) {
    let old_value = flag.fetch_xor(true, Ordering::Relaxed);
    log::debug!("Toggled {} from {} to {}", identifier, old_value, !old_value);
}
