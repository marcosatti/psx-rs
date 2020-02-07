pub mod command;
pub mod command_impl;
pub mod libmirage;

use std::sync::atomic::Ordering;
use crate::utilities::bool_to_flag;
use crate::backends::cdrom::CdromBackend;
use crate::resources::Resources;
use crate::controllers::cdrom::command::*;
use crate::resources::cdrom::*;

pub fn handle_tick(resources: &mut Resources, cdrom_backend: &CdromBackend<'_>) {
    handle_interrupt_flags(resources);
    handle_parameter_fifo(resources);
    handle_response_fifo(resources);
    
    handle_command(resources, cdrom_backend);

    handle_interrupt_check(resources);
}

fn handle_interrupt_flags(resources: &mut Resources) {
    let int_flag = &mut resources.cdrom.int_flag;

    if int_flag.write_latch.load(Ordering::Acquire) {
        resources.cdrom.response.clear();
        int_flag.write_latch.store(false, Ordering::Release);
    }

    if int_flag.parameter_reset.load(Ordering::Acquire) {
        resources.cdrom.parameter.clear();
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

fn handle_interrupt_check(resources: &mut Resources) {
    let int_enable = &resources.cdrom.int_enable;
    let int_flag = &resources.cdrom.int_flag;

    let int_enable_value = INTERRUPT_FLAGS.extract_from(int_enable.register.read_u8());
    let int_flag_value = INTERRUPT_FLAGS.extract_from(int_flag.register.read_u8());
    
    if (int_enable_value & int_flag_value) > 0 {
        use crate::resources::intc::register::Line;
        let stat = &resources.intc.stat;
        stat.assert_line(Line::Cdrom);
    }
}
