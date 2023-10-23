use std::ops::Index;

pub const REGISTER_PROGRAM_COUNTER: u32 = 0x0F;

pub const REGISTER_COUNT: usize = 2_usize.pow(6);

pub struct Cpu {
    pub condition: bool,
    registers: [u32; REGISTER_COUNT],
}

impl Cpu {
    pub fn set(&mut self, register: u32, value: u32) {
        self.registers[register as usize] = value;
    }

    pub fn registers(&self) -> [u32; REGISTER_COUNT] {
        self.registers
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            condition: false,
            registers: [0; REGISTER_COUNT],
        }
    }
}

impl Index<u32> for Cpu {
    type Output = u32;

    fn index(&self, index: u32) -> &Self::Output {
        &self.registers[index as usize]
    }
}
