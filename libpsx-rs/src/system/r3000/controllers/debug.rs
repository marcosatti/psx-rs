pub mod memory;
pub mod disassembler;
pub mod register;

use std::fmt::UpperHex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::cmp::max;
use log::trace;
use log::debug;
use log::warn;
use crate::constants::r3000::INSTRUCTION_SIZE;
use crate::controllers::r3000::hazard::*;
use crate::controllers::r3000::debug::disassembler::*;
use crate::controllers::r3000::debug::register::*;
use crate::controllers::r3000::memory_controller::translate_address;
use crate::system::types::State;
use crate::system::r3000::cp0::*;
use crate::debug::DEBUG_CORE_EXIT;

pub static ENABLE_STATE_TRACING: AtomicBool = AtomicBool::new(false);
const ENABLE_STDOUT_PUTCHAR_TRACE: bool = true;
const ENABLE_HAZARD_TRACING: bool = true;
pub static ENABLE_INTERRUPT_TRACING: AtomicBool = AtomicBool::new(false);
const ENABLE_SYSCALL_TRACING: bool = false;
const ENABLE_RFE_TRACING: bool = false;
const ENABLE_MEMORY_TRACKING_READ: bool = true;
const ENABLE_MEMORY_TRACKING_WRITE: bool = true;
pub static ENABLE_MEMORY_SPIN_LOOP_DETECTION_READ: AtomicBool = AtomicBool::new(false);
pub static ENABLE_MEMORY_SPIN_LOOP_DETECTION_WRITE: AtomicBool = AtomicBool::new(false);
pub static ENABLE_REGISTER_TRACING: AtomicBool = AtomicBool::new(false);
const ENABLE_BIOS_CALL_TRACING: bool = false;

const MEMORY_TRACKING_ADDRESS_RANGE_START: u32 = 0x1F80_1800;
const MEMORY_TRACKING_ADDRESS_RANGE_END: u32 = 0x1F80_1810; 
const MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD: usize = 16;

pub static mut DEBUG_TICK_COUNT: usize = 0;
static mut DEBUG_BIOS_CALL_COUNT: usize = 0;
static mut DEBUG_CRITICAL_SECTION_REFCOUNT: isize = 0;

