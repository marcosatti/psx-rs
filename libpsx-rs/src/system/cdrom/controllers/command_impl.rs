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
        types::{
            ControllerResult,
            State,
        },
    },
    types::bitfield::Bitfield,
};

pub(crate) fn command_01_length(_command_iteration: usize) -> usize {
    0
}

pub(crate) fn command_01_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> ControllerResult<bool> {
    // GetStat
    if command_iteration > 0 {
        return Err("GetStat: command iteration was above 0".into());
    }

    let stat_value = calculate_stat_value(controller_state);
    state.cdrom.response.write_one(stat_value).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
    handle_irq_raise(state, controller_state, 3)?;
    Ok(true)
}

pub(crate) fn command_02_length(_command_iteration: usize) -> usize {
    3
}

pub(crate) fn command_02_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> ControllerResult<bool> {
    // Setloc
    if command_iteration > 0 {
        return Err("Setloc: command iteration was above 0".into());
    }

    let parameter = &state.cdrom.parameter;
    if parameter.read_available() < 3 {
        return Err("Setloc: parameter FIFO length less than 3".into());
    }

    let minute = parameter.read_one().unwrap();
    let second = parameter.read_one().unwrap();
    let frame = parameter.read_one().unwrap();

    controller_state.msf_address_base = (minute, second, frame);
    controller_state.msf_address_offset = 0;

    let stat_value = calculate_stat_value(controller_state);
    state.cdrom.response.write_one(stat_value).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
    handle_irq_raise(state, controller_state, 3)?;
    Ok(true)
}

pub(crate) fn command_06_length(_command_iteration: usize) -> usize {
    0
}

pub(crate) fn command_06_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> ControllerResult<bool> {
    // ReadN
    if command_iteration > 0 {
        return Err("ReadN: command iteration was above 0".into());
    }

    controller_state.reading = true;
    controller_state.sector_delay_counter = SECTOR_DELAY_CYCLES_SINGLE_SPEED;

    let stat_value = calculate_stat_value(controller_state);
    state.cdrom.response.write_one(stat_value).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
    handle_irq_raise(state, controller_state, 3)?;
    Ok(true)
}

pub(crate) fn command_09_length(_command_iteration: usize) -> usize {
    0
}

pub(crate) fn command_09_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> ControllerResult<bool> {
    // Pause
    let (finished, interrupt_index) = match command_iteration {
        0 => {
            controller_state.reading = false;
            (false, 3)
        },
        1 => (true, 2),
        _ => return Err(format!("Pause: command iteration invalid: {}", command_iteration)),
    };

    let stat_value = calculate_stat_value(controller_state);
    state.cdrom.response.write_one(stat_value).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
    handle_irq_raise(state, controller_state, interrupt_index)?;
    Ok(finished)
}

pub(crate) fn command_0e_length(_command_iteration: usize) -> usize {
    1
}

pub(crate) fn command_0e_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> ControllerResult<bool> {
    // Setmode
    if command_iteration > 0 {
        return Err("Setmode: command iteration was above 0".into());
    }

    let mode = state.cdrom.parameter.read_one().map_err(|_| "Couldn't read from the parameter FIFO".to_owned())?;

    if Bitfield::new(5, 1).extract_from(mode) > 0 {
        return Err("Setmode: unhandled sector size (not data only)".into());
    }

    let stat_value = calculate_stat_value(controller_state);
    state.cdrom.response.write_one(stat_value).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
    handle_irq_raise(state, controller_state, 3)?;
    Ok(true)
}

pub(crate) fn command_15_length(_command_iteration: usize) -> usize {
    0
}

pub(crate) fn command_15_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> ControllerResult<bool> {
    // SeekL
    let (finished, interrupt_index) = match command_iteration {
        0 => {
            controller_state.seeking = true;
            (false, 3)
        },
        1 => {
            controller_state.seeking = false;
            (true, 2)
        },
        _ => return Err(format!("SeekL: command iteration invalid: {}", command_iteration)),
    };

    let stat_value = calculate_stat_value(controller_state);
    state.cdrom.response.write_one(stat_value).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
    handle_irq_raise(state, controller_state, interrupt_index)?;
    Ok(finished)
}

pub(crate) fn command_19_length(_command_iteration: usize) -> usize {
    1
}

pub(crate) fn command_19_handler(state: &State, controller_state: &mut ControllerState, _cdrom_backend: &CdromBackend, command_iteration: usize) -> ControllerResult<bool> {
    // Test
    if command_iteration > 0 {
        return Err("Test: command iteration was above 0".into());
    }

    let sub_function = state.cdrom.parameter.read_one().map_err(|_| "Couldn't read from the parameter FIFO".to_owned())?;

    let response = &state.cdrom.response;
    match sub_function {
        0x20 => {
            for &i in VERSION.iter() {
                response.write_one(i).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
            }
        },
        _ => return Err(format!("Test: sub function not implemented: {}", sub_function)),
    }

    handle_irq_raise(state, controller_state, 3)?;
    Ok(true)
}

pub(crate) fn command_1a_length(_command_iteration: usize) -> usize {
    0
}

pub(crate) fn command_1a_handler(state: &State, controller_state: &mut ControllerState, cdrom_backend: &CdromBackend, command_iteration: usize) -> ControllerResult<bool> {
    // GetID
    const NO_DISC_DATA: [u8; 8] = [0x08, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    const DISC_LOADED_DATA: [u8; 8] = [0x02, 0x00, 0x20, 0x00, 0x53, 0x43, 0x45, 0x41]; // SCEx: ASCII A = 0x41, E = 0x45, I = 0x49

    match command_iteration {
        0 => {
            let stat_value = calculate_stat_value(controller_state);
            state.cdrom.response.write_one(stat_value).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
            handle_irq_raise(state, controller_state, 3)?;
            Ok(false)
        },
        1 => {
            let response = &state.cdrom.response;

            let disc_loaded = match backend_dispatch::disc_loaded(cdrom_backend)? {
                Ok(disc_loaded) => disc_loaded,
                Err(()) => false,
            };

            let interrupt_index;

            if disc_loaded {
                let mode = backend_dispatch::disc_mode(cdrom_backend)?.unwrap();
                match mode {
                    2 => {
                        for data in DISC_LOADED_DATA.iter() {
                            response.write_one(*data).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
                        }
                    },
                    _ => return Err(format!("Disc mode {} not handled", mode)),
                }

                interrupt_index = 2;
            } else {
                for data in NO_DISC_DATA.iter() {
                    response.write_one(*data).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
                }

                interrupt_index = 5;
            };

            handle_irq_raise(state, controller_state, interrupt_index)?;
            Ok(true)
        },
        _ => return Err(format!("GetID: command iteration invalid: {}", command_iteration)),
    }
}
