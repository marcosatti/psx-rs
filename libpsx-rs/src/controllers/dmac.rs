pub mod channel;
pub mod debug;

use std::time::Duration;
use std::sync::atomic::Ordering;
use log::warn;
use crate::controllers::ControllerState;
use crate::resources::Resources;
use crate::constants::dmac::*;
use crate::types::bitfield::Bitfield;
use crate::controllers::Event;
use crate::controllers::dmac::channel::*;
use crate::resources::dmac::*;
use crate::resources::dmac::channel::*;

pub fn run(state: &mut ControllerState, event: Event) {
    match event {
        Event::Time(time) => run_time(state.resources, time),
    }
}

fn run_time(resources: &mut Resources, duration: Duration) {
    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

    // TODO: Properly obey priorities of channels - usually its DMA6 -> DMA0, so just do that for now.

    let mut channel_id: isize = 6;

    while ticks > 0 {
        let channel_ticks = tick(resources, channel_id as usize);

        if channel_ticks == 0 {
            ticks -= 1;
            channel_id -= 1;

            if channel_id < 0 {
                channel_id = 6;
            }

            handle_irq_check(resources);
        } else {
            ticks -= channel_ticks as i64;
        }
    }
    
    handle_irq_check(resources);

    handle_bus_lock(resources);
}

fn tick(resources: &mut Resources, channel: usize) -> i32 {
    let dpcr = &resources.dmac.dpcr;

    let enable = DPCR_CHANNEL_ENABLE_BITFIELDS[channel];

    if dpcr.read_bitfield(enable) != 0 {
        handle_transfer(resources, channel)
    } else {
        0
    }
}

/// Check if all channels are finished, and release the bus lock if true.
fn handle_bus_lock(resources: &mut Resources) {
    for channel_id in 0..6 {
        let transfer_state = unsafe { &mut *get_transfer_state(resources, channel_id) };
        
        if transfer_state.started {
            return;
        }
    }

    resources.bus_locked.store(false, Ordering::Release);
}

fn handle_transfer(resources: &mut Resources, channel: usize) -> i32 {
    let transfer_state = unsafe { &mut *get_transfer_state(resources, channel) };
    let chcr = unsafe { &mut *get_chcr(resources, channel) };
    let madr = unsafe { &mut *get_madr(resources, channel) };
    let bcr = unsafe { &mut *get_bcr(resources, channel) };
    let sync_mode = get_sync_mode(chcr);

    if chcr.read_bitfield(CHCR_CHOPPING) != 0 {
        unimplemented!("DMAC transfer logic not done yet (CHCR_CHOPPING, channel = {})", channel);
    }

    if chcr.read_bitfield(CHCR_STARTBUSY) != 0 {
        if !transfer_state.started {
            resources.bus_locked.store(true, Ordering::Release);

            initialize_transfer(transfer_state, sync_mode, madr, bcr);

            debug::transfer_start(resources, channel);

            if sync_mode == SyncMode::Blocks {
                warn!("Blocks transfer not properly implemented - needs to wait for DMA request hardware line before sending next block");
            }
        }

        match sync_mode {
            SyncMode::Continuous => handle_continuous_transfer(resources, channel),
            SyncMode::Blocks => handle_blocks_transfer(resources, channel),
            SyncMode::LinkedList => handle_linked_list_transfer(resources, channel),
        }
    } else {
        0
    }
}

fn handle_continuous_transfer(resources: &mut Resources, channel: usize) -> i32 {
    let chcr = unsafe { &mut *get_chcr(resources, channel) };
    let transfer_state = unsafe { &mut *get_transfer_state(resources, channel) };
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);

    let continuous_state = if let SyncModeState::Continuous(s) = &mut transfer_state.sync_mode_state { s } else { panic!("Unexpected transfer sync mode state"); };

    let last_transfer = (continuous_state.target_count - continuous_state.current_count) == 1;

    match transfer_direction {
        TransferDirection::FromChannel => {
            let result = pop_channel_data(resources, channel, continuous_state.current_address, last_transfer);
            if result.is_err() {
                return 0;
            }
            let value = result.unwrap();
            resources.main_memory.write_u32(continuous_state.current_address as u32, value);
        },
        TransferDirection::ToChannel => {
            let value = resources.main_memory.read_u32(continuous_state.current_address as u32);
            let result = push_channel_data(resources, channel, value);
            if result.is_err() {
                return 0;
            }
        },
    }

    match madr_step_direction {
        StepDirection::Forwards => continuous_state.current_address += DATA_SIZE,
        StepDirection::Backwards => continuous_state.current_address -= DATA_SIZE,
    }

    continuous_state.current_count += 1;

    let finished = (continuous_state.target_count - continuous_state.current_count) == 0;

    if finished {
        transfer_state.started = false;
        chcr.write_bitfield(CHCR_STARTBUSY, 0);
        set_interrupt_flag(resources, channel);

        debug::transfer_end(resources, channel);
    }

    return 1;
}

