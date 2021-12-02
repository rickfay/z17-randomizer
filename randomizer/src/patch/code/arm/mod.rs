mod data;
mod ls;
mod lsm;

use std::array;

pub use data::{add, cmp, mov};
pub use ls::{ldr, ldrb, str_};
pub use lsm::{ldm, pop, push, stm, AddressingMode::*};
pub use Register::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

impl Register {
    pub fn w(self) -> RegisterW {
        RegisterW(self, true)
    }

    fn shift(self, by: u32) -> u32 {
        (self as u32) << by
    }

    fn bit(self) -> u16 {
        1 << (self as u16)
    }
}

#[derive(Debug)]
pub struct RegisterW(Register, bool);

impl From<Register> for RegisterW {
    fn from(r: Register) -> Self {
        Self(r, false)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Condition {
    Eq = 0b0000,
    Ne = 0b0001,
    Al = 0b1110,
}

impl Condition {
    fn shift(self) -> u32 {
        (self as u32) << 28
    }
}

impl Default for Condition {
    fn default() -> Self {
        Self::Al
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Address(u32);

impl Address {
    fn offset(&self, offset: u32) -> Self {
        Self(self.0 + offset)
    }

    fn diff(&self, other: Address) -> i32 {
        self.0 as i32 - other.0 as i32
    }
}

impl From<u32> for Address {
    fn from(address: u32) -> Self {
        Self(address)
    }
}

#[derive(Debug)]
pub enum Pseudo {
    Ldr(ls::Pseudo),
}

impl Pseudo {
    fn into_raw(self, assembler: &mut Assembler) -> Instruction {
        match self {
            Self::Ldr(pseudo) => pseudo.to_raw(assembler),
        }
    }
}

impl From<ls::Pseudo> for Pseudo {
    fn from(pseudo: ls::Pseudo) -> Self {
        Self::Ldr(pseudo)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Raw(u32),
    Branch {
        cond: Condition,
        target_address: Address,
        link: bool,
    },
    Pseudo(Condition, Pseudo),
}

impl Instruction {
    fn new(raw: u32) -> Self {
        Self::Raw(raw).al()
    }

    fn with_condition(self, cond: Condition) -> Self {
        match self {
            Self::Raw(raw) => Self::Raw(raw & 0x0FFFFFFF | cond.shift()),
            Self::Branch {
                target_address,
                link,
                ..
            } => Self::Branch {
                cond,
                target_address,
                link,
            },
            Self::Pseudo(_, pseudo) => Self::Pseudo(cond, pseudo),
        }
    }

    pub fn al(self) -> Self {
        self.with_condition(Condition::Al)
    }

    pub fn eq(self) -> Self {
        self.with_condition(Condition::Eq)
    }

    pub fn ne(self) -> Self {
        self.with_condition(Condition::Ne)
    }

    fn assemble(self, assembler: &mut Assembler) -> u32 {
        match self {
            Self::Raw(code) => code,
            Self::Branch {
                cond,
                target_address,
                link,
            } => {
                let signed_immed_24 = ((target_address.diff(assembler.pc()) - 8) >> 2) & 0xFFFFFF;
                0xA000000
                    | (link as u32) << 24
                    | u32::from_ne_bytes(signed_immed_24.to_ne_bytes())
                    | cond.shift()
            }
            Self::Pseudo(cond, pseudo) => {
                pseudo.into_raw(assembler).assemble(assembler) | cond.shift()
            }
        }
    }
}

#[derive(Debug)]
pub struct Assembler {
    start: Address,
    offset: usize,
    bytes: Vec<u8>,
}

impl Assembler {
    fn new(start: Address, len: usize) -> Self {
        Self {
            start,
            offset: 0,
            bytes: vec![0u8; len],
        }
    }

    fn write(&mut self, code: u32) {
        self.bytes[self.offset..self.offset + 4].copy_from_slice(&code.to_le_bytes());
        self.offset += 4;
    }

    fn pc(&self) -> Address {
        self.start.offset(self.offset as u32)
    }

    fn dcd(&mut self, value: u32) -> Address {
        let addr = self.start.offset(self.bytes.len() as u32);
        self.bytes.extend_from_slice(&value.to_le_bytes());
        addr
    }
}

pub fn b<A>(target_address: A) -> Instruction
where
    A: Into<Address>,
{
    Instruction::Branch {
        cond: Default::default(),
        target_address: target_address.into(),
        link: false,
    }
}

pub fn bl<A>(target_address: A) -> Instruction
where
    A: Into<Address>,
{
    Instruction::Branch {
        cond: Default::default(),
        target_address: target_address.into(),
        link: true,
    }
}

pub fn assemble<A, const N: usize>(start: A, instructions: [Instruction; N]) -> Box<[u8]>
where
    A: Into<Address>,
{
    let mut assembler = Assembler::new(start.into(), N * 4);
    for instruction in array::IntoIter::new(instructions) {
        let code = instruction.assemble(&mut assembler);
        assembler.write(code);
    }
    assembler.bytes.into_boxed_slice()
}

pub const SP: Register = Register::R13;
pub const LR: Register = Register::R14;
pub const PC: Register = Register::R15;
