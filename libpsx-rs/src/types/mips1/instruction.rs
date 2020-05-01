use std::fmt;

#[derive(Copy, Clone)]
pub struct Instruction {
    pub value: u32,
}

impl Instruction {
    pub fn new(value: u32) -> Instruction {
        Instruction {
            value,
        }
    }

    pub fn opcode(&self) -> usize {
        ((self.value >> 26) & 0x3F) as usize
    }

    pub fn c(&self) -> u8 {
        ((self.value >> 25) & 0x1) as u8
    }

    pub fn rs(&self) -> usize {
        ((self.value >> 21) & 0x1F) as usize
    }

    pub fn rs4(&self) -> usize {
        ((self.value >> 21) & 0xF) as usize
    }

    pub fn rt(&self) -> usize {
        ((self.value >> 16) & 0x1F) as usize
    }

    pub fn rd(&self) -> usize {
        ((self.value >> 11) & 0x1F) as usize
    }

    pub fn shamt(&self) -> usize {
        ((self.value >> 6) & 0x1F) as usize
    }

    pub fn funct(&self) -> usize {
        (self.value & 0x3F) as usize
    }

    pub fn i_imm(&self) -> i16 {
        (self.value & 0xFFFF) as i16
    }

    pub fn u_imm(&self) -> u16 {
        (self.value & 0xFFFF) as u16
    }

    pub fn addr(&self) -> u32 {
        (self.value & 0x3FF_FFFF) as u32
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("value", &format!("0x{:08X}", self.value))
            .field("opcode", &self.opcode())
            .field("rs", &self.rs())
            .field("rt", &self.rt())
            .field("rd", &self.rd())
            .field("shamt", &self.shamt())
            .field("funct", &self.funct())
            .field("uimm", &format!("0x{:04X}", self.u_imm()))
            .field("addr (shifted)", &format!("0x{:08X}", self.addr() << 2))
            .finish()
    }
}
