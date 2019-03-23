pub mod crtc;
pub mod gp0;
pub mod gp1;
pub mod data;
pub mod opengl;

use std::time::Duration;
use crate::State;
use crate::constants::gpu::CLOCK_SPEED;
use crate::controllers::Event;
use crate::controllers::gpu::gp0::handle_command as handle_gp0_command;
use crate::controllers::gpu::gp1::handle_command as handle_gp1_command;
use crate::resources::gpu::*;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(time) => run_time(state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    for _ in 0..ticks {
        unsafe { tick(state) };
    }
}

unsafe fn tick(state: &State) {
    handle_status(state);
    handle_gp0_command(state);
    handle_gp1_command(state);
}

unsafe fn handle_status(state: &State) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;

    {
        let gp0 = &resources.gpu.gpu1810.gp0;
        let ready = if gp0.len() == 0 { 1 } else { 0 };
        stat.write_bitfield(STAT_RECV_CMD, ready);
        stat.write_bitfield(STAT_RECV_DMA, ready);
    }

    {
        let read = &resources.gpu.gpu1810.read;
        let ready = if read.len() > 0 { 1 } else { 0 };
        stat.write_bitfield(STAT_SEND_VRAM, ready);
    }
}
