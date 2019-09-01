pub mod memory;
pub mod disassembler;
pub mod register;

use log::trace;
use log::debug;
use log::warn;
use crate::State;
use crate::constants::r3000::INSTRUCTION_SIZE;
use crate::controllers::r3000::Hazard;
use crate::resources::r3000::cp0::STATUS_IEC;
use crate::debug::DEBUG_CORE_EXIT;
use crate::debug::trace_intc;
use crate::controllers::r3000::debug::disassembler::*;
use crate::controllers::r3000::debug::memory::*;
use crate::controllers::r3000::debug::register::*;

const ENABLE_STATE_TRACING: bool = false;
const ENABLE_HAZARD_TRACING: bool = true;
const ENABLE_INTERRUPT_TRACING: bool = false;
const ENABLE_SYSCALL_TRACING: bool = false;
const ENABLE_RFE_TRACING: bool = false;
const ENABLE_IO_SPIN_LOOP_DETECTION_READ: bool = false;
const ENABLE_IO_SPIN_LOOP_DETECTION_WRITE: bool = false;

static mut DEBUG_TICK_COUNT: usize = 0;

const MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD: usize = 1024 * 16;

pub unsafe fn trace_state(state: &State) {
    if !ENABLE_STATE_TRACING {
        return;
    }

    let resources = &mut *state.resources;

    let pc_va = resources.r3000.pc.read_u32() - INSTRUCTION_SIZE;

    if DEBUG_TICK_COUNT >= 0x10001592686D || pc_va == 0x800513FC {
        let iec = resources.r3000.cp0.status.read_bitfield(STATUS_IEC) != 0;
        let branching = resources.r3000.branch_delay.branching();
        debug!("[{:X}] iec = {}, pc = 0x{:0X}, b = {}", DEBUG_TICK_COUNT, iec, pc_va, branching);
        trace_instructions_at_pc(state, None);
        trace_registers(state);
    }

    if false {
        DEBUG_CORE_EXIT = true;
    }

    DEBUG_TICK_COUNT += 1;
}

pub unsafe fn trace_hazard(hazard: Hazard) {
    if ENABLE_HAZARD_TRACING {
        match hazard {
            Hazard::MemoryRead(address) | Hazard::MemoryWrite(address) => warn!("R3000 Hazard {} at address 0x{:08X}", hazard, address),
            // Bus locking is normal and expected occasionally.
            _ => {},
        }
    }
}

pub unsafe fn trace_interrupt(state: &State) {
    if ENABLE_INTERRUPT_TRACING {
        let resources = &mut *state.resources;
        let pc_va = resources.r3000.pc.read_u32();
        debug!("[{:X}] interrupt, pc = 0x{:0X}", DEBUG_TICK_COUNT, pc_va);
        trace_intc(resources, true);
    }
}

pub unsafe fn trace_syscall(state: &State) {
    if ENABLE_SYSCALL_TRACING {
        let resources = &mut *state.resources;
        let pc_va = resources.r3000.pc.read_u32();
        debug!("[{:X}] syscall, pc = 0x{:X}", DEBUG_TICK_COUNT, pc_va);
    }
}

pub unsafe fn trace_rfe(state: &State) {
    if ENABLE_RFE_TRACING {
        let resources = &mut *state.resources;
        let pc_va = resources.r3000.pc.read_u32();
        debug!("[{:X}] rfe, pc = 0x{:X}", DEBUG_TICK_COUNT, pc_va);
    }
}

pub unsafe fn trace_io_spin_loop_detection_read(state: &State, physical_address: u32) {
    if !ENABLE_IO_SPIN_LOOP_DETECTION_READ {
        return;
    }

    if physical_address < 0x1F00_0000 {
        return;
    }

    let count = track_memory_read(physical_address);

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        trace!("Spin loop detected on I/O address = 0x{:08X} (read)", physical_address);
        trace_instructions_at_pc(state, None);
        trace_registers(state);
        track_memory_read_clear(physical_address);
    }
} 

pub unsafe fn trace_io_spin_loop_detection_write(state: &State, physical_address: u32) {
    if !ENABLE_IO_SPIN_LOOP_DETECTION_WRITE {
        return;
    }

    if physical_address < 0x1F00_0000 {
        return;
    }

    let count = track_memory_write(physical_address);

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        trace!("Spin loop detected on I/O address = 0x{:08X} (write)", physical_address);
        trace_instructions_at_pc(state, None);
        trace_registers(state);
        track_memory_write_clear(physical_address);
    }
}