fn handle_blocks_transfer(resources: &mut Resources, channel: usize) -> i32 {
    let chcr = unsafe { &mut *get_chcr(resources, channel) };
    let transfer_state = unsafe { &mut *get_transfer_state(resources, channel) };
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);
    let bcr = unsafe { &mut *get_bcr(resources, channel) };
    let madr = unsafe { &mut *get_madr(resources, channel) };

    let blocks_state = if let SyncModeState::Blocks(s) = &mut transfer_state.sync_mode_state { s } else { panic!("Unexpected transfer sync mode state"); };

    let last_transfer = {
        let target = blocks_state.target_bsize_count * blocks_state.target_bamount_count;
        let current = (blocks_state.current_bamount_count * blocks_state.target_bsize_count) + blocks_state.current_bsize_count;
        (target - current) == 1
    };

    match transfer_direction {
        TransferDirection::FromChannel => {
            let result = pop_channel_data(resources, channel, blocks_state.current_address, last_transfer);
            if result.is_err() {
                return 0;
            }
            let value = result.unwrap();
            resources.main_memory.write_u32(blocks_state.current_address as u32, value);
        },
        TransferDirection::ToChannel => {
            let value = resources.main_memory.read_u32(blocks_state.current_address as u32);
            let result = push_channel_data(resources, channel, value);
            if result.is_err() {
                return 0;
            }
        },
    }

    match madr_step_direction {
        StepDirection::Forwards => blocks_state.current_address += DATA_SIZE,
        StepDirection::Backwards => blocks_state.current_address -= DATA_SIZE,
    }

    blocks_state.current_bsize_count += 1;
    if blocks_state.current_bsize_count == blocks_state.target_bsize_count {
        blocks_state.current_bsize_count = 0;
        blocks_state.current_bamount_count += 1;
    }

    let finished = {
        let target = blocks_state.target_bsize_count * blocks_state.target_bamount_count;
        let current = (blocks_state.current_bamount_count * blocks_state.target_bsize_count) + blocks_state.current_bsize_count;
        (target - current) == 0
    };

    if finished {
        transfer_state.started = false;
        madr.write_u32(blocks_state.current_address as u32);
        bcr.write_bitfield(BCR_BLOCKAMOUNT, 0);
        chcr.write_bitfield(CHCR_STARTBUSY, 0);
        set_interrupt_flag(resources, channel);

        debug::transfer_end(resources, channel);
    }

    return 1;
}

fn handle_linked_list_transfer(resources: &mut Resources, channel: usize) -> i32 {
    let chcr = unsafe { &mut *get_chcr(resources, channel) };
    let transfer_state = unsafe { &mut *get_transfer_state(resources, channel) };
    let transfer_direction = get_transfer_direction(chcr);
    let madr = unsafe { &mut *get_madr(resources, channel) };

    if transfer_direction != TransferDirection::ToChannel {
        panic!("Linked list transfers are ToChannel only");
    }

    let linked_list_state = if let SyncModeState::LinkedList(s) = &mut transfer_state.sync_mode_state { s } else { panic!("Unexpected transfer sync mode state"); };

    if linked_list_state.current_count == linked_list_state.target_count {        
        if linked_list_state.next_address == 0xFF_FFFF {
            transfer_state.started = false;
            chcr.write_bitfield(CHCR_STARTBUSY, 0);
            madr.write_u32(0x00FFFFFF);
            set_interrupt_flag(resources, channel);
            
            debug::transfer_end(resources, channel);

            return 1;
        }

        let header_value = resources.main_memory.read_u32(linked_list_state.next_address as u32);
        let next_address = Bitfield::new(0, 24).extract_from(header_value);
        let target_count = Bitfield::new(24, 8).extract_from(header_value) as usize;

        linked_list_state.current_address = linked_list_state.next_address;
        linked_list_state.next_address = next_address;
        linked_list_state.target_count = target_count;
        linked_list_state.current_count = 0;

        return 1;
    } else {
        let address = (linked_list_state.current_address + DATA_SIZE) + ((linked_list_state.current_count as u32) * DATA_SIZE);
        let value = resources.main_memory.read_u32(address as u32);
        let result = push_channel_data(resources, channel, value);
        if result.is_err() {
            return 0;
        }
        linked_list_state.current_count += 1;

        return 1;
    }
}

fn set_interrupt_flag(resources: &mut Resources, channel: usize) {
    let dicr = &mut resources.dmac.dicr;

    let _lock = dicr.mutex.lock();
    
    if dicr.register.read_bitfield(DICR_IRQ_ENABLE_BITFIELDS[channel]) != 0 {
        dicr.register.write_bitfield(DICR_IRQ_FLAG_BITFIELDS[channel], 1);
    }
}

fn handle_irq_check(resources: &mut Resources) {
    let dicr = &mut resources.dmac.dicr;
    let _icr_lock = dicr.mutex.lock();

    let force_irq = dicr.register.read_bitfield(DICR_IRQ_FORCE) != 0;
    
    let mut channel_irq = false;
    let irq_channel_enable = dicr.register.read_bitfield(DICR_IRQ_MASTER_ENABLE) != 0;
    if irq_channel_enable {
        for (&enable, &flag) in DICR_IRQ_ENABLE_BITFIELDS.iter().zip(DICR_IRQ_FLAG_BITFIELDS.iter()) {
            let enable_value = dicr.register.read_bitfield(enable) != 0;
            let flag_value = dicr.register.read_bitfield(flag) != 0;
            if enable_value && flag_value {
                channel_irq = true;
            }
        }
    }

    if force_irq || channel_irq {
        let master_flag_value = dicr.register.read_bitfield(DICR_IRQ_MASTER_FLAG) != 0;
        if !master_flag_value {
            dicr.register.write_bitfield(DICR_IRQ_MASTER_FLAG, 1);

            use crate::resources::intc::DMA;
            let stat = &mut resources.intc.stat;
            let _stat_lock = stat.mutex.lock();
            stat.register.write_bitfield(DMA, 1);
        }
    }
}
