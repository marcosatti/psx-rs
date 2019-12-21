use std::sync::atomic::{AtomicBool, Ordering};
use crate::types::register::b32_register::B32Register;
use crate::resources::r3000::cp0::*;

#[derive(Copy, Clone, Debug)]
pub enum IrqLine {
    Intc,
}

pub struct Cause {
    pub register: B32Register,
    intc_pending: AtomicBool,
}

impl Cause {
    pub fn new() -> Cause {
        Cause {
            register: B32Register::new(),
            intc_pending: AtomicBool::new(false),
        }
    }

    pub fn assert_irq_line(&self, irq_line: IrqLine) {
        match irq_line {
            IrqLine::Intc => self.intc_pending.store(true, Ordering::Release),
        }
    }

    pub fn deassert_irq_line(&self, irq_line: IrqLine) {
        match irq_line {
            IrqLine::Intc => self.intc_pending.store(false, Ordering::Release),
        }
    }

    pub fn update_ip_field(&mut self) {
        fn bool_to_flag(value: bool) -> u32 { if value { 1 } else { 0 } };
        let intc_value = bool_to_flag(self.intc_pending.load(Ordering::Acquire));
        self.register.write_bitfield(CAUSE_IP_INTC, intc_value);
    }

    pub fn clear_ip_field(&mut self) {
        self.register.write_bitfield(CAUSE_IP, 0);
    }
}
