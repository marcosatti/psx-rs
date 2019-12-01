pub mod debug;
pub mod command;

use std::sync::atomic::Ordering;
use crate::resources::Resources;
use crate::resources::padmc::*;

pub fn tick(resources: &mut Resources) {
    handle_ctrl(resources);
    handle_tx(resources);
    handle_rx(resources);
    handle_baud_timer(resources);
}

fn handle_ctrl(resources: &mut Resources) {
    let ctrl = &mut resources.padmc.ctrl;
    let mode = &mut resources.padmc.mode;
    let stat = &mut resources.padmc.stat;
    let baud = &mut resources.padmc.baud_reload;

    if !ctrl.write_latch.load(Ordering::Acquire) {
        return;
    }

    if ctrl.register.read_bitfield(CTRL_ACK) != 0 {
        stat.write_bitfield(STAT_RXERR_PARITY, 0);
        stat.write_bitfield(STAT_IRQ, 0);
    }

    if ctrl.register.read_bitfield(CTRL_RESET) != 0 {
        stat.write_u32(0);
        mode.write_u16(0);
        ctrl.register.write_u16(0);
        baud.write_u16(0);
    }

    ctrl.write_latch.store(false, Ordering::Release);
}

fn handle_tx(resources: &mut Resources) {
    {
        let tx_fifo = &resources.padmc.tx_fifo;
        let stat = &mut resources.padmc.stat;
        let ctrl = &resources.padmc.ctrl.register;

        if ctrl.read_bitfield(CTRL_TXEN) == 0 {
            return;
        }

        if stat.read_bitfield(STAT_TXRDY_2) == 0 {
            return;
        }

        if tx_fifo.read_available() == 0 {
            return;
        }

        // Start transfer.
        stat.write_bitfield(STAT_TXRDY_1, 1);
        stat.write_bitfield(STAT_TXRDY_2, 0);
    }

    let data = {
        let tx_fifo = &resources.padmc.tx_fifo;
        tx_fifo.read_one().unwrap()
    };

    command::handle_command(resources, data);

    {
        let stat = &mut resources.padmc.stat;
        stat.write_bitfield(STAT_TXRDY_2, 1);
    }
}

fn handle_rx(resources: &mut Resources) {
    let rx_fifo = &resources.padmc.rx_fifo;

    let stat = &mut resources.padmc.stat;

    if rx_fifo.read_available() != 0 {
        stat.write_bitfield(STAT_RXFIFO_READY, 1);
    }
}

fn handle_baud_timer(resources: &mut Resources) {
    let stat = &mut resources.padmc.stat;
    let timer_value = stat.read_bitfield(STAT_TIMER).wrapping_sub(1);
    stat.write_bitfield(STAT_TIMER, timer_value);
}
