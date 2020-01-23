use std::sync::atomic::Ordering;
use crate::resources::Resources;
use crate::resources::cdrom::*;
use crate::controllers::cdrom::command_impl;

pub fn handle_command(resources: &mut Resources) {
    if resources.cdrom.command_index.is_none() {
        // Read a new command if available.

        {
            let status = &mut resources.cdrom.status;
            let command = &mut resources.cdrom.command;
            
            status.write_bitfield(STATUS_BUSYSTS, 0);

            if !command.write_latch.load(Ordering::Acquire) {
                return;
            }
    
            status.write_bitfield(STATUS_BUSYSTS, 1);
        }

        let command_value = {
            let command = &mut resources.cdrom.command;
            let value = command.register.read_u8();
            command.write_latch.store(false, Ordering::Release);
            value
        };

        resources.cdrom.command_index = Some(command_value);
        resources.cdrom.command_iteration = 0;
    }


    if resources.cdrom.command_index.is_some() {
        let command_index = resources.cdrom.command_index.unwrap();
        let command_iteration = resources.cdrom.command_iteration;

        let finished = match command_index {
            0x01 => command_impl::command_01(resources, command_iteration),
            0x19 => command_impl::command_19(resources, command_iteration),
            0x1A => command_impl::command_1a(resources, command_iteration),
            _ => unimplemented!("Command not implemented: 0x{:X}", command_index),
        };

        if !finished {
            resources.cdrom.command_iteration += 1;
        } else {
            resources.cdrom.command_index = None;
        }
    }
}
