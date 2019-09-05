pub mod memory;
pub mod disassembler;
pub mod register;

use log::trace;
use log::debug;
use log::warn;
use crate::constants::r3000::INSTRUCTION_SIZE;
use crate::controllers::r3000::hazard::*;
use crate::controllers::r3000::debug::disassembler::*;
use crate::controllers::r3000::debug::memory::*;
use crate::controllers::r3000::debug::register::*;
use crate::resources::Resources;
use crate::resources::r3000::cp0::*;
use crate::debug::trace_intc;
use crate::debug::DEBUG_CORE_EXIT;

const ENABLE_STATE_TRACING: bool = false;
const ENABLE_HAZARD_TRACING: bool = true;
const ENABLE_INTERRUPT_TRACING: bool = false;
const ENABLE_SYSCALL_TRACING: bool = false;
const ENABLE_RFE_TRACING: bool = false;
const ENABLE_IO_SPIN_LOOP_DETECTION_READ: bool = true;
const ENABLE_IO_SPIN_LOOP_DETECTION_WRITE: bool = false;

static mut DEBUG_TICK_COUNT: usize = 0;

const MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD: usize = 16;
const MEMORY_SPIN_LOOP_DETECTION_ADDRESS_START: u32 = 0x1F80_1800;
const MEMORY_SPIN_LOOP_DETECTION_ADDRESS_END: u32 = 0x1F80_1810; //0x1FBF_FFFF;

pub fn trace_state(resources: &Resources) {
    unsafe {
        DEBUG_TICK_COUNT += 1;

        if !ENABLE_STATE_TRACING {
            return;
        }

        let pc_va = resources.r3000.pc.read_u32() - INSTRUCTION_SIZE;

        if DEBUG_TICK_COUNT >= 0x10001592686D || pc_va == 0x800513FC {
            let iec = resources.r3000.cp0.status.read_bitfield(STATUS_IEC) != 0;
            let branching = resources.r3000.branch_delay.branching();
            debug!("[{:X}] iec = {}, pc = 0x{:0X}, b = {}", DEBUG_TICK_COUNT, iec, pc_va, branching);
            trace_instructions_at_pc(resources, None);
            trace_registers(resources);
        }

        if false {
            DEBUG_CORE_EXIT = true;
        }
    }
}

pub fn trace_pc(resources: &Resources) {
    let pc = resources.r3000.pc.read_u32();
    let kuc = resources.r3000.cp0.status.read_bitfield(STATUS_KUC);
    let iec = resources.r3000.cp0.status.read_bitfield(STATUS_IEC);
    trace!("R3000 pc = 0x{:0X}, kuc = {}, iec = {}", pc, kuc, iec);
}

pub fn trace_hazard(hazard: Hazard) {
    if ENABLE_HAZARD_TRACING {
        match hazard {
            Hazard::MemoryRead(address) | Hazard::MemoryWrite(address) => warn!("R3000 Hazard {} at address 0x{:08X}", hazard, address),
            // Bus locking is normal and expected occasionally.
            _ => {},
        }
    }
}

pub fn trace_interrupt(resources: &Resources) {
    if ENABLE_INTERRUPT_TRACING {
        let debug_tick_count = unsafe { DEBUG_TICK_COUNT };
        let pc_va = resources.r3000.pc.read_u32();
        trace!("[{:X}] interrupt, pc = 0x{:0X}", debug_tick_count, pc_va);
        trace_intc(resources, true);
    }
}

pub fn trace_syscall(resources: &Resources) {
    if ENABLE_SYSCALL_TRACING {
        let debug_tick_count = unsafe { DEBUG_TICK_COUNT };
        let pc_va = resources.r3000.pc.read_u32();
        trace!("[{:X}] syscall, pc = 0x{:X}", debug_tick_count, pc_va);
    }
}

pub fn trace_rfe(resources: &Resources) {
    if ENABLE_RFE_TRACING {
        let debug_tick_count = unsafe { DEBUG_TICK_COUNT };
        let pc_va = resources.r3000.pc.read_u32();
        trace!("[{:X}] rfe, pc = 0x{:X}", debug_tick_count, pc_va);
    }
}

pub fn trace_io_spin_loop_detection_read(resources: &Resources, physical_address: u32) {
    if !ENABLE_IO_SPIN_LOOP_DETECTION_READ {
        return;
    }

    if !(physical_address >= MEMORY_SPIN_LOOP_DETECTION_ADDRESS_START && physical_address < MEMORY_SPIN_LOOP_DETECTION_ADDRESS_END) {
        return;
    }

    let count = track_memory_read(physical_address);

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        trace!("Spin loop detected on I/O address = 0x{:08X} (read)", physical_address);
        trace_instructions_at_pc(resources, None);
        trace_registers(resources);
        track_memory_read_clear(physical_address);
    }
} 

pub fn trace_io_spin_loop_detection_write(resources: &Resources, physical_address: u32) {
    if !ENABLE_IO_SPIN_LOOP_DETECTION_WRITE {
        return;
    }

    if !(physical_address >= MEMORY_SPIN_LOOP_DETECTION_ADDRESS_START && physical_address <= MEMORY_SPIN_LOOP_DETECTION_ADDRESS_END) {
        return;
    }
    
    let count = track_memory_write(physical_address);

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        trace!("Spin loop detected on I/O address = 0x{:08X} (write)", physical_address);
        trace_instructions_at_pc(resources, None);
        trace_registers(resources);
        track_memory_write_clear(physical_address);
    }
}
