mod data;
mod ls;
mod lsm;
pub use {
    self::data::{add, cmp, mov},
    ls::{ldr, ldrb, str_, strb},
    lsm::{ldm, pop, push, stm, AddressingMode::*},
    Register::*,
};

#[allow(unused)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Register {
    R0  = 0,
    R1  = 1,
    R2  = 2,
    R3  = 3,
    R4  = 4,
    R5  = 5,
    R6  = 6,
    R7  = 7,
    R8  = 8,
    R9  = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
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
#[allow(unused)]
pub enum Condition {
    /// Meaning, integer arithmetic: Equal <br />
    /// Meaning, Floating-point arithmetic: Equal <br />
    /// APSR condition flags: Z == 1
    EQ = 0b0000,

    /// Meaning, integer arithmetic: Not equal <br />
    /// Meaning, Floating-point arithmetic: Not equal, or unordered <br />
    /// APSR condition flags: Z == 0
    NE = 0b0001,

    /// Meaning, integer arithmetic: Carry set <br />
    /// Meaning, Floating-point arithmetic: Greater than, equal, or unordered <br />
    /// APSR condition flags: C == 1
    CS = 0b0010,

    /// Meaning, integer arithmetic: Carry clear <br />
    /// Meaning, Floating-point arithmetic: Less than <br />
    /// APSR condition flags: C == 0
    CC = 0b0011,

    /// Meaning, integer arithmetic: Minus, negative <br />
    /// Meaning, Floating-point arithmetic: Less than <br />
    /// APSR condition flags: N == 1
    MI = 0b0100,

    /// Meaning, integer arithmetic: Plus, positive or zero <br />
    /// Meaning, Floating-point arithmetic: Greater than, equal or unordered <br />
    /// APSR condition flags: N == 0
    PL = 0b0101,

    /// Meaning, integer arithmetic: Overflow <br />
    /// Meaning, Floating-point arithmetic: Unordered <br />
    /// APSR condition flags: V == 1
    VS = 0b0110,

    /// Meaning, integer arithmetic: No overflow <br />
    /// Meaning, Floating-point arithmetic: Not unordered <br />
    /// APSR condition flags: V == 0
    VC = 0b0111,

    /// Meaning, integer arithmetic: Unsigned higher <br />
    /// Meaning, Floating-point arithmetic: Greater than or unordered <br />
    /// APSR condition flags: C == 1 and Z == 0
    HI = 0b1000,

    /// Meaning, integer arithmetic: Unsigned lower or same <br />
    /// Meaning, Floating-point arithmetic: Less than or equal <br />
    /// APSR condition flags: C == 0 or Z == 1
    LS = 0b1001,

    /// Meaning, integer arithmetic: Signed greater than or equal <br />
    /// Meaning, Floating-point arithmetic: Greater than or equal <br />
    /// APSR condition flags: N == V
    GE = 0b1010,

    /// Meaning, integer arithmetic: Signed less than <br />
    /// Meaning, Floating-point arithmetic: Less than or unordered <br />
    /// APSR condition flags: N != V
    LT = 0b1011,

    /// Meaning, integer arithmetic: Signed greater than <br />
    /// Meaning, Floating-point arithmetic: Greater than <br />
    /// APSR condition flags: Z == 0 and N == V
    GT = 0b1100,

    /// Meaning, integer arithmetic: Signed less than or equal <br />
    /// Meaning, Floating-point arithmetic: Less than, equal or unordered <br />
    /// APSR condition flags: Z == 1 and N != V
    LE = 0b1101,

    /// Mnemonic extension: None (AL) <br />
    /// Meaning, integer arithmetic: Always (unconditional) <br />
    /// Meaning, Floating-point arithmetic: Always (unconditional) <br />
    /// APSR condition flags: Any
    AL = 0b1110,
}

impl Condition {
    fn shift(self) -> u32 {
        (self as u32) << 28
    }
}

impl Default for Condition {
    fn default() -> Self {
        Self::AL
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
    Branch { cond: Condition, target_address: Address, link: bool },
    Pseudo(Condition, Pseudo),
}

#[allow(unused)]
impl Instruction {
    fn new(raw: u32) -> Self {
        Self::Raw(raw).al()
    }

    fn with_condition(self, cond: Condition) -> Self {
        match self {
            Self::Raw(raw) => Self::Raw(raw & 0x0FFFFFFF | cond.shift()),
            Self::Branch { target_address, link, .. } => {
                Self::Branch { cond, target_address, link }
            }
            Self::Pseudo(_, pseudo) => Self::Pseudo(cond, pseudo),
        }
    }

    fn assemble(self, assembler: &mut Assembler) -> u32 {
        match self {
            Self::Raw(code) => code,
            Self::Branch { cond, target_address, link } => {
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

    /// Equal
    pub fn eq(self) -> Self {
        self.with_condition(Condition::EQ)
    }

    /// Not equal
    pub fn ne(self) -> Self {
        self.with_condition(Condition::NE)
    }

    /// Carry set
    pub fn cs(self) -> Self {
        self.with_condition(Condition::CS)
    }

    /// Carry clear
    pub fn cc(self) -> Self {
        self.with_condition(Condition::CC)
    }

    /// Minus, negative
    pub fn mi(self) -> Self {
        self.with_condition(Condition::MI)
    }

    /// Plus, positive or zero
    pub fn pl(self) -> Self {
        self.with_condition(Condition::PL)
    }

    /// Overflow
    pub fn vs(self) -> Self {
        self.with_condition(Condition::VS)
    }

    /// No overflow
    pub fn vc(self) -> Self {
        self.with_condition(Condition::VC)
    }

    /// Unsigned higher
    pub fn hi(self) -> Self {
        self.with_condition(Condition::HI)
    }

    /// Unsigned lower or same
    pub fn ls(self) -> Self {
        self.with_condition(Condition::LS)
    }

    /// Signed greater than or equal
    pub fn ge(self) -> Self {
        self.with_condition(Condition::GE)
    }

    /// Signed less than
    pub fn lt(self) -> Self {
        self.with_condition(Condition::LT)
    }

    /// Signed greater than
    pub fn gt(self) -> Self {
        self.with_condition(Condition::GT)
    }

    /// Signed less than or equal
    pub fn le(self) -> Self {
        self.with_condition(Condition::LE)
    }

    /// Always (unconditional)
    pub fn al(self) -> Self {
        self.with_condition(Condition::AL)
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
        Self { start, offset: 0, bytes: vec![0u8; len] }
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
    for instruction in IntoIterator::into_iter(instructions) {
        let code = instruction.assemble(&mut assembler);
        assembler.write(code);
    }
    assembler.bytes.into_boxed_slice()
}

pub const SP: Register = Register::R13;
pub const LR: Register = Register::R14;
pub const PC: Register = Register::R15;