pub fn trace_state(resources: &Resources) {
    unsafe { 
        DEBUG_TICK_COUNT += 1; 

        if !ENABLE_STATE_TRACING.load(Ordering::Acquire) {
            return;
        }
    
        let tick_count = DEBUG_TICK_COUNT;
        let pc_va = resources.r3000.pc.read_u32() - INSTRUCTION_SIZE;
    
        // let start = 1195;
        // let end = 1196;
        // if (start..=end).contains(&DEBUG_BIOS_CALL_COUNT) {
        if true {
            let iec = resources.r3000.cp0.status.read_bitfield(STATUS_IEC) != 0;
            let branching = resources.r3000.branch_delay.branching();
            debug!("[{:X}] iec = {}, pc = 0x{:0X}, b = {}", tick_count, iec, pc_va, branching);
            trace_instructions_at_pc(resources, Some(1));
            if ENABLE_REGISTER_TRACING.load(Ordering::Acquire) {
                trace_registers(resources);
            }
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
    use crate::controllers::intc::debug::*;
    use crate::system::intc::{IRQ_BITFIELDS, IRQ_NAMES};

    let line_index = 2; // CDROM
    let line = IRQ_BITFIELDS[line_index];
    let line_name = IRQ_NAMES[line_index];

    if ENABLE_INTERRUPT_TRACING.load(Ordering::Acquire) {
        let debug_tick_count = unsafe { DEBUG_TICK_COUNT };
        let pc_va = resources.r3000.pc.read_u32();
        let branching = resources.r3000.branch_delay.branching();
        if false {
            if is_pending(resources, line) {
                trace!("[{:X}] Interrupt, pc = 0x{:0X}, branching = {}, line = {}", debug_tick_count, pc_va, branching, line_name);
            }
        } else {
            trace!("[{:X}] Interrupt, pc = 0x{:0X}, branching = {}", debug_tick_count, pc_va, branching);
            crate::controllers::intc::debug::trace_intc(resources, true, true);
        }
    }
}

pub fn trace_syscall(resources: &Resources) {
    if ENABLE_SYSCALL_TRACING {
        let debug_tick_count = unsafe { DEBUG_TICK_COUNT };
        let pc_va = resources.r3000.pc.read_u32() - INSTRUCTION_SIZE;

        let opcode = match resources.r3000.gpr[4].read_u32() {
            0 => "NoFunction".to_owned(),
            1 => { 
                unsafe { 
                    DEBUG_CRITICAL_SECTION_REFCOUNT += 1;
                    format!("EnterCriticalSection [{}]", DEBUG_CRITICAL_SECTION_REFCOUNT) 
                }
            },
            2 => { 
                unsafe {
                    DEBUG_CRITICAL_SECTION_REFCOUNT = max(DEBUG_CRITICAL_SECTION_REFCOUNT - 1, 0);
                    format!("ExitCriticalSection [{}]", DEBUG_CRITICAL_SECTION_REFCOUNT) 
                }
            },
            3 => "ChangeThreadSubFunction".to_owned(),
            _ => "DeliverEvent".to_owned(),
        };

        trace!("[{:X}] syscall, pc = 0x{:08X}, opcode = {}", debug_tick_count, pc_va, &opcode);
    }
}

pub fn trace_rfe(resources: &Resources) {
    if ENABLE_RFE_TRACING {
        let debug_tick_count = unsafe { DEBUG_TICK_COUNT };
        let pc_va = resources.r3000.pc.read_u32() - INSTRUCTION_SIZE;
        let branch_target = resources.r3000.branch_delay.target_or_null();
        trace!("[{:X}] rfe, pc = 0x{:08X}, branch target = 0x{:08X}", debug_tick_count, pc_va, branch_target);
    }
}

pub fn track_memory_read_pending<T>(resources: &Resources, physical_address: u32) {
    if !ENABLE_MEMORY_TRACKING_READ {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    if false {
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

    if false {
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

    if false {
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

    if false {
        let tick_count = unsafe { DEBUG_TICK_COUNT };
        let type_name = core::any::type_name::<T>();
        let pc = resources.r3000.pc.read_u32();
        debug!("[{:X}] Write PC = 0x{:08X} {} address = 0x{:08X}, value = 0x{:X} end", tick_count, pc, type_name, physical_address, value);
    }

    trace_memory_spin_loop_detection_write(resources, physical_address, count);
}

fn trace_memory_spin_loop_detection_read(resources: &Resources, physical_address: u32, count: usize) {
    if !ENABLE_MEMORY_SPIN_LOOP_DETECTION_READ.load(Ordering::Acquire) {
        return;
    }

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        let tick_count = unsafe { DEBUG_TICK_COUNT };
        trace!("[{:X}] Memory read spin loop detected on address = 0x{:08X}", tick_count, physical_address);
        trace_instructions_at_pc(resources, Some(1));
        if ENABLE_REGISTER_TRACING.load(Ordering::Acquire) {
            trace_registers(resources);
        }
        memory::clear_state_read(physical_address);
    }
} 

fn trace_memory_spin_loop_detection_write(resources: &Resources, physical_address: u32, count: usize) {
    if !ENABLE_MEMORY_SPIN_LOOP_DETECTION_WRITE.load(Ordering::Acquire) {
        return;
    }

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        let tick_count = unsafe { DEBUG_TICK_COUNT };
        trace!("[{:X}] Memory write spin loop detected on address = 0x{:08X}", tick_count, physical_address);
        trace_instructions_at_pc(resources, Some(1));
        if ENABLE_REGISTER_TRACING.load(Ordering::Acquire) {
            trace_registers(resources);
        }
        memory::clear_state_write(physical_address);
    }
}

pub fn trace_stdout_putchar(resources: &Resources) {
    static mut BUFFER: String = String::new();

    // BIOS call 0xA0, $t1 = 0x3C.
    if !ENABLE_STDOUT_PUTCHAR_TRACE {
        return;
    }
    
    let mut pc = resources.r3000.pc.read_u32();
    pc = translate_address(pc);
    let t1 = resources.r3000.gpr[9].read_u32();

    if ((pc == 0xA0) && (t1 == 0x3C)) || ((pc == 0xB0) && (t1 == 0x3D)) {
        unsafe {
            let a1 = resources.r3000.gpr[4].read_u32();
            assert!(a1 < 128, format!("stdout putchar a1 = 0x{:08X}", a1)); // Assumed to be ASCII encoding.

            let ch = a1 as u8 as char;

            if ch != '\n' {
                BUFFER.push(ch);
            } else {
                let tick_count = DEBUG_TICK_COUNT;
                let iec = resources.r3000.cp0.status.read_bitfield(STATUS_IEC) != 0;
                trace!("[{:X}] stdout: iec = {}, string = {}", tick_count, iec, &BUFFER); 
                BUFFER.clear();
            }
        }
    }
}

pub fn trace_bios_call(resources: &Resources) {
    if !ENABLE_BIOS_CALL_TRACING {
        return;
    }

    let mut pc = resources.r3000.pc.read_u32();
    pc = translate_address(pc);
    let t1 = resources.r3000.gpr[9].read_u32();
    
    let string = match pc {
        0xA0 => {
            let opcode = match t1 {
                0x13 => "SaveState".to_owned(),
                0x17 => "strcmp".to_owned(),
                0x1B => "strlen".to_owned(),
                0x25 => "toupper".to_owned(),
                0x28 => "bzero".to_owned(),
                0x2A => "memcpy".to_owned(),
                0x2F => "rand".to_owned(),
                0x33 => "malloc".to_owned(),
                0x39 => "InitHeap".to_owned(),
                0x3C => "std_out_putchar".to_owned(),
                0x3F => "printf".to_owned(),
                0x44 => "FlushCache".to_owned(),
                0x49 => "GPU_cw".to_owned(),
                0x56 => "CdRemove".to_owned(),
                0x72 => "CdRemove".to_owned(),
                0x96 => "AddCDROMDevice".to_owned(),
                0x97 => "AddMemCardDevice".to_owned(),
                0x99 => "AddDummyTtyDevice".to_owned(),
                0xA3 => "DequeueCdIntr".to_owned(),
                _ => format!("{:X}", t1),
            };
            format!("0xA0({})", opcode)
        },
        0xB0 => {
            let opcode = match t1 {
                0x00 => "alloc_kernel_memory".to_owned(),
                0x07 => "DeliverEvent".to_owned(),
                0x08 => "OpenEvent".to_owned(),
                0x09 => "CloseEvent".to_owned(),
                0x0B => "TestEvent".to_owned(),
                0x0C => "EnableEvent".to_owned(),
                0x12 => "InitPad".to_owned(),
                0x13 => "StartPad".to_owned(),
                0x17 => "ReturnFromException".to_owned(),
                0x18 => "SetDefaultExitFromException".to_owned(),
                0x19 => "SetCustomExitFromException".to_owned(),
                0x3D => "std_out_putchar".to_owned(),
                0x47 => "AddDevice".to_owned(),
                0x5B => "ChangeClearPad".to_owned(),
                _ => format!("{:X}", t1),
            };
            format!("0xB0({})", opcode)
        },
        0xC0 => {
            let opcode = match t1 {
                0x00 => "EnqueueTimerAndVblankIrqs".to_owned(),
                0x01 => "EnqueueSyscallHandler".to_owned(),
                0x02 => "SysEnqIntRP".to_owned(),
                0x03 => "SysDeqIntRP".to_owned(),
                0x07 => "InstallExceptionHandlers".to_owned(),
                0x08 => "SysInitMemory".to_owned(),
                0x09 => "SysInitKernelVariables".to_owned(),
                0x0A => "ChangeClearRCnt".to_owned(),
                0x0B => panic!("BIOS SystemError C0(0x0B) call detected"),
                0x0C => "InitDefInt".to_owned(),
                0x12 => "InstallDevices".to_owned(),
                0x1C => "AdjustA0Table".to_owned(),
                _ => format!("{:X}", t1),
            };
            format!("0xC0({})", &opcode)
        },
        _ => return,
    };

    let ra = resources.r3000.gpr[31].read_u32();

    unsafe { 
        DEBUG_BIOS_CALL_COUNT += 1;
        trace!("[{:X}] BIOS call {} {}, ra = 0x{:08X}", DEBUG_TICK_COUNT, DEBUG_BIOS_CALL_COUNT, &string, ra);
    }
}
