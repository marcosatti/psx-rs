use crate::resources::Resources;
use crate::utilities::mips1::status_push_exception;
use crate::constants::r3000::INSTRUCTION_SIZE;
use crate::resources::r3000::cp0::*;
use crate::controllers::r3000::debug;

pub fn set_exception(resources: &mut Resources, exccode: usize) {
    let pc = &mut resources.r3000.pc;
    let cause = &mut resources.r3000.cp0.cause.register;
    let status = &mut resources.r3000.cp0.status;
    let mut pc_value = pc.read_u32();

    if resources.r3000.branch_delay.branching() {
        cause.write_bitfield(CAUSE_BD, 1);
        pc_value -= INSTRUCTION_SIZE;
        resources.r3000.branch_delay.cancel();
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

pub fn handle_interrupts(resources: &mut Resources) {
    let check_interrupts = {
        let status = &resources.r3000.cp0.status;
        let cause = &mut resources.r3000.cp0.cause;

        if status.read_bitfield(STATUS_IEC) == 0 {
            return;
        }

        cause.handle_irq_messages()
    };

    if check_interrupts {
        let set_bits = {
            let status = &resources.r3000.cp0.status;
            let cause = &resources.r3000.cp0.cause.register;
            status.read_bitfield(STATUS_IM) & cause.read_bitfield(CAUSE_IP)
        };

        if set_bits != 0 {
            debug::trace_interrupt(resources);
            set_exception(resources, CAUSE_EXCCODE_INT);
        }
    }
}
