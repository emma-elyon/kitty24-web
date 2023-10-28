use std::ops::Index;

use common::*;

pub const REGISTER_COUNT: usize = 2_usize.pow(6);

const BANK_COUNT: usize = 2_usize.pow(8);
const BANK_MASK: usize = BANK_COUNT - 1;

const BITS: usize = 24;
const MASK: u32 = 2_u32.pow(BITS as u32) - 1;

pub struct Cpu {
    pub condition: bool,
    registers: [[u32; REGISTER_COUNT]; BANK_COUNT],
    bank: usize,
}

impl Cpu {
    /// Sets the value of a specified register.
    ///
    /// If the specified register is register 0, this method will also
    /// swap to a different register bank. The bank to swap to
    /// is determined by the lower byte of the value being written.
    ///
    /// # Arguments
    ///
    /// * `register` - The index of the register to set.
    /// * `value`    - The value to set the register to, masked with `MASK`.
    pub fn set(&mut self, register: u32, value: u32) {
        let value = value & MASK;

        if register == REGISTER_INTERRUPT {
            let bank = value as usize & BANK_MASK;
            self.registers[bank][REGISTER_GLOBAL as usize] = self.registers[self.bank][REGISTER_GLOBAL as usize];
            self.bank = bank;
        }

        self.registers[self.bank][register as usize] = value;
    }

    pub fn registers(&self) -> [u32; REGISTER_COUNT] {
        self.registers[0]
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            condition: false,
            registers: [[0; REGISTER_COUNT]; BANK_COUNT],
            bank: 0
        }
    }
}

impl Index<u32> for Cpu {
    type Output = u32;

    fn index(&self, index: u32) -> &Self::Output {
        &self.registers[self.bank][index as usize]
    }
}
