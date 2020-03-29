use std::intrinsics::{likely, unlikely};
use crate::system::types::State;
use crate::constants::r3000::INSTRUCTION_SIZE;
use crate::types::mips1::instruction::Instruction;
use crate::controllers::r3000::{InstResult, set_exception};
use crate::controllers::r3000::memory_controller::*;
use crate::controllers::r3000::register::*;
use crate::system::r3000::cp0::{STATUS_ISC, CAUSE_EXCCODE_SYSCALL};
use crate::utilities::mips1::{pc_calc_jump_target, status_pop_exception};
use crate::controllers::r3000::debug;

pub fn sll(state: &mut State, instruction: Instruction) -> InstResult {
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32();
    let shamt = instruction.shamt();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value << shamt);
    handle_zero(resources);
    Ok(())
}

pub fn srl(state: &mut State, instruction: Instruction) -> InstResult {
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32();
    let shamt = instruction.shamt();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value >> shamt);
    handle_zero(resources);
    Ok(())
}

pub fn sra(state: &mut State, instruction: Instruction) -> InstResult {
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32() as i32;
    let shamt = instruction.shamt();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32((value >> shamt) as u32);
    handle_zero(resources);
    Ok(())
}

pub fn sllv(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];

    if unlikely(value1 >= 32) {
        rd.write_u32(0);
    } else {
        rd.write_u32(value2 << value1);
    }

    handle_zero(resources);
    Ok(())
}

pub fn srlv(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];

    if unlikely(value1 >= 32) {
        rd.write_u32(0);
    } else {
        rd.write_u32(value2 >> value1);
    }
    
    handle_zero(resources);
    Ok(())
}

pub fn srav(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32;
    let rd = &mut resources.r3000.gpr[instruction.rd()];

    if unlikely(value1 >= 32) {
        if value2 < 0 {
            rd.write_u32(0xFFFF_FFFF);
        } else {
            rd.write_u32(0);
        }
    } else {
        rd.write_u32((value2 >> value1) as u32);
    }
    
    handle_zero(resources);
    Ok(())
}

pub fn jr(state: &mut State, instruction: Instruction) -> InstResult {
    let target = resources.r3000.gpr[instruction.rs()].read_u32();
    resources.r3000.branch_delay.set(target, 1);
    Ok(())
}

pub fn jalr(state: &mut State, instruction: Instruction) -> InstResult {
    let target = resources.r3000.gpr[instruction.rs()].read_u32();
    resources.r3000.branch_delay.set(target, 1);
    let pc = resources.r3000.pc.read_u32();
    resources.r3000.gpr[instruction.rd()].write_u32(pc + INSTRUCTION_SIZE);
    handle_zero(resources);
    Ok(())
}

pub fn syscall(state: &mut State, _instruction: Instruction) -> InstResult {
    debug::trace_syscall(resources);

    if resources.r3000.branch_delay.branching() {
        unimplemented!("SYSCALL in branch delay slot not handled");    
    }

    set_exception(resources, CAUSE_EXCCODE_SYSCALL);
    Ok(())
}

pub fn break_(_state: &mut State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction break not implemented");
}

pub fn mfhi(state: &mut State, instruction: Instruction) -> InstResult {
    let value = resources.r3000.hi.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value);
    handle_zero(resources);
    Ok(())
}

pub fn mthi(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    resources.r3000.hi.write_u32(value);
    Ok(())
}

pub fn mflo(state: &mut State, instruction: Instruction) -> InstResult {
    let value = resources.r3000.lo.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value);
    handle_zero(resources);
    Ok(())
}

pub fn mtlo(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    resources.r3000.lo.write_u32(value);
    Ok(())
}

pub fn mult(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32() as i32 as i64;
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32 as i64;
    
    let result = (value1 * value2) as u64;
    let hi_value = ((result >> 32) & 0xFFFF_FFFF) as u32;
    let lo_value = (result & 0xFFFF_FFFF) as u32;

    let hi = &mut resources.r3000.hi;
    hi.write_u32(hi_value);
    let lo = &mut resources.r3000.lo;
    lo.write_u32(lo_value);

    handle_zero(resources);
    Ok(())
}

