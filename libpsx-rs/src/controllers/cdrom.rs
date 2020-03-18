pub mod backend_dispatch;
pub mod command;
pub mod command_impl;
pub mod interrupt;
pub mod debug;
pub mod read;
pub mod state;

use std::sync::atomic::Ordering;
use log::warn;
use crate::utilities::bool_to_flag;
use crate::backends::cdrom::CdromBackend;
use crate::resources::Resources;
use crate::controllers::cdrom::command::*;
use crate::controllers::cdrom::read::*;
use crate::resources::cdrom::*;

pub fn handle_tick(resources: &mut Resources, cdrom_backend: &CdromBackend) {
    handle_interrupt_enable(resources);
    handle_interrupt_flags(resources);
    handle_request(resources);

    handle_state(resources, cdrom_backend);

    handle_parameter_fifo(resources);
    handle_response_fifo(resources);
    handle_data_fifo(resources);
}

fn handle_state(resources: &mut Resources, cdrom_backend: &CdromBackend) {
    // Don't run anything until all previous interrupts have been acknowledged, otherwise new ones could be missed.
    {
        let int_flag = &resources.cdrom.int_flag;
        if int_flag.register.read_bitfield(INTERRUPT_FLAGS) != 0 {
            return;
        }
    }

    // Can only do one action per cycle.
    // Commands get priority over anything else.
    let mut handled = false;
    
    if !handled {
        handled = handle_command(resources, cdrom_backend);
    }

    if !handled {
        handled = handle_read(resources, cdrom_backend);
    }

    if !handled {
        // ...
    }
}

fn handle_request(resources: &mut Resources) {
    let request = &mut resources.cdrom.request;

    if request.write_latch.load(Ordering::Acquire) {
        assert!(request.register.read_bitfield(REQUEST_SMEN) == 0);
        assert!(request.register.read_bitfield(REQUEST_BFWR) == 0);

        let reset_data_fifo = request.register.read_bitfield(REQUEST_BFRD) == 0;
        if reset_data_fifo {
            let count = resources.cdrom.data.read_available();
            //resources.cdrom.data.clear();
            warn!("Ignored Reset CDROM data FIFO (has {} bytes)", count);
        }

        request.write_latch.store(false, Ordering::Release);
    }
}

fn handle_interrupt_enable(resources: &mut Resources) {
    let int_enable = &mut resources.cdrom.int_enable;

    if int_enable.write_latch.load(Ordering::Acquire) {
        int_enable.write_latch.store(false, Ordering::Release);
    }
}

fn handle_interrupt_flags(resources: &mut Resources) {
    let int_flag = &mut resources.cdrom.int_flag;

    if int_flag.write_latch.load(Ordering::Acquire) {
        resources.cdrom.response.clear();
        int_flag.write_latch.store(false, Ordering::Release);
    }

    if int_flag.parameter_reset.load(Ordering::Acquire) {
        // TODO: actually performing a reset causes problems, where the BIOS is writing the clear bit and the parameters at the same time,
        // before the controller gets a chance to run - this is an emulator level issue. There are asserts in the command handler that 
        // check if the parameter is empty after a command has been run (which it should be).
        //resources.cdrom.parameter.clear();
        int_flag.parameter_reset.store(false, Ordering::Release);
    }
}

fn handle_parameter_fifo(resources: &mut Resources) {
    let status = &mut resources.cdrom.status;
    let fifo = &mut resources.cdrom.parameter;

    let empty_bit = bool_to_flag(fifo.is_empty()) as u8;
    status.write_bitfield(STATUS_PRMEMPT, empty_bit);

    let ready_bit = bool_to_flag(!fifo.is_full()) as u8;
    status.write_bitfield(STATUS_PRMWRDY, ready_bit);
}

fn handle_response_fifo(resources: &mut Resources) {
    let status = &mut resources.cdrom.status;
    let fifo = &mut resources.cdrom.response;

    let ready_bit = bool_to_flag(!fifo.is_empty()) as u8;
    status.write_bitfield(STATUS_RSLRRDY, ready_bit);
}

fn handle_data_fifo(resources: &mut Resources) {
    let status = &mut resources.cdrom.status;
    let fifo = &mut resources.cdrom.response;

    let empty_bit = bool_to_flag(!fifo.is_empty()) as u8;
    status.write_bitfield(STATUS_DRQSTS, empty_bit);
}
