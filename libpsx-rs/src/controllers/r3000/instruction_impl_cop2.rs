use typenum::*;
use crate::utilities::numeric::*;
use crate::utilities::packed::*;
use crate::resources::Resources;
use crate::types::mips1::instruction::Instruction;
use crate::resources::r3000::cp2::instruction::GteInstruction;
use crate::controllers::r3000::InstResult;

fn push_sz(resources: &mut Resources) {
    let sz1_value = resources.r3000.cp2.gd[17].read_u32();
    let sz2_value = resources.r3000.cp2.gd[18].read_u32();
    let sz3_value = resources.r3000.cp2.gd[19].read_u32();
    resources.r3000.cp2.gd[16].write_u32(sz1_value); // SZ0 = SZ1
    resources.r3000.cp2.gd[17].write_u32(sz2_value); // SZ1 = SZ2
    resources.r3000.cp2.gd[18].write_u32(sz3_value); // SZ2 = SZ3
}

fn push_sxy(resources: &mut Resources) {
    let sxy1_value = resources.r3000.cp2.gd[13].read_u32();
    let sxy2_value = resources.r3000.cp2.gd[14].read_u32();
    resources.r3000.cp2.gd[12].write_u32(sxy1_value); // SXY0 = SXY1
    resources.r3000.cp2.gd[13].write_u32(sxy2_value); // SXY1 = SXY2
}

pub fn mfc2(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let value = resources.r3000.cp2.gd[instruction.rd()].read_u32();
    resources.r3000.gpr[instruction.rt()].write_u32(value);
    Ok(())
}

pub fn cfc2(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let value = resources.r3000.cp2.gc[instruction.rd()].read_u32();
    resources.r3000.gpr[instruction.rt()].write_u32(value);
    Ok(())
}

pub fn mtc2(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let value = resources.r3000.gpr[instruction.rt()].read_u32();
    resources.r3000.cp2.gd[instruction.rd()].write_u32(value);
    Ok(())
}

pub fn ctc2(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let value = resources.r3000.gpr[instruction.rt()].read_u32();
    resources.r3000.cp2.gc[instruction.rd()].write_u32(value);
    Ok(())
}

pub fn rtps(resources: &mut Resources, instruction: Instruction) -> InstResult {
    let instruction = GteInstruction::new(instruction);

    // Perform calculations.
    let trx_value = resources.r3000.cp2.gc[5].read_u32() as i32 as f64;
    let try_value = resources.r3000.cp2.gc[6].read_u32() as i32 as f64;
    let trz_value = resources.r3000.cp2.gc[7].read_u32() as i32 as f64;

    let (rt11_value, rt12_value) = split_32_fixedi16_f64::<U12>(resources.r3000.cp2.gc[0].read_u32());
    let (rt13_value, rt21_value) = split_32_fixedi16_f64::<U12>(resources.r3000.cp2.gc[1].read_u32());
    let (rt22_value, rt23_value) = split_32_fixedi16_f64::<U12>(resources.r3000.cp2.gc[2].read_u32());
    let (rt31_value, rt32_value) = split_32_fixedi16_f64::<U12>(resources.r3000.cp2.gc[3].read_u32());
    let (rt33_value, _) = split_32_fixedi16_f64::<U12>(resources.r3000.cp2.gc[4].read_u32());

    let (vx0_value, vy0_value) = split_32_i16_f64(resources.r3000.cp2.gd[0].read_u32());
    let (vz0_value, _) = split_32_i16_f64(resources.r3000.cp2.gd[1].read_u32());

    let constant = 0x1000 as f64;
    let mut mac1_value = trx_value * constant + rt11_value * vx0_value + rt12_value * vy0_value + rt13_value * vz0_value;
    let mut mac2_value = try_value * constant + rt21_value * vx0_value + rt22_value * vy0_value + rt23_value * vz0_value;
    let mut mac3_value = trz_value * constant + rt31_value * vx0_value + rt32_value * vy0_value + rt33_value * vz0_value;
    
    if instruction.sf() {
        // Equivilant to SAR 12.
        mac1_value /= 4096.0;
        mac2_value /= 4096.0;
        mac3_value /= 4096.0;
    }

    let mut sz3_value = mac3_value;

    if !instruction.sf() {
        // Equivilant to SAR 12.
        sz3_value /= 4096.0;
    }

    let ofx_value = f64::from_fixed_bits_u32::<U16>(resources.r3000.cp2.gc[24].read_u32());
    let ofy_value = f64::from_fixed_bits_u32::<U16>(resources.r3000.cp2.gc[25].read_u32());
    let h_value = resources.r3000.cp2.gc[26].read_u16(0) as f64;
    let dqa_value = f64::from_fixed_bits_u16::<U8>(resources.r3000.cp2.gc[27].read_u16(0));
    let dqb_value = f64::from_fixed_bits_u32::<U24>(resources.r3000.cp2.gc[28].read_u32());

    let constant = ((h_value * (0x20000 as f64) / sz3_value) + 1.0) / 2.0;

    let mut mac0_value;
    mac0_value = constant * mac1_value + ofx_value;
    let sx2_value = mac0_value / (0x10000 as f64);
    mac0_value = constant * mac2_value + ofy_value;
    let sy2_value = mac0_value / (0x10000 as f64);
    mac0_value = constant * dqa_value + dqb_value;
    let ir0_value = mac0_value / (0x10000 as f64);

    // Write back.
    // TODO: saturation / clamping, proper casting etc.
    push_sz(resources);
    push_sxy(resources);
    resources.r3000.cp2.gd[25].write_u32(mac1_value as i32 as u32); // MAC1
    resources.r3000.cp2.gd[9].write_u32(mac1_value as i32 as u32); // IR1
    resources.r3000.cp2.gd[26].write_u32(mac2_value as i32 as u32); // MAC2
    resources.r3000.cp2.gd[10].write_u32(mac2_value as i32 as u32); // IR2
    resources.r3000.cp2.gd[27].write_u32(mac3_value as i32 as u32); // MAC3
    resources.r3000.cp2.gd[11].write_u32(mac3_value as i32 as u32); // IR3
    resources.r3000.cp2.gd[19].write_u32(sz3_value as i32 as u32); // SZ3
    resources.r3000.cp2.gd[14].write_u16(0, sx2_value as i32 as i16 as u16); // SX2
    resources.r3000.cp2.gd[14].write_u16(1, sy2_value as i32 as i16 as u16); // SY2
    resources.r3000.cp2.gd[8].write_u32(ir0_value as i32 as u32); // IR0
    resources.r3000.cp2.gd[24].write_u32(mac0_value as i32 as u32); // MAC0
    resources.r3000.cp2.gd[15].write_u16(0, sx2_value as i32 as i16 as u16); // SXP
    resources.r3000.cp2.gd[15].write_u16(1, sy2_value as i32 as i16 as u16); // SYP

    Ok(())
}

