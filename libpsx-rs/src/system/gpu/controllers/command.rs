use crate::{
    backends::video::VideoBackend,
    system::{
        gpu::{
            constants::*,
            controllers::{
                command_gp0::handle_command as handle_command_gp0,
                command_gp1::handle_command as handle_command_gp1,
            },
            types::ControllerState,
        },
        types::State,
    },
};

pub fn handle_command(state: &State, gpu_state: &mut ControllerState, video_backend: &VideoBackend) {
    // TODO: what's the priority of command handling?
    // Doesn't really mention what happens if there is a command waiting in GP0 queue then a command gets written to
    // GP1.
    handle_command_gp1(state, gpu_state, video_backend);
    handle_command_gp0(state, gpu_state, video_backend);
    handle_read(state, gpu_state);

    handle_stat_dma_request(state);
    handle_stat_recv_cmd(state);
    handle_stat_send_vram(state, gpu_state);
    handle_stat_recv_dma(state);
}

fn handle_read(state: &State, gpu_state: &mut ControllerState) {
    let read_buffer = &mut gpu_state.gp0_read_buffer;
    let read = &state.gpu.read;

    loop {
        if read.is_full() {
            break;
        }

        let data = match read_buffer.pop_front() {
            None => break,
            Some(v) => v,
        };

        read.write_one(data).unwrap();
    }
}

fn handle_stat_dma_request(state: &State) {
    let stat = &state.gpu.stat;

    // TODO: currently GPU says it always wants commands/data.

    stat.write_bitfield(STAT_DMA_REQUEST, 1);
}

fn handle_stat_recv_cmd(state: &State) {
    let stat = &state.gpu.stat;
    let _gp0 = &state.gpu.gp0;

    // TODO: currently GPU says it always wants commands/data.

    stat.write_bitfield(STAT_RECV_CMD, 1);
}

fn handle_stat_send_vram(state: &State, gpu_state: &ControllerState) {
    let stat = &state.gpu.stat;
    let read_buffer = &gpu_state.gp0_read_buffer;
    let read_fifo = &state.gpu.read;

    let buffer_data_available = !read_buffer.is_empty();
    let fifo_data_available = !read_fifo.is_empty();

    let data_available = if buffer_data_available || fifo_data_available {
        1
    } else {
        0
    };

    stat.write_bitfield(STAT_SEND_VRAM, data_available);
}

fn handle_stat_recv_dma(state: &State) {
    let stat = &state.gpu.stat;

    // TODO: currently GPU says it always wants commands/data.
    // This bit is used for DMA block mode - the DMAC is meant to wait for the line to be asserted before sending the
    // next block.

    stat.write_bitfield(STAT_RECV_DMA, 1);
}
