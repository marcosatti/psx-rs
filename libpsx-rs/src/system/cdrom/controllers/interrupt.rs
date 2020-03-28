use crate::system::Resources;
use crate::system::cdrom::*;

pub fn raise_irq(resources: &mut Resources, irq_line: usize) {
    let int_enable = &resources.cdrom.int_enable;
    let int_flag = &mut resources.cdrom.int_flag;

    int_flag.set_interrupt(irq_line);

    let int_enable_value = int_enable.register.read_bitfield(INTERRUPT_FLAGS);
    let int_flag_value = int_flag.register.read_bitfield(INTERRUPT_FLAGS);

    if int_flag_value != 0 {
        if (int_enable_value & int_flag_value) != int_flag_value {
            panic!("IRQ pending but corresponding enable flag not set - will never trigger!");
        }
    }
    
    if (int_enable_value & int_flag_value) > 0 {
        use crate::system::intc::register::Line;
        let stat = &resources.intc.stat;
        stat.assert_line(Line::Cdrom);
    }
}
