pub mod debug;
pub mod exception;
pub mod instruction;
pub mod instruction_impl;
pub mod instruction_impl_cop2;
pub mod memory_controller;
pub mod register;

use crate::{
    backends::cdrom::CdromBackend,
    system::{
        cdrom::controllers::handle_tick as tick_cdrom,
        r3000::{
            constants::{
                CLOCK_SPEED,
                INSTRUCTION_SIZE,
            },
            controllers::{
                exception::*,
                instruction::lookup as instruction_lookup,
                memory_controller::translate_address,
            },
        },
        types::{
            ControllerContext,
            Event,
            State,
        },
    },
    types::mips1::instruction::Instruction,
};
use log::debug;
use std::{
    intrinsics::unlikely,
    time::Duration,
};

pub fn run(context: &mut ControllerContext, event: Event) {
    match event {
        Event::Time(duration) => run_time(context.state, context.cdrom_backend, duration),
    }
}

fn run_time(state: &mut State, cdrom_backend: &CdromBackend, duration: Duration) {
    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

    while ticks > 0 {
        let cycles = tick(state);
        for _ in 0..cycles {
            ticks -= 1;

            // Synchronous controllers - timing is way off when done asynchronously, causing problems.
            if ticks % 128 == 0 {
                tick_cdrom(state, cdrom_backend);
            }
        }
    }
}

fn tick(state: &mut State) -> i64 {
    handle_interrupts(state);

    if let Some(target) = state.r3000.branch_delay.advance() {
        if translate_address(target) < 0x80 {
            debug!("PC about to jump into invalid memory! Breaking...");
            debug::trace_pc(state);
            debug::disassembler::trace_instructions_at_pc(state, Some(50));
            debug::register::trace_registers(state);
            unsafe {
                std::intrinsics::breakpoint();
            }
        }
        state.r3000.pc.write_u32(target);
    }

    debug::trace_bios_call(state);
    debug::trace_stdout_putchar(state);

    let pc_va = state.r3000.pc.read_u32();
    let pc_pa = translate_address(pc_va);

    let inst_value = state.r3000.memory_mapper.read_u32(pc_pa).unwrap();
    let inst = Instruction::new(inst_value);

    state.r3000.pc.write_u32(pc_va + INSTRUCTION_SIZE);

    let (fn_ptr, cycles) = instruction_lookup(inst).unwrap_or_else(|| {
        unimplemented!("Unknown R3000 instruction 0x{:08X} (address = 0x{:08X})", inst.value, pc_va)
    });

    debug::trace_state(state);

    let result = fn_ptr(state, inst);

    if unlikely(result.is_err()) {
        // "Pipeline" hazard, go back to previous state, instruction was not performed.
        state.r3000.branch_delay.back();
        state.r3000.pc.write_u32(pc_va);

        debug::trace_hazard(result.unwrap_err());
    }

    cycles as i64
}
