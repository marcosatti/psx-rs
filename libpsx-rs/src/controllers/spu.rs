pub mod voice;
pub mod transfer;
pub mod adpcm;
pub mod sound;
pub mod openal;
pub mod dac;

use std::time::Duration;
use crate::State;
use crate::constants::spu::CLOCK_SPEED;
use crate::controllers::Event;
use crate::controllers::spu::transfer::*;
use crate::resources::spu::*;
use crate::resources::spu::register::*;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(time) => run_time(state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    for _ in 0..ticks {
        unsafe { tick(state) };
    }
}

unsafe fn tick(state: &State) {
    let resources = &mut *state.resources;
    let control = &resources.spu.control;

    if control.read_bitfield(CONTROL_ENABLE) == 0 {
        return;
    }

    handle_transfer(state);
    handle_interrupt_check(state);
}

unsafe fn handle_transfer(state: &State) {
    let resources = &mut *state.resources;
    let current_transfer_mode = &mut resources.spu.current_transfer_mode;

    handle_current_transfer_address(state);

    match *current_transfer_mode {
        TransferMode::Stop => {
            handle_new_transfer_initialization(state);
        },
        TransferMode::ManualWrite => {
            handle_manual_write_transfer(state);
        },
        TransferMode::DmaWrite => {
            handle_dma_write_transfer(state);
        }, 
        TransferMode::DmaRead => {
            handle_dma_read_transfer(state);
        }, 
    } 
}

unsafe fn handle_interrupt_check(_state: &State) {

}
