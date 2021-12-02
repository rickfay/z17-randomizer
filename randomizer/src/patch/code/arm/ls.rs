use super::{Assembler, Instruction, Register, PC};

#[derive(Debug)]
pub enum Operand {
    AddressingMode(AddressingMode),
    Pseudo(u32),
}

impl<T> From<T> for Operand
where
    T: Into<AddressingMode>,
{
    fn from(addressing_mode: T) -> Self {
        Self::AddressingMode(addressing_mode.into())
    }
}

impl From<u32> for Operand {
    fn from(expr: u32) -> Self {
        Self::Pseudo(expr)
    }
}

#[derive(Debug)]
pub struct AddressingMode {
    rn: Register,
    plus: bool,
    offset: Offset,
}

impl AddressingMode {
    pub fn code(&self) -> u32 {
        self.offset.code() | (self.plus as u32) << 23 | self.rn.shift(16)
    }
}

impl From<(Register, i32)> for AddressingMode {
    fn from(parameter: (Register, i32)) -> Self {
        let (rn, offset) = parameter;
        Self {
            rn,
            plus: offset >= 0,
            offset: Offset::Immediate(offset.abs() as u32),
        }
    }
}

impl From<(Register, Register)> for AddressingMode {
    fn from(parameter: (Register, Register)) -> Self {
        let (rn, rm) = parameter;
        Self {
            rn,
            plus: true,
            offset: Offset::Register(rm),
        }
    }
}

#[derive(Debug)]
enum Offset {
    Immediate(u32),
    Register(Register),
}

impl Offset {
    pub fn code(&self) -> u32 {
        (match self {
            Self::Immediate(offset) => (*offset) | 0x1000000,
            Self::Register(register) => register.shift(0) | 0x3000000,
        }) | 0x4000000
    }
}

#[derive(Debug)]
pub struct Pseudo {
    rt: Register,
    expr: u32,
}

impl Pseudo {
    pub fn to_raw(&self, assembler: &mut Assembler) -> Instruction {
        let label = assembler.dcd(self.expr);
        let offset = label.diff(assembler.pc()) - 8;
        ldr(self.rt, (PC, offset))
    }
}

fn instruction(code: u32, byte: bool, load: bool, rd: Register) -> Instruction {
    Instruction::new(code | (byte as u32) << 22 | (load as u32) << 20 | rd.shift(12))
}

pub fn ldr<P>(rd: Register, addressing_mode: P) -> Instruction
where
    P: Into<Operand>,
{
    match addressing_mode.into() {
        Operand::AddressingMode(addressing_mode) => {
            instruction(addressing_mode.code(), false, true, rd)
        }
        Operand::Pseudo(expr) => {
            Instruction::Pseudo(Default::default(), Pseudo { rt: rd, expr }.into())
        }
    }
}

pub fn ldrb<A>(rd: Register, addressing_mode: A) -> Instruction
where
    A: Into<AddressingMode>,
{
    instruction(addressing_mode.into().code(), true, true, rd)
}

pub fn str_<A>(rd: Register, addressing_mode: A) -> Instruction
where
    A: Into<AddressingMode>,
{
    instruction(addressing_mode.into().code(), false, false, rd)
}
