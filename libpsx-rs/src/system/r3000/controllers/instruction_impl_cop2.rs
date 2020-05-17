use crate::{
    system::r3000::{
        controllers::{
            memory_controller::*,
            register::*,
        },
        cp0::constants::STATUS_ISC,
        cp2::types::GteInstruction,
        types::{
            ControllerContext,
            InstResult,
        },
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

fn rtps_vector(context: &mut ControllerContext, shift: bool, vector_xy: u32, vector_z_: u32) {
    handle_cp2_flag_reset(context.cp2_state);

    let trx_value = context.cp2_state.gc[5].read_u32() as i32 as f64;
    let try_value = context.cp2_state.gc[6].read_u32() as i32 as f64;
    let trz_value = context.cp2_state.gc[7].read_u32() as i32 as f64;

    let (rt11_value, rt12_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[0].read_u32());
    let (rt13_value, rt21_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[1].read_u32());
    let (rt22_value, rt23_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[2].read_u32());
    let (rt31_value, rt32_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[3].read_u32());
    let (rt33_value, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[4].read_u32());

    let (vx0_value, vy0_value) = split_32_i16_f64(vector_xy);
    let (vz0_value, _) = split_32_i16_f64(vector_z_);

    let mut mac1_value = trx_value * 4096.0 + rt11_value * vx0_value + rt12_value * vy0_value + rt13_value * vz0_value;
    let mut mac2_value = try_value * 4096.0 + rt21_value * vx0_value + rt22_value * vy0_value + rt23_value * vz0_value;
    let mut mac3_value = trz_value * 4096.0 + rt31_value * vx0_value + rt32_value * vy0_value + rt33_value * vz0_value;

    if shift {
        mac1_value /= 4096.0;
        mac2_value /= 4096.0;
        mac3_value /= 4096.0;
    }

    let (ir1_value, ir1_overflow_flag) = checked_clamp(mac1_value, std::i16::MIN as f64, std::i16::MAX as f64);
    let (ir2_value, ir2_overflow_flag) = checked_clamp(mac2_value, std::i16::MIN as f64, std::i16::MAX as f64);
    let (ir3_value, ir3_overflow_flag) = checked_clamp(mac3_value, std::i16::MIN as f64, std::i16::MAX as f64);

    let mac1_overflow_flag = f64::abs(mac1_value) >= ((1u64 << 44) as f64);
    let mac1_negative_flag = mac1_value < 0.0;
    let mac2_overflow_flag = f64::abs(mac2_value) >= ((1u64 << 44) as f64);
    let mac2_negative_flag = mac2_value < 0.0;
    let mac3_overflow_flag = f64::abs(mac3_value) >= ((1u64 << 44) as f64);
    let mac3_negative_flag = mac3_value < 0.0;

    let mut sz3_value = mac3_value;

    if !shift {
        sz3_value /= 4096.0;
    }

    let (sz3_value, sz3_overflow_flag) = checked_clamp(sz3_value, std::u16::MIN as f64, std::u16::MAX as f64);

    let ofx_value = f64::from_fixed_bits_u32::<U16>(context.cp2_state.gc[24].read_u32());
    let ofy_value = f64::from_fixed_bits_u32::<U16>(context.cp2_state.gc[25].read_u32());
    let h_value = context.cp2_state.gc[26].read_u16(0) as f64;
    let dqa_value = f64::from_fixed_bits_u16::<U8>(context.cp2_state.gc[27].read_u16(0));
    let dqb_value = f64::from_fixed_bits_u32::<U24>(context.cp2_state.gc[28].read_u32());

    let mut plane_constant = ((h_value * (0x20000 as f64) / sz3_value) + 1.0) / 2.0;
    let plane_overflow_flag = f64::abs(plane_constant) > (0x1FFFF as f64);

    if plane_overflow_flag {
        plane_constant = 0x1FFFF as f64;
    }

    let mut mac0_value;
    mac0_value = plane_constant * mac1_value + ofx_value;
    let sx2_value = mac0_value / (0x10000 as f64);
    let (sx2_value, sx2_overflow_flag) = checked_clamp(sx2_value, -(0x400 as f64), 0x3FF as f64);
    mac0_value = plane_constant * mac2_value + ofy_value;
    let sy2_value = mac0_value / (0x10000 as f64);
    let (sy2_value, sy2_overflow_flag) = checked_clamp(sy2_value, -(0x400 as f64), 0x3FF as f64);
    mac0_value = plane_constant * dqa_value + dqb_value;
    let ir0_value = mac0_value / (0x10000 as f64);
    let (ir0_value, ir0_overflow_flag) = checked_clamp(ir0_value, 0.0, 0x1000 as f64);

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
    context.cp2_state.gd[8].write_u32(ir0_value as u32);
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
}

fn normal_color(context: &mut ControllerContext, shift: bool, lm: bool, color: bool, depth: bool, vector_xy: u32, vector_z_: u32) {
    if depth {
        assert!(color, "Depth calculation shouldn't be set without color calculation");
    }

    handle_cp2_flag_reset(context.cp2_state);

    let (llm11_value, llm12_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[8].read_u32());
    let (llm13_value, llm21_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[9].read_u32());
    let (llm22_value, llm23_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[10].read_u32());
    let (llm31_value, llm32_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[11].read_u32());
    let (llm33_value, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[12].read_u32());

    let (vx0_value, vy0_value) = split_32_i16_f64(vector_xy);
    let (vz0_value, _) = split_32_i16_f64(vector_z_);

    let mut ir1_value = (vx0_value * llm11_value) + (vy0_value * llm12_value) + (vz0_value * llm13_value);
    let mut ir2_value = (vx0_value * llm21_value) + (vy0_value * llm22_value) + (vz0_value * llm23_value);
    let mut ir3_value = (vx0_value * llm31_value) + (vy0_value * llm32_value) + (vz0_value * llm33_value);

    if shift {
        ir1_value /= 4096.0;
        ir2_value /= 4096.0;
        ir3_value /= 4096.0;
    }

    let (lcm11_value, lcm12_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[16].read_u32());
    let (lcm13_value, lcm21_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[17].read_u32());
    let (lcm22_value, lcm23_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[18].read_u32());
    let (lcm31_value, lcm32_value) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[19].read_u32());
    let (lcm33_value, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gc[20].read_u32());

    let rbk_value = f64::from_fixed_bits_i32::<U12>(context.cp2_state.gc[13].read_u32() as i32);
    let gbk_value = f64::from_fixed_bits_i32::<U12>(context.cp2_state.gc[14].read_u32() as i32);
    let bbk_value = f64::from_fixed_bits_i32::<U12>(context.cp2_state.gc[15].read_u32() as i32);

    let lcm1_value = (ir1_value * lcm11_value) + (ir2_value * lcm12_value) + (ir3_value * lcm13_value);
    let lcm2_value = (ir1_value * lcm21_value) + (ir2_value * lcm22_value) + (ir3_value * lcm23_value);
    let lcm3_value = (ir1_value * lcm31_value) + (ir2_value * lcm32_value) + (ir3_value * lcm33_value);

    ir1_value = (rbk_value * 4096.0) + lcm1_value;
    ir2_value = (gbk_value * 4096.0) + lcm2_value;
    ir3_value = (bbk_value * 4096.0) + lcm3_value;

    if shift {
        ir1_value /= 4096.0;
        ir2_value /= 4096.0;
        ir3_value /= 4096.0;
    }

    let mut mac1_value = ir1_value;
    let mut mac2_value = ir2_value;
    let mut mac3_value = ir3_value;

    if color {
        let r_value = context.cp2_state.gd[6].read_u8(0) as f64;
        let g_value = context.cp2_state.gd[6].read_u8(1) as f64;
        let b_value = context.cp2_state.gd[6].read_u8(2) as f64;

        mac1_value = (r_value * ir1_value) * 16.0;
        mac2_value = (g_value * ir2_value) * 16.0;
        mac3_value = (b_value * ir3_value) * 16.0;

        if depth {
            let rfc_value = f64::from_fixed_bits_i32::<U4>(context.cp2_state.gc[21].read_u32() as i32);
            let gfc_value = f64::from_fixed_bits_i32::<U4>(context.cp2_state.gc[22].read_u32() as i32);
            let bfc_value = f64::from_fixed_bits_i32::<U4>(context.cp2_state.gc[23].read_u32() as i32);
            let (ir0_value, _) = split_32_i16_f64(context.cp2_state.gd[8].read_u32());

            mac1_value = mac1_value + ((rfc_value - mac1_value) * ir0_value);
            mac2_value = mac2_value + ((gfc_value - mac2_value) * ir0_value);
            mac3_value = mac3_value + ((bfc_value - mac3_value) * ir0_value);
        }

        if shift {
            mac1_value /= 4096.0;
            mac2_value /= 4096.0;
            mac3_value /= 4096.0;
        }
    }

    let mut ir_clamp_min = 0;
    if !lm {
        ir_clamp_min = std::i16::MIN;
    }

    let (ir1_value, ir1_overflow_flag) = checked_clamp(mac1_value, ir_clamp_min as f64, std::i16::MAX as f64);
    let (ir2_value, ir2_overflow_flag) = checked_clamp(mac1_value, ir_clamp_min as f64, std::i16::MAX as f64);
    let (ir3_value, ir3_overflow_flag) = checked_clamp(mac1_value, ir_clamp_min as f64, std::i16::MAX as f64);

    mac1_value /= 16.0;
    mac2_value /= 16.0;
    mac3_value /= 16.0;

    let mac1_overflow_flag = f64::abs(mac1_value) >= ((1u64 << 44) as f64);
    let mac1_negative_flag = mac1_value < 0.0;
    let mac2_overflow_flag = f64::abs(mac2_value) >= ((1u64 << 44) as f64);
    let mac2_negative_flag = mac2_value < 0.0;
    let mac3_overflow_flag = f64::abs(mac3_value) >= ((1u64 << 44) as f64);
    let mac3_negative_flag = mac3_value < 0.0;

    let rgb1_value = checked_clamp(mac1_value, std::u8::MIN as f64, std::u8::MAX as f64).0;
    let rgb2_value = checked_clamp(mac2_value, std::u8::MIN as f64, std::u8::MAX as f64).0;
    let rgb3_value = checked_clamp(mac3_value, std::u8::MIN as f64, std::u8::MAX as f64).0;
    let code_value = context.cp2_state.gd[6].read_u8(3);

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
}

pub(crate) fn lwc2(context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;
    let value = if likely(!isc) {
        read_u32(context.state, context.r3000_state, addr)?
    } else {
        0
    };

    context.cp2_state.gd[instruction.rt()].write_u32(value);

    handle_cp2_sxyp_write(context.cp2_state, instruction.rt());
    handle_cp2_sxyp_mirror(context.cp2_state);
    Ok(())
}

pub(crate) fn swc2(context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let value = context.cp2_state.gd[instruction.rt()].read_u32();
    let mut addr = context.r3000_state.gpr[instruction.rs()].read_u32();
    addr = addr.wrapping_add(instruction.i_imm() as i32 as u32);
    addr = translate_address(addr);

    let isc = context.cp0_state.status.read_bitfield(STATUS_ISC) != 0;

    if likely(!isc) {
        write_u32(context.state, context.r3000_state, addr, value)?
    }

    Ok(())
}

pub(crate) fn mfc2(context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let value = context.cp2_state.gd[instruction.rd()].read_u32();
    context.r3000_state.gpr[instruction.rt()].write_u32(value);
    Ok(())
}

pub(crate) fn cfc2(context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let value = context.cp2_state.gc[instruction.rd()].read_u32();
    context.r3000_state.gpr[instruction.rt()].write_u32(value);
    Ok(())
}

pub(crate) fn mtc2(context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let value = context.r3000_state.gpr[instruction.rt()].read_u32();
    context.cp2_state.gd[instruction.rd()].write_u32(value);
    handle_cp2_sxyp_write(context.cp2_state, instruction.rd());
    handle_cp2_sxyp_mirror(context.cp2_state);
    Ok(())
}

pub(crate) fn ctc2(context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let value = context.r3000_state.gpr[instruction.rt()].read_u32();
    context.cp2_state.gc[instruction.rd()].write_u32(value);
    Ok(())
}

pub(crate) fn rtps(context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    // Operates on V0.
    let instruction = GteInstruction::new(instruction);
    let vector_0_xy = context.cp2_state.gd[0].read_u32();
    let vector_0_z_ = context.cp2_state.gd[1].read_u32();
    rtps_vector(context, instruction.sf(), vector_0_xy, vector_0_z_);
    Ok(())
}

pub(crate) fn nclip(context: &mut ControllerContext, _instruction: Instruction) -> InstResult {
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

    Ok(())
}

pub(crate) fn op(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction op not implemented");
}

pub(crate) fn dpcs(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction dpcs not implemented");
}

pub(crate) fn intpl(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction intpl not implemented");
}

pub(crate) fn mvmva(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction mvmva not implemented");
}

pub(crate) fn ncds(context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    // Operates on V0.
    let instruction = GteInstruction::new(instruction);
    let vector_0_xy = context.cp2_state.gd[0].read_u32();
    let vector_0_z_ = context.cp2_state.gd[1].read_u32();
    normal_color(context, instruction.sf(), instruction.lm(), true, true, vector_0_xy, vector_0_z_);
    Ok(())
}

pub(crate) fn cdp(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction cdp not implemented");
}

pub(crate) fn ncdt(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    log::debug!("Instruction ncdt not implemented");
    Ok(())
}

pub(crate) fn nccs(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    log::debug!("Instruction nccs not implemented");
    Ok(())
}

pub(crate) fn cc(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction cc not implemented");
}

pub(crate) fn ncs(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    log::debug!("Instruction ncs not implemented");
    Ok(())
}

pub(crate) fn nct(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    log::debug!("Instruction nct not implemented");
    Ok(())
}

pub(crate) fn sqr(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction sqr not implemented");
}

pub(crate) fn dcpl(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction dcpl not implemented");
}

pub(crate) fn dpct(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction dpct not implemented");
}

pub(crate) fn avsz3(context: &mut ControllerContext, _instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(_instruction);

    handle_cp2_flag_reset(context.cp2_state);

    let sz1 = context.cp2_state.gd[17].read_u16(0) as f64;
    let sz2 = context.cp2_state.gd[18].read_u16(0) as f64;
    let sz3 = context.cp2_state.gd[19].read_u16(0) as f64;
    let (zsf3, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gd[29].read_u32());

    let mac0_value = zsf3 * (sz1 + sz2 + sz3);
    let mac0_overflow_flag = f64::abs(mac0_value) >= ((1u64 << 32) as f64);
    let mac0_negative_flag = mac0_value < 0.0;

    let otz_value = mac0_value / (0x1000 as f64);
    let (otz_value, otz_overflow_flag) = checked_clamp(otz_value, std::u16::MIN as f64, std::u16::MAX as f64);

    context.cp2_state.gd[7].write_u32(otz_value as i32 as u32);
    context.cp2_state.gd[24].write_u32(mac0_value as i32 as u32);

    context.cp2_state.gc[31].write_bitfield(Bitfield::new(15, 1), bool_to_flag(mac0_overflow_flag && mac0_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(16, 1), bool_to_flag(mac0_overflow_flag && (!mac0_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(18, 1), bool_to_flag(otz_overflow_flag));

    handle_cp2_flag_error_bit(context.cp2_state);

    Ok(())
}

pub(crate) fn avsz4(context: &mut ControllerContext, _instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(_instruction);

    handle_cp2_flag_reset(context.cp2_state);

    let sz0 = context.cp2_state.gd[16].read_u16(0) as f64;
    let sz1 = context.cp2_state.gd[17].read_u16(0) as f64;
    let sz2 = context.cp2_state.gd[18].read_u16(0) as f64;
    let sz3 = context.cp2_state.gd[19].read_u16(0) as f64;
    let (zsf4, _) = split_32_fixedi16_f64::<U12>(context.cp2_state.gd[30].read_u32());

    let mac0_value = zsf4 * (sz0 + sz1 + sz2 + sz3);
    let mac0_overflow_flag = f64::abs(mac0_value) >= ((1u64 << 32) as f64);
    let mac0_negative_flag = mac0_value < 0.0;

    let otz_value = mac0_value / (0x1000 as f64);
    let (otz_value, otz_overflow_flag) = checked_clamp(otz_value, std::u16::MIN as f64, std::u16::MAX as f64);

    context.cp2_state.gd[7].write_u32(otz_value as i32 as u32);
    context.cp2_state.gd[24].write_u32(mac0_value as i32 as u32);

    context.cp2_state.gc[31].write_bitfield(Bitfield::new(15, 1), bool_to_flag(mac0_overflow_flag && mac0_negative_flag));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(16, 1), bool_to_flag(mac0_overflow_flag && (!mac0_negative_flag)));
    context.cp2_state.gc[31].write_bitfield(Bitfield::new(18, 1), bool_to_flag(otz_overflow_flag));

    handle_cp2_flag_error_bit(context.cp2_state);

    Ok(())
}

pub(crate) fn rtpt(context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    // Operates on V0, V1, V2.
    let instruction = GteInstruction::new(instruction);
    for i in 0..3 {
        let vector_xy = context.cp2_state.gd[i * 2 + 0].read_u32();
        let vector_z_ = context.cp2_state.gd[i * 2 + 1].read_u32();
        rtps_vector(context, instruction.sf(), vector_xy, vector_z_);
    }

    Ok(())
}

pub(crate) fn gpf(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction gpf not implemented");
}

pub(crate) fn gpl(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction gpl not implemented");
}

pub(crate) fn ncct(_context: &mut ControllerContext, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    log::debug!("Instruction ncct not implemented");
    Ok(())
}
