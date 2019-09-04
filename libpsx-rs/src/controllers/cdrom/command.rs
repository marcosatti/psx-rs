use log::debug;
use crate::State;
use crate::resources::cdrom::*;

pub unsafe fn handle_command(state: &State) {
    let resources = &mut *state.resources;
    let status = &mut resources.cdrom.status;
    let command = &mut resources.cdrom.command;
    
    if !command.write_latch {
        return;
    }

    status.write_bitfield(STATUS_BUSYSTS, 1);

    let command_value = command.register.read_u8();

    match command_value {
        0x19 => handle_command_19(state),
        _ => unimplemented!("Command not implemented: 0x{:X}", command_value),
    }

    command.write_latch = false;

    status.write_bitfield(STATUS_BUSYSTS, 0);
}

unsafe fn handle_command_19(state: &State) {
    debug!("CDROM commmand 0x19: test");

    let resources = &mut *state.resources;
    let parameter = &resources.cdrom.parameter;
    let response = &resources.cdrom.response;

    let sub_function = parameter.read_one().unwrap();

    debug!("Sub function = 0x{:X}", sub_function);

    match sub_function {
        0x20 => {
            response.write_one(0x94).unwrap();
            response.write_one(0x09).unwrap();
            response.write_one(0x19).unwrap();
            response.write_one(0x19).unwrap();
        },
        _ => unimplemented!(),
    }

    let int_flag = &mut resources.cdrom.int_flag;
    let int_flag_value = int_flag.register.read_u8() | 0x2;
    int_flag.register.write_u8(int_flag_value);
}
