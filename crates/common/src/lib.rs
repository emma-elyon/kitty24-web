#[derive(Clone, Copy)]
pub enum Op {
    Let = 0o00,
    Lethi = 0o01,
    Load = 0o05,
    Store = 0o15,
    Ori = 0o20,
    Nori = 0o21,
    Andi = 0o22,
    Xori = 0o23,
    Lessi = 0o24,
    Addi = 0o25,
    Subi = 0o26,
    Less = 0o34,
}

impl From<u32> for Op {
    fn from(value: u32) -> Self {
        use Op::*;
        match value {
            0o00 => Let,
            0o01 => Lethi,
            0o05 => Load,
            0o15 => Store,
            0o20 => Ori,
            0o21 => Nori,
            0o22 => Andi,
            0o23 => Xori,
            0o24 => Lessi,
            0o25 => Addi,
            0o26 => Subi,
            0o34 => Less,
            0o40..=u32::MAX => panic!("{}: Impossible operation.", value),
            _ => todo!("Operation: {:02o}", value),
        }
    }
}
