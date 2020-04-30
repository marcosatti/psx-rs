use crate::{
    backends::cdrom::CdromBackend,
    system::{
        cdrom::{
            constants::*,
            controllers::{
                backend_dispatch,
                interrupt::*,
                state::*,
            },
            types::ControllerState,
        },
        types::State,
    },
};

pub fn command_01_length(_command_iteration: usize) -> usize {
    0
}

pub fn command_01_handler(state: &State, cdrom_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // GetStat

    let response = &state.cdrom.response;

    assert_eq!(command_iteration, 0);
    let stat_value = stat_value(cdrom_state);
    response.write_one(stat_value).unwrap();
    raise_irq(state, 3);
    true
}

pub fn command_02_length(_command_iteration: usize) -> usize {
    3
}

pub fn command_02_handler(state: &State, cdrom_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // Setloc

    // TODO: Assumed to be absolute addressing?

    assert_eq!(command_iteration, 0);
    let parameter = &state.cdrom.parameter;

    let minute = parameter.read_one().unwrap();
    let second = parameter.read_one().unwrap();
    let frame = parameter.read_one().unwrap();

    cdrom_state.msf_address_base = (minute, second, frame);
    cdrom_state.msf_address_offset = 0;

    let stat_value = stat_value(cdrom_state);
    let response = &state.cdrom.response;
    response.write_one(stat_value).unwrap();
    raise_irq(state, 3);
    true
}

pub fn command_06_length(_command_iteration: usize) -> usize {
    0
}

pub fn command_06_handler(state: &State, cdrom_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // ReadN

    assert_eq!(command_iteration, 0);

    // Set CDROM controller state to reading.
    cdrom_state.reading = true;

    let stat_value = stat_value(cdrom_state);
    let response = &state.cdrom.response;
    response.write_one(stat_value).unwrap();
    raise_irq(state, 3);

    true
}

pub fn command_09_length(_command_iteration: usize) -> usize {
    0
}

pub fn command_09_handler(state: &State, cdrom_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // Pause

    assert_eq!(command_iteration, 0);

    cdrom_state.pausing = true;

    let stat_value = stat_value(cdrom_state);
    let response = &state.cdrom.response;
    response.write_one(stat_value).unwrap();
    raise_irq(state, 3);

    true
}

pub fn command_0e_length(_command_iteration: usize) -> usize {
    1
}

pub fn command_0e_handler(state: &State, cdrom_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // Setmode

    assert_eq!(command_iteration, 0);
    let parameter = &state.cdrom.parameter;

    let _mode = parameter.read_one().unwrap();

    let stat_value = stat_value(cdrom_state);
    let response = &state.cdrom.response;
    response.write_one(stat_value).unwrap();
    raise_irq(state, 3);
    true
}

pub fn command_15_length(_command_iteration: usize) -> usize {
    0
}

pub fn command_15_handler(state: &State, cdrom_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // SeekL

    match command_iteration {
        0 => {
            // Set CDROM controller state to seeking, but we don't actually have to do anything.
            // Unset upon next command iteration...
            cdrom_state.seeking = true;

            let stat_value = stat_value(cdrom_state);
            let response = &state.cdrom.response;
            response.write_one(stat_value).unwrap();
            raise_irq(state, 3);
            false
        },
        1 => {
            cdrom_state.seeking = false;

            let stat_value = stat_value(cdrom_state);
            let response = &state.cdrom.response;
            response.write_one(stat_value).unwrap();
            raise_irq(state, 2);
            true
        },
        _ => panic!(),
    }
}

pub fn command_19_length(_command_iteration: usize) -> usize {
    1
}

pub fn command_19_handler(state: &State, _cdrom_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // Test

    assert_eq!(command_iteration, 0);

    let parameter = &state.cdrom.parameter;
    let response = &state.cdrom.response;

    let sub_function = parameter.read_one().unwrap();

    match sub_function {
        0x20 => {
            for &i in VERSION.iter() {
                response.write_one(i).unwrap();
            }
        },
        _ => unimplemented!(),
    }

    raise_irq(state, 3);

    true
}

pub fn command_1a_length(_command_iteration: usize) -> usize {
    0
}

pub fn command_1a_handler(state: &State, cdrom_state: &mut ControllerState, cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // GetID

    match command_iteration {
        0 => {
            let stat_value = stat_value(cdrom_state);
            let response = &state.cdrom.response;
            response.write_one(stat_value).unwrap();
            raise_irq(state, 3);
            false
        },
        1 => {
            let response = &state.cdrom.response;

            let disc_loaded = match backend_dispatch::disc_loaded(cdrom_backend) {
                Ok(disc_loaded) => disc_loaded,
                Err(()) => false,
            };

            if disc_loaded {
                let mode = backend_dispatch::disc_mode(cdrom_backend).expect("GetID was called when no backend is available");
                match mode {
                    2 => {
                        response.write_one(0x02).unwrap();
                        response.write_one(0x00).unwrap();
                        response.write_one(0x20).unwrap();
                        response.write_one(0x00).unwrap();
                        response.write_one(0x53).unwrap();
                        response.write_one(0x43).unwrap();
                        response.write_one(0x45).unwrap();
                        // SCEx: ASCII A = 0x41, E = 0x45, I = 0x49
                        response.write_one(0x41).unwrap();
                    },
                    _ => unimplemented!("Disc mode {} not handled", mode),
                }
                raise_irq(state, 2);
            } else {
                response.write_one(0x08).unwrap();
                response.write_one(0x40).unwrap();
                response.write_one(0x00).unwrap();
                response.write_one(0x00).unwrap();
                response.write_one(0x00).unwrap();
                response.write_one(0x00).unwrap();
                response.write_one(0x00).unwrap();
                response.write_one(0x00).unwrap();
                raise_irq(state, 5);
            }

            true
        },
        _ => panic!(),
    }
}
