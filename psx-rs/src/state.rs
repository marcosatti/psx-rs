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
    let mut core = Core::new(core_config).unwrap();
    handle_change_disc(&mut core);
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
                            quit_fn();
                        },
                        Keycode::F3 => {
                            reset(&mut core, false);
                        },
                        Keycode::F4 => {
                            reset(&mut core, true);
                        },
                        Keycode::F11 => {
                            save_state(&mut core);
                        },
                        Keycode::F12 => {
                            load_state(&mut core);
                        },
                        _ => return false,
                    }
                    true
                };

                if !handle_events(event_pump, quit_fn, keydown_fn) {
                    if let Err(()) = handle_core_step(&mut core) {
                        state.set(State::Exception);
                        log::error!("Exception");
                    }
                }
            },
            State::Paused => {
                let keydown_fn = |key| {
                    match key {
                        Keycode::F1 => {
                            state.set(State::Running);
                            log::info!("Running");
                        },
                        Keycode::F2 => {
                            quit_fn();
                        },
                        Keycode::F3 => {
                            reset(&mut core, false);
                        },
                        Keycode::F4 => {
                            reset(&mut core, true);
                        },
                        Keycode::F11 => {
                            save_state(&mut core);
                        },
                        Keycode::F12 => {
                            load_state(&mut core);
                        },
                        _ => return false,
                    }
                    true
                };

                handle_events(event_pump, quit_fn, keydown_fn);

                sleep(Duration::from_millis(16));
            },
            State::Exception => {
                if config.quit_on_exception {
                    state.set(State::Quit);
                    log::error!("Quit");
                } else {
                    let keydown_fn = |key| {
                        match key {
                            Keycode::F1 => {
                                log::error!("Cannot resume from an exception state");
                            },
                            Keycode::F2 => {
                                quit_fn();
                            },
                            Keycode::F3 => {
                                if reset(&mut core, false) {
                                    state.set(State::Paused);
                                    log::info!("Paused");
                                }
                            },
                            Keycode::F4 => {
                                if reset(&mut core, true) {
                                    state.set(State::Paused);
                                    log::info!("Paused");
                                }
                            },
                            Keycode::F11 => {
                                log::error!("Cannot save state from an exception state");
                            },
                            Keycode::F12 => {
                                if load_state(&mut core) {
                                    state.set(State::Paused);
                                    log::info!("Paused");
                                }
                            },
                            _ => return false,
                        }
                        true
                    };

                    handle_events(event_pump, quit_fn, keydown_fn);

                    sleep(Duration::from_millis(16));
                }
            },
            State::Quit => {
                break;
            },
        }
    }

    // Post mortem
    core.analyze().unwrap();
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
    core.step().map_err(|errors| {
        log::error!("Error occurred while stepping controller(s):");
        for error in errors.iter() {
            log::error!("    {}", &error);
        }
    })
}

fn handle_change_disc(core: &mut Core) {
    match args().nth(1) {
        Some(disc_path_raw) => {
            let disc_path = Path::new(&disc_path_raw);
            core.change_disc(disc_path).unwrap();
            log::info!("Changed disc to {}", disc_path.display());
        },
        None => {},
    }
}

fn load_state(core: &mut Core) -> bool {
    match core.load_state(None) {
        Ok(()) => {
            log::info!("Loaded state ok");
            true
        },
        Err(s) => {
            log::error!("Loading state failed: {}", &s);
            false
        },
    }
}

fn save_state(core: &mut Core) -> bool {
    match core.save_state(None) {
        Ok(()) => {
            log::info!("Saved state ok");
            true
        },
        Err(s) => {
            log::error!("Saving state failed: {}", &s);
            false
        },
    }
}

fn reset(core: &mut Core, hard_reset: bool) -> bool {
    match core.reset(hard_reset) {
        Ok(()) => {
            if hard_reset {
                log::info!("Hard reset ok");
            } else {
                log::info!("Soft reset ok");
            }
            true
        },
        Err(e) => {
            log::error!("Error resetting: {}", &e);
            false
        },
    }
}

fn _toggle_debug_option(flag: &'static AtomicBool, identifier: &str) {
    let old_value = flag.fetch_xor(true, Ordering::Relaxed);
    log::debug!("Toggled {} from {} to {}", identifier, old_value, !old_value);
}
