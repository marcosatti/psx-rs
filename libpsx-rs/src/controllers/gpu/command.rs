use crate::State;
use crate::resources::gpu::*;
use crate::controllers::gpu::command_gp0::handle_command as handle_command_gp0;
use crate::controllers::gpu::command_gp1::handle_command as handle_command_gp1;

pub unsafe fn handle_command(state: &State) {
    // TODO: what's the priority of command handling?
    // Doesn't really mention what happens if there is a command waiting in GP0 queue then a command gets written to GP1.
    
    handle_command_gp1(state);
    handle_command_gp0(state);
    handle_read(state);

    handle_stat_dma_request(state);
    handle_stat_recv_cmd(state);
    handle_stat_send_vram(state);
    handle_stat_recv_dma(state);
}

unsafe fn handle_read(state: &State) {
    let resources = &mut *state.resources;
    let read_buffer = &mut resources.gpu.gp0_read_buffer;
    let read = &mut resources.gpu.gpu1810.read;

    loop {
        let data = match read_buffer.pop_front() {
            None => break,
            Some(v) => v,
        };
        match read.write_one(data) {
            Ok(_) => {},
            Err(_) => break,
        }
    }
}

unsafe fn handle_stat_dma_request(state: &State) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;
    
    // TODO: currently GPU says it always wants commands/data.
    
    stat.write_bitfield(STAT_DMA_REQUEST, 1);
}

unsafe fn handle_stat_recv_cmd(state: &State) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;
    let _gp0 = &resources.gpu.gpu1810.gp0;
    
    // TODO: currently GPU says it always wants commands/data.
    
    stat.write_bitfield(STAT_RECV_CMD, 1);
}

unsafe fn handle_stat_send_vram(state: &State) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;
    let read_buffer = &resources.gpu.gp0_read_buffer;
    let read_fifo = &resources.gpu.gpu1810.read;

    let buffer_data_available = !read_buffer.is_empty();
    let fifo_data_available = !read_fifo.is_empty();
    
    let data_available = if buffer_data_available || fifo_data_available { 
        1 
    } else { 
        0
    };

    stat.write_bitfield(STAT_SEND_VRAM, data_available);
}

unsafe fn handle_stat_recv_dma(state: &State) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;

    // TODO: currently GPU says it always wants commands/data.
    // This bit is used for DMA block mode - the DMAC is meant to wait for the line to be asserted before sending the next block.

    stat.write_bitfield(STAT_RECV_DMA, 1);
}
