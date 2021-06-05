use crate::system::{
    intc::controllers::debug::trace_intc,
    r3000::{
        constants::INSTRUCTION_SIZE,
        controllers::memory_controller::translate_address,
        cp0::{
            constants::*,
            types::ControllerState as Cp0ControllerState,
        },
        types::{
            ControllerState,
            Hazard,
        },
    },
    types::{
        ControllerResult,
        State,
    },
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::{
    fmt::UpperHex,
    sync::atomic::{
        AtomicIsize,
        AtomicUsize,
        Ordering,
    },
};

const ENABLE_STDOUT_PUTCHAR_TRACE: bool = true;
const ENABLE_HAZARD_TRACING: bool = false;
const ENABLE_INTERRUPT_TRACING: bool = false;
const ENABLE_SYSCALL_TRACING: bool = false;
const ENABLE_RFE_TRACING: bool = false;
const ENABLE_MEMORY_TRACKING_READ: bool = false;
const ENABLE_MEMORY_TRACKING_WRITE: bool = false;
const ENABLE_BIOS_CALL_TRACING: bool = false;

const MEMORY_TRACKING_ADDRESS_RANGE_START: u32 = 0x1F80_1800;
const MEMORY_TRACKING_ADDRESS_RANGE_END: u32 = 0x1F80_1804;

static DEBUG_TICK_COUNT: AtomicUsize = AtomicUsize::new(0);
static DEBUG_BIOS_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static DEBUG_CRITICAL_SECTION_REFCOUNT: AtomicIsize = AtomicIsize::new(0);

pub(crate) fn update_state() {
    DEBUG_TICK_COUNT.fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn trace_hazard(hazard: Result<(), Hazard>) {
    if !ENABLE_HAZARD_TRACING {
        return;
    }

    if let Err(hazard) = hazard {
        match hazard {
            Hazard::MemoryRead(_) | Hazard::MemoryWrite(_) => {
                log::warn!("R3000 memory hazard: {}", hazard);
            },
            Hazard::BusLockedMemoryRead(_) | Hazard::BusLockedMemoryWrite(_) => {
                // Bus locking is normal and expected occasionally.
            },
        }
    }
}

pub(crate) fn trace_interrupt(state: &State, r3000_state: &ControllerState) {
    if !ENABLE_INTERRUPT_TRACING {
        return;
    }

    let debug_tick_count = DEBUG_TICK_COUNT.load(Ordering::Relaxed);
    let pc_va = r3000_state.pc.read_u32();
    let branching = r3000_state.branch_delay.branching();

    log::trace!("[{:X}] Interrupt, pc = 0x{:0X}, branching = {}", debug_tick_count, pc_va, branching);
    trace_intc(state, true, true);
}

pub(crate) fn trace_syscall(state: &ControllerState) {
    if ENABLE_SYSCALL_TRACING {
        let debug_tick_count = DEBUG_TICK_COUNT.load(Ordering::Relaxed);
        let pc_va = state.pc.read_u32() - INSTRUCTION_SIZE;

        let opcode = match state.gpr[4].read_u32() {
            0 => "NoFunction".into(),
            1 => {
                let count = DEBUG_CRITICAL_SECTION_REFCOUNT.fetch_add(1, Ordering::Relaxed) + 1;
                format!("EnterCriticalSection [{}]", count)
            },
            2 => {
                let count = DEBUG_CRITICAL_SECTION_REFCOUNT.fetch_sub(1, Ordering::Relaxed) - 1;
                if count == -1 {
                    DEBUG_CRITICAL_SECTION_REFCOUNT.store(0, Ordering::Relaxed);
                }
                format!("ExitCriticalSection [{}]", count)
            },
            3 => "ChangeThreadSubFunction".into(),
            _ => "DeliverEvent".into(),
        };

        log::trace!("[{:X}] syscall, pc = 0x{:08X}, opcode = {}", debug_tick_count, pc_va, &opcode);
    }
}

pub(crate) fn trace_rfe(state: &ControllerState) {
    if ENABLE_RFE_TRACING {
        let debug_tick_count = DEBUG_TICK_COUNT.load(Ordering::Relaxed);
        let pc_va = state.pc.read_u32() - INSTRUCTION_SIZE;
        let branch_target = state.branch_delay.target_or_null();
        log::trace!("[{:X}] rfe, pc = 0x{:08X}, branch target = 0x{:08X}", debug_tick_count, pc_va, branch_target);
    }
}

pub(crate) fn track_memory_read_pending<T>(state: &ControllerState, physical_address: u32) {
    if !ENABLE_MEMORY_TRACKING_READ {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    if false {
        let tick_count = DEBUG_TICK_COUNT.load(Ordering::Relaxed);
        let type_name = core::any::type_name::<T>();
        let pc = state.pc.read_u32();
        log::debug!("[{:X}] Read PC = 0x{:08X} {} address = 0x{:08X} start", tick_count, pc, type_name, physical_address);
    }
}

pub(crate) fn track_memory_read<T: Copy + UpperHex>(r3000_state: &ControllerState, physical_address: u32, value: T) {
    if !ENABLE_MEMORY_TRACKING_READ {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    if true {
        let tick_count = DEBUG_TICK_COUNT.load(Ordering::Relaxed);
        let type_name = core::any::type_name::<T>();
        let pc = r3000_state.pc.read_u32();
        log::debug!("[{:X}] Read PC = 0x{:08X} {} address = 0x{:08X}, value = 0x{:X} end", tick_count, pc, type_name, physical_address, value);
    }
}

pub(crate) fn track_memory_write_pending<T: Copy + UpperHex>(state: &ControllerState, physical_address: u32, value: T) {
    if !ENABLE_MEMORY_TRACKING_WRITE {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    if false {
        let tick_count = DEBUG_TICK_COUNT.load(Ordering::Relaxed);
        let type_name = core::any::type_name::<T>();
        let pc = state.pc.read_u32();
        log::debug!("[{:X}] Write PC = 0x{:08X} {} address = 0x{:08X}, value = 0x{:X} start", tick_count, pc, type_name, physical_address, value);
    }
}

pub(crate) fn track_memory_write<T: Copy + UpperHex>(r3000_state: &ControllerState, physical_address: u32, value: T) {
    if !ENABLE_MEMORY_TRACKING_WRITE {
        return;
    }

    if !(physical_address >= MEMORY_TRACKING_ADDRESS_RANGE_START && physical_address < MEMORY_TRACKING_ADDRESS_RANGE_END) {
        return;
    }

    if true {
        let tick_count = DEBUG_TICK_COUNT.load(Ordering::Relaxed);
        let type_name = core::any::type_name::<T>();
        let pc = r3000_state.pc.read_u32();
        log::debug!("[{:X}] Write PC = 0x{:08X} {} address = 0x{:08X}, value = 0x{:X} end", tick_count, pc, type_name, physical_address, value);
    }
}

pub(crate) fn trace_stdout_putchar(state: &ControllerState, cp0_state: &Cp0ControllerState) {
    lazy_static! {
        static ref BUFFER: Mutex<String> = Mutex::new(String::new());
    }

    if !ENABLE_STDOUT_PUTCHAR_TRACE {
        return;
    }

    let mut pc = state.pc.read_u32();
    pc = translate_address(pc);

    if (pc != 0xA0) && (pc != 0xB0) {
        return;
    }

    let t1 = state.gpr[9].read_u32();

    if ((pc == 0xA0) && (t1 == 0x3C)) || ((pc == 0xB0) && (t1 == 0x3D)) {
        let buffer = &mut BUFFER.lock();

        let a1 = state.gpr[4].read_u32();
        assert!(a1 < 128, "stdout putchar a1 = 0x{:08X}", a1); // Assumed to be ASCII encoding.

        let ch = a1 as u8 as char;

        if ch != '\n' {
            buffer.push(ch);
        } else {
            let tick_count = DEBUG_TICK_COUNT.load(Ordering::Relaxed);
            let iec = cp0_state.status.read_bitfield(STATUS_IEC) != 0;
            log::trace!("[{:X}] stdout: iec = {}, string = {}", tick_count, iec, &buffer);
            buffer.clear();
        }
    }
}

pub(crate) fn trace_bios_call(state: &ControllerState) -> ControllerResult<()> {
    if !ENABLE_BIOS_CALL_TRACING {
        return Ok(());
    }

    let mut pc = state.pc.read_u32();
    pc = translate_address(pc);
    let t1 = state.gpr[9].read_u32();

    let string = match pc {
        0xA0 => {
            let opcode = match t1 {
                0x13 => "SaveState".into(),
                0x17 => "strcmp".into(),
                0x1B => "strlen".into(),
                0x25 => "toupper".into(),
                0x28 => "bzero".into(),
                0x2A => "memcpy".into(),
                0x2F => "rand".into(),
                0x33 => "malloc".into(),
                0x39 => "InitHeap".into(),
                0x3C => "std_out_putchar".into(),
                0x3F => "printf".into(),
                0x44 => "FlushCache".into(),
                0x49 => "GPU_cw".into(),
                0x56 => "CdRemove".into(),
                0x72 => "CdRemove".into(),
                0x96 => "AddCDROMDevice".into(),
                0x97 => "AddMemCardDevice".into(),
                0x99 => "AddDummyTtyDevice".into(),
                0xA3 => "DequeueCdIntr".into(),
                _ => format!("{:X}", t1),
            };
            format!("0xA0({})", opcode)
        },
        0xB0 => {
            let opcode = match t1 {
                0x00 => "alloc_kernel_memory".into(),
                0x07 => "DeliverEvent".into(),
                0x08 => "OpenEvent".into(),
                0x09 => "CloseEvent".into(),
                0x0B => "TestEvent".into(),
                0x0C => "EnableEvent".into(),
                0x12 => "InitPad".into(),
                0x13 => "StartPad".into(),
                0x17 => "ReturnFromException".into(),
                0x18 => "SetDefaultExitFromException".into(),
                0x19 => "SetCustomExitFromException".into(),
                0x3D => "std_out_putchar".into(),
                0x47 => "AddDevice".into(),
                0x5B => "ChangeClearPad".into(),
                _ => format!("{:X}", t1),
            };
            format!("0xB0({})", opcode)
        },
        0xC0 => {
            let opcode = match t1 {
                0x00 => "EnqueueTimerAndVblankIrqs".into(),
                0x01 => "EnqueueSyscallHandler".into(),
                0x02 => "SysEnqIntRP".into(),
                0x03 => "SysDeqIntRP".into(),
                0x07 => "InstallExceptionHandlers".into(),
                0x08 => "SysInitMemory".into(),
                0x09 => "SysInitKernelVariables".into(),
                0x0A => "ChangeClearRCnt".into(),
                0x0B => return Err("BIOS SystemError C0(0x0B) call detected".into()),
                0x0C => "InitDefInt".into(),
                0x12 => "InstallDevices".into(),
                0x1C => "AdjustA0Table".into(),
                _ => format!("{:X}", t1),
            };
            format!("0xC0({})", &opcode)
        },
        _ => return Ok(()),
    };

    let ra = state.gpr[31].read_u32();

    let call_count = DEBUG_BIOS_CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    let tick_count = DEBUG_TICK_COUNT.load(Ordering::Relaxed);
    log::trace!("[{:X}] BIOS call {} {}, ra = 0x{:08X}", tick_count, call_count, &string, ra);

    Ok(())
}
