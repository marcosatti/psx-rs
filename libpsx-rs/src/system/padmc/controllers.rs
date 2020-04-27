pub mod command;
pub mod debug;
pub mod memory;

use crate::system::{
    padmc::constants::*,
    types::{
        ControllerContext,
        Event,
        State,
    },
};
use std::{
    sync::atomic::Ordering,
    time::Duration,
};

pub fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(duration) => run_time(context.state, duration),
    }
}

fn run_time(state: &State, duration: Duration) {
    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    ticks /= 16;

    for _ in 0..ticks {
        tick(state);
    }
}

pub fn tick(state: &State) {
    handle_ctrl(state);
    handle_tx(state);
    handle_rx(state);
    handle_baud_timer(state);
}

fn handle_ctrl(state: &State) {
    let ctrl = &state.padmc.ctrl;
    let mode = &state.padmc.mode;
    let stat = &state.padmc.stat;
    let baud = &state.padmc.baud_reload;

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

fn handle_tx(state: &State) {
    {
        let tx_fifo = &state.padmc.tx_fifo;
        let stat = &state.padmc.stat;
        let ctrl = &state.padmc.ctrl.register;

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
        let tx_fifo = &state.padmc.tx_fifo;
        tx_fifo.read_one().unwrap()
    };

    command::handle_command(state, data);

    {
        let stat = &mut state.padmc.stat;
        stat.write_bitfield(STAT_TXRDY_1, 0);
        stat.write_bitfield(STAT_TXRDY_2, 1);
    }
}

fn handle_rx(state: &State) {
    let rx_fifo = &state.padmc.rx_fifo;

    if rx_fifo.is_empty() {
        return;
    }

    let stat = &state.padmc.stat;
    stat.write_bitfield(STAT_RXFIFO_READY, 1);
}

fn handle_baud_timer(state: &State) {
    let stat = &state.padmc.stat;
    let timer_value = stat.read_bitfield(STAT_TIMER).wrapping_sub(1);
    stat.write_bitfield(STAT_TIMER, timer_value);
}
