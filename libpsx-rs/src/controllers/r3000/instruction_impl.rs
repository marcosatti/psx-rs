use crate::resources::Resources;
use crate::constants::r3000::INSTRUCTION_SIZE;
use crate::types::mips1::instruction::Instruction;
use crate::controllers::r3000::{InstResult, set_exception};
use crate::controllers::r3000::memory_controller::*;
use crate::resources::r3000::cp0::{STATUS_ISC, CAUSE_EXCCODE_SYSCALL};
use crate::utilities::mips1::{pc_calc_jump_target, status_pop_exception};
use crate::controllers::r3000::debug;

pub fn sll(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32();
    let shamt = instruction.shamt();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value << shamt);
    Ok(())
}

pub fn srl(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32();
    let shamt = instruction.shamt();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value >> shamt);
    Ok(())
}

pub fn sra(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32() as i32;
    let shamt = instruction.shamt();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32((value >> shamt) as u32);
    Ok(())
}

pub fn sllv(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];

    if value1 >= 32 {
        rd.write_u32(0);
    } else {
        rd.write_u32(value2 << value1);
    }

    Ok(())
}

pub fn srlv(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];

    if value1 >= 32 {
        rd.write_u32(0);
    } else {
        rd.write_u32(value2 >> value1);
    }
    
    Ok(())
}

pub fn srav(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32;
    let rd = &mut resources.r3000.gpr[instruction.rd()];

    if value1 >= 32 {
        if value2 < 0 {
            rd.write_u32(0xFFFF_FFFF);
        } else {
            rd.write_u32(0);
        }
    } else {
        rd.write_u32((value2 >> value1) as u32);
    }
    
    Ok(())
}

pub fn jr(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let target = resources.r3000.gpr[instruction.rs()].read_u32();
    resources.r3000.branch_delay.set(target, 1);
    Ok(())
}

pub fn jalr(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let target = resources.r3000.gpr[instruction.rs()].read_u32();
    resources.r3000.branch_delay.set(target, 1);
    let pc = resources.r3000.pc.read_u32();
    resources.r3000.gpr[instruction.rd()].write_u32(pc + INSTRUCTION_SIZE);
    Ok(())
}

pub fn syscall(resources: &mut Resources, _instruction: Instruction) -> InstResult {
    debug::trace_syscall(resources);

    if resources.r3000.branch_delay.branching() {
        unimplemented!("SYSCALL in branch delay slot not handled");    
    }

    set_exception(resources, CAUSE_EXCCODE_SYSCALL);
    Ok(())
}

pub fn break_(_resources: &mut Resources, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction break not implemented");
}

pub fn mfhi(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let value = resources.r3000.hi.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value);
    Ok(())
}

pub fn mthi(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    resources.r3000.hi.write_u32(value);
    Ok(())
}

pub fn mflo(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let value = resources.r3000.lo.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value);
    Ok(())
}

pub fn mtlo(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    resources.r3000.lo.write_u32(value);
    Ok(())
}

pub fn mult(resources: &mut Resources, instruction: Instruction) -> InstResult {
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
    Ok(())
}

pub fn multu(resources: &mut Resources, instruction: Instruction) -> InstResult {
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
    Ok(())
}

pub fn div(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32() as i32;
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32;

    // Undefined results for MIPS if denominator is 0.
    if value2 != 0 {
        let remainder = value1 % value2;
        let quotient = value1 / value2;

        let hi = &mut resources.r3000.hi;
        let lo = &mut resources.r3000.lo;
        hi.write_u32(remainder as u32);
        lo.write_u32(quotient as u32);
    }
    Ok(())
}

pub fn divu(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();

    // Undefined results for MIPS if denominator is 0.
    if value2 != 0 {
        let remainder = value1 % value2;
        let quotient = value1 / value2;

        let hi = &mut resources.r3000.hi;
        let lo = &mut resources.r3000.lo;
        hi.write_u32(remainder);
        lo.write_u32(quotient);
    }
    Ok(())
}

pub fn add(resources: &mut Resources, instruction: Instruction) -> InstResult {
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
    Ok(())
}

