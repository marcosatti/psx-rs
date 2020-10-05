use crate::{
    system::{
        r3000::{
            constants::INSTRUCTION_SIZE,
            controllers::{
                debug,
                exception::{
                    clear_ip_field,
                    set_exception,
                },
                memory_controller::*,
                register::*,
            },
            cp0::constants::{
                CAUSE_EXCCODE_SYSCALL,
                STATUS_ISC,
            },
            types::*,
        },
        types::ControllerResult,
    },
    types::mips1::instruction::Instruction,
    utilities::mips1::{
        pc_calc_jump_target,
        status_pop_exception,
    },
};
use std::intrinsics::{
    likely,
    unlikely,
};

pub(crate) fn sll(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value = rt.read_u32();
    let shamt = instruction.shamt();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(value << shamt);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn srl(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value = rt.read_u32();
    let shamt = instruction.shamt();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(value >> shamt);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn sra(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value = rt.read_u32() as i32;
    let shamt = instruction.shamt();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32((value >> shamt) as u32);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn sllv(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];

    if unlikely(value1 >= 32) {
        rd.write_u32(0);
    } else {
        rd.write_u32(value2 << value1);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn srlv(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];

    if unlikely(value1 >= 32) {
        rd.write_u32(0);
    } else {
        rd.write_u32(value2 >> value1);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn srav(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32;
    let rd = &mut context.r3000_state.gpr[instruction.rd()];

    if unlikely(value1 >= 32) {
        if value2 < 0 {
            rd.write_u32(0xFFFF_FFFF);
        } else {
            rd.write_u32(0);
        }
    } else {
        rd.write_u32((value2 >> value1) as u32);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn jr(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let target = context.r3000_state.gpr[instruction.rs()].read_u32();
    context.r3000_state.branch_delay.set(target, 1);
    Ok(Ok(()))
}

pub(crate) fn jalr(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let target = context.r3000_state.gpr[instruction.rs()].read_u32();
    context.r3000_state.branch_delay.set(target, 1);
    let pc = context.r3000_state.pc.read_u32();
    context.r3000_state.gpr[instruction.rd()].write_u32(pc + INSTRUCTION_SIZE);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn syscall(context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    debug::trace_syscall(context.r3000_state);

    if context.r3000_state.branch_delay.branching() {
        return Err("SYSCALL in branch delay slot not handled".into());
    }

    set_exception(context.r3000_state, context.cp0_state, CAUSE_EXCCODE_SYSCALL);
    Ok(Ok(()))
}

pub(crate) fn break_(_context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    Err("Instruction break not implemented".into())
}

pub(crate) fn mfhi(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let value = context.r3000_state.hi.read_u32();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(value);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn mthi(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value = rs.read_u32();
    context.r3000_state.hi.write_u32(value);
    Ok(Ok(()))
}

pub(crate) fn mflo(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let value = context.r3000_state.lo.read_u32();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(value);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn mtlo(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value = rs.read_u32();
    context.r3000_state.lo.write_u32(value);
    Ok(Ok(()))
}

pub(crate) fn mult(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32() as i32 as i64;
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32 as i64;

    let result = (value1 * value2) as u64;
    let hi_value = ((result >> 32) & 0xFFFF_FFFF) as u32;
    let lo_value = (result & 0xFFFF_FFFF) as u32;

    let hi = &mut context.r3000_state.hi;
    hi.write_u32(hi_value);
    let lo = &mut context.r3000_state.lo;
    lo.write_u32(lo_value);

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn multu(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32() as u64;
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32() as u64;

    let result = value1 * value2;
    let hi_value = (result >> 32) as u32;
    let lo_value = result as u32;

    let hi = &mut context.r3000_state.hi;
    hi.write_u32(hi_value);
    let lo = &mut context.r3000_state.lo;
    lo.write_u32(lo_value);

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn div(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32() as i32;
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32;

    // Undefined results for MIPS if denominator is 0.
    if likely(value2 != 0) {
        let remainder = value1 % value2;
        let quotient = value1 / value2;

        let hi = &mut context.r3000_state.hi;
        let lo = &mut context.r3000_state.lo;
        hi.write_u32(remainder as u32);
        lo.write_u32(quotient as u32);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn divu(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();

    // Undefined results for MIPS if denominator is 0.
    if likely(value2 != 0) {
        let remainder = value1 % value2;
        let quotient = value1 / value2;

        let hi = &mut context.r3000_state.hi;
        let lo = &mut context.r3000_state.lo;
        hi.write_u32(remainder);
        lo.write_u32(quotient);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn add(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let (result, of_flag) = (value1 as i32).overflowing_add(value2 as i32);

    if of_flag {
        return Err("Overflowing exception not implemented (add)".into());
    } else {
        let rd = &mut context.r3000_state.gpr[instruction.rd()];
        rd.write_u32(result as u32);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn addu(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(value1.wrapping_add(value2));
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn sub(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32() as i32;
    let rt = &context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32;
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(value1.wrapping_sub(value2) as u32);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn subu(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(value1.wrapping_sub(value2));
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn and(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(value1 & value2);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn or(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(value1 | value2);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn xor(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(value1 ^ value2);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn nor(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();
    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    rd.write_u32(!(value1 | value2));
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn slt(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32() as i32;
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32() as i32;

    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    if value1 < value2 {
        rd.write_u32(1);
    } else {
        rd.write_u32(0);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn sltu(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value1 = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value2 = rt.read_u32();

    let rd = &mut context.r3000_state.gpr[instruction.rd()];
    if value1 < value2 {
        rd.write_u32(1);
    } else {
        rd.write_u32(0);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn bltz(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = context.r3000_state.gpr[instruction.rs()].read_u32() as i32;

    if value < 0 {
        let pc = context.r3000_state.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        context.r3000_state.branch_delay.set(target, 1);
    }

    Ok(Ok(()))
}

pub(crate) fn bgez(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = context.r3000_state.gpr[instruction.rs()].read_u32() as i32;

    if value >= 0 {
        let pc = context.r3000_state.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        context.r3000_state.branch_delay.set(target, 1);
    }

    Ok(Ok(()))
}

pub(crate) fn bltzal(_context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    Err("Instruction bltzal not implemented".into())
}

pub(crate) fn bgezal(_context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    Err("Instruction bgezal not implemented".into())
}

pub(crate) fn j(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let target = pc_calc_jump_target(context.r3000_state.pc.read_u32(), instruction.addr());
    context.r3000_state.branch_delay.set(target, 1);
    Ok(Ok(()))
}

pub(crate) fn jal(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let target = pc_calc_jump_target(context.r3000_state.pc.read_u32(), instruction.addr());
    context.r3000_state.branch_delay.set(target, 1);

    let pc = context.r3000_state.pc.read_u32();
    context.r3000_state.gpr[31].write_u32(pc + INSTRUCTION_SIZE);

    Ok(Ok(()))
}

pub(crate) fn beq(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let offset = (instruction.i_imm() as i32) << 2;
    let value1 = context.r3000_state.gpr[instruction.rs()].read_u32();
    let value2 = context.r3000_state.gpr[instruction.rt()].read_u32();

    if value1 == value2 {
        let pc = context.r3000_state.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        context.r3000_state.branch_delay.set(target, 1);
    }

    Ok(Ok(()))
}

pub(crate) fn bne(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let offset = (instruction.i_imm() as i32) << 2;
    let value1 = context.r3000_state.gpr[instruction.rs()].read_u32();
    let value2 = context.r3000_state.gpr[instruction.rt()].read_u32();

    if value1 != value2 {
        let pc = context.r3000_state.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        context.r3000_state.branch_delay.set(target, 1);
    }

    Ok(Ok(()))
}

pub(crate) fn blez(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = context.r3000_state.gpr[instruction.rs()].read_u32() as i32;

    if value <= 0 {
        let pc = context.r3000_state.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        context.r3000_state.branch_delay.set(target, 1);
    }

    Ok(Ok(()))
}

pub(crate) fn bgtz(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let offset = (instruction.i_imm() as i32) << 2;
    let value = context.r3000_state.gpr[instruction.rs()].read_u32() as i32;

    if value > 0 {
        let pc = context.r3000_state.pc.read_u32();
        let target = pc.wrapping_add(offset as u32);
        context.r3000_state.branch_delay.set(target, 1);
    }

    Ok(Ok(()))
}

pub(crate) fn addi(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let imm = instruction.i_imm() as i32;
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value: u32 = rs.read_u32();
    let (result, of_flag) = (value as i32).overflowing_add(imm);

    if of_flag {
        return Err("Overflowing exception not implemented (addi)".into());
    } else {
        let rt = &mut context.r3000_state.gpr[instruction.rt()];
        rt.write_u32(result as u32);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn addiu(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let imm = instruction.i_imm() as i32 as u32;
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    rt.write_u32(value.wrapping_add(imm));
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn slti(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value = rs.read_u32() as i32;
    let imm = instruction.i_imm() as i32;

    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    if value < imm {
        rt.write_u32(1);
    } else {
        rt.write_u32(0);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn sltiu(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &mut context.r3000_state.gpr[instruction.rs()];
    let value = rs.read_u32();
    let imm = instruction.i_imm() as i32 as u32;

    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    if value < imm {
        rt.write_u32(1);
    } else {
        rt.write_u32(0);
    }

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn andi(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    rt.write_u32(value & (instruction.u_imm() as u32));
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn ori(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    rt.write_u32(value | (instruction.u_imm() as u32));
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn xori(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rs = &context.r3000_state.gpr[instruction.rs()];
    let value = rs.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    rt.write_u32(value ^ (instruction.u_imm() as u32));
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn lui(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rt = instruction.rt();
    let imm = (instruction.u_imm() as u32) << 16;
    context.r3000_state.gpr[rt].write_u32(imm);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn mfc0(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rd = get_cp0_register(context.cp0_state, instruction.rd());
    let value = rd.read_u32();
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    rt.write_u32(value);
    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn mtc0(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let value = rt.read_u32();
    let rd = get_cp0_register(context.cp0_state, instruction.rd());
    rd.write_u32(value);
    Ok(Ok(()))
}

pub(crate) fn bc0f(_context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    Err("Instruction bc0f not implemented".into())
}

pub(crate) fn bc0t(_context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    Err("Instruction bc0t not implemented".into())
}

pub(crate) fn tlbr(_context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    Err("Instruction tlbr not implemented".into())
}

pub(crate) fn tlbwi(_context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    Err("Instruction tlbwi not implemented".into())
}

pub(crate) fn tlbwr(_context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    Err("Instruction tlbwr not implemented".into())
}

pub(crate) fn tlbp(_context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    Err("Instruction tlbp not implemented".into())
}

pub(crate) fn rfe(context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    debug::trace_rfe(context.r3000_state);
    let status = &mut context.cp0_state.status;
    let old_status_value = status.read_u32();
    let new_status_value = status_pop_exception(old_status_value);
    status.write_u32(new_status_value);

    // Flush the branch delay slot if any.
    let target = context.r3000_state.branch_delay.advance_all();
    if likely(target.is_some()) {
        context.r3000_state.pc.write_u32(target.unwrap());
    }

    // Also flush the cause register to make sure no stray interrupts are pending as a result of the emulator being out
    // of sync temporarily. If there is an actual interrupt pending, then it will be asserted again shortly (R3000
    // interrupts are level triggered).
    clear_ip_field(context.state, context.cp0_state);

    Ok(Ok(()))
}

pub(crate) fn lb(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) {
        match read_u8(context.state, context.r3000_state, addr)?.map(|v| v as i8 as i32 as u32) {
            Ok(v) => v,
            Err(h) => return Ok(Err(h)),
        }
    } else {
        0
    };

    context.r3000_state.gpr[instruction.rt()].write_u32(value);

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn lh(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) {
        match read_u16(context.state, context.r3000_state, addr)?.map(|v| v as i16 as i32 as u32) {
            Ok(v) => v,
            Err(h) => return Ok(Err(h)),
        }
    } else {
        0
    };

    context.r3000_state.gpr[instruction.rt()].write_u32(value);

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn lwl(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    const MASK: [u32; 4] = [0x00FF_FFFF, 0x0000_FFFF, 0x0000_00FF, 0x0000_0000];
    const SHIFT: [usize; 4] = [24, 16, 8, 0];

    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) {
        match read_u32(context.state, context.r3000_state, addr)? {
            Ok(v) => v,
            Err(h) => return Ok(Err(h)),
        }
    } else {
        0
    };

    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let rt_value = rt.read_u32();
    let value = (rt_value & MASK[shift]) | (value << SHIFT[shift]);

    rt.write_u32(value);

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn lw(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) {
        match read_u32(context.state, context.r3000_state, addr)? {
            Ok(v) => v,
            Err(h) => return Ok(Err(h)),
        }
    } else {
        0
    };

    context.r3000_state.gpr[instruction.rt()].write_u32(value);

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn lbu(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) {
        match read_u8(context.state, context.r3000_state, addr)?.map(|v| v as u32) {
            Ok(v) => v,
            Err(h) => return Ok(Err(h)),
        }
    } else {
        0
    };

    context.r3000_state.gpr[instruction.rt()].write_u32(value);

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn lhu(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) {
        match read_u16(context.state, context.r3000_state, addr)?.map(|v| v as u32) {
            Ok(v) => v,
            Err(h) => return Ok(Err(h)),
        }
    } else {
        0
    };

    context.r3000_state.gpr[instruction.rt()].write_u32(value);

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn lwr(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    const MASK: [u32; 4] = [0x0000_0000, 0xFF00_0000, 0xFFFF_0000, 0xFFFF_FF00];
    const SHIFT: [usize; 4] = [0, 8, 16, 24];

    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) {
        match read_u32(context.state, context.r3000_state, addr)? {
            Ok(v) => v,
            Err(h) => return Ok(Err(h)),
        }
    } else {
        0
    };

    let rt = &mut context.r3000_state.gpr[instruction.rt()];
    let rt_value = rt.read_u32();
    let value = (rt_value & MASK[shift]) | (value >> SHIFT[shift]);

    rt.write_u32(value);

    handle_zero(context.r3000_state);
    Ok(Ok(()))
}

pub(crate) fn sb(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let value = context.r3000_state.gpr[instruction.rt()].read_u8(0);
    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;

    if likely(!isc) {
        match write_u8(context.state, context.r3000_state, addr, value)? {
            Ok(()) => {},
            Err(h) => return Ok(Err(h)),
        }
    }

    Ok(Ok(()))
}

pub(crate) fn sh(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let value = context.r3000_state.gpr[instruction.rt()].read_u16(0);
    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;

    if likely(!isc) {
        match write_u16(context.state, context.r3000_state, addr, value)? {
            Ok(()) => {},
            Err(h) => return Ok(Err(h)),
        }
    }

    Ok(Ok(()))
}

pub(crate) fn swl(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    const MASK: [u32; 4] = [0xFFFF_FF00, 0xFFFF_0000, 0xFF00_0000, 0x0000_0000];
    const SHIFT: [usize; 4] = [24, 16, 8, 0];

    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;

    let mem_value = if likely(!isc) {
        match read_u32(context.state, context.r3000_state, addr)? {
            Ok(v) => v,
            Err(h) => return Ok(Err(h)),
        }
    } else {
        0
    };

    let rt_value = context.r3000_state.gpr[instruction.rt()].read_u32();

    let value = (rt_value >> SHIFT[shift]) | (mem_value & MASK[shift]);

    if likely(!isc) {
        match write_u32(context.state, context.r3000_state, addr, value)? {
            Ok(()) => {},
            Err(h) => return Ok(Err(h)),
        }
    }

    Ok(Ok(()))
}

pub(crate) fn sw(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let value = context.r3000_state.gpr[instruction.rt()].read_u32();
    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;

    if likely(!isc) {
        match write_u32(context.state, context.r3000_state, addr, value)? {
            Ok(()) => {},
            Err(h) => return Ok(Err(h)),
        }
    }

    Ok(Ok(()))
}

pub(crate) fn swr(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    const MASK: [u32; 4] = [0x0000_0000, 0x0000_00FF, 0x0000_FFFF, 0x00FF_FFFF];
    const SHIFT: [usize; 4] = [0, 8, 16, 24];

    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let shift = (addr & 3) as usize;
    addr &= !3;

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;

    let mem_value = if likely(!isc) {
        match read_u32(context.state, context.r3000_state, addr)? {
            Ok(v) => v,
            Err(h) => return Ok(Err(h)),
        }
    } else {
        0
    };

    let rt_value = context.r3000_state.gpr[instruction.rt()].read_u32();

    let value = (rt_value << SHIFT[shift]) | (mem_value & MASK[shift]);

    if likely(!isc) {
        match write_u32(context.state, context.r3000_state, addr, value)? {
            Ok(()) => {},
            Err(h) => return Ok(Err(h)),
        }
    }

    Ok(Ok(()))
}
