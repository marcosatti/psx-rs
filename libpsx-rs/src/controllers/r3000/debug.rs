pub mod memory;
pub mod disassembler;
pub mod register;

use std::fmt::UpperHex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::ffi::CStr;
use log::trace;
use log::debug;
use log::warn;
use crate::constants::r3000::INSTRUCTION_SIZE;
use crate::controllers::r3000::hazard::*;
use crate::controllers::r3000::debug::disassembler::*;
use crate::controllers::r3000::debug::register::*;
use crate::controllers::r3000::memory_controller::translate_address;
use crate::resources::Resources;
use crate::resources::r3000::cp0::*;
use crate::debug::DEBUG_CORE_EXIT;

pub static ENABLE_STATE_TRACING: AtomicBool = AtomicBool::new(true);
const ENABLE_DETECT_SYSTEMERROR: bool = true;
const ENABLE_PRINTF_TRACE: bool = true;
const ENABLE_HAZARD_TRACING: bool = true;
const ENABLE_INTERRUPT_TRACING: bool = false;
const ENABLE_SYSCALL_TRACING: bool = false;
const ENABLE_RFE_TRACING: bool = false;
const ENABLE_MEMORY_TRACKING_READ: bool = false;
const ENABLE_MEMORY_TRACKING_WRITE: bool = false;
const ENABLE_MEMORY_SPIN_LOOP_DETECTION_READ: bool = false;
const ENABLE_MEMORY_SPIN_LOOP_DETECTION_WRITE: bool = false;

const MEMORY_TRACKING_ADDRESS_RANGE_START: u32 = 0xe288; //0x79ddc; //0x1F80_1040;
const MEMORY_TRACKING_ADDRESS_RANGE_END: u32 = 0xe28c; //0x79ddf; //0x1F80_1050; 
const MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD: usize = 16;

pub static mut DEBUG_TICK_COUNT: usize = 0;

pub fn trace_state(resources: &Resources) {
    unsafe { 
        DEBUG_TICK_COUNT += 1; 

        if !ENABLE_STATE_TRACING.load(Ordering::Acquire) {
            return;
        }
    
        let tick_count = DEBUG_TICK_COUNT;
        let pc_va = resources.r3000.pc.read_u32() - INSTRUCTION_SIZE;
    
        let start = 0xF00000125DB53;
        let end = 0xF00000125DB57;
        if (start..=end).contains(&tick_count) {
            let iec = resources.r3000.cp0.status.read_bitfield(STATUS_IEC) != 0;
            let branching = resources.r3000.branch_delay.branching();
            debug!("[{:X}] iec = {}, pc = 0x{:0X}, b = {}", tick_count, iec, pc_va, branching);
            trace_instructions_at_pc(resources, Some(1));
            //trace_registers(resources);
        }

        if false {
            DEBUG_CORE_EXIT.store(true, Ordering::Release);
        }
    }
}

pub fn trace_pc(resources: &Resources) {
    let pc = resources.r3000.pc.read_u32();
    let kuc = resources.r3000.cp0.status.read_bitfield(STATUS_KUC);
    let iec = resources.r3000.cp0.status.read_bitfield(STATUS_IEC);
    let tick_count = unsafe { DEBUG_TICK_COUNT };
    trace!("[{:X}] R3000 pc = 0x{:0X}, kuc = {}, iec = {}", tick_count, pc, kuc, iec);
}

pub fn trace_hazard(hazard: Hazard) {
    if ENABLE_HAZARD_TRACING {
        match hazard {
            Hazard::MemoryRead(_) | Hazard::MemoryWrite(_) => warn!("R3000 memory hazard: {}", hazard),
            Hazard::BusLockedMemoryRead(_) | Hazard::BusLockedMemoryWrite(_) => {
                // Bus locking is normal and expected occasionally.
            },
        }
    }
}

