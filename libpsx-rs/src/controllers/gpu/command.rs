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

    let space_available = read.write_available();
    let data_available = read_buffer.len();
    let length = space_available.min(data_available);
    let data: Vec<u32> = read_buffer.drain(0..length).collect();

    for i in 0..data.len() {
        read.write_one(data[i]).unwrap();
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
    
    let ready = if read_buffer.is_empty() && read_fifo.is_empty() { 
        0 
    } else { 
        1
    };

    stat.write_bitfield(STAT_SEND_VRAM, ready);
}

unsafe fn handle_stat_recv_dma(state: &State) {
    let resources = &mut *state.resources;
    let stat = &mut resources.gpu.gpu1814.stat;

    // TODO: currently GPU says it always wants commands/data.
    // This bit is used for DMA block mode - the DMAC is meant to wait for the line to be asserted before sending the next block.

    stat.write_bitfield(STAT_RECV_DMA, 1);
}
