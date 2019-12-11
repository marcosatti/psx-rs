pub mod timer;

use std::sync::atomic::Ordering;
use std::time::Duration;
use log::debug;
use crate::controllers::ControllerState;
use crate::resources::Resources;
use crate::resources::timers::*;
use crate::constants::timers::*;
use crate::controllers::Event;
use crate::controllers::timers::timer::*;

#[derive(Copy, Clone, Debug)]
enum IrqType {
    None,
    Overflow,
    Target,
}

pub fn run(state: &mut ControllerState, event: Event) {
    match event {
        Event::Time(time) => run_time(state.resources, time),
    }
}

fn run_time(resources: &mut Resources, duration: Duration) {
    // Update internal HBLANK counter.
    resources.timers.hblank_counter += duration;

    let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

    for _ in 0..ticks {
        tick(resources);
    }
}

fn tick(resources: &mut Resources) {
    for i in 0..3 {
        handle_mode_write(resources, i);
        handle_mode_read(resources, i);
        handle_count(resources, i);
    }
}

fn handle_mode_write(resources: &mut Resources, timer_id: usize) {
    let mode = get_mode(resources, timer_id);

    if !mode.write_latch.load(Ordering::Acquire) {
        return;
    }

    let value = mode.register.read_u32();
    
    if MODE_SYNC_EN.extract_from(value) > 0 {
        let sync_mode = MODE_SYNC_MODE.extract_from(value);
        unimplemented!("Sync via bit1-2 not implemented: {}, timer_id = {}", sync_mode, timer_id);
    }

    let clock_src = MODE_CLK_SRC.extract_from(value);
    if clock_src > 0 {
        if timer_id == 0 {
            if clock_src != 2 {
                unimplemented!("Non system clock src: {}, timer_id = {}", clock_src, timer_id);
            }
        } else if timer_id == 1 {
            // All implemented.
        } else if timer_id == 2 {
            if clock_src != 1 {
                unimplemented!("Non system clock src: {}, timer_id = {}", clock_src, timer_id);
            }
        }
    }

    handle_count_clear(resources, timer_id);

    mode.write_latch.store(false, Ordering::Release);
    debug!("Timer {} mode write acknowledged, cleared count", timer_id);
}

fn handle_mode_read(resources: &mut Resources, timer_id: usize) {
    let mode = get_mode(resources, timer_id);

    if !mode.read_latch.load(Ordering::Acquire) {
        return;
    }

    mode.register.write_bitfield(MODE_OVERFLOW_HIT, 0);
    mode.register.write_bitfield(MODE_TARGET_HIT, 0);

    mode.write_latch.store(false, Ordering::Release);
    debug!("Timer {} mode read acknowledged, cleared ack bits", timer_id);
}

fn handle_count(resources: &mut Resources, timer_id: usize) {
    let count = get_count(resources, timer_id);
    
    if timer_id == 1 {
        let clk_src = resources.timers.timer1_mode.register.read_bitfield(MODE_CLK_SRC);
        if (clk_src == 1) || (clk_src == 3) {
            let mut hblank_ticks = 0;
            while resources.timers.hblank_counter >= HBLANK_INTERVAL_NTSC {
                resources.timers.hblank_counter -= HBLANK_INTERVAL_NTSC;
                hblank_ticks += 1;
            }

            let value = count.read_u32() + hblank_ticks;
            count.write_u32(value);
        } else {
            let value = count.read_u32() + 1;
            count.write_u32(value);
        }
    } else {
        let value = count.read_u32() + 1;
        count.write_u32(value);
    }

    let irq_type = handle_count_reset(resources, timer_id);
    handle_irq_trigger(resources, timer_id, irq_type);
}

fn handle_count_clear(resources: &mut Resources, timer_id: usize) {
    let count = get_count(resources, timer_id);
    count.write_u32(0);
}

fn handle_count_reset(resources: &mut Resources, timer_id: usize) -> IrqType {
    let mode = get_mode(resources, timer_id);
    let count = get_count(resources, timer_id);
    let count_value = count.read_u32() & 0xFFFF;
    
    let mut irq_type = IrqType::None;
    
    match mode.register.read_bitfield(MODE_RESET) {
        0 => {
            // When counter equals 0xFFFF.
            if count_value == (std::u16::MAX as u32) {
                handle_count_clear(resources, timer_id);
                mode.register.write_bitfield(MODE_OVERFLOW_HIT, 1);
                irq_type = IrqType::Overflow;
            }
        },
        1 => {
            // When counter equals target.
            let target = get_target(resources, timer_id);
            let target_value = target.read_u32() & 0xFFFF;
            if count_value == target_value {
                handle_count_clear(resources, timer_id);
                debug!("Cleared count for timer {} by target 0x{:04X}", timer_id, target_value);
                mode.register.write_bitfield(MODE_TARGET_HIT, 0);
                irq_type = IrqType::Target;
            }
        },
        _ => unreachable!(),
    };

    irq_type
}

fn handle_irq_trigger(resources: &mut Resources, timer_id: usize, irq_type: IrqType) {
    let mode = get_mode(resources, timer_id);

    match irq_type {
        IrqType::None => {},
        IrqType::Overflow => {
            let overflow_trigger = mode.register.read_bitfield(MODE_IRQ_OVERFLOW) > 0;
            
            if overflow_trigger {
                handle_irq_raise(resources, timer_id);
            }
        },
        IrqType::Target => {
            let target_trigger = mode.register.read_bitfield(MODE_IRQ_TARGET) > 0;
            
            if target_trigger {
                handle_irq_raise(resources, timer_id);
            }
        },
    }
}

fn handle_irq_raise(resources: &mut Resources, timer_id: usize) {
    let mode = get_mode(resources, timer_id);
    mode.register.write_bitfield(MODE_IRQ_STATUS, 0);

    use crate::resources::intc::{TMR0, TMR1, TMR2};

    let irq_bit = match timer_id {
        0 => TMR0,
        1 => TMR1,
        2 => TMR2,
        _ => unreachable!(),
    };

    let stat = &mut resources.intc.stat;
    let _stat_lock = stat.mutex.lock();
    stat.register.write_bitfield(irq_bit, 1);

    debug!("Raised INTC IRQ for timer {}", timer_id);
}
