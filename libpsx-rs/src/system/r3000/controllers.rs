pub(crate) mod debug;
pub(crate) mod exception;
pub(crate) mod instruction;
pub(crate) mod instruction_impl;
pub(crate) mod instruction_impl_cop2;
pub(crate) mod memory_controller;
pub(crate) mod register;

use crate::{
    system::{
        bus::memory::bus_read_u32,
        r3000::{
            constants::*,
            controllers::{
                exception::*,
                instruction::lookup as instruction_lookup,
                memory_controller::translate_address,
            },
            types::ControllerContext as R3000ControllerContext,
        },
        types::{
            ControllerContext,
            ControllerResult,
            Event,
            State,
        },
    },
    types::mips1::instruction::Instruction,
};
use log::debug;
use std::intrinsics::unlikely;

pub(crate) fn run(context: &ControllerContext, event: Event) -> ControllerResult<()> {
    match event {
        Event::Time(duration) => run_time(context.state, duration),
    }
}

fn run_time(state: &State, duration: f64) -> ControllerResult<()> {
    let r3000_state = &mut state.r3000.controller_state.lock();
    r3000_state.clock += duration;

    let cp0_state = &mut state.r3000.cp0.controller_state.lock();
    let cp2_state = &mut state.r3000.cp2.controller_state.lock();

    while r3000_state.clock > 0.0 {
        let ticks = {
            let mut context = R3000ControllerContext {
                state,
                r3000_state,
                cp0_state,
                cp2_state,
            };
            tick(&mut context)? as f64
        };

        r3000_state.clock -= CLOCK_SPEED_PERIOD * ticks;
    }

    Ok(())
}

fn tick(context: &mut R3000ControllerContext) -> ControllerResult<usize> {
    handle_interrupts(context.state, context.r3000_state, context.cp0_state);

    if let Some(target) = context.r3000_state.branch_delay.advance() {
        context.r3000_state.pc.write_u32(target);
    }

    debug::trace_bios_call(context.r3000_state)?;
    debug::trace_stdout_putchar(context.r3000_state, context.cp0_state);

    let pc_va = context.r3000_state.pc.read_u32();
    let pc_pa = translate_address(pc_va);

    if pc_pa < 0x80 {
        return Err(format!("PC is in invalid region (likely): pc_pa = 0x{:08X}", pc_pa));
    }

    let inst_value = bus_read_u32(context.state, pc_pa).map_err(|e| format!("Error reading instruction from memory: {:?}", e))?;
    let inst = Instruction::new(inst_value);

    context.r3000_state.pc.write_u32(pc_va + INSTRUCTION_SIZE);

    let (fn_ptr, cycles) = instruction_lookup(inst);
    let result = fn_ptr(context, inst);
    debug::trace_hazard(result);

    if unlikely(result.is_err()) {
        // "Pipeline" hazard, go back to previous state, instruction was not performed.
        context.r3000_state.branch_delay.back();
        context.r3000_state.pc.write_u32(pc_va);
    }

    debug::update_state();

    Ok(cycles)
}
