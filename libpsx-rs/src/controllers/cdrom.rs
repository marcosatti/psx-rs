pub mod command;
pub mod command_impl;

use std::sync::atomic::Ordering;
use crate::resources::Resources;
use crate::controllers::cdrom::command::*;
use crate::resources::cdrom::*;

pub fn handle_tick(resources: &mut Resources) {
    handle_parameter_fifo(resources);
    handle_response_fifo(resources);
    
    handle_command(resources);

    handle_interrupt_check(resources);
}

fn handle_parameter_fifo(resources: &mut Resources) {
    let status = &mut resources.cdrom.status;
    let fifo = &mut resources.cdrom.parameter;
    let int_flag = &mut resources.cdrom.int_flag;

    if int_flag.parameter_reset.load(Ordering::Acquire) {
        fifo.clear();
        int_flag.parameter_reset.store(false, Ordering::Release);
    }

    let empty_bit = if fifo.is_empty() {
        1
    } else {
        0
    };

    status.write_bitfield(STATUS_PRMEMPT, empty_bit);

    let ready_bit = if !fifo.is_full() {
        1
    } else {
        0
    };

    status.write_bitfield(STATUS_PRMWRDY, ready_bit);
}

fn handle_response_fifo(resources: &mut Resources) {
    let status = &mut resources.cdrom.status;
    let fifo = &mut resources.cdrom.response;

    let ready_bit = if !fifo.is_empty() {
        1
    } else {
        0
    };

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
