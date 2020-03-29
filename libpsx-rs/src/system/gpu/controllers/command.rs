use crate::backends::video::VideoBackend;
use crate::system::types::State;
use crate::system::gpu::controllers::command_gp0::handle_command as handle_command_gp0;
use crate::system::gpu::controllers::command_gp1::handle_command as handle_command_gp1;
use crate::system::gpu::constants::*;

pub fn handle_command(state: &mut State, video_backend: &VideoBackend) {
    // TODO: what's the priority of command handling?
    // Doesn't really mention what happens if there is a command waiting in GP0 queue then a command gets written to GP1.
    handle_command_gp1(state, video_backend);
    handle_command_gp0(state, video_backend);
    handle_read(state);

    handle_stat_dma_request(state);
    handle_stat_recv_cmd(state);
    handle_stat_send_vram(state);
    handle_stat_recv_dma(state);
}

fn handle_read(state: &mut State) {
    let read_buffer = &mut state.gpu.gp0_read_buffer;
    let read = &mut state.gpu.gpu1810.read;

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

fn handle_stat_dma_request(state: &mut State) {
    let stat = &mut state.gpu.gpu1814.stat;
    
    // TODO: currently GPU says it always wants commands/data.
    
    stat.write_bitfield(STAT_DMA_REQUEST, 1);
}

fn handle_stat_recv_cmd(state: &mut State) {
    let stat = &mut state.gpu.gpu1814.stat;
    let _gp0 = &state.gpu.gpu1810.gp0;
    
    // TODO: currently GPU says it always wants commands/data.
    
    stat.write_bitfield(STAT_RECV_CMD, 1);
}

fn handle_stat_send_vram(state: &mut State) {
    let stat = &mut state.gpu.gpu1814.stat;
    let read_buffer = &state.gpu.gp0_read_buffer;
    let read_fifo = &state.gpu.gpu1810.read;

    let buffer_data_available = !read_buffer.is_empty();
    let fifo_data_available = !read_fifo.is_empty();
    
    let data_available = if buffer_data_available || fifo_data_available { 
        1 
    } else { 
        0
    };

    stat.write_bitfield(STAT_SEND_VRAM, data_available);
}

fn handle_stat_recv_dma(state: &mut State) {
    let stat = &mut state.gpu.gpu1814.stat;

    // TODO: currently GPU says it always wants commands/data.
    // This bit is used for DMA block mode - the DMAC is meant to wait for the line to be asserted before sending the next block.

    stat.write_bitfield(STAT_RECV_DMA, 1);
}
