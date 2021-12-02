use super::{Instruction, Register, RegisterW, SP};

#[derive(Debug)]
pub enum AddressingMode {
    Ia,
    Db,
}

impl AddressingMode {
    fn code(self) -> u32 {
        match self {
            Self::Ia => 0x8800000,
            Self::Db => 0x9000000,
        }
    }
}

impl Default for AddressingMode {
    fn default() -> Self {
        Self::Ia
    }
}

pub fn instruction<R, L>(
    addressing_mode: AddressingMode,
    load: bool,
    rn: R,
    registers: L,
) -> Instruction
where
    R: Into<RegisterW>,
    L: AsRef<[Register]>,
{
    let RegisterW(rn, w) = rn.into();
    let register_list = registers
        .as_ref()
        .iter()
        .fold(0, |list, &register| list | register.bit());
    Instruction::new(
        addressing_mode.code()
            | (w as u32) << 21
            | (load as u32) << 20
            | rn.shift(16)
            | (register_list as u32),
    )
}

pub fn ldm<R, L>(addressing_mode: AddressingMode, rn: R, registers: L) -> Instruction
where
    R: Into<RegisterW>,
    L: AsRef<[Register]>,
{
    instruction(addressing_mode, true, rn, registers)
}

pub fn stm<R, L>(addressing_mode: AddressingMode, rn: R, registers: L) -> Instruction
where
    R: Into<RegisterW>,
    L: AsRef<[Register]>,
{
    instruction(addressing_mode, false, rn, registers)
}

pub fn pop<L>(reglist: L) -> Instruction
where
    L: AsRef<[Register]>,
{
    ldm(AddressingMode::Ia, SP.w(), reglist)
}

pub fn push<L>(reglist: L) -> Instruction
where
    L: AsRef<[Register]>,
{
    stm(AddressingMode::Db, SP.w(), reglist)
}
