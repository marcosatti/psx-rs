use crate::resources::Resources;
use crate::types::mips1::instruction::Instruction;
use crate::types::gte::GteInstruction;
use crate::controllers::r3000::InstResult;

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

pub fn rtps(_resources: &mut Resources, instruction: Instruction) -> InstResult {
    let _instruction = GteInstruction::new(instruction);
    unimplemented!("Instruction rtps not implemented");
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