pub fn multu(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32() as u64;
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32() as u64;

    let result = value1 * value2;
    let hi_value = (result >> 32) as u32;
    let lo_value = result as u32;

    let hi = &mut resources.r3000.hi;
    hi.write_u32(hi_value);
    let lo = &mut resources.r3000.lo;
    lo.write_u32(lo_value);

    handle_zero(resources);
    Ok(())
}

pub fn div(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32() as i32;
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32;

    // Undefined results for MIPS if denominator is 0.
    if likely(value2 != 0) {
        let remainder = value1 % value2;
        let quotient = value1 / value2;

        let hi = &mut resources.r3000.hi;
        let lo = &mut resources.r3000.lo;
        hi.write_u32(remainder as u32);
        lo.write_u32(quotient as u32);
    }

    handle_zero(resources);
    Ok(())
}

pub fn divu(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();

    // Undefined results for MIPS if denominator is 0.
    if likely(value2 != 0) {
        let remainder = value1 % value2;
        let quotient = value1 / value2;

        let hi = &mut resources.r3000.hi;
        let lo = &mut resources.r3000.lo;
        hi.write_u32(remainder);
        lo.write_u32(quotient);
    }

    handle_zero(resources);
    Ok(())
}

pub fn add(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let (result, of_flag) = (value1 as i32).overflowing_add(value2 as i32);

    if of_flag {
        unimplemented!("Overflowing exception not implemented (add)");
    } else {
        let rd = &mut resources.r3000.gpr[instruction.rd()];
        rd.write_u32(result as u32);
    }

    handle_zero(resources);
    Ok(())
}

pub fn addu(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1.wrapping_add(value2));

    handle_zero(resources);
    Ok(())
}

pub fn sub(_state: &mut State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction sub not implemented");
}

pub fn subu(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1.wrapping_sub(value2));
    handle_zero(resources);
    Ok(())
}

pub fn and(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1 & value2);
    handle_zero(resources);
    Ok(())
}

pub fn or(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1 | value2);
    handle_zero(resources);
    Ok(())
}

pub fn xor(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1 ^ value2);
    handle_zero(resources);
    Ok(())
}

pub fn nor(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(!(value1 | value2));
    handle_zero(resources);
    Ok(())
}

pub fn slt(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32() as i32;
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32;

    let rd = &mut resources.r3000.gpr[instruction.rd()];
    if value1 < value2 {
        rd.write_u32(1);
    } else {
        rd.write_u32(0);
    }
    
    handle_zero(resources);
    Ok(())
}

pub fn sltu(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();

    let rd = &mut resources.r3000.gpr[instruction.rd()];
    if value1 < value2 {
        rd.write_u32(1);
    } else {
        rd.write_u32(0);
    }

    handle_zero(resources);
    Ok(())
}

