use log::debug;
use crate::State;
use crate::constants::r3000::INSTRUCTION_SIZE;
use crate::types::mips1::instruction::Instruction;
use crate::controllers::r3000::*;
use crate::controllers::r3000::memory_controller::*;
use crate::resources::r3000::cp0::{STATUS_ISC, CAUSE_EXCCODE_SYSCALL};
use crate::utilities::mips1::{pc_calc_jump_target, status_pop_exception};

pub unsafe fn sll(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32();
    let shamt = instruction.shamt();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value << shamt);
    Ok(())
}

pub unsafe fn srl(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32();
    let shamt = instruction.shamt();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value >> shamt);
    Ok(())
}

pub unsafe fn sra(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32() as i32;
    let shamt = instruction.shamt();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32((value >> shamt) as u32);
    Ok(())
}

pub unsafe fn sllv(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value2 << value1);
    Ok(())
}

pub unsafe fn srlv(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value2 >> value1);
    Ok(())
}

pub unsafe fn srav(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32;
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32((value2 >> value1) as u32);
    Ok(())
}

pub unsafe fn jr(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let target = resources.r3000.gpr[instruction.rs()].read_u32();
    resources.r3000.branch_delay.set(target, 1);
    Ok(())
}

pub unsafe fn jalr(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let target = resources.r3000.gpr[instruction.rs()].read_u32();
    resources.r3000.branch_delay.set(target, 1);
    let pc = resources.r3000.pc.read_u32();
    resources.r3000.gpr[instruction.rd()].write_u32(pc + INSTRUCTION_SIZE);
    Ok(())
}

pub unsafe fn syscall(state: &State, _instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    debug!("[{:X}] syscall, pc = 0x{:X}", DEBUG_TICK_COUNT, resources.r3000.pc.read_u32());
    set_exception(state, CAUSE_EXCCODE_SYSCALL);
    Ok(())
}

pub unsafe fn break_(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction break not implemented");
}

pub unsafe fn mfhi(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let value = resources.r3000.hi.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value);
    Ok(())
}

pub unsafe fn mthi(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    resources.r3000.hi.write_u32(value);
    Ok(())
}

pub unsafe fn mflo(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let value = resources.r3000.lo.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value);
    Ok(())
}

pub unsafe fn mtlo(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    resources.r3000.lo.write_u32(value);
    Ok(())
}

pub unsafe fn mult(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn multu(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn div(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn divu(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn add(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn addu(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1.wrapping_add(value2));
    Ok(())
}

pub unsafe fn sub(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction sub not implemented");
}

pub unsafe fn subu(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1.wrapping_sub(value2));
    Ok(())
}

pub unsafe fn and(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1 & value2);
    Ok(())
}

pub unsafe fn or(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1 | value2);
    Ok(())
}

pub unsafe fn xor(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(value1 ^ value2);
    Ok(())
}

pub unsafe fn nor(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut resources.r3000.gpr[instruction.rd()];
    rd.write_u32(!(value1 | value2));
    Ok(())
}

pub unsafe fn slt(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn sltu(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn bltz(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value < 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub unsafe fn bgez(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value >= 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub unsafe fn bltzal(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bltzal not implemented");
}

pub unsafe fn bgezal(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bgezal not implemented");
}

pub unsafe fn j(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let target = pc_calc_jump_target(resources.r3000.pc.read_u32(), instruction.addr());
    resources.r3000.branch_delay.set(target, 1);
    Ok(())
}

pub unsafe fn jal(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let target = pc_calc_jump_target(resources.r3000.pc.read_u32(), instruction.addr());
    resources.r3000.branch_delay.set(target, 1);

    let pc = resources.r3000.pc.read_u32();
    resources.r3000.gpr[31].write_u32(pc + INSTRUCTION_SIZE);

    Ok(())
}

pub unsafe fn beq(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn bne(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn blez(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value <= 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub unsafe fn bgtz(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let offset = (instruction.i_imm() as i32) << 2;
    let value = resources.r3000.gpr[instruction.rs()].read_u32() as i32;
    
    if value > 0 {
        let pc = resources.r3000.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        resources.r3000.branch_delay.set(target, 1);
    }

    Ok(())
}

pub unsafe fn addi(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn addiu(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let imm = instruction.i_imm() as i32 as u32;
    let rs = &mut resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value.wrapping_add(imm));
    Ok(())
}

pub unsafe fn slti(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn sltiu(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
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

pub unsafe fn andi(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value & (instruction.u_imm() as u32));
    Ok(())
}

pub unsafe fn ori(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rs = &resources.r3000.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value | (instruction.u_imm() as u32));
    Ok(())
}

pub unsafe fn xori(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction xori not implemented");
}

pub unsafe fn lui(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rt = instruction.rt();
    let imm = (instruction.u_imm() as u32) << 16;
    resources.r3000.gpr[rt].write_u32(imm);
    Ok(())
}

pub unsafe fn mfc0(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let rd = resources.r3000.cp0.register[instruction.rd()].as_mut().unwrap().as_mut();
    let value = rd.read_u32();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    rt.write_u32(value);
    Ok(())
}

pub unsafe fn mtc0(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let _lock = resources.r3000.cp0.mutex.lock();
    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let value = rt.read_u32();
    let rd = resources.r3000.cp0.register[instruction.rd()].as_mut().unwrap().as_mut();
    rd.write_u32(value);
    Ok(())
}

pub unsafe fn bc0f(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bc0f not implemented");
}

pub unsafe fn bc0t(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction bc0t not implemented");
}

pub unsafe fn tlbr(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbr not implemented");
}

pub unsafe fn tlbwi(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbwi not implemented");
}

pub unsafe fn tlbwr(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbwr not implemented");
}

pub unsafe fn tlbp(_state: &State, _instruction: Instruction) -> InstResult {
    unimplemented!("Instruction tlbp not implemented");
}

pub unsafe fn rfe(state: &State, _instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let _lock = resources.r3000.cp0.mutex.lock();
    debug!("[{:X}] rfe, pc = 0x{:X}", DEBUG_TICK_COUNT, resources.r3000.pc.read_u32());
    let status = &mut resources.r3000.cp0.status;
    let status_value = status_pop_exception(status.read_u32());
    status.write_u32(status_value);
    Ok(())
}

pub unsafe fn lb(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u8(state, addr).map(|v| v as i8 as i32 as u32)?
    } else { 
        0 
    };
    
    //debug!("lb (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    Ok(())
}

pub unsafe fn lh(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u16(state, addr).map(|v| v as i16 as i32 as u32)?
    } else { 
        0 
    };

    //debug!("lh (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    Ok(())
}

pub unsafe fn lwl(state: &State, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0x00FF_FFFF, 0x0000_FFFF, 0x0000_00FF, 0x0000_0000];
    const SHIFT: [usize; 4] = [24, 16, 8, 0];

    let resources = &mut *state.resources;
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u32(state, addr)?
    } else { 
        0
    };

    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let rt_value = rt.read_u32();
    let value = (rt_value & MASK[shift]) | (value << SHIFT[shift]);

    //debug!("lwl (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    rt.write_u32(value);

    Ok(())
}

pub unsafe fn lw(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u32(state, addr)?
    } else { 
        0
    };

    //debug!("lw (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    Ok(())
}

pub unsafe fn lbu(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u8(state, addr).map(|v| v as u32)?
    } else { 
        0 
    };

    //debug!("lb (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    Ok(())
}

pub unsafe fn lhu(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u16(state, addr).map(|v| v as u32)?
    } else { 
        0 
    };

    //debug!("lhu (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    resources.r3000.gpr[instruction.rt()].write_u32(value);

    Ok(())
}

pub unsafe fn lwr(state: &State, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0x0000_0000, 0xFF00_0000, 0xFFFF_0000, 0xFFFF_FF00];
    const SHIFT: [usize; 4] = [0, 8, 16, 24];

    let resources = &mut *state.resources;
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;
    let value = if !isc { 
        read_u32(state, addr)?
    } else { 
        0
    };

    let rt = &mut resources.r3000.gpr[instruction.rt()];
    let rt_value = rt.read_u32();
    let value = (rt_value & MASK[shift]) | (value >> SHIFT[shift]);

    //debug!("lwr (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    rt.write_u32(value);

    Ok(())
}

