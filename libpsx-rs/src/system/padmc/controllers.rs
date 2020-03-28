pub mod debug;
pub mod command;

use std::sync::atomic::Ordering;
use std::time::Duration;
use crate::resources::Resources;
use crate::resources::padmc::*;
use crate::constants::padmc::*;
use crate::controllers::{Event, ControllerState};

pub fn run(state: &mut ControllerState, event: Event) {
    match event {
        Event::Time(duration) => run_time(state.resources, duration),
    }
}

fn run_time(resources: &mut Resources, duration: Duration) {
    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    ticks /= 16;
    
    for _ in 0..ticks {
        tick(resources); 
    }
}

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
        ctrl.register.write_bitfield(CTRL_ACK, 0);
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

        stat.write_bitfield(STAT_TXRDY_1, 1);
        stat.write_bitfield(STAT_TXRDY_2, 0);

        if tx_fifo.is_empty() {
            return;
        }
    }

    // Start transfer.
    let data = {
        let tx_fifo = &resources.padmc.tx_fifo;
        tx_fifo.read_one().unwrap()
    };

    command::handle_command(resources, data);

    {
        let stat = &mut resources.padmc.stat;
        stat.write_bitfield(STAT_TXRDY_1, 0);
        stat.write_bitfield(STAT_TXRDY_2, 1);
    }
}

fn handle_rx(resources: &mut Resources) {
    let rx_fifo = &resources.padmc.rx_fifo;

    if rx_fifo.is_empty() {
        return;
    }

    let stat = &mut resources.padmc.stat;
    stat.write_bitfield(STAT_RXFIFO_READY, 1);
}

fn handle_baud_timer(resources: &mut Resources) {
    let stat = &mut resources.padmc.stat;
    let timer_value = stat.read_bitfield(STAT_TIMER).wrapping_sub(1);
    stat.write_bitfield(STAT_TIMER, timer_value);
}
