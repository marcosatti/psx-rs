mod instruction;
mod memory_controller;
mod instruction_impl;

use std::time::Duration;
use log::debug;
use crate::State;
use crate::constants::r3000::{CLOCK_SPEED, INSTRUCTION_SIZE};
use crate::controllers::Event;
use crate::controllers::r3000::memory_controller::translate_address;
use crate::controllers::r3000::instruction::lookup as instruction_lookup;
use crate::types::mips1::instruction::Instruction;
use crate::utilities::mips1::status_push_exception;
use crate::resources::r3000::cp0::{STATUS_BEV, STATUS_IM, CAUSE_IP, CAUSE_BD, STATUS_IEC, CAUSE_EXCCODE, CAUSE_EXCCODE_INT, CAUSE_EXCCODE_SYSCALL};
use crate::debug::{DEBUG_CORE_EXIT, trace_intc};

pub enum Hazard {
    MemoryRead,
    MemoryWrite
}

pub type InstResult = Result<(), Hazard>;

static mut ENABLE_DEBUG: bool = false;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(duration) => run_time(state, duration),
    }
}

fn run_time(state: &State, duration: Duration) {
    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    while ticks > 0 {
        ticks -= unsafe { tick(state) };
    }
}

pub static mut DEBUG_TICK_COUNT: usize = 0;
pub static mut DEBUG_BREAK_REACHED: bool = false;
pub static mut DEBUG_LOG_INSTRUCTION: bool = false;

unsafe fn tick(state: &State) -> i64 {
    let resources = &mut *state.resources;

    handle_interrupts(state);

    if let Some(target) = resources.r3000.branch_delay.advance() {
        resources.r3000.pc.write_u32(target);
    }

    let pc_va = resources.r3000.pc.read_u32();
    let pc_pa = translate_address(pc_va);

    let inst_value = resources.r3000.memory_mapper.read_u32(pc_pa).unwrap();   
    let inst = Instruction::new(inst_value);                

    resources.r3000.pc.write_u32(pc_va + INSTRUCTION_SIZE);

    let (fn_ptr, mut mnemonic, cycles) = instruction_lookup(inst).unwrap();

    if ENABLE_DEBUG {
        if pc_va == 0x800513FC && !DEBUG_BREAK_REACHED {
            debug!("Break");
            DEBUG_BREAK_REACHED = true;
        }

        if DEBUG_TICK_COUNT >= 0x1592686D {
            debug!("Exiting");
            DEBUG_CORE_EXIT = true;
        }

        if DEBUG_TICK_COUNT >= 0x10001592686D || DEBUG_LOG_INSTRUCTION {
            let iec = resources.r3000.cp0.status.read_bitfield(STATUS_IEC) != 0;
            let branching = resources.r3000.branch_delay.branching();
            if inst_value == 0 { mnemonic = "nop"; }
            debug!("[{:X}] iec = {}, pc = 0x{:0X}, inst = {}, b = {}", DEBUG_TICK_COUNT, iec, pc_va, mnemonic, branching);
        }
    }

    let result = fn_ptr(state, inst);

    if result.is_err() {
        // Pipeline hazard, go back to previous state, instruction was not performed.
        resources.r3000.pc.write_u32(pc_va);
    }
    
    DEBUG_TICK_COUNT += 1;

    cycles as i64
} 

unsafe fn set_exception(state: &State, exccode: usize) {
    let resources = &mut *state.resources;
    let pc = &mut resources.r3000.pc;
    let cause = &mut resources.r3000.cp0.cause;
    let status = &mut resources.r3000.cp0.status;
    let mut pc_value = pc.read_u32();

    let _cp0_lock = resources.r3000.cp0.mutex.lock();

    if resources.r3000.branch_delay.branching() {
        cause.write_bitfield(CAUSE_BD, 1);
        pc_value -= INSTRUCTION_SIZE;
    }

    // Push IEc & KUc (stack).
    let status_value = status_push_exception(status.read_u32());
    status.write_u32(status_value);

    // Set ExcCode cause.
    cause.write_bitfield(CAUSE_EXCCODE, exccode as u32);

    // Set EPC address.
    let epc = &mut resources.r3000.cp0.epc;
    epc.write_u32(pc_value);
    
    // Figure out base exception vector address.
    let bev = status.read_bitfield(STATUS_BEV) != 0;
    let mut vector_offset = if bev {
        0xBF80_0100
    } else {
        0x8000_0000
    };

    // Figure out exception vector offset.
    match exccode {
        CAUSE_EXCCODE_INT | CAUSE_EXCCODE_SYSCALL => {
            // General exception vector.
            vector_offset += 0x80;
        },
        _ => {
            unimplemented!("Unimplemented exception type encountered")
        },
    }

    // Set PC to exception vector.
    pc.write_u32(vector_offset);
}

unsafe fn handle_interrupts(state: &State) {
    let resources = &mut *state.resources;
    
    if resources.r3000.cp0.status.read_bitfield(STATUS_IEC) == 0 {
        return;
    }

    let im = resources.r3000.cp0.status.read_bitfield(STATUS_IM);
    let ip = resources.r3000.cp0.cause.read_bitfield(CAUSE_IP);
    if (im & ip) > 0 {
        debug!("[{:X}] interrupt, pc = 0x{:0X}", DEBUG_TICK_COUNT, resources.r3000.pc.read_u32());
        trace_intc(resources, true);
        set_exception(state, CAUSE_EXCCODE_INT);
    }
}