pub unsafe fn sb(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let value = resources.r3000.gpr[instruction.rt()].read_u8(0);
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    //debug!("sb (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    if !isc { 
        write_u8(state, addr, value)?
    }

    Ok(())
}

pub unsafe fn sh(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let value = resources.r3000.gpr[instruction.rt()].read_u16(0);
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);
        
    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    //debug!("sh (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    if !isc { 
        write_u16(state, addr, value)?
    }

    Ok(())
}

pub unsafe fn swl(state: &State, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0xFFFF_FF00, 0xFFFF_0000, 0xFF00_0000, 0x0000_0000];
    const SHIFT: [usize; 4] = [24, 16, 8, 0];

    let resources = &mut *state.resources;
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    let mem_value = if !isc { 
        read_u32(state, addr)?
    } else { 
        0
    };

    let rt_value = resources.r3000.gpr[instruction.rt()].read_u32();

    let value = (rt_value >> SHIFT[shift]) | (mem_value & MASK[shift]);

    //debug!("swl (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    if !isc { 
        write_u32(state, addr, value)?
    }

    Ok(())
}

pub unsafe fn sw(state: &State, instruction: Instruction) -> InstResult {
    let resources = &mut *state.resources;
    let value = resources.r3000.gpr[instruction.rt()].read_u32();
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    //debug!("sw (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    if !isc { 
        write_u32(state, addr, value)?
    }

    Ok(())
}

pub unsafe fn swr(state: &State, instruction: Instruction) -> InstResult {
    const MASK: [u32; 4] = [0x0000_0000, 0x0000_00FF, 0x0000_FFFF, 0x00FF_FFFF];
    const SHIFT: [usize; 4] = [0, 8, 16, 24];

    let resources = &mut *state.resources;
    let mut addr = resources.r3000.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

	let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = resources.r3000.cp0.status.read_bitfield(STATUS_ISC) != 0;

    let mem_value = if !isc { 
        read_u32(state, addr)?
    } else { 
        0
    };

    let rt_value = resources.r3000.gpr[instruction.rt()].read_u32();

    let value = (rt_value << SHIFT[shift]) | (mem_value & MASK[shift]);

    //debug!("swl (isc {}): address = 0x{:0X}, value = {} (0x{:0X})", isc, addr, value, value);

    if !isc { 
        write_u32(state, addr, value)?
    }

    Ok(())
}
