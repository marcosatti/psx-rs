pub mod instruction;
pub mod memory_controller;
pub mod instruction_impl;
pub mod instruction_impl_cop2;
pub mod hazard;
pub mod debug;
pub mod exception;
pub mod register;

use std::time::Duration;
use std::intrinsics::unlikely;
use log::debug;
use crate::backends::cdrom::CdromBackend;
use crate::controllers::ControllerState;
use crate::system::Resources;
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
        Event::Time(duration) => run_time(state.resources, state.cdrom_backend, duration),
    }
}

fn run_time(resources: &mut Resources, cdrom_backend: &CdromBackend, duration: Duration) {
    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    
    while ticks > 0 {
        let cycles = tick(resources); 
        for _ in 0..cycles {
            ticks -= 1;
        
            // Synchronous controllers - timing is way off when done asynchronously, causing problems.
            if ticks % 128 == 0 {
                tick_cdrom(resources, cdrom_backend);
            }
        } 
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
    
    debug::trace_bios_call(resources);
    debug::trace_stdout_putchar(resources);

    let pc_va = resources.r3000.pc.read_u32();
    let pc_pa = translate_address(pc_va);

    let inst_value = resources.r3000.memory_mapper.read_u32(pc_pa).unwrap();   
    let inst = Instruction::new(inst_value);                

    resources.r3000.pc.write_u32(pc_va + INSTRUCTION_SIZE);

    let (fn_ptr, cycles) = instruction_lookup(inst).unwrap_or_else(|| unimplemented!("Unknown R3000 instruction 0x{:08X} (address = 0x{:08X})", inst.value, pc_va));

    debug::trace_state(resources);

    let result = fn_ptr(resources, inst);

    if unlikely(result.is_err()) {
        // "Pipeline" hazard, go back to previous state, instruction was not performed.
        resources.r3000.branch_delay.back();
        resources.r3000.pc.write_u32(pc_va);

        debug::trace_hazard(result.unwrap_err());
    }
    
    cycles as i64
} 
