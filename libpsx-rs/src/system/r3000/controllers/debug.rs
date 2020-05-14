pub mod disassembler;
pub mod memory;
pub mod register;

use crate::{
    debug::DEBUG_CORE_EXIT,
    system::{
        r3000::{
            constants::INSTRUCTION_SIZE,
            controllers::{
                debug::{
                    disassembler::*,
                    register::*,
                },
                memory_controller::translate_address,
            },
            cp0::{
                constants::*,
                types::ControllerState as Cp0ControllerState,
            },
            types::{
                ControllerState,
                Hazard,
            },
        },
        types::State,
    },
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::{
    fmt::UpperHex,
    sync::atomic::{
        AtomicBool,
        AtomicIsize,
        AtomicUsize,
        Ordering,
    },
};

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
const MEMORY_TRACKING_ADDRESS_RANGE_END: u32 = 0x1F80_180F;
const MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD: usize = 16;

const HAZARD_WARNING_THRESHOLD: usize = 128;

pub static DEBUG_TICK_COUNT: AtomicUsize = AtomicUsize::new(0);
static DEBUG_BIOS_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static DEBUG_CRITICAL_SECTION_REFCOUNT: AtomicIsize = AtomicIsize::new(0);

lazy_static! {
    static ref DEBUG_HAZARD_REPEAT_COUNT: Mutex<(u32, usize)> = Mutex::new((0, 0));
}

pub fn trace_state(state: &State, r3000_state: &ControllerState, cp0_state: &Cp0ControllerState) {
    let tick_count = DEBUG_TICK_COUNT.fetch_add(1, Ordering::AcqRel) + 1;

    if !ENABLE_STATE_TRACING.load(Ordering::Acquire) {
        return;
    }

    let pc_va = r3000_state.pc.read_u32() - INSTRUCTION_SIZE;

    // let start = 1195;
    // let end = 1196;
    // if (start..=end).contains(&DEBUG_BIOS_CALL_COUNT) {
    if true {
        let iec = cp0_state.status.read_bitfield(STATUS_IEC) != 0;
        let branching = r3000_state.branch_delay.branching();
        log::debug!("[{:X}] iec = {}, pc = 0x{:0X}, b = {}", tick_count, iec, pc_va, branching);
        let pc = r3000_state.pc.read_u32();
        trace_instructions_at_pc(&state.memory.main_memory, &state.memory.bios, pc - INSTRUCTION_SIZE, Some(1));
        if ENABLE_REGISTER_TRACING.load(Ordering::Acquire) {
            trace_registers(r3000_state);
        }
    }

    if false {
        DEBUG_CORE_EXIT.store(true, Ordering::Release);
    }
}

pub fn trace_pc(state: &ControllerState, cp0_state: &Cp0ControllerState) {
    let pc = state.pc.read_u32();
    let kuc = cp0_state.status.read_bitfield(STATUS_KUC);
    let iec = cp0_state.status.read_bitfield(STATUS_IEC);
    let tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
    log::trace!("[{:X}] R3000 pc = 0x{:0X}, kuc = {}, iec = {}", tick_count, pc, kuc, iec);
}

pub fn trace_hazard(hazard: Result<(), Hazard>) {
    if ENABLE_HAZARD_TRACING {
        match hazard {
            Ok(()) => {
                let state = &mut DEBUG_HAZARD_REPEAT_COUNT.lock();
                state.0 = 0;
                state.1 = 0;
            },
            Err(hazard) => {
                match hazard {
                    Hazard::MemoryRead(addr) | Hazard::MemoryWrite(addr) => {
                        let state = &mut DEBUG_HAZARD_REPEAT_COUNT.lock();
                        if state.0 == addr {
                            state.1 += 1;
                            if state.1 > HAZARD_WARNING_THRESHOLD {
                                log::warn!("R3000 memory hazard: {}", hazard);
                                state.1 = 0;
                            }
                        } else {
                            state.0 = addr;
                            state.1 = 0;
                        }
                    },
                    Hazard::BusLockedMemoryRead(_) | Hazard::BusLockedMemoryWrite(_) => {
                        // Bus locking is normal and expected occasionally.
                    },
                }
            },
        }
    }
}

pub fn trace_interrupt(state: &State, r3000_state: &ControllerState) {
    use crate::system::intc::{
        constants::{
            IRQ_BITFIELDS,
            IRQ_NAMES,
        },
        controllers::debug::*,
    };

    let line_index = 2; // CDROM
    let line = IRQ_BITFIELDS[line_index];
    let line_name = IRQ_NAMES[line_index];

    if ENABLE_INTERRUPT_TRACING.load(Ordering::Acquire) {
        let debug_tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
        let pc_va = r3000_state.pc.read_u32();
        let branching = r3000_state.branch_delay.branching();
        if false {
            if is_pending(state, line) {
                log::trace!("[{:X}] Interrupt, pc = 0x{:0X}, branching = {}, line = {}", debug_tick_count, pc_va, branching, line_name);
            }
        } else {
            log::trace!("[{:X}] Interrupt, pc = 0x{:0X}, branching = {}", debug_tick_count, pc_va, branching);
            crate::system::intc::controllers::debug::trace_intc(state, true, true);
        }
    }
}

pub fn trace_syscall(state: &ControllerState) {
    if ENABLE_SYSCALL_TRACING {
        let debug_tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
        let pc_va = state.pc.read_u32() - INSTRUCTION_SIZE;

        let opcode = match state.gpr[4].read_u32() {
            0 => "NoFunction".to_owned(),
            1 => {
                let count = DEBUG_CRITICAL_SECTION_REFCOUNT.fetch_add(1, Ordering::AcqRel) + 1;
                format!("EnterCriticalSection [{}]", count)
            },
            2 => {
                let count = DEBUG_CRITICAL_SECTION_REFCOUNT.fetch_sub(1, Ordering::AcqRel) - 1;
                if count == -1 {
                    DEBUG_CRITICAL_SECTION_REFCOUNT.store(0, Ordering::Release);
                }
                format!("ExitCriticalSection [{}]", count)
            },
            3 => "ChangeThreadSubFunction".to_owned(),
            _ => "DeliverEvent".to_owned(),
        };

        log::trace!("[{:X}] syscall, pc = 0x{:08X}, opcode = {}", debug_tick_count, pc_va, &opcode);
    }
}

pub fn trace_rfe(state: &ControllerState) {
    if ENABLE_RFE_TRACING {
        let debug_tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
        let pc_va = state.pc.read_u32() - INSTRUCTION_SIZE;
        let branch_target = state.branch_delay.target_or_null();
        log::trace!("[{:X}] rfe, pc = 0x{:08X}, branch target = 0x{:08X}", debug_tick_count, pc_va, branch_target);
    }
}

pub fn track_memory_read_pending<T>(state: &ControllerState, physical_address: u32) {
    if !ENABLE_MEMORY_TRACKING_READ {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    if true {
        let tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
        let type_name = core::any::type_name::<T>();
        let pc = state.pc.read_u32();
        log::debug!("[{:X}] Read PC = 0x{:08X} {} address = 0x{:08X} start", tick_count, pc, type_name, physical_address);
    }
}

pub fn track_memory_read<T: Copy + UpperHex>(state: &State, r3000_state: &ControllerState, physical_address: u32, value: T) {
    if !ENABLE_MEMORY_TRACKING_READ {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    let count = memory::update_state_read(physical_address);

    if true {
        let tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
        let type_name = core::any::type_name::<T>();
        let pc = r3000_state.pc.read_u32();
        log::debug!("[{:X}] Read PC = 0x{:08X} {} address = 0x{:08X}, value = 0x{:X} end", tick_count, pc, type_name, physical_address, value);
    }

    trace_memory_spin_loop_detection_read(state, r3000_state, physical_address, count);
}

pub fn track_memory_write_pending<T: Copy + UpperHex>(state: &ControllerState, physical_address: u32, value: T) {
    if !ENABLE_MEMORY_TRACKING_WRITE {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    if true {
        let tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
        let type_name = core::any::type_name::<T>();
        let pc = state.pc.read_u32();
        log::debug!("[{:X}] Write PC = 0x{:08X} {} address = 0x{:08X}, value = 0x{:X} start", tick_count, pc, type_name, physical_address, value);
    }
}

pub fn track_memory_write<T: Copy + UpperHex>(state: &State, r3000_state: &ControllerState, physical_address: u32, value: T) {
    if !ENABLE_MEMORY_TRACKING_WRITE {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    let count = memory::update_state_write(physical_address);

    if true {
        let tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
        let type_name = core::any::type_name::<T>();
        let pc = r3000_state.pc.read_u32();
        log::debug!("[{:X}] Write PC = 0x{:08X} {} address = 0x{:08X}, value = 0x{:X} end", tick_count, pc, type_name, physical_address, value);
    }

    trace_memory_spin_loop_detection_write(state, r3000_state, physical_address, count);
}

fn trace_memory_spin_loop_detection_read(state: &State, r3000_state: &ControllerState, physical_address: u32, count: usize) {
    if !ENABLE_MEMORY_SPIN_LOOP_DETECTION_READ.load(Ordering::Acquire) {
        return;
    }

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        let tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
        log::trace!("[{:X}] Memory read spin loop detected on address = 0x{:08X}", tick_count, physical_address);
        let pc = r3000_state.pc.read_u32();
        trace_instructions_at_pc(&state.memory.main_memory, &state.memory.bios, pc, Some(1));
        if ENABLE_REGISTER_TRACING.load(Ordering::Acquire) {
            trace_registers(r3000_state);
        }
        memory::clear_state_read(physical_address);
    }
}

fn trace_memory_spin_loop_detection_write(state: &State, r3000_state: &ControllerState, physical_address: u32, count: usize) {
    if !ENABLE_MEMORY_SPIN_LOOP_DETECTION_WRITE.load(Ordering::Acquire) {
        return;
    }

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        let tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
        log::trace!("[{:X}] Memory write spin loop detected on address = 0x{:08X}", tick_count, physical_address);
        let pc = r3000_state.pc.read_u32();
        trace_instructions_at_pc(&state.memory.main_memory, &state.memory.bios, pc, Some(1));
        if ENABLE_REGISTER_TRACING.load(Ordering::Acquire) {
            trace_registers(r3000_state);
        }
        memory::clear_state_write(physical_address);
    }
}

pub fn trace_stdout_putchar(state: &ControllerState, cp0_state: &Cp0ControllerState) {
    lazy_static! {
        static ref BUFFER: Mutex<String> = Mutex::new(String::new());
    }

    // BIOS call 0xA0, $t1 = 0x3C.
    if !ENABLE_STDOUT_PUTCHAR_TRACE {
        return;
    }

    let mut pc = state.pc.read_u32();
    pc = translate_address(pc);
    let t1 = state.gpr[9].read_u32();

    if ((pc == 0xA0) && (t1 == 0x3C)) || ((pc == 0xB0) && (t1 == 0x3D)) {
        let buffer = &mut BUFFER.lock();

        let a1 = state.gpr[4].read_u32();
        assert!(a1 < 128, format!("stdout putchar a1 = 0x{:08X}", a1)); // Assumed to be ASCII encoding.

        let ch = a1 as u8 as char;

        if ch != '\n' {
            buffer.push(ch);
        } else {
            let tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
            let iec = cp0_state.status.read_bitfield(STATUS_IEC) != 0;
            log::trace!("[{:X}] stdout: iec = {}, string = {}", tick_count, iec, &buffer);
            buffer.clear();
        }
    }
}

pub fn trace_bios_call(state: &ControllerState) {
    if !ENABLE_BIOS_CALL_TRACING {
        return;
    }

    let mut pc = state.pc.read_u32();
    pc = translate_address(pc);
    let t1 = state.gpr[9].read_u32();

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

    let ra = state.gpr[31].read_u32();

    let call_count = DEBUG_BIOS_CALL_COUNT.fetch_add(1, Ordering::AcqRel) + 1;
    let tick_count = DEBUG_TICK_COUNT.load(Ordering::Acquire);
    log::trace!("[{:X}] BIOS call {} {}, ra = 0x{:08X}", tick_count, call_count, &string, ra);
}
