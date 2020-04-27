use crate::{
    system::{
        r3000::{
            constants::INSTRUCTION_SIZE,
            controllers::debug,
            cp0::constants::*,
            cp0::types::{IrqLine, ControllerState as Cp0ControllerState}, 
            types::ControllerState,
        },
        types::State,
    },
    utilities::{bool_to_flag, mips1::status_push_exception},
};
use std::intrinsics::unlikely;

pub fn update_ip_field(state: &State, cp0_state: &mut Cp0ControllerState) {
    let intc_pending = state.r3000.cp0.interrupt.line_interrupted(IrqLine::Intc);
    cp0_state.cause.write_bitfield(CAUSE_IP_INTC, bool_to_flag(intc_pending));
}

pub fn clear_ip_field(state: &State, cp0_state: &mut Cp0ControllerState) {
    state.r3000.cp0.interrupt.deassert_line(IrqLine::Intc);
    cp0_state.cause.write_bitfield(CAUSE_IP, 0);
}

pub fn set_exception(r3000_state: &mut ControllerState, cp0_state: &mut Cp0ControllerState, exccode: usize) {
    let pc = &mut r3000_state.pc;
    let cause = &mut cp0_state.cause;
    let status = &mut cp0_state.status;
    let mut pc_value = pc.read_u32() - INSTRUCTION_SIZE;

    if exccode == CAUSE_EXCCODE_INT {
        pc_value += INSTRUCTION_SIZE;
    }

    assert!(!r3000_state.branch_delay.branching(), "Exception handling while branching not implmeneted");

    // Push IEc & KUc (stack).
    let old_status_value = status.read_u32();
    let new_status_value = status_push_exception(old_status_value);
    status.write_u32(new_status_value);

    // Set ExcCode cause.
    cause.write_bitfield(CAUSE_EXCCODE, exccode as u32);

    // Set EPC address.
    let epc = &mut cp0_state.epc;
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
        _ => unimplemented!("Unimplemented exception type encountered"),
    }

    // Set PC to exception vector.
    pc.write_u32(vector_offset);
}

pub fn handle_interrupts(state: &State, r3000_state: &mut ControllerState, cp0_state: &mut Cp0ControllerState) {
    let status = &cp0_state.status;
    let cause = &mut cp0_state.cause;

    if status.read_bitfield(STATUS_IEC) == 0 {
        return;
    }

    if r3000_state.branch_delay.branching() {
        // Unimplemented for now, can just wait until we are not branching to handle this.
        return;
    }

    update_ip_field(state, cp0_state);

    let set_bits = {
        let status = &cp0_state.status;
        let cause = &cp0_state.cause;
        status.read_bitfield(STATUS_IM) & cause.read_bitfield(CAUSE_IP)
    };

    if unlikely(set_bits != 0) {
        debug::trace_interrupt(state, r3000_state);
        set_exception(r3000_state, cp0_state, CAUSE_EXCCODE_INT);
    }
}