pub fn nclip(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction nclip not implemented");
}

pub fn op(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction op not implemented");
}

pub fn dpcs(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction dpcs not implemented");
}

pub fn intpl(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction intpl not implemented");
}

pub fn mvmva(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction mvmva not implemented");
}

pub fn ncds(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction ncds not implemented");
}

pub fn cdp(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction cdp not implemented");
}

pub fn ncdt(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction ncdt not implemented");
}

pub fn nccs(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction nccs not implemented");
}

pub fn cc(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction cc not implemented");
}

pub fn ncs(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction ncs not implemented");
}

pub fn nct(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction nct not implemented");
}

pub fn sqr(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction sqr not implemented");
}

pub fn dcpl(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction dcpl not implemented");
}

pub fn dpct(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction dpct not implemented");
}

pub fn avsz3(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction avsz3 not implemented");
}

pub fn avsz4(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction avsz4 not implemented");
}

pub fn rtpt(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction rtpt not implemented");
    // IR1 = MAC1 = (TRX*1000h + RT11*VX0 + RT12*VY0 + RT13*VZ0) SAR (sf*12)
    // IR2 = MAC2 = (TRY*1000h + RT21*VX0 + RT22*VY0 + RT23*VZ0) SAR (sf*12)
    // IR3 = MAC3 = (TRZ*1000h + RT31*VX0 + RT32*VY0 + RT33*VZ0) SAR (sf*12)
    // SZ3 = MAC3 SAR ((1-sf)*12)                           ;ScreenZ FIFO 0..+FFFFh
    // MAC0=(((H*20000h/SZ3)+1)/2)*IR1+OFX, SX2=MAC0/10000h ;ScrX FIFO -400h..+3FFh
    // MAC0=(((H*20000h/SZ3)+1)/2)*IR2+OFY, SY2=MAC0/10000h ;ScrY FIFO -400h..+3FFh
    // MAC0=(((H*20000h/SZ3)+1)/2)*DQA+DQB, IR0=MAC0/1000h  ;Depth cueing 0..+1000h
    // Repeat for V1, V2.
}

pub fn gpf(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction gpf not implemented");
}

pub fn gpl(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction gpl not implemented");
}

pub fn ncct(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction ncct not implemented");
}
