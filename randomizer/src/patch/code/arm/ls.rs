use super::{Assembler, FloatRegister, Instruction, PC, Register};

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

impl From<u16> for Operand {
    fn from(expr: u16) -> Self {
        Self::Pseudo(expr as u32)
    }
}

impl From<u32> for Operand {
    fn from(expr: u32) -> Self {
        Self::Pseudo(expr)
    }
}

impl From<i32> for Operand {
    fn from(expr: i32) -> Self {
        Self::Pseudo(expr as u32)
    }
}

#[derive(Debug)]
pub enum OperandFloat {
    AddressingMode(AddressingMode),
    Pseudo(f32),
}

impl<T> From<T> for OperandFloat
where
    T: Into<AddressingMode>,
{
    fn from(addressing_mode: T) -> Self {
        Self::AddressingMode(addressing_mode.into())
    }
}

impl From<f32> for OperandFloat {
    fn from(expr: f32) -> Self {
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

    pub fn float_code(&self) -> u32 {
        self.offset.float_code() | (self.plus as u32) << 23 | self.rn.shift(16)
    }

    pub fn halfword_code(&self) -> u32 {
        self.offset.halfword_code() | (self.plus as u32) << 23 | self.rn.shift(16)
    }
}

impl From<(Register, i32)> for AddressingMode {
    fn from(parameter: (Register, i32)) -> Self {
        let (rn, offset) = parameter;
        Self { rn, plus: offset >= 0, offset: Offset::Immediate(offset.unsigned_abs()) }
    }
}

impl From<(Register, Register)> for AddressingMode {
    fn from(parameter: (Register, Register)) -> Self {
        let (rn, rm) = parameter;
        Self { rn, plus: true, offset: Offset::Register(rm, 0) }
    }
}

impl From<(Register, Register, u32)> for AddressingMode {
    fn from(parameter: (Register, Register, u32)) -> Self {
        let (rn, rm, shift) = parameter;
        Self { rn, plus: true, offset: Offset::Register(rm, shift) }
    }
}

#[derive(Debug)]
enum Offset {
    Immediate(u32),
    Register(Register, u32),
}

impl Offset {
    pub fn code(&self) -> u32 {
        (match self {
            Self::Immediate(offset) => (*offset) | 0x1000000,
            Self::Register(register, shift) => register.shift(0) | (shift << 7) | 0x3000000,
        }) | 0x4000000
    }

    pub fn float_code(&self) -> u32 {
        match self {
            Self::Immediate(offset) => (*offset >> 2) | 0xd000a00,
            Self::Register(_, _) => { panic!("Invalid operand to floating point instruction"); }
        }
    }

    pub fn halfword_code(&self) -> u32 {
        match self {
            Self::Immediate(offset) => (*offset & 0xf) | ((*offset & 0xf0) << 4) | 0x00400000,
            Self::Register(register, _) => register.shift(0),
        }
    }
}

#[derive(Debug)]
pub struct Pseudo {
    rt: Register,
    expr: u32,
}

impl Pseudo {
    pub fn to_raw(&self, assembler: &mut Assembler) -> Instruction {
        let label = assembler.dcd(&self.expr.to_le_bytes());
        let offset = label.diff(assembler.pc()) - 8;
        ldr(self.rt, (PC, offset))
    }
}

#[derive(Debug)]
pub struct PseudoFloat {
    rt: FloatRegister,
    expr: f32,
}

impl PseudoFloat {
    pub fn to_raw(&self, assembler: &mut Assembler) -> Instruction {
        let label = assembler.dcd(&self.expr.to_le_bytes());
        let offset = label.diff(assembler.pc()) - 8;
        vldr(self.rt, (PC, offset))
    }
}

fn instruction(code: u32, byte: bool, load: bool, rd: Register) -> Instruction {
    Instruction::new(code | (byte as u32) << 22 | (load as u32) << 20 | rd.shift(12))
}

fn float_instruction(code: u32, load: bool, rd: FloatRegister) -> Instruction {
    Instruction::new(code | (load as u32) << 20 | (rd as u32 >> 1) << 12 | (rd as u32 & 1) << 22)
}

fn halfword_instruction(code: u32, load: bool, rd: Register) -> Instruction {
    Instruction::new(code | (load as u32) << 20 | rd.shift(12) | 0x010000b0)
}

pub fn ldr<P>(rd: Register, addressing_mode: P) -> Instruction
where
    P: Into<Operand>,
{
    match addressing_mode.into() {
        Operand::AddressingMode(addressing_mode) => instruction(addressing_mode.code(), false, true, rd),
        Operand::Pseudo(expr) => Instruction::Pseudo(Default::default(), Pseudo { rt: rd, expr }.into()),
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

pub fn strb<A>(rd: Register, addressing_mode: A) -> Instruction
where
    A: Into<AddressingMode>,
{
    instruction(addressing_mode.into().code(), true, false, rd)
}

pub fn strh<A>(rd: Register, addressing_mode: A) -> Instruction
where
    A: Into<AddressingMode>,
{
    halfword_instruction(addressing_mode.into().halfword_code(), false, rd)
}

pub fn vldr<P>(rd: FloatRegister, addressing_mode: P) -> Instruction
where
    P: Into<OperandFloat>,
{
    match addressing_mode.into() {
        OperandFloat::AddressingMode(addressing_mode) => float_instruction(addressing_mode.float_code(), true, rd),
        OperandFloat::Pseudo(expr) => Instruction::Pseudo(Default::default(), PseudoFloat { rt: rd, expr }.into()),
    }
}
