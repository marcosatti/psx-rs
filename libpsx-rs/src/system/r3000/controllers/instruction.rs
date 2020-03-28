use crate::types::mips1::instruction::Instruction;
use crate::system::Resources;
use crate::controllers::r3000::*;
use crate::controllers::r3000::instruction_impl::*;
use crate::controllers::r3000::instruction_impl_cop2::*;

type InstructionFn = fn(&mut Resources, Instruction) -> InstResult;

pub fn lookup(inst: Instruction) -> Option<(InstructionFn, usize)> {
    match inst.opcode() {
        0 => {
            match inst.funct() {
                0 => {
                    Some((sll, 1))
                },
                2 => {
                    Some((srl, 1))
                },
                3 => {
                    Some((sra, 1))
                },
                4 => {
                    Some((sllv, 1))
                },
                6 => {
                    Some((srlv, 1))
                },
                7 => {
                    Some((srav, 1))
                },
                8 => {
                    Some((jr, 2))
                },
                9 => {
                    Some((jalr, 2))
                },
                12 => {
                    Some((syscall, 2))
                },
                13 => {
                    Some((break_, 2))
                },
                16 => {
                    Some((mfhi, 1))
                },
                17 => {
                    Some((mthi, 1))
                },
                18 => {
                    Some((mflo, 1))
                },
                19 => {
                    Some((mtlo, 1))
                },
                24 => {
                    Some((mult, 1))
                },
                25 => {
                    Some((multu, 1))
                },
                26 => {
                    Some((div, 2))
                },
                27 => {
                    Some((divu, 2))
                },
                32 => {
                    Some((add, 1))
                },
                33 => {
                    Some((addu, 1))
                },
                34 => {
                    Some((sub, 1))
                },
                35 => {
                    Some((subu, 1))
                },
                36 => {
                    Some((and, 1))
                },
                37 => {
                    Some((or, 1))
                },
                38 => {
                    Some((xor, 1))
                },
                39 => {
                    Some((nor, 1))
                },
                42 => {
                    Some((slt, 1))
                },
                43 => {
                    Some((sltu, 1))
                },
                _ => {
                    None
                },
            }
        },
        1 => {
            match inst.rt() {
                0 => {
                    Some((bltz, 2))
                },
                1 => {
                    Some((bgez, 2))
                },
                16 => {
                    Some((bltzal, 2))
                },
                17 => {
                    Some((bgezal, 2))
                },
                _ => {
                    None
                },
            }
        },
        2 => {
            Some((j, 2))
        },
        3 => {
            Some((jal, 2))
        },
        4 => {
            Some((beq, 2))
        },
        5 => {
            Some((bne, 2))
        },
        6 => {
            Some((blez, 2))
        },
        7 => {
            Some((bgtz, 2))
        },
        8 => {
            Some((addi, 1))
        },
        9 => {
            Some((addiu, 1))
        },
        10 => {
            Some((slti, 1))
        },
        11 => {
            Some((sltiu, 1))
        },
        12 => {
            Some((andi, 1))
        },
        13 => {
            Some((ori, 1))
        },
        14 => {
            Some((xori, 1))
        },
        15 => {
            Some((lui, 1))
        },
        16 => {
            match inst.c() {
                false => {
                    match inst.rs4() {
                        0 => {
                            Some((mfc0, 2))
                        },
                        4 => {
                            Some((mtc0, 2))
                        },
                        8 => {
                            match inst.rt() {
                                0 => {
                                    Some((bc0f, 2))
                                },
                                1 => {
                                    Some((bc0t, 2))
                                },
                                _ => {
                                    None
                                },
                            }
                        },
                        _ => {
                            None
                        },
                    }
                },
                true => {
                    match inst.rs4() {
                        0 => {
                            match inst.funct() {
                                1 => {
                                    Some((tlbr, 2))
                                },
                                2 => {
                                    Some((tlbwi, 2))
                                },
                                6 => {
                                    Some((tlbwr, 2))
                                },
                                8 => {
                                    Some((tlbp, 2))
                                },
                                16 => {
                                    Some((rfe, 2))
                                },
                                _ => {
                                    None
                                },
                            }
                        },
                        _ => {
                            None
                        },
                    }
                },
            }
        },
        32 => {
            Some((lb, 2))
        },
        33 => {
            Some((lh, 2))
        },
        34 => {
            Some((lwl, 2))
        },
        35 => {
            Some((lw, 2))
        },
        36 => {
            Some((lbu, 2))
        },
        37 => {
            Some((lhu, 2))
        },
        38 => {
            Some((lwr, 2))
        },
        40 => {
            Some((sb, 2))
        },
        41 => {
            Some((sh, 2))
        },
        42 => {
            Some((swl, 2))
        },
        43 => {
            Some((sw, 2))
        },
        46 => {
            Some((swr, 2))
        },
        18 => {
            lookup_cop2(inst)
        },
        50 => {
            Some((lwc2, 2))
        },
        58 => {
            Some((swc2, 2))
        },
        _ => {
            None
        },
    }
}

pub fn lookup_cop2(inst: Instruction) -> Option<(InstructionFn, usize)> {
    match inst.funct() {
        0 => {
            match inst.rs() {
                0 => {
                    Some((mfc2, 2))
                },
                2 => {
                    Some((cfc2, 2))
                },
                4 => {
                    Some((mtc2, 2))
                },
                6 => {
                    Some((ctc2, 2))
                },
                _ => {
                    None
                },
            }
        },
        1 => {
            Some((rtps, 2))
        },
        6 => {
            Some((nclip, 2))
        },
        12 => {
            Some((op, 2))
        },
        16 => {
            Some((dpcs, 2))
        },
        17 => {
            Some((intpl, 2))
        },
        18 => {
            Some((mvmva, 2))
        },
        19 => {
            Some((ncds, 2))
        },
        20 => {
            Some((cdp, 2))
        },
        22 => {
            Some((ncdt, 2))
        },
        27 => {
            Some((nccs, 2))
        },
        28 => {
            Some((cc, 2))
        },
        30 => {
            Some((ncs, 2))
        },
        32 => {
            Some((nct, 2))
        },
        40 => {
            Some((sqr, 2))
        },
        41 => {
            Some((dcpl, 2))
        },
        42 => {
            Some((dpct, 2))
        },
        45 => {
            Some((avsz3, 2))
        },
        46 => {
            Some((avsz4, 2))
        },
        48 => {
            Some((rtpt, 2))
        },
        61 => {
            Some((gpf, 2))
        },
        62 => {
            Some((gpl, 2))
        },
        63 => {
            Some((ncct, 2))
        },
        _ => {
            None
        },
    }
}