pub fn trace_interrupt(resources: &Resources) {
    if ENABLE_INTERRUPT_TRACING {
        let debug_tick_count = unsafe { DEBUG_TICK_COUNT };
        let pc_va = resources.r3000.pc.read_u32();
        let branching = resources.r3000.branch_delay.branching();
        trace!("R3000 interrupt, cycle = 0x{:X}, pc = 0x{:0X}, branching = {}", debug_tick_count, pc_va, branching);
        crate::controllers::intc::debug::trace_intc(resources, true);
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

pub fn track_memory_read_pending<T>(resources: &Resources, physical_address: u32) {
    if !ENABLE_MEMORY_TRACKING_READ {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    if true {
        let tick_count = unsafe { DEBUG_TICK_COUNT };
        let type_name = core::any::type_name::<T>();
        let pc = resources.r3000.pc.read_u32();
        debug!("[{:X}] Read PC = 0x{:08X} {} address = 0x{:08X} start", tick_count, pc, type_name, physical_address);
    }
}

pub fn track_memory_read<T: Copy + UpperHex>(resources: &Resources, physical_address: u32, value: T) {
    if !ENABLE_MEMORY_TRACKING_READ {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    let count = memory::update_state_read(physical_address);

    if true {
        let tick_count = unsafe { DEBUG_TICK_COUNT };
        let type_name = core::any::type_name::<T>();
        let pc = resources.r3000.pc.read_u32();
        debug!("[{:X}] Read PC = 0x{:08X} {} address = 0x{:08X}, value = 0x{:X} end", tick_count, pc, type_name, physical_address, value);
    }

    trace_memory_spin_loop_detection_read(resources, physical_address, count);
}

pub fn track_memory_write_pending<T: Copy + UpperHex>(resources: &Resources, physical_address: u32, value: T) {
    if !ENABLE_MEMORY_TRACKING_WRITE {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    if true {
        let tick_count = unsafe { DEBUG_TICK_COUNT };
        let type_name = core::any::type_name::<T>();
        let pc = resources.r3000.pc.read_u32();
        debug!("[{:X}] Write PC = 0x{:08X} {} address = 0x{:08X}, value = 0x{:X} start", tick_count, pc, type_name, physical_address, value);
    }
}

pub fn track_memory_write<T: Copy + UpperHex>(resources: &Resources, physical_address: u32, value: T) {
    if !ENABLE_MEMORY_TRACKING_WRITE {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    let count = memory::update_state_write(physical_address);

    if true {
        let tick_count = unsafe { DEBUG_TICK_COUNT };
        let type_name = core::any::type_name::<T>();
        let pc = resources.r3000.pc.read_u32();
        debug!("[{:X}] Write PC = 0x{:08X} {} address = 0x{:08X}, value = 0x{:X} end", tick_count, pc, type_name, physical_address, value);
    }

    trace_memory_spin_loop_detection_write(resources, physical_address, count);
}

fn trace_memory_spin_loop_detection_read(resources: &Resources, physical_address: u32, count: usize) {
    if !ENABLE_MEMORY_SPIN_LOOP_DETECTION_READ {
        return;
    }

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        trace!("Memory read spin loop detected on address = 0x{:08X}", physical_address);
        trace_instructions_at_pc(resources, None);
        trace_registers(resources);
        memory::clear_state_read(physical_address);
    }
} 

fn trace_memory_spin_loop_detection_write(resources: &Resources, physical_address: u32, count: usize) {
    if !ENABLE_MEMORY_SPIN_LOOP_DETECTION_WRITE {
        return;
    }

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        trace!("Memory write spin loop detected on address = 0x{:08X}", physical_address);
        trace_instructions_at_pc(resources, None);
        trace_registers(resources);
        memory::clear_state_write(physical_address);
    }
}

pub fn trace_printf(resources: &Resources) {
    // BIOS call 0xA0, $t1 = 0x3F.
    if !ENABLE_PRINTF_TRACE {
        return;
    }
    
    let mut pc = resources.r3000.pc.read_u32();
    pc = translate_address(pc);
    let t1 = resources.r3000.gpr[9].read_u32();

    if (pc == 0xA0) && (t1 == 0x3F) {
        unsafe {
            let mut fmt_string_ptr = resources.r3000.gpr[4].read_u32();
            fmt_string_ptr = translate_address(fmt_string_ptr);

            let a1 = resources.r3000.gpr[5].read_u32();
            let a2 = resources.r3000.gpr[6].read_u32();
            let a3 = resources.r3000.gpr[7].read_u32();

            let memory_offset;
            let memory = match fmt_string_ptr {
                0..=0x1F_FFFF => {
                    memory_offset = fmt_string_ptr;
                    &resources.main_memory.memory
                },
                0x1FC0_0000..=0x1FC7_FFFF => {
                    memory_offset = fmt_string_ptr - 0x1FC0_0000;
                    &resources.bios.memory
                },
                _ => panic!("fmt_string_ptr = 0x{:08X} is not inside memory", fmt_string_ptr)
            };
    
            let ptr = &memory[memory_offset as usize] as *const u8 as *const i8;
            let string = CStr::from_ptr(ptr).to_string_lossy().to_owned();
            let string_trimmed = string.trim();
            let tick_count = DEBUG_TICK_COUNT;
            let iec = resources.r3000.cp0.status.read_bitfield(STATUS_IEC) != 0;
            trace!("[{:X}] printf call: iec: {}, fmt: {}, a1 = 0x{:X}, a2 = 0x{:X}, a3 = 0x{:X}", tick_count, iec, string_trimmed, a1, a2, a3);       
        }
    }
}

pub fn detect_systemerror(resources: &Resources) {
    // BIOS call 0xC0, $t1 = 0x0B.
    if !ENABLE_DETECT_SYSTEMERROR {
        return;
    }
    
    let mut pc = resources.r3000.pc.read_u32();
    pc = translate_address(pc);
    let t1 = resources.r3000.gpr[9].read_u32();

    if (pc == 0xC0) && (t1 == 0x0B) {
        panic!("BIOS SystemError C0(0x0B) call detected");
    }
}
