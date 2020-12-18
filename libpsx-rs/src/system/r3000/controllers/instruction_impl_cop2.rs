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
    types::mips1::instruction::Instruction,
    utilities::*,
};
use std::intrinsics::likely;

// TODO: Proper flag register handling!

fn perspective_transform(context: &mut ControllerContext, sf_bit: bool, lm_bit: bool, r_vector_xy: usize, r_vector_z: usize) -> ControllerResult<()> {
    if !sf_bit {
        return Err("SF bit was not on - unhandled".into());
    }

    if lm_bit {
        return Err("LM bit was not off - unhandled".into());
    }

    let mut ir_clamp_min = 0;
    if !lm_bit {
        ir_clamp_min = std::i16::MIN;
    }

    let vx_value = context.cp2_state.gd[r_vector_xy].read_u16(0) as i16 as i64;
    let vy_value = context.cp2_state.gd[r_vector_xy].read_u16(1) as i16 as i64;
    let vz_value = context.cp2_state.gd[r_vector_z].read_u16(0) as i16 as i64;

    let trx_value = (context.cp2_state.gc[5].read_u32() as i32 as i64) << 12;
    let try_value = (context.cp2_state.gc[6].read_u32() as i32 as i64) << 12;
    let trz_value = (context.cp2_state.gc[7].read_u32() as i32 as i64) << 12;

    let rt11_value = context.cp2_state.gc[0].read_u16(0) as i16 as i64;
    let rt12_value = context.cp2_state.gc[0].read_u16(1) as i16 as i64;
    let rt13_value = context.cp2_state.gc[1].read_u16(0) as i16 as i64;
    let rt21_value = context.cp2_state.gc[1].read_u16(1) as i16 as i64;
    let rt22_value = context.cp2_state.gc[2].read_u16(0) as i16 as i64;
    let rt23_value = context.cp2_state.gc[2].read_u16(1) as i16 as i64;
    let rt31_value = context.cp2_state.gc[3].read_u16(0) as i16 as i64;
    let rt32_value = context.cp2_state.gc[3].read_u16(1) as i16 as i64;
    let rt33_value = context.cp2_state.gc[4].read_u16(0) as i16 as i64;

    let mac1_value = (trx_value + ((rt11_value * vx_value) + (rt12_value * vy_value) + (rt13_value * vz_value))) >> 12;
    let mac2_value = (try_value + ((rt21_value * vx_value) + (rt22_value * vy_value) + (rt23_value * vz_value))) >> 12;
    let mac3_value = (trz_value + ((rt31_value * vx_value) + (rt32_value * vy_value) + (rt33_value * vz_value))) >> 12;

    let (ir1_value, _) = checked_clamp(mac1_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir2_value, _) = checked_clamp(mac2_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir3_value, _) = checked_clamp(mac3_value, ir_clamp_min as i64, std::i16::MAX as i64);

    let (mut sz3_value, _) = checked_clamp(mac3_value, 0, std::u16::MAX as i64);

    if sz3_value == 0 {
        sz3_value = -1;
        log::warn!("SZ3 is 0! FLAG register not handled!");
    }

    let h_value = context.cp2_state.gc[26].read_u16(0) as i16 as i64;

    let h_over_sz3_value = ((h_value * 0x2_0000 / sz3_value) + 1) / 2;
    // TODO: Check min value.
    let (h_over_sz3_value, _) = checked_clamp(h_over_sz3_value, 0, 0x1_FFFF);

    let ofx_value = context.cp2_state.gc[24].read_u32() as i32 as i64;
    let ofy_value = context.cp2_state.gc[25].read_u32() as i32 as i64;

    let sx2_value = (ofx_value + (ir1_value * h_over_sz3_value)) >> 16;
    let (sx2_value, _) = checked_clamp(sx2_value, -0x400, 0x3FF);
    let sy2_value = (ofy_value + (ir2_value * h_over_sz3_value)) >> 16;
    let (sy2_value, _) = checked_clamp(sy2_value, -0x400, 0x3FF);

    let dqa_value = context.cp2_state.gc[27].read_u16(0) as i16 as i64;
    let dqb_value = context.cp2_state.gc[28].read_u32() as i32 as i64;

    let mac0_value = (h_over_sz3_value * dqa_value) + dqb_value;
    let ir0_value = mac0_value >> 12;
    let (ir0_value, _) = checked_clamp(ir0_value, 0, 0x1000);

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

    handle_cp2_sxyp_mirror(context.cp2_state);

    Ok(())
}

