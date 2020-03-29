use std::intrinsics::unlikely;
use crate::system::types::State;
use crate::utilities::mips1::status_push_exception;
use crate::system::r3000::constants::INSTRUCTION_SIZE;
use crate::system::r3000::cp0::constants::*;
use crate::system::r3000::controllers::debug;

pub fn set_exception(state: &mut State, exccode: usize) {
    let pc = &mut state.r3000.pc;
    let cause = &mut state.r3000.cp0.cause.register;
    let status = &mut state.r3000.cp0.status;
    let mut pc_value = pc.read_u32() - INSTRUCTION_SIZE;

    if exccode == CAUSE_EXCCODE_INT {
        pc_value += INSTRUCTION_SIZE;
    }

    assert!(!state.r3000.branch_delay.branching(), "Exception handling while branching not implmeneted");

    // Push IEc & KUc (stack).
    let old_status_value = status.read_u32();
    let new_status_value = status_push_exception(old_status_value);
    status.write_u32(new_status_value);

    // Set ExcCode cause.
    cause.write_bitfield(CAUSE_EXCCODE, exccode as u32);

    // Set EPC address.
    let epc = &mut state.r3000.cp0.epc;
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

pub fn handle_interrupts(state: &mut State) {
    let status = &state.r3000.cp0.status;
    let cause = &mut state.r3000.cp0.cause;

    if status.read_bitfield(STATUS_IEC) == 0 {
        return;
    }

    if state.r3000.branch_delay.branching() {
        // Unimplemented for now, can just wait until we are not branching to handle this.
        return;
    }

    cause.update_ip_field();

    let set_bits = {
        let status = &state.r3000.cp0.status;
        let cause = &state.r3000.cp0.cause.register;
        status.read_bitfield(STATUS_IM) & cause.read_bitfield(CAUSE_IP)
    };

    if unlikely(set_bits != 0) {
        debug::trace_interrupt(state);
        set_exception(state, CAUSE_EXCCODE_INT);
    }
}
