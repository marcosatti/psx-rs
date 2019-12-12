use log::debug;
use crate::resources::Resources;
use crate::resources::timers::*;
use crate::controllers::timers::timer::*;

pub fn handle_irq_trigger(resources: &mut Resources, timer_id: usize, irq_type: IrqType) {
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

pub fn handle_irq_raise(resources: &mut Resources, timer_id: usize) {
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