fn normal_color_color(context: &mut ControllerContext, shift: bool, lm: bool, r_vector_xy: usize, r_vector_z: usize) -> ControllerResult<()> {
    if !shift {
        return Err("Assumes shift is on for now".into());
    }

    let mut ir_clamp_min = 0;
    if !lm {
        ir_clamp_min = std::i16::MIN;
    }

    let vx_value = context.cp2_state.gd[r_vector_xy].read_u16(0) as i16 as i64;
    let vy_value = context.cp2_state.gd[r_vector_xy].read_u16(1) as i16 as i64;
    let vz_value = context.cp2_state.gd[r_vector_z].read_u16(0) as i16 as i64;

    let llm11_value = context.cp2_state.gc[8].read_u16(0) as i16 as i64;
    let llm12_value = context.cp2_state.gc[8].read_u16(1) as i16 as i64;
    let llm13_value = context.cp2_state.gc[9].read_u16(0) as i16 as i64;
    let llm21_value = context.cp2_state.gc[9].read_u16(1) as i16 as i64;
    let llm22_value = context.cp2_state.gc[10].read_u16(0) as i16 as i64;
    let llm23_value = context.cp2_state.gc[10].read_u16(1) as i16 as i64;
    let llm31_value = context.cp2_state.gc[11].read_u16(0) as i16 as i64;
    let llm32_value = context.cp2_state.gc[11].read_u16(1) as i16 as i64;
    let llm33_value = context.cp2_state.gc[12].read_u16(0) as i16 as i64;

    let mac1_value = ((vx_value * llm11_value) + (vy_value * llm12_value) + (vz_value * llm13_value)) >> 12;
    let mac2_value = ((vx_value * llm21_value) + (vy_value * llm22_value) + (vz_value * llm23_value)) >> 12;
    let mac3_value = ((vx_value * llm31_value) + (vy_value * llm32_value) + (vz_value * llm33_value)) >> 12;

    let (ir1_value, _) = checked_clamp(mac1_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir2_value, _) = checked_clamp(mac2_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir3_value, _) = checked_clamp(mac3_value, ir_clamp_min as i64, std::i16::MAX as i64);

    let lcm11_value = context.cp2_state.gc[16].read_u16(0) as i16 as i64;
    let lcm12_value = context.cp2_state.gc[16].read_u16(1) as i16 as i64;
    let lcm13_value = context.cp2_state.gc[17].read_u16(0) as i16 as i64;
    let lcm21_value = context.cp2_state.gc[17].read_u16(1) as i16 as i64;
    let lcm22_value = context.cp2_state.gc[18].read_u16(0) as i16 as i64;
    let lcm23_value = context.cp2_state.gc[18].read_u16(1) as i16 as i64;
    let lcm31_value = context.cp2_state.gc[19].read_u16(0) as i16 as i64;
    let lcm32_value = context.cp2_state.gc[19].read_u16(1) as i16 as i64;
    let lcm33_value = context.cp2_state.gc[20].read_u16(0) as i16 as i64;

    let rbk_value = (context.cp2_state.gc[13].read_u32() as i32 as i64) << 12;
    let gbk_value = (context.cp2_state.gc[14].read_u32() as i32 as i64) << 12;
    let bbk_value = (context.cp2_state.gc[15].read_u32() as i32 as i64) << 12;

    let mac1_value = (rbk_value + (ir1_value * lcm11_value) + (ir2_value * lcm12_value) + (ir3_value * lcm13_value)) >> 12;
    let mac2_value = (gbk_value + (ir1_value * lcm21_value) + (ir2_value * lcm22_value) + (ir3_value * lcm23_value)) >> 12;
    let mac3_value = (bbk_value + (ir1_value * lcm31_value) + (ir2_value * lcm32_value) + (ir3_value * lcm33_value)) >> 12;

    let (ir1_value, _) = checked_clamp(mac1_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir2_value, _) = checked_clamp(mac2_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir3_value, _) = checked_clamp(mac3_value, ir_clamp_min as i64, std::i16::MAX as i64);

    let r_value = (context.cp2_state.gd[6].read_u8(0) as i64) << 4;
    let g_value = (context.cp2_state.gd[6].read_u8(1) as i64) << 4;
    let b_value = (context.cp2_state.gd[6].read_u8(2) as i64) << 4;
    let code_value = context.cp2_state.gd[6].read_u8(3);

    let mac1_value = (r_value * ir1_value) >> 12;
    let mac2_value = (g_value * ir2_value) >> 12;
    let mac3_value = (b_value * ir3_value) >> 12;

    let (ir1_value, _) = checked_clamp(mac1_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir2_value, _) = checked_clamp(mac2_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir3_value, _) = checked_clamp(mac3_value, ir_clamp_min as i64, std::i16::MAX as i64);

    let rgb_r_value = mac1_value / 16;
    let rgb_g_value = mac2_value / 16;
    let rgb_b_value = mac3_value / 16;

    handle_cp2_push_rgb(context.cp2_state);
    context.cp2_state.gd[22].write_u8(0, rgb_r_value as u8);
    context.cp2_state.gd[22].write_u8(1, rgb_g_value as u8);
    context.cp2_state.gd[22].write_u8(2, rgb_b_value as u8);
    context.cp2_state.gd[22].write_u8(3, code_value);
    context.cp2_state.gd[25].write_u32(mac1_value as i32 as u32);
    context.cp2_state.gd[9].write_u32(ir1_value as i32 as u32);
    context.cp2_state.gd[26].write_u32(mac2_value as i32 as u32);
    context.cp2_state.gd[10].write_u32(ir2_value as i32 as u32);
    context.cp2_state.gd[27].write_u32(mac3_value as i32 as u32);
    context.cp2_state.gd[11].write_u32(ir3_value as i32 as u32);

    Ok(())
}

