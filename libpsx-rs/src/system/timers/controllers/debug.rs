use log::trace;
use crate::system::timers::*;
use crate::controllers::timers::timer::*;
use crate::system::Resources;

const ENABLE_MODE_WRITE_TRACE: bool = false;

pub fn trace_timers(resources: &mut Resources) {
    for i in 0..3 {
        trace_timer(resources, i);
    }
}

pub fn trace_timer(resources: &mut Resources, timer_id: usize) {
    let count = get_count(resources, timer_id);
    let mode = get_mode(resources, timer_id);
    let target = get_target(resources, timer_id);

    trace!(
        "Timer {}: count = 0x{:08X}, mode = 0x{:08X}, target = 0x{:08X}",
        timer_id,
        count.read_u32(),
        mode.register.read_u32(),
        target.read_u32(),
    );

    trace_mode(resources, timer_id);
}

pub fn trace_mode(resources: &mut Resources, timer_id: usize) {
    let mode = get_mode(resources, timer_id);

    let sync_enable = mode.register.read_bitfield(MODE_SYNC_EN);
    let sync_mode = mode.register.read_bitfield(MODE_SYNC_MODE);
    let reset = mode.register.read_bitfield(MODE_RESET);
    let irq_target = mode.register.read_bitfield(MODE_IRQ_TARGET);
    let irq_overflow = mode.register.read_bitfield(MODE_IRQ_OVERFLOW);
    let irq_repeat = mode.register.read_bitfield(MODE_IRQ_REPEAT);
    let irq_pulse = mode.register.read_bitfield(MODE_IRQ_PULSE);
    let clk_src = mode.register.read_bitfield(MODE_CLK_SRC);
    let irq_status = mode.register.read_bitfield(MODE_IRQ_STATUS);
    let target_hit = mode.register.read_bitfield(MODE_TARGET_HIT);
    let overflow_hit = mode.register.read_bitfield(MODE_OVERFLOW_HIT);

    trace!("Timer {} mode details:", timer_id);
    trace!("sync_enable = {}, sync_mode = {}, reset = {}, irq_target = {}", sync_enable, sync_mode, reset, irq_target);
    trace!("irq_overflow = {}, irq_repeat = {}, irq_pulse = {}, clk_src = {}", irq_overflow, irq_repeat, irq_pulse, clk_src);
    trace!("irq_status = {}, target_hit = {}, overflow_hit = {}", irq_status, target_hit, overflow_hit);
}

pub fn trace_mode_write(resources: &mut Resources, timer_id: usize) {
    if !ENABLE_MODE_WRITE_TRACE {
        return;
    }

    trace_mode(resources, timer_id);
}
