use crate::resources::Resources;
use crate::resources::cdrom::*;
use crate::controllers::cdrom::command_impl;

pub fn handle_command(resources: &mut Resources) {
    {
        let status = &mut resources.cdrom.status;
        let command = &mut resources.cdrom.command;
        
        if !command.write_latch {
            return;
        }

        status.write_bitfield(STATUS_BUSYSTS, 1);
    }


    let command_value = {
        let command = &mut resources.cdrom.command;
        command.register.read_u8()
    };

    match command_value {
        0x01 => command_impl::command_01(resources),
        0x19 => command_impl::command_19(resources),
        _ => unimplemented!("Command not implemented: 0x{:X}", command_value),
    }

    {
        let status = &mut resources.cdrom.status;
        let command = &mut resources.cdrom.command;
        command.write_latch = false;
        status.write_bitfield(STATUS_BUSYSTS, 0);
    }
}