fn normal_color_depth_cue(context: &mut ControllerContext, shift: bool, lm: bool, r_vector_xy: usize, r_vector_z: usize) -> ControllerResult<()> {
    if !shift {
        return Err("Assumes shift is on for now".into());
    }

    let mut ir_clamp_min = 0;
    if !lm {
        ir_clamp_min = std::i16::MIN;
    }

    let vx_value = context.cp2_state.gd[r_vector_xy].read_u16(0) as i16 as i64;
    let vy_value = context.cp2_state.gd[r_vector_xy].read_u16(1) as i16 as i64;
    let vz_value = context.cp2_state.gd[r_vector_z].read_u16(0) as i16 as i64;

    let llm11_value = context.cp2_state.gc[8].read_u16(0) as i16 as i64;
    let llm12_value = context.cp2_state.gc[8].read_u16(1) as i16 as i64;
    let llm13_value = context.cp2_state.gc[9].read_u16(0) as i16 as i64;
    let llm21_value = context.cp2_state.gc[9].read_u16(1) as i16 as i64;
    let llm22_value = context.cp2_state.gc[10].read_u16(0) as i16 as i64;
    let llm23_value = context.cp2_state.gc[10].read_u16(1) as i16 as i64;
    let llm31_value = context.cp2_state.gc[11].read_u16(0) as i16 as i64;
    let llm32_value = context.cp2_state.gc[11].read_u16(1) as i16 as i64;
    let llm33_value = context.cp2_state.gc[12].read_u16(0) as i16 as i64;

    let mac1_value = ((vx_value * llm11_value) + (vy_value * llm12_value) + (vz_value * llm13_value)) >> 12;
    let mac2_value = ((vx_value * llm21_value) + (vy_value * llm22_value) + (vz_value * llm23_value)) >> 12;
    let mac3_value = ((vx_value * llm31_value) + (vy_value * llm32_value) + (vz_value * llm33_value)) >> 12;

    let (ir1_value, _) = checked_clamp(mac1_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir2_value, _) = checked_clamp(mac2_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir3_value, _) = checked_clamp(mac3_value, ir_clamp_min as i64, std::i16::MAX as i64);

    let lcm11_value = context.cp2_state.gc[16].read_u16(0) as i16 as i64;
    let lcm12_value = context.cp2_state.gc[16].read_u16(1) as i16 as i64;
    let lcm13_value = context.cp2_state.gc[17].read_u16(0) as i16 as i64;
    let lcm21_value = context.cp2_state.gc[17].read_u16(1) as i16 as i64;
    let lcm22_value = context.cp2_state.gc[18].read_u16(0) as i16 as i64;
    let lcm23_value = context.cp2_state.gc[18].read_u16(1) as i16 as i64;
    let lcm31_value = context.cp2_state.gc[19].read_u16(0) as i16 as i64;
    let lcm32_value = context.cp2_state.gc[19].read_u16(1) as i16 as i64;
    let lcm33_value = context.cp2_state.gc[20].read_u16(0) as i16 as i64;

    let rbk_value = (context.cp2_state.gc[13].read_u32() as i32 as i64) << 12;
    let gbk_value = (context.cp2_state.gc[14].read_u32() as i32 as i64) << 12;
    let bbk_value = (context.cp2_state.gc[15].read_u32() as i32 as i64) << 12;

    let mac1_value = (rbk_value + (ir1_value * lcm11_value) + (ir2_value * lcm12_value) + (ir3_value * lcm13_value)) >> 12;
    let mac2_value = (gbk_value + (ir1_value * lcm21_value) + (ir2_value * lcm22_value) + (ir3_value * lcm23_value)) >> 12;
    let mac3_value = (bbk_value + (ir1_value * lcm31_value) + (ir2_value * lcm32_value) + (ir3_value * lcm33_value)) >> 12;

    let (ir1_value, _) = checked_clamp(mac1_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir2_value, _) = checked_clamp(mac2_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir3_value, _) = checked_clamp(mac3_value, ir_clamp_min as i64, std::i16::MAX as i64);

    let r_value = (context.cp2_state.gd[6].read_u8(0) as i64) << 4;
    let g_value = (context.cp2_state.gd[6].read_u8(1) as i64) << 4;
    let b_value = (context.cp2_state.gd[6].read_u8(2) as i64) << 4;
    let code_value = context.cp2_state.gd[6].read_u8(3);

    let mac1_value = r_value * ir1_value;
    let mac2_value = g_value * ir2_value;
    let mac3_value = b_value * ir3_value;

    let rfc_value = (context.cp2_state.gc[21].read_u32() as i32 as i64) << 12;
    let gfc_value = (context.cp2_state.gc[22].read_u32() as i32 as i64) << 12;
    let bfc_value = (context.cp2_state.gc[23].read_u32() as i32 as i64) << 12;

    let ir1_value = (rfc_value - mac1_value) >> 12;
    let (ir1_value, _) = checked_clamp(ir1_value, std::i16::MIN as i64, std::i16::MAX as i64);
    let ir2_value = (gfc_value - mac2_value) >> 12;
    let (ir2_value, _) = checked_clamp(ir2_value, std::i16::MIN as i64, std::i16::MAX as i64);
    let ir3_value = (bfc_value - mac3_value) >> 12;
    let (ir3_value, _) = checked_clamp(ir3_value, std::i16::MIN as i64, std::i16::MAX as i64);

    let ir0_value = context.cp2_state.gd[8].read_u16(0) as i16 as i64;

    let mac1_value = (mac1_value + ir1_value * ir0_value) >> 12;
    let mac2_value = (mac2_value + ir2_value * ir0_value) >> 12;
    let mac3_value = (mac3_value + ir3_value * ir0_value) >> 12;

    let (ir1_value, _) = checked_clamp(mac1_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir2_value, _) = checked_clamp(mac2_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir3_value, _) = checked_clamp(mac3_value, ir_clamp_min as i64, std::i16::MAX as i64);

    let rgb_r_value = mac1_value / 16;
    let rgb_g_value = mac2_value / 16;
    let rgb_b_value = mac3_value / 16;

    handle_cp2_push_rgb(context.cp2_state);
    context.cp2_state.gd[22].write_u8(0, rgb_r_value as u8);
    context.cp2_state.gd[22].write_u8(1, rgb_g_value as u8);
    context.cp2_state.gd[22].write_u8(2, rgb_b_value as u8);
    context.cp2_state.gd[22].write_u8(3, code_value);
    context.cp2_state.gd[25].write_u32(mac1_value as i32 as u32);
    context.cp2_state.gd[9].write_u32(ir1_value as i32 as u32);
    context.cp2_state.gd[26].write_u32(mac2_value as i32 as u32);
    context.cp2_state.gd[10].write_u32(ir2_value as i32 as u32);
    context.cp2_state.gd[27].write_u32(mac3_value as i32 as u32);
    context.cp2_state.gd[11].write_u32(ir3_value as i32 as u32);

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
    perspective_transform(context, instruction.sf(), instruction.lm(), 0, 1)?;
    Ok(Ok(()))
}