pub fn addu(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1.wrapping_add(value2));
    Ok(())
}

pub fn sub(_resources: &mut Resources, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction sub not implemented");
}

pub fn subu(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1.wrapping_sub(value2));
    Ok(())
}

pub fn and(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1 & value2);
    Ok(())
}

pub fn or(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1 | value2);
    Ok(())
}

pub fn xor(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1 ^ value2);
    Ok(())
}

pub fn nor(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(!(value1 | value2));
    Ok(())
}

pub fn slt(resources: &mut Resources, instruction: Instruction) -> InstResult {
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
    
    Ok(())
}

pub fn sltu(resources: &mut Resources, instruction: Instruction) -> InstResult {
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

    Ok(())
}

pub fn bltz(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value < 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub fn bgez(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value >= 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub fn bltzal(_resources: &mut Resources, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bltzal not implemented");
}

pub fn bgezal(_resources: &mut Resources, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bgezal not implemented");
}

pub fn j(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let target = pc_calc_jump_target(resources.r3000.pc.read_u32(), instruction.addr());
    resources.r3000.branch_delay.set(target, 1);
    Ok(())
}

pub fn jal(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let target = pc_calc_jump_target(resources.r3000.pc.read_u32(), instruction.addr());
    resources.r3000.branch_delay.set(target, 1);

    let pc = resources.r3000.pc.read_u32();
    resources.r3000.gpr[31].write_u32(pc + INSTRUCTION_SIZE);

    Ok(())
}

pub fn beq(resources: &mut Resources, instruction: Instruction) -> InstResult {
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

pub fn bne(resources: &mut Resources, instruction: Instruction) -> InstResult {
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

pub fn blez(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value <= 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub fn bgtz(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value > 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub fn addi(resources: &mut Resources, instruction: Instruction) -> InstResult {
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

    Ok(())
}

pub fn addiu(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let imm = instruction.i_imm() as i32 as u32;
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value.wrapping_add(imm));
    Ok(())
}

pub fn slti(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32() as i32;
    let imm = instruction.i_imm() as i32;

    let rt = &mut resources.r3000.gpr[instruction.rt()];
    if value < imm {
        rt.write_u32(1);
    } else {
        rt.write_u32(0);
    }

    Ok(())
}

pub fn sltiu(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let imm = instruction.i_imm() as i32 as u32;

    let rt = &mut resources.r3000.gpr[instruction.rt()];
    if value < imm {
        rt.write_u32(1);
    } else {
        rt.write_u32(0);
    }

    Ok(())
}

pub fn andi(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value & (instruction.u_imm() as u32));
    Ok(())
}

pub fn ori(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value | (instruction.u_imm() as u32));
    Ok(())
}

pub fn xori(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value ^ (instruction.u_imm() as u32));
    Ok(())
}

pub fn lui(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rt = instruction.rt();
    let imm = (instruction.u_imm() as u32) << 16;
    resources.r3000.gpr[rt].write_u32(imm);
    Ok(())
}

pub fn mfc0(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rd = unsafe { resources.r3000.cp0.register[instruction.rd()].as_mut().unwrap().as_mut() };
    let value = rd.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value);
    Ok(())
}

pub fn mtc0(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32();
    let rd = unsafe { resources.r3000.cp0.register[instruction.rd()].as_mut().unwrap().as_mut() };
    rd.write_u32(value);
    Ok(())
}

pub fn bc0f(_resources: &mut Resources, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bc0f not implemented");
}

pub fn bc0t(_resources: &mut Resources, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bc0t not implemented");
}

pub fn tlbr(_resources: &mut Resources, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbr not implemented");
}

pub fn tlbwi(_resources: &mut Resources, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbwi not implemented");
}

pub fn tlbwr(_resources: &mut Resources, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbwr not implemented");
}

pub fn tlbp(_resources: &mut Resources, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbp not implemented");
}

pub fn rfe(resources: &mut Resources, _instruction: Instruction) -> InstResult {
    debug::trace_rfe(resources);
    let status = &mut resources.r3000.cp0.status;
    let old_status_value = status.read_u32();
    let new_status_value = status_pop_exception(old_status_value);
    status.write_u32(new_status_value);
    
    // Flush the branch delay slot if any.
    if let Some(target) = resources.r3000.branch_delay.advance_all() {
        resources.r3000.pc.write_u32(target);
    }

    // Also flush the cause register to make sure no stray interrupts are pending as a result of the emulator being out of sync temporarily.
    // If there is an actual interrupt pending, then it will be asserted again shortly (R3000 interrupts are level triggered).
    resources.r3000.cp0.cause.clear_ip_field();

    Ok(())
}

pub fn lb(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u8(resources, addr).map(|v| v as i8 as i32 as u32)?
    } else { 
        0 
    };
    
    resources.r3000.gpr[instruction.rt()].write_u32(value);

    Ok(())
}

