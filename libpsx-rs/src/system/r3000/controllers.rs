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
            types::ControllerContext as R3000ControllerContext,
        },
        types::{
            ControllerContext,
            Event,
            State,
        }, bus::controllers::memory::bus_read_u32,
    },
    types::mips1::instruction::Instruction,
};
use log::debug;
use std::{
    intrinsics::unlikely,
    time::Duration,
};

pub fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(duration) => run_time(context.state, context.cdrom_backend, duration),
    }
}

fn run_time(state: &State, cdrom_backend: &CdromBackend, duration: Duration) {
    let r3000_state = &mut state.r3000.controller_state.lock();
    let cp0_state = &mut state.r3000.cp0.controller_state.lock();
    let cp2_state = &mut state.r3000.cp2.controller_state.lock();
    let cdrom_state = &mut state.cdrom.controller_state.lock();

    let mut context = R3000ControllerContext {
        state,
        r3000_state,
        cp0_state,
        cp2_state,
    };
    
    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

    while ticks > 0 {
        let cycles = tick(&mut context);
        for _ in 0..cycles {
            ticks -= 1;

            // Synchronous controllers - timing is way off when done asynchronously, causing problems.
            if ticks % 128 == 0 {
                tick_cdrom(state, cdrom_state, cdrom_backend);
            }
        }
    }
}

fn tick(context: &mut R3000ControllerContext) -> i64 {
    handle_interrupts(context.state, context.r3000_state, context.cp0_state);

    if let Some(target) = context.r3000_state.branch_delay.advance() {
        if translate_address(target) < 0x80 {
            debug!("PC about to jump into invalid memory! Breaking...");
            debug::trace_pc(context.r3000_state, context.cp0_state);
            debug::disassembler::trace_instructions_at_pc(&context.state.memory.main_memory, &context.state.memory.bios, context.r3000_state.pc.read_u32(), Some(50));
            debug::register::trace_registers(context.r3000_state);
            unsafe {
                std::intrinsics::breakpoint();
            }
        }
        context.r3000_state.pc.write_u32(target);
    }

    debug::trace_bios_call(context.r3000_state);
    debug::trace_stdout_putchar(context.r3000_state, context.cp0_state);

    let pc_va = context.r3000_state.pc.read_u32();
    let pc_pa = translate_address(pc_va);

    let inst_value = bus_read_u32(context.state, pc_pa).unwrap();
    let inst = Instruction::new(inst_value);

    context.r3000_state.pc.write_u32(pc_va + INSTRUCTION_SIZE);

    let (fn_ptr, cycles) = instruction_lookup(inst).unwrap_or_else(|| unimplemented!("Unknown R3000 instruction 0x{:08X} (address = 0x{:08X})", inst.value, pc_va));

    debug::trace_state(context.state, context.r3000_state, context.cp0_state);

    let result = fn_ptr(context, inst);

    if unlikely(result.is_err()) {
        // "Pipeline" hazard, go back to previous state, instruction was not performed.
        context.r3000_state.branch_delay.back();
        context.r3000_state.pc.write_u32(pc_va);

        debug::trace_hazard(result.unwrap_err());
    }

    cycles as i64
}
