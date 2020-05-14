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
    types::bitfield::Bitfield,
};

pub fn command_01_length(_command_iteration: usize) -> usize {
    0
}

pub fn command_01_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // GetStat
    assert_eq!(command_iteration, 0);
    state.cdrom.response.write_one(calculate_stat_value(controller_state)).unwrap();
    handle_irq_raise(state, controller_state, 3);
    true
}

pub fn command_02_length(_command_iteration: usize) -> usize {
    3
}

pub fn command_02_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // Setloc
    assert_eq!(command_iteration, 0);

    let parameter = &state.cdrom.parameter;
    let minute = parameter.read_one().unwrap();
    let second = parameter.read_one().unwrap();
    let frame = parameter.read_one().unwrap();

    controller_state.msf_address_base = (minute, second, frame);
    controller_state.msf_address_offset = 0;

    state.cdrom.response.write_one(calculate_stat_value(controller_state)).unwrap();
    handle_irq_raise(state, controller_state, 3);
    true
}

pub fn command_06_length(_command_iteration: usize) -> usize {
    0
}

pub fn command_06_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // ReadN
    assert_eq!(command_iteration, 0);
    controller_state.reading = true;
    controller_state.sector_delay_counter = SECTOR_DELAY_CYCLES_SINGLE_SPEED;
    controller_state.sector_buffer.clear();
    state.cdrom.response.write_one(calculate_stat_value(controller_state)).unwrap();
    handle_irq_raise(state, controller_state, 3);
    true
}

pub fn command_09_length(_command_iteration: usize) -> usize {
    0
}

pub fn command_09_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // Pause
    match command_iteration {
        0 => {
            state.cdrom.response.write_one(calculate_stat_value(controller_state)).unwrap();
            handle_irq_raise(state, controller_state, 3);
            controller_state.reading = false;
            log::debug!("Paused");
            false
        },
        1 => {
            state.cdrom.response.write_one(calculate_stat_value(controller_state)).unwrap();
            handle_irq_raise(state, controller_state, 2);
            true
        },
        _ => panic!(),
    }
}

pub fn command_0e_length(_command_iteration: usize) -> usize {
    1
}

pub fn command_0e_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // Setmode
    assert_eq!(command_iteration, 0);
    let mode = state.cdrom.parameter.read_one().unwrap();
    assert_eq!(Bitfield::new(5, 1).extract_from(mode), 0);
    state.cdrom.response.write_one(calculate_stat_value(controller_state)).unwrap();
    handle_irq_raise(state, controller_state, 3);
    true
}

pub fn command_15_length(_command_iteration: usize) -> usize {
    0
}

pub fn command_15_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // SeekL
    match command_iteration {
        0 => {
            controller_state.seeking = true;
            state.cdrom.response.write_one(calculate_stat_value(controller_state)).unwrap();
            handle_irq_raise(state, controller_state, 3);
            false
        },
        1 => {
            controller_state.seeking = false;
            state.cdrom.response.write_one(calculate_stat_value(controller_state)).unwrap();
            handle_irq_raise(state, controller_state, 2);
            true
        },
        _ => panic!(),
    }
}

pub fn command_19_length(_command_iteration: usize) -> usize {
    1
}

pub fn command_19_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // Test
    assert_eq!(command_iteration, 0);

    let sub_function = state.cdrom.parameter.read_one().unwrap();

    let response = &state.cdrom.response;
    match sub_function {
        0x20 => {
            for &i in VERSION.iter() {
                response.write_one(i).unwrap();
            }
        },
        _ => unimplemented!(),
    }

    handle_irq_raise(state, controller_state, 3);
    true
}

pub fn command_1a_length(_command_iteration: usize) -> usize {
    0
}

pub fn command_1a_handler(state: &State, controller_state: &mut ControllerState, cdrom_backend: &CdromBackend, command_iteration: usize) -> bool {
    // GetID
    match command_iteration {
        0 => {
            state.cdrom.response.write_one(calculate_stat_value(controller_state)).unwrap();
            handle_irq_raise(state, controller_state, 3);
            false
        },
        1 => {
            let response = &state.cdrom.response;

            let disc_loaded = match backend_dispatch::disc_loaded(cdrom_backend) {
                Ok(disc_loaded) => disc_loaded,
                Err(()) => false,
            };

            let interrupt_index;
            if disc_loaded {
                let mode = backend_dispatch::disc_mode(cdrom_backend).unwrap();
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
                interrupt_index = 2;
            } else {
                response.write_one(0x08).unwrap();
                response.write_one(0x40).unwrap();
                response.write_one(0x00).unwrap();
                response.write_one(0x00).unwrap();
                response.write_one(0x00).unwrap();
                response.write_one(0x00).unwrap();
                response.write_one(0x00).unwrap();
                response.write_one(0x00).unwrap();
                interrupt_index = 5;
            }

            handle_irq_raise(state, controller_state, interrupt_index);
            true
        },
        _ => panic!(),
    }
}
