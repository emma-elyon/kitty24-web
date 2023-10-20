#[derive(Clone, Copy)]
pub enum Op {
    Let = 000,
    Lethi = 001,
    Load = 005,
    Store = 015,
    Ori = 020,
    Nori = 021,
    Andi = 022,
    Xori = 023,
}

impl From<u32> for Op {
    fn from(value: u32) -> Self {
        use Op::*;
        match value {
            000 => Let,
            001 => Lethi,
            005 => Load,
            015 => Store,
            020 => Ori,
            040..=u32::MAX => panic!("{}: Impossible operation.", value),
            _ => todo!("{}: Operation not yet implemented.", value),
        }
    }
}