pub fn lh(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u16(resources, addr).map(|v| v as i16 as i32 as u32)?
    } else { 
        0 
    };

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    Ok(())
}

pub fn lwl(resources: &mut Resources, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0x00FF_FFFF, 0x0000_FFFF, 0x0000_00FF, 0x0000_0000];
    const SHIFT: [usize; 4] = [24, 16, 8, 0];

    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let rt_value = rt.read_u32();
    let value = (rt_value & MASK[shift]) | (value << SHIFT[shift]);

    rt.write_u32(value);

    Ok(())
}

pub fn lw(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    Ok(())
}

pub fn lbu(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u8(resources, addr).map(|v| v as u32)?
    } else { 
        0 
    };

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    Ok(())
}

pub fn lhu(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u16(resources, addr).map(|v| v as u32)?
    } else { 
        0 
    };

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    Ok(())
}

pub fn lwr(resources: &mut Resources, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0x0000_0000, 0xFF00_0000, 0xFFFF_0000, 0xFFFF_FF00];
    const SHIFT: [usize; 4] = [0, 8, 16, 24];

    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let rt_value = rt.read_u32();
    let value = (rt_value & MASK[shift]) | (value >> SHIFT[shift]);

    rt.write_u32(value);

    Ok(())
}

pub fn sb(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let value = resources.r3000.gpr[instruction.rt()].read_u8(0);
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    if !isc { 
        write_u8(resources, addr, value)?
    }

    Ok(())
}

pub fn sh(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let value = resources.r3000.gpr[instruction.rt()].read_u16(0);
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);
        
    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    if !isc { 
        write_u16(resources, addr, value)?
    }

    Ok(())
}

pub fn swl(resources: &mut Resources, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0xFFFF_FF00, 0xFFFF_0000, 0xFF00_0000, 0x0000_0000];
    const SHIFT: [usize; 4] = [24, 16, 8, 0];

    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    let mem_value = if !isc { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    let rt_value = resources.r3000.gpr[instruction.rt()].read_u32();

    let value = (rt_value >> SHIFT[shift]) | (mem_value & MASK[shift]);

    if !isc { 
        write_u32(resources, addr, value)?
    }

    Ok(())
}

pub fn sw(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let value = resources.r3000.gpr[instruction.rt()].read_u32();
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    if !isc { 
        write_u32(resources, addr, value)?
    }

    Ok(())
}

pub fn swr(resources: &mut Resources, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0x0000_0000, 0x0000_00FF, 0x0000_FFFF, 0x00FF_FFFF];
    const SHIFT: [usize; 4] = [0, 8, 16, 24];

    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    let mem_value = if !isc { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    let rt_value = resources.r3000.gpr[instruction.rt()].read_u32();

    let value = (rt_value << SHIFT[shift]) | (mem_value & MASK[shift]);

    if !isc { 
        write_u32(resources, addr, value)?
    }

    Ok(())
}

pub fn lwc2(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u32(resources, addr)?
    } else { 
        0
    };

    resources.r3000.cp2.gd[instruction.rt()].write_u32(value);

    Ok(())
}

pub fn swc2(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let value = resources.r3000.cp2.gd[instruction.rt()].read_u32();
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    if !isc { 
        write_u32(resources, addr, value)?
    }

    Ok(())
}