pub(crate) fn nclip(context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(_instruction);

    let sx0 = context.cp2_state.gd[12].read_u16(0) as i16 as i64;
    let sy0 = context.cp2_state.gd[12].read_u16(1) as i16 as i64;
    let sx1 = context.cp2_state.gd[13].read_u16(0) as i16 as i64;
    let sy1 = context.cp2_state.gd[13].read_u16(1) as i16 as i64;
    let sx2 = context.cp2_state.gd[14].read_u16(0) as i16 as i64;
    let sy2 = context.cp2_state.gd[14].read_u16(1) as i16 as i64;

    let mac0_value = (sx0 * sy1) + (sx1 * sy2) + (sx2 * sy0) - (sx0 * sy2) - (sx1 * sy0) - (sx2 * sy1);

    context.cp2_state.gd[24].write_u32(mac0_value as i32 as u32);

    Ok(Ok(()))
}

pub(crate) fn op(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction op not implemented".into())
}

pub(crate) fn dpcs(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let instruction = GteInstruction::new(instruction);

    if !instruction.sf() {
        return Err("Assumes sf bit is on for now".into());
    }

    let mut ir_clamp_min = 0;
    if !instruction.lm() {
        ir_clamp_min = std::i16::MIN;
    }

    let r_value = (context.cp2_state.gd[6].read_u8(0) as i64) << 16;
    let g_value = (context.cp2_state.gd[6].read_u8(1) as i64) << 16;
    let b_value = (context.cp2_state.gd[6].read_u8(2) as i64) << 16;
    let code_value = context.cp2_state.gd[6].read_u8(3);

    let rfc_value = (context.cp2_state.gc[21].read_u32() as i32 as i64) << 12;
    let gfc_value = (context.cp2_state.gc[22].read_u32() as i32 as i64) << 12;
    let bfc_value = (context.cp2_state.gc[23].read_u32() as i32 as i64) << 12;

    let ir1_value = (rfc_value - r_value) >> 12;
    let (ir1_value, _) = checked_clamp(ir1_value, std::i16::MIN as i64, std::i16::MAX as i64);
    let ir2_value = (gfc_value - g_value) >> 12;
    let (ir2_value, _) = checked_clamp(ir2_value, std::i16::MIN as i64, std::i16::MAX as i64);
    let ir3_value = (bfc_value - b_value) >> 12;
    let (ir3_value, _) = checked_clamp(ir3_value, std::i16::MIN as i64, std::i16::MAX as i64);

    let ir0_value = context.cp2_state.gd[8].read_u16(0) as i16 as i64;

    let mac1_value = (r_value + ir1_value * ir0_value) >> 12;
    let mac2_value = (g_value + ir2_value * ir0_value) >> 12;
    let mac3_value = (b_value + ir3_value * ir0_value) >> 12;

    let (ir1_value, _) = checked_clamp(mac1_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir2_value, _) = checked_clamp(mac2_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir3_value, _) = checked_clamp(mac3_value, ir_clamp_min as i64, std::i16::MAX as i64);

    let rgb_r_value = mac1_value / 16;
    let rgb_g_value = mac2_value / 16;
    let rgb_b_value = mac3_value / 16;

    handle_cp2_push_rgb(context.cp2_state);
    context.cp2_state.gd[22].write_u8(0, rgb_r_value as u8);
    context.cp2_state.gd[22].write_u8(1, rgb_g_value as u8);
    context.cp2_state.gd[22].write_u8(2, rgb_b_value as u8);
    context.cp2_state.gd[22].write_u8(3, code_value);
    context.cp2_state.gd[25].write_u32(mac1_value as i32 as u32);
    context.cp2_state.gd[9].write_u32(ir1_value as i32 as u32);
    context.cp2_state.gd[26].write_u32(mac2_value as i32 as u32);
    context.cp2_state.gd[10].write_u32(ir2_value as i32 as u32);
    context.cp2_state.gd[27].write_u32(mac3_value as i32 as u32);
    context.cp2_state.gd[11].write_u32(ir3_value as i32 as u32);

    Ok(Ok(()))
}

