use crate::backends::video::VideoBackend;
use crate::resources::Resources;
use crate::resources::gpu::*;
use crate::controllers::gpu::command_gp0::handle_command as handle_command_gp0;
use crate::controllers::gpu::command_gp1::handle_command as handle_command_gp1;

pub fn handle_command(resources: &mut Resources, video_backend: &VideoBackend) {
    // TODO: what's the priority of command handling?
    // Doesn't really mention what happens if there is a command waiting in GP0 queue then a command gets written to GP1.
    handle_command_gp1(resources, video_backend);
    handle_command_gp0(resources, video_backend);
    handle_read(resources);

    handle_stat_dma_request(resources);
    handle_stat_recv_cmd(resources);
    handle_stat_send_vram(resources);
    handle_stat_recv_dma(resources);
}

fn handle_read(resources: &mut Resources) {
    let read_buffer = &mut resources.gpu.gp0_read_buffer;
    let read = &mut resources.gpu.gpu1810.read;

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

fn handle_stat_dma_request(resources: &mut Resources) {
    let stat = &mut resources.gpu.gpu1814.stat;
    
    // TODO: currently GPU says it always wants commands/data.
    
    stat.write_bitfield(STAT_DMA_REQUEST, 1);
}

fn handle_stat_recv_cmd(resources: &mut Resources) {
    let stat = &mut resources.gpu.gpu1814.stat;
    let _gp0 = &resources.gpu.gpu1810.gp0;
    
    // TODO: currently GPU says it always wants commands/data.
    
    stat.write_bitfield(STAT_RECV_CMD, 1);
}

fn handle_stat_send_vram(resources: &mut Resources) {
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

fn handle_stat_recv_dma(resources: &mut Resources) {
    let stat = &mut resources.gpu.gpu1814.stat;

    // TODO: currently GPU says it always wants commands/data.
    // This bit is used for DMA block mode - the DMAC is meant to wait for the line to be asserted before sending the next block.

    stat.write_bitfield(STAT_RECV_DMA, 1);
}
