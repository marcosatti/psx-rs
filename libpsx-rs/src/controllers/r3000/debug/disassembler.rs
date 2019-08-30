use capstone::prelude::*;
use capstone::Endian;
use log::trace;
use ansi_term::Colour::Red;
use crate::State;
use crate::constants::r3000::INSTRUCTION_SIZE;
use crate::controllers::r3000::memory_controller::translate_address;

static DEFAULT_TRACE_INSTRUCTIONS_LENGTH: usize = 10;

pub unsafe fn trace_instructions_at_pc(state: &State, instruction_count: Option<usize>) {
    let resources = &mut *state.resources;
    let main_memory = &resources.main_memory;

    let pc = translate_address(resources.r3000.pc.read_u32() - INSTRUCTION_SIZE);
    if (pc as usize) >= main_memory.memory.len() {
        panic!("PC = 0x{:08X} is not inside main memory", pc);
    }

    let instruction_count = instruction_count.unwrap_or(DEFAULT_TRACE_INSTRUCTIONS_LENGTH);
    let u8_length2 = (instruction_count as u32 / 2) * INSTRUCTION_SIZE;
    let u8_start = pc.checked_sub(u8_length2).unwrap() as usize;
    let u8_end = pc.checked_add(u8_length2).unwrap() as usize;
    let slice = &main_memory.memory[u8_start..u8_end];
    let trace_info = dump_instructions(u8_start as u32, slice, instruction_count);
    trace!("Instruction dump at 0x{:08X}:\n{}", pc, &trace_info);
}

fn dump_instructions(base_address: u32, raw_instructions: &[u8], count: usize) -> String {
    let cs = Capstone::new()
        .mips()
        .mode(arch::mips::ArchMode::Mips32)
        .endian(Endian::Little)
        .detail(true)
        .build()
        .unwrap();

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
