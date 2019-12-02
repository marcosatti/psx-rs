pub mod channel;
pub mod debug;
pub mod linked_list;
pub mod transfer;

use log::debug;
use std::time::Duration;
use std::sync::atomic::Ordering;
use crate::controllers::ControllerState;
use crate::resources::Resources;
use crate::constants::dmac::*;
use crate::controllers::Event;
use crate::controllers::dmac::channel::*;
use crate::controllers::dmac::transfer::*;
use crate::resources::dmac::*;

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
        let transfer_state = get_transfer_state(resources, channel_id);
        
        if transfer_state.started {
            return;
        }
    }

    resources.bus_locked.store(false, Ordering::Release);
}

/// Performs interrupt check for raising an IRQ on the INTC.
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