pub fn bltz(state: &mut State, instruction: Instruction) -> InstResult {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value < 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub fn bgez(state: &mut State, instruction: Instruction) -> InstResult {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value >= 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub fn bltzal(_state: &mut State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bltzal not implemented");
}

pub fn bgezal(_state: &mut State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bgezal not implemented");
}

pub fn j(state: &mut State, instruction: Instruction) -> InstResult {
    let target = pc_calc_jump_target(resources.r3000.pc.read_u32(), instruction.addr());
    resources.r3000.branch_delay.set(target, 1);
    Ok(())
}

pub fn jal(state: &mut State, instruction: Instruction) -> InstResult {
    let target = pc_calc_jump_target(resources.r3000.pc.read_u32(), instruction.addr());
    resources.r3000.branch_delay.set(target, 1);

    let pc = resources.r3000.pc.read_u32();
    resources.r3000.gpr[31].write_u32(pc + INSTRUCTION_SIZE);

    Ok(())
}

pub fn beq(state: &mut State, instruction: Instruction) -> InstResult {
    let offset = (instruction.i_imm() as i32) << 2;
    let value1 = resources.r3000.gpr[instruction.rs()].read_u32();
    let value2 = resources.r3000.gpr[instruction.rt()].read_u32();
    
    if value1 == value2 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub fn bne(state: &mut State, instruction: Instruction) -> InstResult {
    let offset = (instruction.i_imm() as i32) << 2;
    let value1 = resources.r3000.gpr[instruction.rs()].read_u32();
    let value2 = resources.r3000.gpr[instruction.rt()].read_u32();
    
    if value1 != value2 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub fn blez(state: &mut State, instruction: Instruction) -> InstResult {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value <= 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub fn bgtz(state: &mut State, instruction: Instruction) -> InstResult {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value > 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub fn addi(state: &mut State, instruction: Instruction) -> InstResult {
    let imm = instruction.i_imm() as i32;
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value: u32 = rs.read_u32();
    let (result, of_flag) = (value as i32).overflowing_add(imm);

    if of_flag {
        unimplemented!("Overflowing exception not implemented (addi)");
    } else {
        let rt = &mut resources.r3000.gpr[instruction.rt()];
        rt.write_u32(result as u32);
    }

    handle_zero(resources);
    Ok(())
}

pub fn addiu(state: &mut State, instruction: Instruction) -> InstResult {
    let imm = instruction.i_imm() as i32 as u32;
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value.wrapping_add(imm));
    handle_zero(resources);
    Ok(())
}

pub fn slti(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32() as i32;
    let imm = instruction.i_imm() as i32;

    let rt = &mut resources.r3000.gpr[instruction.rt()];
    if value < imm {
        rt.write_u32(1);
    } else {
        rt.write_u32(0);
    }

    handle_zero(resources);
    Ok(())
}

pub fn sltiu(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let imm = instruction.i_imm() as i32 as u32;

    let rt = &mut resources.r3000.gpr[instruction.rt()];
    if value < imm {
        rt.write_u32(1);
    } else {
        rt.write_u32(0);
    }

    handle_zero(resources);
    Ok(())
}

pub fn andi(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value & (instruction.u_imm() as u32));
    handle_zero(resources);
    Ok(())
}

pub fn ori(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value | (instruction.u_imm() as u32));
    handle_zero(resources);
    Ok(())
}

pub fn xori(state: &mut State, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value ^ (instruction.u_imm() as u32));
    handle_zero(resources);
    Ok(())
}

pub fn lui(state: &mut State, instruction: Instruction) -> InstResult {
    let rt = instruction.rt();
    let imm = (instruction.u_imm() as u32) << 16;
    resources.r3000.gpr[rt].write_u32(imm);
    handle_zero(resources);
    Ok(())
}

pub fn mfc0(state: &mut State, instruction: Instruction) -> InstResult {
    let rd = unsafe { resources.r3000.cp0.register[instruction.rd()].as_mut().unwrap().as_mut() };
    let value = rd.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value);
    handle_zero(resources);
    Ok(())
}

pub fn mtc0(state: &mut State, instruction: Instruction) -> InstResult {
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32();
    let rd = unsafe { resources.r3000.cp0.register[instruction.rd()].as_mut().unwrap().as_mut() };
    rd.write_u32(value);
    Ok(())
}

pub fn bc0f(_state: &mut State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bc0f not implemented");
}

pub fn bc0t(_state: &mut State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bc0t not implemented");
}

pub fn tlbr(_state: &mut State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbr not implemented");
}

pub fn tlbwi(_state: &mut State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbwi not implemented");
}

pub fn tlbwr(_state: &mut State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbwr not implemented");
}

pub fn tlbp(_state: &mut State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbp not implemented");
}

pub fn rfe(state: &mut State, _instruction: Instruction) -> InstResult {
    debug::trace_rfe(resources);
    let status = &mut resources.r3000.cp0.status;
    let old_status_value = status.read_u32();
    let new_status_value = status_pop_exception(old_status_value);
    status.write_u32(new_status_value);
    
    // Flush the branch delay slot if any.
    let target = resources.r3000.branch_delay.advance_all();
    if likely(target.is_some()) {
        resources.r3000.pc.write_u32(target.unwrap());
    }

    // Also flush the cause register to make sure no stray interrupts are pending as a result of the emulator being out of sync temporarily.
    // If there is an actual interrupt pending, then it will be asserted again shortly (R3000 interrupts are level triggered).
    resources.r3000.cp0.cause.clear_ip_field();

    Ok(())
}

pub fn lb(state: &mut State, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) { 
        read_u8(resources, addr).map(|v| v as i8 as i32 as u32)?
    } else { 
        0 
    };
    
    resources.r3000.gpr[instruction.rt()].write_u32(value);

    handle_zero(resources);
    Ok(())
}

