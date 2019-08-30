pub mod memory;
pub mod disassembler;
pub mod register;

use log::trace;
use crate::State;
use crate::controllers::r3000::debug::disassembler::*;
use crate::controllers::r3000::debug::memory::*;
use crate::controllers::r3000::debug::register::*;

pub static ENABLE_IO_SPIN_LOOP_DETECTION_READ: bool = true;
pub static ENABLE_IO_SPIN_LOOP_DETECTION_WRITE: bool = false;

static MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD: usize = 1024 * 16;

pub unsafe fn trace_io_spin_loop_detection_read(state: &State, physical_address: u32) {
    if physical_address < 0x1F00_000 {
        return;
    }

    let count = track_memory_read(physical_address);

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        trace!("Spin loop detected on I/O address = 0x{:08X} (read)", physical_address);
        trace_instructions_at_pc(state);
        trace_registers(state);
        track_memory_read_clear(physical_address);
    }
} 

pub unsafe fn trace_io_spin_loop_detection_write(state: &State, physical_address: u32) {
    if physical_address < 0x1F00_000 {
        return;
    }

    let count = track_memory_write(physical_address);

    if count >= MEMORY_SPIN_LOOP_DETECTION_ACCESS_THRESHOLD {
        trace!("Spin loop detected on I/O address = 0x{:08X} (write)", physical_address);
        trace_instructions_at_pc(state);
        trace_registers(state);
        track_memory_write_clear(physical_address);
    }
}
