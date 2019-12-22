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

    use crate::resources::intc::register::Line;

    let irq_line = match timer_id {
        0 => Line::Tmr0,
        1 => Line::Tmr1,
        2 => Line::Tmr2,
        _ => unreachable!(),
    };

    let stat = &resources.intc.stat;
    stat.assert_line(irq_line);

    debug!("Raised INTC IRQ for timer {}", timer_id);
}
