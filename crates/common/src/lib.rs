pub const REGISTER_COUNT: usize = 0x40;

pub const REGISTER_PROGRAM_COUNTER: u32 = 0x3F;
pub const REGISTER_GLOBAL: u32 = 0x0;
pub const REGISTER_INTERRUPT: u32 = 0x3E;

pub const INTERRUPT_VBLANK: u32 = 0x0000_04;

#[derive(Clone, Copy)]
pub enum Op {
    Let = 0o00,
    Lethi = 0o01,
    Shri = 0o02,
    Shli = 0o03,
    Slessi = 0o04,
    Load = 0o05,
    Load2 = 0o06,
    Load3 = 0o07,
    Ashr = 0o10,
    Rol = 0o11,
    Shr = 0o12,
    Shl = 0o13,
    Sless = 0o14,
    Store = 0o15,
    Store2 = 0o16,
    Store3 = 0o17,
    Ori = 0o20,
    Nori = 0o21,
    Andi = 0o22,
    Xori = 0o23,
    Lessi = 0o24,
    Addi = 0o25,
    Subi = 0o26,
    Muli = 0o27,
    Or = 0o30,
    Nor = 0o31,
    And = 0o32,
    Xor = 0o33,
    Less = 0o34,
    Add = 0o35,
    Sub = 0o36,
    Mul = 0o37,
}

impl From<u32> for Op {
    fn from(value: u32) -> Self {
        use Op::*;
        match value {
            0o00 => Let,
            0o01 => Lethi,
            0o02 => Shri,
            0o03 => Shli,
            0o04 => Slessi,
            0o05 => Load,
            0o06 => Load2,
            0o07 => Load3,
            0o10 => Ashr,
            0o11 => Rol,
            0o12 => Shr,
            0o13 => Shl,
            0o14 => Sless,
            0o15 => Store,
            0o16 => Store2,
            0o17 => Store3,
            0o20 => Ori,
            0o21 => Nori,
            0o22 => Andi,
            0o23 => Xori,
            0o24 => Lessi,
            0o25 => Addi,
            0o26 => Subi,
            0o27 => Muli,
            0o30 => Or,
            0o31 => Nor,
            0o32 => And,
            0o33 => Xor,
            0o34 => Less,
            0o35 => Add,
            0o36 => Sub,
            0o37 => Mul,
            _ => panic!("{}: Impossible operation.", value),
        }
    }
}
