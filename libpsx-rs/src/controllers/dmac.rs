pub mod channel;
pub mod debug;

use std::time::Duration;
use log::warn;
//use log::debug;
use crate::State;
use crate::constants::dmac::*;
use crate::types::bitfield::Bitfield;
use crate::controllers::Event;
use crate::controllers::dmac::channel::*;
use crate::resources::dmac::*;
use crate::resources::dmac::channel::*;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(time) => run_time(state, time),
    }
}

fn run_time(state: &State, duration: Duration) {
    let mut ticks = (CLOCK_SPEED * duration.as_float_secs()) as i64;

    // TODO: Properly obey priorities of channels - usually its DMA6 -> DMA0, so just do that for now.

    let mut channel_id: isize = 6;

    while ticks > 0 {
        let channel_ticks = unsafe { tick(state, channel_id as usize) };

        if channel_ticks == 0 {
            ticks -= 1;
            channel_id -= 1;

            if channel_id < 0 {
                channel_id = 6;
            }
        } else {
            ticks -= channel_ticks;
        }
    }
    
    unsafe { handle_irq_check(state); }
}

unsafe fn tick(state: &State, channel: usize) -> i64 {
    let resources = &mut *state.resources;
    let dpcr = &resources.dmac.dpcr;

    let enable = DPCR_CHANNEL_ENABLE_BITFIELDS[channel];

    if dpcr.read_bitfield(enable) != 0 {
        handle_transfer(state, channel)
    } else {
        0
    }
}

unsafe fn handle_transfer(state: &State, channel: usize) -> i64 {
    let resources = &mut *state.resources;
    let transfer_state = &mut *get_transfer_state(state, channel);
    let chcr = &mut *get_chcr(state, channel);
    let madr = &mut *get_madr(state, channel);
    let bcr = &mut *get_bcr(state, channel);
    let sync_mode = get_sync_mode(chcr);

    if chcr.read_bitfield(CHCR_CHOPPING) != 0 {
        unimplemented!("DMAC transfer logic not done yet (CHCR_CHOPPING, channel = {})", channel);
    }

    if chcr.read_bitfield(CHCR_STARTBUSY) != 0 {
        if !transfer_state.started {
            resources.bus_locked = true;

            initialize_transfer(transfer_state, sync_mode, madr, bcr);
            debug::transfer_start(state, channel);

            if sync_mode == SyncMode::Blocks {
                warn!("Blocks transfer not properly implemented - needs to wait for DMA request hardware line before sending next block");
            }
        }

        match sync_mode {
            SyncMode::Continuous => handle_continuous_transfer(state, channel),
            SyncMode::Blocks => handle_blocks_transfer(state, channel),
            SyncMode::LinkedList => handle_linked_list_transfer(state, channel),
        }

        1
    } else {
        0
    }
}

unsafe fn handle_continuous_transfer(state: &State, channel: usize) {
    let resources = &mut *state.resources;
    let main_memory = &mut resources.main_memory;
    let chcr = &mut *get_chcr(state, channel);
    let transfer_state = &mut *get_transfer_state(state, channel);
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);

    let continuous_state = if let SyncModeState::Continuous(s) = &mut transfer_state.sync_mode_state { s } else { panic!("Unexpected transfer sync mode state"); };

    let last_transfer = (continuous_state.target_count - continuous_state.current_count) == 1;

    match transfer_direction {
        TransferDirection::FromChannel => {
            let value = pop_channel_data(state, channel, continuous_state.current_address, last_transfer);
            main_memory.write_u32(continuous_state.current_address as usize, value);
        },
        TransferDirection::ToChannel => {
            let value = main_memory.read_u32(continuous_state.current_address as usize);
            push_channel_data(state, channel, value);
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
        set_interrupt_flag(state, channel);
        debug::transfer_end(state, channel);
        resources.bus_locked = false;
    }
}

unsafe fn handle_blocks_transfer(state: &State, channel: usize) {
    let resources = &mut *state.resources;
    let main_memory = &mut resources.main_memory;
    let chcr = &mut *get_chcr(state, channel);
    let transfer_state = &mut *get_transfer_state(state, channel);
    let transfer_direction = get_transfer_direction(chcr);
    let madr_step_direction = get_step_direction(chcr);
    let bcr = &mut *get_bcr(state, channel);
    let madr = &mut *get_madr(state, channel);

    let blocks_state = if let SyncModeState::Blocks(s) = &mut transfer_state.sync_mode_state { s } else { panic!("Unexpected transfer sync mode state"); };

    let last_transfer = {
        let target = blocks_state.target_bsize_count * blocks_state.target_bamount_count;
        let current = (blocks_state.current_bamount_count * blocks_state.target_bsize_count) + blocks_state.current_bsize_count;
        (target - current) == 1
    };

    match transfer_direction {
        TransferDirection::FromChannel => {
            let value = pop_channel_data(state, channel, blocks_state.current_address, last_transfer);
            main_memory.write_u32(blocks_state.current_address as usize, value);
        },
        TransferDirection::ToChannel => {
            let value = main_memory.read_u32(blocks_state.current_address as usize);
            push_channel_data(state, channel, value);
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
        set_interrupt_flag(state, channel);
        debug::transfer_end(state, channel);
        resources.bus_locked = false;
    }
}

unsafe fn handle_linked_list_transfer(state: &State, channel: usize) {
    let resources = &mut *state.resources;
    let main_memory = &mut resources.main_memory;
    let chcr = &mut *get_chcr(state, channel);
    let transfer_state = &mut *get_transfer_state(state, channel);
    let transfer_direction = get_transfer_direction(chcr);
    let madr = &mut *get_madr(state, channel);

    if transfer_direction != TransferDirection::ToChannel {
        panic!("Linked list transfers are ToChannel only");
    }

    let linked_list_state = if let SyncModeState::LinkedList(s) = &mut transfer_state.sync_mode_state { s } else { panic!("Unexpected transfer sync mode state"); };

    if linked_list_state.current_count == linked_list_state.target_count {        
        if linked_list_state.next_address == 0xFF_FFFF {
            transfer_state.started = false;
            chcr.write_bitfield(CHCR_STARTBUSY, 0);
            madr.write_u32(0x00FFFFFF);
            set_interrupt_flag(state, channel);
            debug::transfer_end(state, channel);
            resources.bus_locked = false;
            return;
        }

        let header_value = main_memory.read_u32(linked_list_state.next_address as usize);
        let next_address = Bitfield::new(0, 24).extract_from(header_value);
        let target_count = Bitfield::new(24, 8).extract_from(header_value) as usize;

        linked_list_state.current_address = linked_list_state.next_address;
        linked_list_state.next_address = next_address;
        linked_list_state.target_count = target_count;
        linked_list_state.current_count = 0;
    } else {
        let address = (linked_list_state.current_address + DATA_SIZE) + ((linked_list_state.current_count as u32) * DATA_SIZE);
        let value = main_memory.read_u32(address as usize);
        push_channel_data(state, channel, value);
        linked_list_state.current_count += 1;
    }
}

unsafe fn set_interrupt_flag(state: &State, channel: usize) {
    let resources = &mut *state.resources;
    let dicr = &mut resources.dmac.dicr;

    let _lock = dicr.mutex.lock();
    
    if dicr.register.read_bitfield(DICR_IRQ_ENABLE_BITFIELDS[channel]) != 0 {
        dicr.register.write_bitfield(DICR_IRQ_FLAG_BITFIELDS[channel], 1);
    }
}

unsafe fn handle_irq_check(state: &State) {
    let resources = &mut *state.resources;
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
