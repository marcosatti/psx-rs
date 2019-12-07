pub mod instruction;
pub mod memory_controller;
pub mod instruction_impl;
pub mod instruction_impl_cop2;
pub mod hazard;
pub mod debug;
pub mod exception;

use std::time::Duration;
use log::debug;
use crate::controllers::ControllerState;
use crate::resources::Resources;
use crate::constants::r3000::{CLOCK_SPEED, INSTRUCTION_SIZE};
use crate::controllers::Event;
use crate::controllers::r3000::hazard::*;
use crate::controllers::r3000::exception::*;
use crate::controllers::cdrom::handle_tick as tick_cdrom;
use crate::controllers::r3000::memory_controller::translate_address;
use crate::controllers::r3000::instruction::lookup as instruction_lookup;
use crate::types::mips1::instruction::Instruction;

pub type InstResult = Result<(), Hazard>;

pub fn run(state: &mut ControllerState, event: Event) {
    match event {
        Event::Time(duration) => run_time(state.resources, duration),
    }
}

fn run_time(resources: &mut Resources, duration: Duration) {
    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    
    while ticks > 0 {
        ticks -= tick(resources); 
        // Synchronous controllers - timing is way off when done asynchronously, causing problems.
        tick_cdrom(resources);
    }
}

fn tick(resources: &mut Resources) -> i64 {
    handle_interrupts(resources);

    if let Some(target) = resources.r3000.branch_delay.advance() {
        if translate_address(target) < 0x80 {
            debug!("PC about to jump into invalid memory! Breaking...");
            debug::trace_pc(resources);
            debug::disassembler::trace_instructions_at_pc(resources, Some(50));
            debug::register::trace_registers(resources);
            unsafe { std::intrinsics::breakpoint(); }
        }
        resources.r3000.pc.write_u32(target);
    }

    let pc_va = resources.r3000.pc.read_u32();
    let pc_pa = translate_address(pc_va);

    let inst_value = resources.r3000.memory_mapper.read_u32(pc_pa).unwrap();   
    let inst = Instruction::new(inst_value);                

    resources.r3000.pc.write_u32(pc_va + INSTRUCTION_SIZE);

    let (fn_ptr, cycles) = instruction_lookup(inst)
        .ok_or_else(|| format!("Unknown R3000 instruction 0x{:08X} (address = 0x{:08X})", inst.value, pc_pa))
        .unwrap();

    debug::trace_state(resources);

    let result = fn_ptr(resources, inst);

    if result.is_err() {
        // "Pipeline" hazard, go back to previous state, instruction was not performed.
        resources.r3000.branch_delay.back();
        resources.r3000.pc.write_u32(pc_va);

        debug::trace_hazard(result.unwrap_err());
    }
    
    cycles as i64
} 
