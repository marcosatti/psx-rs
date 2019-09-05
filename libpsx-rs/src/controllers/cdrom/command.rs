use crate::resources::Resources;
use crate::constants::cdrom::*;
use crate::resources::cdrom::*;

pub fn handle_command(resources: &mut Resources) {
    let status = &mut resources.cdrom.status;
    let command = &mut resources.cdrom.command;
    
    if !command.write_latch {
        return;
    }

    status.write_bitfield(STATUS_BUSYSTS, 1);

    let command_value = command.register.read_u8();

    match command_value {
        0x19 => handle_command_19(resources),
        _ => unimplemented!("Command not implemented: 0x{:X}", command_value),
    }

    command.write_latch = false;

    status.write_bitfield(STATUS_BUSYSTS, 0);
}

fn handle_command_19(resources: &mut Resources) {
    let parameter = &resources.cdrom.parameter;
    let response = &resources.cdrom.response;

    let sub_function = parameter.read_one().unwrap();

    match sub_function {
        0x20 => {
            for i in VERSION.iter() {
                response.write_one(*i).unwrap();
            }
        },
        _ => unimplemented!(),
    }

    let int_flag = &mut resources.cdrom.int_flag;
    let int_flag_value = int_flag.register.read_u8() | 0x2;
    int_flag.register.write_u8(int_flag_value);
}