pub fn lh(state: &mut State, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) { 
        read_u16(resources, addr).map(|v| v as i16 as i32 as u32)?
    } else { 
        0 
    };

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    handle_zero(resources);
    Ok(())
}

pub fn lwl(state: &mut State, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0x00FF_FFFF, 0x0000_FFFF, 0x0000_00FF, 0x0000_0000];
    const SHIFT: [usize; 4] = [24, 16, 8, 0];

    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let rt_value = rt.read_u32();
    let value = (rt_value & MASK[shift]) | (value << SHIFT[shift]);

    rt.write_u32(value);

    handle_zero(resources);
    Ok(())
}

pub fn lw(state: &mut State, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    handle_zero(resources);
    Ok(())
}

pub fn lbu(state: &mut State, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) { 
        read_u8(resources, addr).map(|v| v as u32)?
    } else { 
        0 
    };

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    handle_zero(resources);
    Ok(())
}

pub fn lhu(state: &mut State, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) { 
        read_u16(resources, addr).map(|v| v as u32)?
    } else { 
        0 
    };

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    handle_zero(resources);
    Ok(())
}

pub fn lwr(state: &mut State, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0x0000_0000, 0xFF00_0000, 0xFFFF_0000, 0xFFFF_FF00];
    const SHIFT: [usize; 4] = [0, 8, 16, 24];

    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let rt_value = rt.read_u32();
    let value = (rt_value & MASK[shift]) | (value >> SHIFT[shift]);

    rt.write_u32(value);

    handle_zero(resources);
    Ok(())
}

pub fn sb(state: &mut State, instruction: Instruction) -> InstResult {
    let value = resources.r3000.gpr[instruction.rt()].read_u8(0);
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    if likely(!isc) { 
        write_u8(resources, addr, value)?
    }

    Ok(())
}

pub fn sh(state: &mut State, instruction: Instruction) -> InstResult {
    let value = resources.r3000.gpr[instruction.rt()].read_u16(0);
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);
        
    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    if likely(!isc) { 
        write_u16(resources, addr, value)?
    }

    Ok(())
}

pub fn swl(state: &mut State, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0xFFFF_FF00, 0xFFFF_0000, 0xFF00_0000, 0x0000_0000];
    const SHIFT: [usize; 4] = [24, 16, 8, 0];

    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    let mem_value = if likely(!isc) { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    let rt_value = resources.r3000.gpr[instruction.rt()].read_u32();

    let value = (rt_value >> SHIFT[shift]) | (mem_value & MASK[shift]);

    if likely(!isc) { 
        write_u32(resources, addr, value)?
    }

    Ok(())
}

pub fn sw(state: &mut State, instruction: Instruction) -> InstResult {
    let value = resources.r3000.gpr[instruction.rt()].read_u32();
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    if likely(!isc) { 
        write_u32(resources, addr, value)?
    }

    Ok(())
}

pub fn swr(state: &mut State, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0x0000_0000, 0x0000_00FF, 0x0000_FFFF, 0x00FF_FFFF];
    const SHIFT: [usize; 4] = [0, 8, 16, 24];

    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    let mem_value = if likely(!isc) { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    let rt_value = resources.r3000.gpr[instruction.rt()].read_u32();

    let value = (rt_value << SHIFT[shift]) | (mem_value & MASK[shift]);

    if likely(!isc) { 
        write_u32(resources, addr, value)?
    }

    Ok(())
}