pub(crate) fn intpl(_context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(instruction);
    Err("Instruction intpl not implemented".into())
}

pub(crate) fn mvmva(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    let instruction = GteInstruction::new(instruction);

    if !instruction.sf() {
        return Err("sf bit was off - unhandled".into());
    }

    let mut ir_clamp_min = 0;
    if !instruction.lm() {
        ir_clamp_min = std::i16::MIN;
    }

    let mx11_value;
    let mx12_value;
    let mx13_value;
    let mx21_value;
    let mx22_value;
    let mx23_value;
    let mx31_value;
    let mx32_value;
    let mx33_value;

    match instruction.mvmva_mm() {
        MultiplyMatrix::Rotation => {
            mx11_value = context.cp2_state.gc[0].read_u16(0) as i16 as i64;
            mx12_value = context.cp2_state.gc[0].read_u16(1) as i16 as i64;
            mx13_value = context.cp2_state.gc[1].read_u16(0) as i16 as i64;
            mx21_value = context.cp2_state.gc[1].read_u16(1) as i16 as i64;
            mx22_value = context.cp2_state.gc[2].read_u16(0) as i16 as i64;
            mx23_value = context.cp2_state.gc[2].read_u16(1) as i16 as i64;
            mx31_value = context.cp2_state.gc[3].read_u16(0) as i16 as i64;
            mx32_value = context.cp2_state.gc[3].read_u16(1) as i16 as i64;
            mx33_value = context.cp2_state.gc[4].read_u16(0) as i16 as i64;
        },
        MultiplyMatrix::Light => {
            mx11_value = context.cp2_state.gc[8].read_u16(0) as i16 as i64;
            mx12_value = context.cp2_state.gc[8].read_u16(1) as i16 as i64;
            mx13_value = context.cp2_state.gc[9].read_u16(0) as i16 as i64;
            mx21_value = context.cp2_state.gc[9].read_u16(1) as i16 as i64;
            mx22_value = context.cp2_state.gc[10].read_u16(0) as i16 as i64;
            mx23_value = context.cp2_state.gc[10].read_u16(1) as i16 as i64;
            mx31_value = context.cp2_state.gc[11].read_u16(0) as i16 as i64;
            mx32_value = context.cp2_state.gc[11].read_u16(1) as i16 as i64;
            mx33_value = context.cp2_state.gc[12].read_u16(0) as i16 as i64;
        },
        MultiplyMatrix::Color => {
            mx11_value = context.cp2_state.gc[16].read_u16(0) as i16 as i64;
            mx12_value = context.cp2_state.gc[16].read_u16(1) as i16 as i64;
            mx13_value = context.cp2_state.gc[17].read_u16(0) as i16 as i64;
            mx21_value = context.cp2_state.gc[17].read_u16(1) as i16 as i64;
            mx22_value = context.cp2_state.gc[18].read_u16(0) as i16 as i64;
            mx23_value = context.cp2_state.gc[18].read_u16(1) as i16 as i64;
            mx31_value = context.cp2_state.gc[19].read_u16(0) as i16 as i64;
            mx32_value = context.cp2_state.gc[19].read_u16(1) as i16 as i64;
            mx33_value = context.cp2_state.gc[20].read_u16(0) as i16 as i64;
        },
        MultiplyMatrix::Reserved => return Err("Invalid mvmva_mm bitfield value".into()),
    }

    let vxx_value;
    let vxy_value;
    let vxz_value;

    match instruction.mvmva_mv() {
        MultiplyVector::V0 => {
            vxx_value = context.cp2_state.gd[0].read_u16(0) as i16 as i64;
            vxy_value = context.cp2_state.gd[0].read_u16(1) as i16 as i64;
            vxz_value = context.cp2_state.gd[1].read_u16(0) as i16 as i64;
        },
        MultiplyVector::V1 => {
            vxx_value = context.cp2_state.gd[2].read_u16(0) as i16 as i64;
            vxy_value = context.cp2_state.gd[2].read_u16(1) as i16 as i64;
            vxz_value = context.cp2_state.gd[3].read_u16(0) as i16 as i64;
        },
        MultiplyVector::V2 => {
            vxx_value = context.cp2_state.gd[4].read_u16(0) as i16 as i64;
            vxy_value = context.cp2_state.gd[4].read_u16(1) as i16 as i64;
            vxz_value = context.cp2_state.gd[5].read_u16(0) as i16 as i64;
        },
        MultiplyVector::IR => {
            vxx_value = context.cp2_state.gd[9].read_u16(0) as i16 as i64;
            vxy_value = context.cp2_state.gd[10].read_u16(0) as i16 as i64;
            vxz_value = context.cp2_state.gd[11].read_u16(0) as i16 as i64;
        },
    }

    let txx_value;
    let txy_value;
    let txz_value;

    match instruction.mvmva_tv() {
        TranslationVector::TR => {
            txx_value = (context.cp2_state.gc[5].read_u32() as i32 as i64) << 12;
            txy_value = (context.cp2_state.gc[6].read_u32() as i32 as i64) << 12;
            txz_value = (context.cp2_state.gc[7].read_u32() as i32 as i64) << 12;
        },
        TranslationVector::BK => {
            txx_value = (context.cp2_state.gc[13].read_u32() as i32 as i64) << 12;
            txy_value = (context.cp2_state.gc[14].read_u32() as i32 as i64) << 12;
            txz_value = (context.cp2_state.gc[15].read_u32() as i32 as i64) << 12;
        },
        TranslationVector::FC => return Err("Bugged behaviour not implemented".into()),
        TranslationVector::None => {
            txx_value = 0;
            txy_value = 0;
            txz_value = 0;
        },
    }

    let mac1_value = txx_value + (mx11_value * vxx_value) + (mx12_value * vxy_value) + (mx13_value * vxz_value);
    let mac2_value = txy_value + (mx21_value * vxx_value) + (mx22_value * vxy_value) + (mx23_value * vxz_value);
    let mac3_value = txz_value + (mx31_value * vxx_value) + (mx32_value * vxy_value) + (mx33_value * vxz_value);

    let mac1_value = mac1_value >> 12;
    let mac2_value = mac2_value >> 12;
    let mac3_value = mac3_value >> 12;

    let (ir1_value, _) = checked_clamp(mac1_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir2_value, _) = checked_clamp(mac2_value, ir_clamp_min as i64, std::i16::MAX as i64);
    let (ir3_value, _) = checked_clamp(mac3_value, ir_clamp_min as i64, std::i16::MAX as i64);

    context.cp2_state.gd[25].write_u32(mac1_value as i32 as u32);
    context.cp2_state.gd[9].write_u32(ir1_value as i32 as u32);
    context.cp2_state.gd[26].write_u32(mac2_value as i32 as u32);
    context.cp2_state.gd[10].write_u32(ir2_value as i32 as u32);
    context.cp2_state.gd[27].write_u32(mac3_value as i32 as u32);
    context.cp2_state.gd[11].write_u32(ir3_value as i32 as u32);

    Ok(Ok(()))
}

