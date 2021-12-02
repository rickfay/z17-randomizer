use super::{Instruction, Register, R0};

#[derive(Debug)]
pub enum ShifterOperand {
    Immediate { immed_8: u8, rotate_imm: u8 },
    Register { rm: Register },
}

impl ShifterOperand {
    pub fn code(&self) -> u32 {
        match self {
            Self::Immediate {
                immed_8,
                rotate_imm,
            } => 0x2000000 | (*rotate_imm as u32) << 8 | *immed_8 as u32,
            Self::Register { rm } => rm.shift(0),
        }
    }
}

impl From<u32> for ShifterOperand {
    fn from(immediate: u32) -> Self {
        if immediate & 0xFF == immediate {
            Self::Immediate {
                immed_8: immediate as u8,
                rotate_imm: 0,
            }
        } else {
            let mut shifted = immediate;
            ((1..=0xF)
                .rev()
                .find_map(|rotate_imm| {
                    shifted >>= 2;
                    if shifted & 0xFF == shifted {
                        Some(Self::Immediate {
                            immed_8: shifted as u8,
                            rotate_imm,
                        })
                    } else {
                        None
                    }
                })
                .ok_or_else(|| format!("Could not convert {} into an operand.", immediate)))
            .expect("Failed to assemble.")
        }
    }
}

impl From<Register> for ShifterOperand {
    fn from(rm: Register) -> Self {
        Self::Register { rm }
    }
}

fn instruction(code: u32, opcode: u32, s: bool, rn: Register, rd: Register) -> Instruction {
    Instruction::new(code | opcode << 21 | (s as u32) << 20 | rn.shift(16) | rd.shift(12))
}

pub fn add<O>(rd: Register, rn: Register, operand2: O) -> Instruction
where
    O: Into<ShifterOperand>,
{
    instruction(operand2.into().code(), 0b0100, false, rn, rd)
}

pub fn cmp<O>(rn: Register, operand2: O) -> Instruction
where
    O: Into<ShifterOperand>,
{
    instruction(operand2.into().code(), 0b1010, true, rn, R0)
}

pub fn mov<O>(rd: Register, operand2: O) -> Instruction
where
    O: Into<ShifterOperand>,
{
    instruction(operand2.into().code(), 0b1101, false, R0, rd)
}
