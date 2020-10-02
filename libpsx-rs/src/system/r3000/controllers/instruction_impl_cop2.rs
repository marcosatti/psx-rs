use crate::{
    system::{
        r3000::{
            controllers::{
                memory_controller::*,
                register::*,
            },
            cp0::constants::STATUS_ISC,
            cp2::types::{
                GteInstruction,
                MultiplyMatrix,
                MultiplyVector,
                TranslationVector,
            },
            types::{
                ControllerContext,
                InstructionResult,
            },
        },
        types::ControllerResult,
    },
    types::{
        bitfield::Bitfield,
        mips1::instruction::Instruction,
    },
    utilities::{
        numeric::*,
        packed::*,
        *,
    },
};
use std::intrinsics::likely;
use typenum::*;

// Note: probably ok to disregard SRA != division by 2^N (https://en.wikipedia.org/wiki/Arithmetic_shift), as it just results in a small rounding error.
// In practice, this means its ok to perform the SRA's as divisions by 4096, etc below.

fn rtps_vector(context: &mut ControllerContext, sf_bit: bool, lm_bit: bool, vector_xy: u32, vector_z_: u32) -> ControllerResult<()> {
    handle_cp2_flag_reset(context.cp2_state);

    if !sf_bit {
        return Err("SF bit was not on - unhandled".into());
    }

    if lm_bit {
        return Err("LM bit was not off - unhandled".into());
    }

    let (vx_value, vy_value) = split_32_i16_f64(vector_xy);
    let (vz_value, _) = split_32_i16_f64(vector_z_);

    let trx_value = context.cp2_state.gc[5].read_u32() as i32 as f64;
    let try_value = context.cp2_state.gc[6].read_u32() as i32 as f64;
    let trz_value = context.cp2_state.gc[7].read_u32() as i32 as f64;

    let (rt11_value, rt12_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[0].read_u32());
    let (rt13_value, rt21_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[1].read_u32());
    let (rt22_value, rt23_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[2].read_u32());
    let (rt31_value, rt32_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[3].read_u32());
    let (rt33_value, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[4].read_u32());

    let mac1_value = trx_value + ((rt11_value * vx_value) + (rt12_value * vy_value) + (rt13_value * vz_value));
    let mac2_value = try_value + ((rt21_value * vx_value) + (rt22_value * vy_value) + (rt23_value * vz_value));
    let mac3_value = trz_value + ((rt31_value * vx_value) + (rt32_value * vy_value) + (rt33_value * vz_value));

    let mac1_overflow_flag = f64::abs(mac1_value) >= ((1u64 << 44) as f64);
    let mac1_negative_flag = mac1_value < 0.0;
    let mac2_overflow_flag = f64::abs(mac2_value) >= ((1u64 << 44) as f64);
    let mac2_negative_flag = mac2_value < 0.0;
    let mac3_overflow_flag = f64::abs(mac3_value) >= ((1u64 << 44) as f64);
    let mac3_negative_flag = mac3_value < 0.0;

    let (ir1_value, ir1_overflow_flag) = checked_clamp(mac1_value, std::i16::MIN as f64, std::i16::MAX as f64);
    let (ir2_value, ir2_overflow_flag) = checked_clamp(mac2_value, std::i16::MIN as f64, std::i16::MAX as f64);
    let (ir3_value, ir3_overflow_flag) = checked_clamp(mac3_value, std::i16::MIN as f64, std::i16::MAX as f64);
    let (sz3_value, sz3_overflow_flag) = checked_clamp(mac3_value, std::u16::MIN as f64, std::u16::MAX as f64);

    let ofx_value = f64::from_fixed_bits_i32::<U16>(context.cp2_state.gc[24].read_u32() as i32);
    let ofy_value = f64::from_fixed_bits_i32::<U16>(context.cp2_state.gc[25].read_u32() as i32);
    let h_value = context.cp2_state.gc[26].read_u16(0) as f64;

    let (plane_constant, plane_overflow_flag) = if h_value < (sz3_value * 2.0) {
        (h_value / sz3_value, false)
    } else {
        return Err("Plane constant overflow - unimplemented".into());
    };

    let sx2_value = ofx_value + ir1_value * plane_constant;
    let (sx2_value, sx2_overflow_flag) = checked_clamp(sx2_value, -(0x400 as f64), 0x3FF as f64);
    let sy2_value = ofy_value + ir2_value * plane_constant;
    let (sy2_value, sy2_overflow_flag) = checked_clamp(sy2_value, -(0x400 as f64), 0x3FF as f64);

    let dqa_value = f64::from_fixed_bits_i16::<U8>(context.cp2_state.gc[27].read_u16(0) as i16);
    let dqb_value = f64::from_fixed_bits_i32::<U24>(context.cp2_state.gc[28].read_u32() as i32);

    let mac0_value = plane_constant * dqa_value + dqb_value;
    let (ir0_value, ir0_overflow_flag) = checked_clamp(mac0_value, 0.0, 0x1000 as f64);
    let ir0_value = f64::to_fixed_bits_i16::<U12>(ir0_value, true);

    let mac0_overflow_flag = f64::abs(mac0_value) >= ((1u64 << 32) as f64);
    let mac0_negative_flag = mac0_value < 0.0;

    // Write back.
    handle_cp2_push_sz(context.cp2_state);
    handle_cp2_push_sxy(context.cp2_state);
    context.cp2_state.gd[25].write_u32(mac1_value as i32 as u32);
    context.cp2_state.gd[9].write_u32(ir1_value as i32 as u32);
    context.cp2_state.gd[26].write_u32(mac2_value as i32 as u32);
    context.cp2_state.gd[10].write_u32(ir2_value as i32 as u32);
    context.cp2_state.gd[27].write_u32(mac3_value as i32 as u32);
    context.cp2_state.gd[11].write_u32(ir3_value as i32 as u32);
    context.cp2_state.gd[19].write_u32(sz3_value as u16 as u32);
    context.cp2_state.gd[14].write_u16(0, sx2_value as i16 as u16);
    context.cp2_state.gd[14].write_u16(1, sy2_value as i16 as u16);
    context.cp2_state.gd[8].write_u32(ir0_value as i32 as u32);
    context.cp2_state.gd[24].write_u32(mac0_value as i32 as u32);

    // Flag register.
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(12, 1), bool_to_flag(ir0_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(13, 1), bool_to_flag(sy2_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(14, 1), bool_to_flag(sx2_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(15, 1), bool_to_flag(mac0_overflow_flag && mac0_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(16, 1), bool_to_flag(mac0_overflow_flag && (!mac0_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(17, 1), bool_to_flag(plane_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(18, 1), bool_to_flag(sz3_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(22, 1), bool_to_flag(ir3_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(23, 1), bool_to_flag(ir2_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(24, 1), bool_to_flag(ir1_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(25, 1), bool_to_flag(mac3_overflow_flag && mac3_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(26, 1), bool_to_flag(mac2_overflow_flag && mac2_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(27, 1), bool_to_flag(mac1_overflow_flag && mac1_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(28, 1), bool_to_flag(mac3_overflow_flag && (!mac3_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(29, 1), bool_to_flag(mac2_overflow_flag && (!mac2_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(30, 1), bool_to_flag(mac1_overflow_flag && (!mac1_negative_flag)));

    handle_cp2_flag_error_bit(context.cp2_state);
    handle_cp2_sxyp_mirror(context.cp2_state);

    Ok(())
}

fn normal_color(context: &mut ControllerContext, shift: bool, lm: bool, color: bool, depth: bool, vector_xy: u32, vector_z_: u32) -> ControllerResult<()> {
    // TODO: proper overflow flag handling.

    if depth {
        if !color {
            return Err("Depth calculation shouldn't be set without color calculation(?)".into());
        }
    }

    if !depth {
        return Err("Assumes depth is on for now".into());
    }

    if !color {
        return Err("Assumes color is on for now".into());
    }

    if !shift {
        return Err("Assumes shift is on for now".into());
    }

    let mut ir_clamp_min = 0;
    if !lm {
        ir_clamp_min = std::i16::MIN;
    }

    handle_cp2_flag_reset(context.cp2_state);

    let (vx_value, vy_value) = split_32_fixedi16_f64::<U12>(vector_xy);
    let (vz_value, _) = split_32_fixedi16_f64::<U12>(vector_z_);

    let (llm11_value, llm12_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[8].read_u32());
    let (llm13_value, llm21_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[9].read_u32());
    let (llm22_value, llm23_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[10].read_u32());
    let (llm31_value, llm32_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[11].read_u32());
    let (llm33_value, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[12].read_u32());

    let mac1_value = (vx_value * llm11_value) + (vy_value * llm12_value) + (vz_value * llm13_value);
    let mac2_value = (vx_value * llm21_value) + (vy_value * llm22_value) + (vz_value * llm23_value);
    let mac3_value = (vx_value * llm31_value) + (vy_value * llm32_value) + (vz_value * llm33_value);

    let (ir1_value, ir1_overflow_flag) = checked_clamp(mac1_value, ir_clamp_min as f64, std::i16::MAX as f64);
    let (ir2_value, ir2_overflow_flag) = checked_clamp(mac2_value, ir_clamp_min as f64, std::i16::MAX as f64);
    let (ir3_value, ir3_overflow_flag) = checked_clamp(mac3_value, ir_clamp_min as f64, std::i16::MAX as f64);

    let (lcm11_value, lcm12_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[16].read_u32());
    let (lcm13_value, lcm21_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[17].read_u32());
    let (lcm22_value, lcm23_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[18].read_u32());
    let (lcm31_value, lcm32_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[19].read_u32());
    let (lcm33_value, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[20].read_u32());

    let rbk_value = f64::from_fixed_bits_i32::<U12>(context.cp2_state.gc[13].read_u32() as i32);
    let gbk_value = f64::from_fixed_bits_i32::<U12>(context.cp2_state.gc[14].read_u32() as i32);
    let bbk_value = f64::from_fixed_bits_i32::<U12>(context.cp2_state.gc[15].read_u32() as i32);

    let mac1_value = rbk_value + (ir1_value * lcm11_value) + (ir2_value * lcm12_value) + (ir3_value * lcm13_value);
    let mac2_value = gbk_value + (ir1_value * lcm21_value) + (ir2_value * lcm22_value) + (ir3_value * lcm23_value);
    let mac3_value = bbk_value + (ir1_value * lcm31_value) + (ir2_value * lcm32_value) + (ir3_value * lcm33_value);

    let (ir1_value, ir1_overflow_flag) = checked_clamp(mac1_value, ir_clamp_min as f64, std::i16::MAX as f64);
    let (ir2_value, ir2_overflow_flag) = checked_clamp(mac2_value, ir_clamp_min as f64, std::i16::MAX as f64);
    let (ir3_value, ir3_overflow_flag) = checked_clamp(mac3_value, ir_clamp_min as f64, std::i16::MAX as f64);

    let r_value = context.cp2_state.gd[6].read_u8(0) as f64;
    let g_value = context.cp2_state.gd[6].read_u8(1) as f64;
    let b_value = context.cp2_state.gd[6].read_u8(2) as f64;
    let code_value = context.cp2_state.gd[6].read_u8(3);

    let rfc_value = f64::from_fixed_bits_i32::<U4>(context.cp2_state.gc[21].read_u32() as i32);
    let gfc_value = f64::from_fixed_bits_i32::<U4>(context.cp2_state.gc[22].read_u32() as i32);
    let bfc_value = f64::from_fixed_bits_i32::<U4>(context.cp2_state.gc[23].read_u32() as i32);

    let (ir0_value, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gd[8].read_u32());

    let mac1_value = (r_value * ir1_value) + ir0_value * (rfc_value - (r_value * ir1_value));
    let mac2_value = (g_value * ir2_value) + ir0_value * (gfc_value - (g_value * ir2_value));
    let mac3_value = (b_value * ir3_value) + ir0_value * (bfc_value - (b_value * ir3_value));

    let (ir1_value, ir1_overflow_flag) = checked_clamp(mac1_value, ir_clamp_min as f64, std::i16::MAX as f64);
    let (ir2_value, ir2_overflow_flag) = checked_clamp(mac2_value, ir_clamp_min as f64, std::i16::MAX as f64);
    let (ir3_value, ir3_overflow_flag) = checked_clamp(mac3_value, ir_clamp_min as f64, std::i16::MAX as f64);

    let mac1_overflow_flag = f64::abs(mac1_value) >= ((1u64 << 44) as f64);
    let mac1_negative_flag = mac1_value < 0.0;
    let mac2_overflow_flag = f64::abs(mac2_value) >= ((1u64 << 44) as f64);
    let mac2_negative_flag = mac2_value < 0.0;
    let mac3_overflow_flag = f64::abs(mac3_value) >= ((1u64 << 44) as f64);
    let mac3_negative_flag = mac3_value < 0.0;

    let rgb1_value = checked_clamp(mac1_value, std::u8::MIN as f64, std::u8::MAX as f64).0;
    let rgb2_value = checked_clamp(mac2_value, std::u8::MIN as f64, std::u8::MAX as f64).0;
    let rgb3_value = checked_clamp(mac3_value, std::u8::MIN as f64, std::u8::MAX as f64).0;

    // Write back.
    handle_cp2_push_rgb(context.cp2_state);
    context.cp2_state.gd[22].write_u8(0, rgb1_value as u8);
    context.cp2_state.gd[22].write_u8(1, rgb2_value as u8);
    context.cp2_state.gd[22].write_u8(2, rgb3_value as u8);
    context.cp2_state.gd[22].write_u8(3, code_value as u8);
    context.cp2_state.gd[25].write_u32(mac1_value as i32 as u32);
    context.cp2_state.gd[9].write_u32(ir1_value as i32 as u32);
    context.cp2_state.gd[26].write_u32(mac2_value as i32 as u32);
    context.cp2_state.gd[10].write_u32(ir2_value as i32 as u32);
    context.cp2_state.gd[27].write_u32(mac3_value as i32 as u32);
    context.cp2_state.gd[11].write_u32(ir3_value as i32 as u32);

    // Flag register.
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(22, 1), bool_to_flag(ir3_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(23, 1), bool_to_flag(ir2_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(24, 1), bool_to_flag(ir1_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(25, 1), bool_to_flag(mac3_overflow_flag && mac3_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(26, 1), bool_to_flag(mac2_overflow_flag && mac2_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(27, 1), bool_to_flag(mac1_overflow_flag && mac1_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(28, 1), bool_to_flag(mac3_overflow_flag && (!mac3_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(29, 1), bool_to_flag(mac2_overflow_flag && (!mac2_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(30, 1), bool_to_flag(mac1_overflow_flag && (!mac1_negative_flag)));

    handle_cp2_flag_error_bit(context.cp2_state);

    Ok(())
}

pub(crate) fn lwc2(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
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

    context.cp2_state.gd[instruction.rt()].write_u32(value);

    handle_cp2_sxyp_write(context.cp2_state, instruction.rt());
    handle_cp2_sxyp_mirror(context.cp2_state);
    Ok(Ok(()))
}

pub(crate) fn swc2(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let value = context.cp2_state.gd[instruction.rt()].read_u32();
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

pub(crate) fn mfc2(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let value = context.cp2_state.gd[instruction.rd()].read_u32();
    context.r3000_state.gpr[instruction.rt()].write_u32(value);
    Ok(Ok(()))
}

pub(crate) fn cfc2(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let value = context.cp2_state.gc[instruction.rd()].read_u32();
    context.r3000_state.gpr[instruction.rt()].write_u32(value);
    Ok(Ok(()))
}

pub(crate) fn mtc2(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let value = context.r3000_state.gpr[instruction.rt()].read_u32();
    context.cp2_state.gd[instruction.rd()].write_u32(value);
    handle_cp2_sxyp_write(context.cp2_state, instruction.rd());
    handle_cp2_sxyp_mirror(context.cp2_state);
    Ok(Ok(()))
}

pub(crate) fn ctc2(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let value = context.r3000_state.gpr[instruction.rt()].read_u32();
    context.cp2_state.gc[instruction.rd()].write_u32(value);
    Ok(Ok(()))
}

pub(crate) fn rtps(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    // Operates on V0.
    let instruction = GteInstruction::new(instruction);
    let vector_0_xy = context.cp2_state.gd[0].read_u32();
    let vector_0_z_ = context.cp2_state.gd[1].read_u32();
    rtps_vector(context, instruction.sf(), instruction.lm(), vector_0_xy, vector_0_z_)?;
    Ok(Ok(()))
}

pub(crate) fn nclip(context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(_instruction);

    handle_cp2_flag_reset(context.cp2_state);

    let sxy0 = context.cp2_state.gd[12].read_u32();
    let sxy1 = context.cp2_state.gd[13].read_u32();
    let sxy2 = context.cp2_state.gd[14].read_u32();

    let (sx0, sy0) = split_32_i16_f64(sxy0);
    let (sx1, sy1) = split_32_i16_f64(sxy1);
    let (sx2, sy2) = split_32_i16_f64(sxy2);

    let mac0_value = (sx0 * sy1) + (sx1 * sy2) + (sx2 * sy0) - (sx0 * sy2) - (sx1 * sy0) - (sx2 * sy1);
    let mac0_overflow_flag = f64::abs(mac0_value) >= ((1u64 << 32) as f64);
    let mac0_negative_flag = mac0_value < 0.0;

    context.cp2_state.gd[24].write_u32(mac0_value as i32 as u32);

    context.cp2_state.gc[31].write_bitfield(Bitfield::new(15, 1), bool_to_flag(mac0_overflow_flag && mac0_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(16, 1), bool_to_flag(mac0_overflow_flag && (!mac0_negative_flag)));

    handle_cp2_flag_error_bit(context.cp2_state);

    Ok(Ok(()))
}

pub(crate) fn op(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction op not implemented".into())
}

pub(crate) fn dpcs(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction dpcs not implemented".into())
}

pub(crate) fn intpl(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction intpl not implemented".into())
}

pub(crate) fn mvmva(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let instruction = GteInstruction::new(instruction);

    if !instruction.sf() {
        return Err("Assumes sf bit is on for now".into());
    }

    let mut ir_clamp_min = 0;
    if !instruction.lm() {
        ir_clamp_min = std::i16::MIN;
    }

    let mx1_raw_value;
    let mx2_raw_value;
    let mx3_raw_value;
    let mx4_raw_value;
    let mx5_raw_value;

    match instruction.mvmva_mm() {
        MultiplyMatrix::Rotation => {
            mx1_raw_value = context.cp2_state.gc[0].read_u32();
            mx2_raw_value = context.cp2_state.gc[1].read_u32();
            mx3_raw_value = context.cp2_state.gc[2].read_u32();
            mx4_raw_value = context.cp2_state.gc[3].read_u32();
            mx5_raw_value = context.cp2_state.gc[4].read_u32();
        },
        MultiplyMatrix::Light => {
            mx1_raw_value = context.cp2_state.gc[8].read_u32();
            mx2_raw_value = context.cp2_state.gc[9].read_u32();
            mx3_raw_value = context.cp2_state.gc[10].read_u32();
            mx4_raw_value = context.cp2_state.gc[11].read_u32();
            mx5_raw_value = context.cp2_state.gc[12].read_u32();
        },
        MultiplyMatrix::Color => {
            mx1_raw_value = context.cp2_state.gc[16].read_u32();
            mx2_raw_value = context.cp2_state.gc[17].read_u32();
            mx3_raw_value = context.cp2_state.gc[18].read_u32();
            mx4_raw_value = context.cp2_state.gc[19].read_u32();
            mx5_raw_value = context.cp2_state.gc[20].read_u32();
        },
        MultiplyMatrix::Reserved => return Err("Invalid mvmva_mm bitfield value".into()),
    }

    let (mx11_value, mx12_value) = split_32_fixedi16_f64::<U12>(mx1_raw_value);
    let (mx13_value, mx21_value) = split_32_fixedi16_f64::<U12>(mx2_raw_value);
    let (mx22_value, mx23_value) = split_32_fixedi16_f64::<U12>(mx3_raw_value);
    let (mx31_value, mx32_value) = split_32_fixedi16_f64::<U12>(mx4_raw_value);
    let (mx33_value, _) = split_32_fixedi16_f64::<U12>(mx5_raw_value);

    let vx1_raw_value;
    let vx2_raw_value;
    let vx3_raw_value;
    let mut ir_mode = false;

    match instruction.mvmva_mv() {
        MultiplyVector::V0 => {
            vx1_raw_value = context.cp2_state.gd[0].read_u32();
            vx2_raw_value = context.cp2_state.gd[1].read_u32();
            vx3_raw_value = 0;
        },
        MultiplyVector::V1 => {
            vx1_raw_value = context.cp2_state.gd[2].read_u32();
            vx2_raw_value = context.cp2_state.gd[3].read_u32();
            vx3_raw_value = 0;
        },
        MultiplyVector::V2 => {
            vx1_raw_value = context.cp2_state.gd[4].read_u32();
            vx2_raw_value = context.cp2_state.gd[5].read_u32();
            vx3_raw_value = 0;
        },
        MultiplyVector::IR => {
            vx1_raw_value = context.cp2_state.gd[9].read_u32();
            vx2_raw_value = context.cp2_state.gd[10].read_u32();
            vx3_raw_value = context.cp2_state.gd[11].read_u32();
            ir_mode = true;
        },
    }

    let vxx_value;
    let vxy_value;
    let vxz_value;

    if !ir_mode {
        let temp = split_32_fixedi16_f64::<U12>(vx1_raw_value);
        vxx_value = temp.0;
        vxy_value = temp.1;
        let temp = split_32_fixedi16_f64::<U12>(vx2_raw_value);
        vxz_value = temp.0;
    } else {
        vxx_value = split_32_fixedi16_f64::<U12>(vx1_raw_value).0;
        vxy_value = split_32_fixedi16_f64::<U12>(vx2_raw_value).0;
        vxz_value = split_32_fixedi16_f64::<U12>(vx3_raw_value).0;
    }

    let txx_value;
    let txy_value;
    let txz_value;

    match instruction.mvmva_tv() {
        TranslationVector::TR => {
            txx_value = context.cp2_state.gc[5].read_u32() as i32 as f64;
            txy_value = context.cp2_state.gc[6].read_u32() as i32 as f64;
            txz_value = context.cp2_state.gc[7].read_u32() as i32 as f64;
        },
        TranslationVector::BK => {
            txx_value = f64::from_fixed_bits_i32::<U12>(context.cp2_state.gc[13].read_u32() as i32);
            txy_value = f64::from_fixed_bits_i32::<U12>(context.cp2_state.gc[14].read_u32() as i32);
            txz_value = f64::from_fixed_bits_i32::<U12>(context.cp2_state.gc[15].read_u32() as i32);
        },
        TranslationVector::FC => return Err("Bugged behaviour not implemented".into()),
        TranslationVector::None => {
            txx_value = 0.0;
            txy_value = 0.0;
            txz_value = 0.0;
        },
    }

    let mac1_value = txx_value + (mx11_value * vxx_value) + (mx12_value * vxy_value) + (mx13_value * vxz_value);
    let mac2_value = txy_value + (mx21_value * vxx_value) + (mx22_value * vxy_value) + (mx23_value * vxz_value);
    let mac3_value = txz_value + (mx31_value * vxx_value) + (mx32_value * vxy_value) + (mx33_value * vxz_value);

    let (ir1_value, ir1_overflow_flag) = checked_clamp(mac1_value, ir_clamp_min as f64, std::i16::MAX as f64);
    let (ir2_value, ir2_overflow_flag) = checked_clamp(mac2_value, ir_clamp_min as f64, std::i16::MAX as f64);
    let (ir3_value, ir3_overflow_flag) = checked_clamp(mac3_value, ir_clamp_min as f64, std::i16::MAX as f64);

    let mac1_overflow_flag = f64::abs(mac1_value) >= ((1u64 << 44) as f64);
    let mac1_negative_flag = mac1_value < 0.0;
    let mac2_overflow_flag = f64::abs(mac2_value) >= ((1u64 << 44) as f64);
    let mac2_negative_flag = mac2_value < 0.0;
    let mac3_overflow_flag = f64::abs(mac3_value) >= ((1u64 << 44) as f64);
    let mac3_negative_flag = mac3_value < 0.0;

    context.cp2_state.gd[25].write_u32(mac1_value as i32 as u32);
    context.cp2_state.gd[9].write_u32(ir1_value as i32 as u32);
    context.cp2_state.gd[26].write_u32(mac2_value as i32 as u32);
    context.cp2_state.gd[10].write_u32(ir2_value as i32 as u32);
    context.cp2_state.gd[27].write_u32(mac3_value as i32 as u32);
    context.cp2_state.gd[11].write_u32(ir3_value as i32 as u32);

    // Flag register.
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(22, 1), bool_to_flag(ir3_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(23, 1), bool_to_flag(ir2_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(24, 1), bool_to_flag(ir1_overflow_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(25, 1), bool_to_flag(mac3_overflow_flag && mac3_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(26, 1), bool_to_flag(mac2_overflow_flag && mac2_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(27, 1), bool_to_flag(mac1_overflow_flag && mac1_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(28, 1), bool_to_flag(mac3_overflow_flag && (!mac3_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(29, 1), bool_to_flag(mac2_overflow_flag && (!mac2_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(30, 1), bool_to_flag(mac1_overflow_flag && (!mac1_negative_flag)));

    handle_cp2_flag_error_bit(context.cp2_state);

    Ok(Ok(()))
}

pub(crate) fn ncds(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    // Operates on V0.
    let instruction = GteInstruction::new(instruction);
    let vector_0_xy = context.cp2_state.gd[0].read_u32();
    let vector_0_z_ = context.cp2_state.gd[1].read_u32();
    normal_color(context, instruction.sf(), instruction.lm(), true, true, vector_0_xy, vector_0_z_)?;
    Ok(Ok(()))
}

pub(crate) fn cdp(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction cdp not implemented".into())
}

pub(crate) fn ncdt(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    // Operates on V0, V1, V2.
    let instruction = GteInstruction::new(instruction);
    for i in 0..3 {
        let vector_xy = context.cp2_state.gd[i * 2 + 0].read_u32();
        let vector_z_ = context.cp2_state.gd[i * 2 + 1].read_u32();
        normal_color(context, instruction.sf(), instruction.lm(), true, true, vector_xy, vector_z_)?;
    }

    Ok(Ok(()))
}

pub(crate) fn nccs(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction nccs not implemented".into())
}

pub(crate) fn cc(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction cc not implemented".into())
}

pub(crate) fn ncs(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction ncs not implemented".into())
}

pub(crate) fn nct(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction nct not implemented".into())
}

pub(crate) fn sqr(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction sqr not implemented".into())
}

pub(crate) fn dcpl(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction dcpl not implemented".into())
}

pub(crate) fn dpct(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction dpct not implemented".into())
}

pub(crate) fn avsz3(context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(_instruction);

    handle_cp2_flag_reset(context.cp2_state);

    let sz1 = context.cp2_state.gd[17].read_u16(0) as f64;
    let sz2 = context.cp2_state.gd[18].read_u16(0) as f64;
    let sz3 = context.cp2_state.gd[19].read_u16(0) as f64;
    let (zsf3, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[29].read_u32());

    let mac0_value = zsf3 * (sz1 + sz2 + sz3);
    let mac0_overflow_flag = f64::abs(mac0_value) >= ((1u64 << 32) as f64);
    let mac0_negative_flag = mac0_value < 0.0;

    let (otz_value, otz_overflow_flag) = checked_clamp(mac0_value, std::u16::MIN as f64, std::u16::MAX as f64);

    context.cp2_state.gd[7].write_u32(otz_value as i32 as u32);
    context.cp2_state.gd[24].write_u32(mac0_value as i32 as u32);

    context.cp2_state.gc[31].write_bitfield(Bitfield::new(15, 1), bool_to_flag(mac0_overflow_flag && mac0_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(16, 1), bool_to_flag(mac0_overflow_flag && (!mac0_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(18, 1), bool_to_flag(otz_overflow_flag));

    handle_cp2_flag_error_bit(context.cp2_state);

    Ok(Ok(()))
}

pub(crate) fn avsz4(context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(_instruction);

    handle_cp2_flag_reset(context.cp2_state);

    let sz0 = context.cp2_state.gd[16].read_u16(0) as f64;
    let sz1 = context.cp2_state.gd[17].read_u16(0) as f64;
    let sz2 = context.cp2_state.gd[18].read_u16(0) as f64;
    let sz3 = context.cp2_state.gd[19].read_u16(0) as f64;
    let (zsf4, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[30].read_u32());

    let mac0_value = zsf4 * (sz0 + sz1 + sz2 + sz3);
    let mac0_overflow_flag = f64::abs(mac0_value) >= ((1u64 << 32) as f64);
    let mac0_negative_flag = mac0_value < 0.0;

    let (otz_value, otz_overflow_flag) = checked_clamp(mac0_value, std::u16::MIN as f64, std::u16::MAX as f64);

    context.cp2_state.gd[7].write_u32(otz_value as i32 as u32);
    context.cp2_state.gd[24].write_u32(mac0_value as i32 as u32);

    context.cp2_state.gc[31].write_bitfield(Bitfield::new(15, 1), bool_to_flag(mac0_overflow_flag && mac0_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(16, 1), bool_to_flag(mac0_overflow_flag && (!mac0_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(18, 1), bool_to_flag(otz_overflow_flag));

    handle_cp2_flag_error_bit(context.cp2_state);

    Ok(Ok(()))
}

pub(crate) fn rtpt(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    // Operates on V0, V1, V2.
    let instruction = GteInstruction::new(instruction);
    for i in 0..3 {
        let vector_xy = context.cp2_state.gd[i * 2 + 0].read_u32();
        let vector_z_ = context.cp2_state.gd[i * 2 + 1].read_u32();
        rtps_vector(context, instruction.sf(), instruction.lm(), vector_xy, vector_z_)?;
    }

    Ok(Ok(()))
}

pub(crate) fn gpf(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction gpf not implemented".into())
}

pub(crate) fn gpl(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction gpl not implemented".into())
}

pub(crate) fn ncct(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction ncct not implemented".into())
}
