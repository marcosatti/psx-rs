type InstructionFn = unsafe fn(&State, Instruction);
pub fn lookup(inst: Instruction) -> Option<(InstructionFn, &'static str, usize)> {
    match inst.opcode() {
        0 => {
            match inst.funct() {
                0 => {
                    Some((sll, "sll", 1))
                },
                2 => {
                    Some((srl, "srl", 1))
                },
                3 => {
                    Some((sra, "sra", 1))
                },
                4 => {
                    Some((sllv, "sllv", 1))
                },
                6 => {
                    Some((srlv, "srlv", 1))
                },
                7 => {
                    Some((srav, "srav", 1))
                },
                8 => {
                    Some((jr, "jr", 2))
                },
                9 => {
                    Some((jalr, "jalr", 2))
                },
                12 => {
                    Some((syscall, "syscall", 2))
                },
                13 => {
                    Some((break, "break", 2))
                },
                16 => {
                    Some((mfhi, "mfhi", 1))
                },
                17 => {
                    Some((mthi, "mthi", 1))
                },
                18 => {
                    Some((mflo, "mflo", 1))
                },
                19 => {
                    Some((mtlo, "mtlo", 1))
                },
                24 => {
                    Some((mult, "mult", 1))
                },
                25 => {
                    Some((multu, "multu", 1))
                },
                26 => {
                    Some((div, "div", 2))
                },
                27 => {
                    Some((divu, "divu", 2))
                },
                32 => {
                    Some((add, "add", 1))
                },
                33 => {
                    Some((addu, "addu", 1))
                },
                34 => {
                    Some((sub, "sub", 1))
                },
                35 => {
                    Some((subu, "subu", 1))
                },
                36 => {
                    Some((and, "and", 1))
                },
                37 => {
                    Some((or, "or", 1))
                },
                38 => {
                    Some((xor, "xor", 1))
                },
                39 => {
                    Some((nor, "nor", 1))
                },
                42 => {
                    Some((slt, "slt", 1))
                },
                43 => {
                    Some((sltu, "sltu", 1))
                },
                _ => {
                    None
                },
            }
        },
        1 => {
            match inst.rt() {
                0 => {
                    Some((bltz, "bltz", 2))
                },
                1 => {
                    Some((bgez, "bgez", 2))
                },
                16 => {
                    Some((bltzal, "bltzal", 2))
                },
                17 => {
                    Some((bgezal, "bgezal", 2))
                },
                _ => {
                    None
                },
            }
        },
        2 => {
            Some((j, "j", 2))
        },
        3 => {
            Some((jal, "jal", 2))
        },
        4 => {
            Some((beq, "beq", 2))
        },
        5 => {
            Some((bne, "bne", 2))
        },
        6 => {
            Some((blez, "blez", 2))
        },
        7 => {
            Some((bgtz, "bgtz", 2))
        },
        8 => {
            Some((addi, "addi", 1))
        },
        9 => {
            Some((addiu, "addiu", 1))
        },
        10 => {
            Some((slti, "slti", 1))
        },
        11 => {
            Some((sltiu, "sltiu", 1))
        },
        12 => {
            Some((andi, "andi", 1))
        },
        13 => {
            Some((ori, "ori", 1))
        },
        14 => {
            Some((xori, "xori", 1))
        },
        15 => {
            Some((lui, "lui", 1))
        },
        16 => {
            match inst.c() {
                0 => {
                    match inst.rs4() {
                        0 => {
                            Some((mfc0, "mfc0", 2))
                        },
                        4 => {
                            Some((mtc0, "mtc0", 2))
                        },
                        8 => {
                            match inst.rt() {
                                0 => {
                                    Some((bc0f, "bc0f", 2))
                                },
                                1 => {
                                    Some((bc0t, "bc0t", 2))
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
                1 => {
                    match inst.rs4() {
                        0 => {
                            match inst.funct() {
                                1 => {
                                    Some((tlbr, "tlbr", 2))
                                },
                                2 => {
                                    Some((tlbwi, "tlbwi", 2))
                                },
                                6 => {
                                    Some((tlbwr, "tlbwr", 2))
                                },
                                8 => {
                                    Some((tlbp, "tlbp", 2))
                                },
                                16 => {
                                    Some((rfe, "rfe", 2))
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
                _ => {
                    None
                },
            }
        },
        32 => {
            Some((lb, "lb", 2))
        },
        33 => {
            Some((lh, "lh", 2))
        },
        34 => {
            Some((lwl, "lwl", 2))
        },
        35 => {
            Some((lw, "lw", 2))
        },
        36 => {
            Some((lbu, "lbu", 2))
        },
        37 => {
            Some((lhu, "lhu", 2))
        },
        38 => {
            Some((lwr, "lwr", 2))
        },
        40 => {
            Some((sb, "sb", 2))
        },
        41 => {
            Some((sh, "sh", 2))
        },
        42 => {
            Some((swl, "swl", 2))
        },
        43 => {
            Some((sw, "sw", 2))
        },
        46 => {
            Some((swr, "swr", 2))
        },
        _ => {
            None
        },
    }
}
