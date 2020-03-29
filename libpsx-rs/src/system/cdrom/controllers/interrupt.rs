use crate::system::types::State;
use crate::system::cdrom::constants::*;
use crate::system::intc::types::Line;

pub fn raise_irq(state: &mut State, irq_line: usize) {
    let int_enable = &state.cdrom.int_enable;
    let int_flag = &mut state.cdrom.int_flag;

    int_flag.set_interrupt(irq_line);

    let int_enable_value = int_enable.register.read_bitfield(INTERRUPT_FLAGS);
    let int_flag_value = int_flag.register.read_bitfield(INTERRUPT_FLAGS);

    if int_flag_value != 0 {
        if (int_enable_value & int_flag_value) != int_flag_value {
            panic!("IRQ pending but corresponding enable flag not set - will never trigger!");
        }
    }
    
    if (int_enable_value & int_flag_value) > 0 {
        let stat = &state.intc.stat;
        stat.assert_line(Line::Cdrom);
    }
}