pub(crate) fn ncds(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    // Operates on V0.
    let instruction = GteInstruction::new(instruction);
    normal_color_depth_cue(context, instruction.sf(), instruction.lm(), 0, 1)?;
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
        let r_vector_xy = i * 2 + 0;
        let r_vector_z = i * 2 + 1;
        normal_color_depth_cue(context, instruction.sf(), instruction.lm(), r_vector_xy, r_vector_z)?;
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

    let sz1 = context.cp2_state.gd[17].read_u16(0) as i64;
    let sz2 = context.cp2_state.gd[18].read_u16(0) as i64;
    let sz3 = context.cp2_state.gd[19].read_u16(0) as i64;
    let zsf3 = context.cp2_state.gc[29].read_u16(0) as i16 as i64;

    let mac0_value = zsf3 * (sz1 + sz2 + sz3);

    let otz_value = mac0_value >> 12;
    let (otz_value, _) = checked_clamp(otz_value, 0, std::u16::MAX as i64);
    let otz_value = otz_value as u16;

    context.cp2_state.gd[24].write_u32(mac0_value as i32 as u32);
    context.cp2_state.gd[7].write_u32(otz_value as u32);

    Ok(Ok(()))
}

pub(crate) fn avsz4(context: &mut ControllerContext, _instruction: Instruction) -> ControllerResult<InstructionResult> {
    let _instruction = GteInstruction::new(_instruction);

    let sz0 = context.cp2_state.gd[16].read_u16(0) as i64;
    let sz1 = context.cp2_state.gd[17].read_u16(0) as i64;
    let sz2 = context.cp2_state.gd[18].read_u16(0) as i64;
    let sz3 = context.cp2_state.gd[19].read_u16(0) as i64;
    let zsf4 = context.cp2_state.gc[30].read_u16(0) as i16 as i64;

    let mac0_value = zsf4 * (sz0 + sz1 + sz2 + sz3);

    let otz_value = mac0_value >> 12;
    let (otz_value, _) = checked_clamp(otz_value, 0, std::u16::MAX as i64);
    let otz_value = otz_value as u16;

    context.cp2_state.gd[24].write_u32(mac0_value as i32 as u32);
    context.cp2_state.gd[7].write_u32(otz_value as u32);

    Ok(Ok(()))
}

pub(crate) fn rtpt(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    // Operates on V0, V1, V2.
    let instruction = GteInstruction::new(instruction);
    for i in 0..3 {
        let r_vector_xy = i * 2 + 0;
        let r_vector_z = i * 2 + 1;
        perspective_transform(context, instruction.sf(), instruction.lm(), r_vector_xy, r_vector_z)?;
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

pub(crate) fn ncct(context: &mut ControllerContext, instruction: Instruction) -> ControllerResult<InstructionResult> {
    // Operates on V0, V1, V2.
    let instruction = GteInstruction::new(instruction);
    for i in 0..3 {
        let r_vector_xy = i * 2 + 0;
        let r_vector_z = i * 2 + 1;
        normal_color_color(context, instruction.sf(), instruction.lm(), r_vector_xy, r_vector_z)?;
    }

    Ok(Ok(()))
}
