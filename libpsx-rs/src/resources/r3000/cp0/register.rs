use spsc_ringbuffer::SpscRingbuffer;
use crate::types::register::b32_register::B32Register;
use crate::resources::r3000::cp0::*;

#[derive(Copy, Clone, Debug)]
pub enum IrqLine {
    Intc,
}

fn get_irq_line_index(irq_line: IrqLine) -> usize {
    match irq_line {
        IrqLine::Intc => CAUSE_IP_INTC_OFFSET.start,
    }
}

#[derive(Copy, Clone, Debug)]
pub enum IrqMessage {
    Unknown,
    Trigger(IrqLine),
    Reset(IrqLine),
}

impl Default for IrqMessage {
    fn default() -> IrqMessage {
        IrqMessage::Unknown
    }
}

pub struct Cause {
    pub register: B32Register,
    irq_message_queue: SpscRingbuffer<IrqMessage>,
}

impl Cause {
    pub fn new() -> Cause {
        Cause {
            register: B32Register::new(),
            irq_message_queue: SpscRingbuffer::new(16),
        }
    }

    pub fn raise_irq(&self, irq_line: IrqLine) {
        self.irq_message_queue.push(IrqMessage::Trigger(irq_line)).unwrap();
    }

    pub fn reset_irq(&self, irq_line: IrqLine) {
        self.irq_message_queue.push(IrqMessage::Reset(irq_line)).unwrap();
    }

    pub fn handle_irq_messages(&mut self) -> bool {
        if self.irq_message_queue.is_empty() {
            return false;
        }

        loop {
            match self.irq_message_queue.pop() {
                Ok(m) => {
                    match m {
                        IrqMessage::Unknown => unreachable!(),
                        IrqMessage::Trigger(l) => {
                            let bit = CAUSE_IP.start + get_irq_line_index(l);
                            self.register.write_bitfield(Bitfield::new(bit, 1), 1);
                        },
                        IrqMessage::Reset(l) => {
                            let bit = CAUSE_IP.start + get_irq_line_index(l);
                            self.register.write_bitfield(Bitfield::new(bit, 1), 0);
                        },
                    }
                },
                Err(_) => break,
            }
        }

        true
    }
}
