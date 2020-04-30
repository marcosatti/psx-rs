use crate::{
    system::{
        r3000::{
            constants::INSTRUCTION_SIZE,
            controllers::memory_controller::translate_address,
        },
    },
    types::memory::*,
};
use ansi_term::Colour::Red;
use capstone::{
    prelude::*,
    Endian,
};
use log::trace;

const DEFAULT_TRACE_INSTRUCTIONS_LENGTH: usize = 10;

pub fn trace_instructions_at_pc(main_memory: &B8Memory, bios: &B8Memory, pc: u32, instruction_count: Option<usize>) {
    let pc = translate_address(pc);

    let memory_offset;
    let memory = match pc {
        0..=0x1F_FFFF => {
            memory_offset = pc;
            main_memory
        },
        0x1FC0_0000..=0x1FC7_FFFF => {
            memory_offset = pc - 0x1FC0_0000;
            bios
        },
        _ => panic!("PC = 0x{:08X} is not inside memory", pc),
    };

    let instruction_count = instruction_count.unwrap_or(DEFAULT_TRACE_INSTRUCTIONS_LENGTH);
    let u8_length2 = (instruction_count as u32 / 2) * INSTRUCTION_SIZE;
    let u8_start = memory_offset.checked_sub(u8_length2).unwrap() as usize;
    let u8_end = (memory_offset.checked_add(u8_length2).unwrap() + INSTRUCTION_SIZE) as usize;
    let slice = &memory.read_raw(u8_start as u32)[..u8_end];
    let trace_info = dump_instructions(u8_start as u32, slice, instruction_count);
    trace!("Instruction dump at 0x{:08X}:\n{}", pc, &trace_info);
}

fn dump_instructions(base_address: u32, raw_instructions: &[u8], count: usize) -> String {
    let cs = Capstone::new().mips().mode(arch::mips::ArchMode::Mips32).endian(Endian::Little).detail(true).build().unwrap();

    let instructions = cs.disasm_all(raw_instructions, base_address as u64).unwrap();

    let mut string = String::new();
    for (i, value) in instructions.iter().enumerate() {
        if i == (count / 2) {
            string.push_str(&format!("{}\n", Red.paint(&value.to_string())));
        } else {
            string.push_str(&format!("{}\n", value));
        }
    }

    string
}
