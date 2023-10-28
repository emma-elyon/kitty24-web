use std::{ops::Index, collections::BinaryHeap};

use common::*;

const CONTEXT_COUNT: usize = 2_usize.pow(8);
const CONTEXT_MASK: u32 = CONTEXT_COUNT as u32 - 1;

const BITS: usize = 24;
const MASK: u32 = 2_u32.pow(BITS as u32) - 1;

pub struct Cpu {
    condition: [bool; CONTEXT_COUNT],
    registers: [[u32; REGISTER_COUNT]; CONTEXT_COUNT],
    context: usize,
    pending_interrupts: BinaryHeap<Interrupt>,
}

#[derive(Eq)]
struct Interrupt(u32);

impl PartialEq for Interrupt {
    fn eq(&self, other: &Self) -> bool {
        self.0 & CONTEXT_MASK == other.0 & CONTEXT_MASK
    }
}

impl PartialOrd for Interrupt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0 & CONTEXT_MASK).partial_cmp(&(other.0 & CONTEXT_MASK))
    }
}

impl Ord for Interrupt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 & CONTEXT_MASK).cmp(&(other.0 & CONTEXT_MASK))
    }
}

impl Cpu {
    /// Sets the value of a specified register.
    ///
    /// If the specified register is register 0, this method will also
    /// swap to a different register context. The context to swap to
    /// is determined by the lower byte of the value being written.
    ///
    /// # Arguments
    ///
    /// * `register` - The index of the register to set.
    /// * `value`    - The value to set the register to, masked with `MASK`.
    pub fn set(&mut self, register: u32, value: u32) {
        let value = value & MASK;

        if register == REGISTER_INTERRUPT {
            let next_interrupt = value;
            let current_interrupt = self.registers[self.context][REGISTER_INTERRUPT as usize];

            match (current_interrupt, next_interrupt) {
                (0, 0) => todo!("Interrupt 0 overlap."),
                (0, interrupt) => self.start_interrupt(interrupt),
                (_, 0) => self.pop_interrupt(),
                (current, next) => self.push_interrupt(current, next),
            }
        } else {
            self.registers[self.context][register as usize] = value;
        }
    }

    pub fn condition(&self) -> bool {
        self.condition[self.context]
    }

    pub fn set_condition(&mut self, condition: bool) {
        self.condition[self.context] = condition;
    }

    pub fn registers(&self) -> [u32; REGISTER_COUNT] {
        self.registers[0]
    }

    fn reset(&mut self) {
        // Reset program counter in context.
        self.registers[self.context][REGISTER_PROGRAM_COUNTER as usize] = 0;
    }

    fn start_interrupt(&mut self, interrupt: u32) {
        self.switch_context(interrupt);
        self.reset();
    }

    fn switch_context(&mut self, interrupt: u32) {
        // Copy global register to new context.
        let context = (interrupt & CONTEXT_MASK) as usize;
        self.registers[context][REGISTER_GLOBAL as usize] = self.registers[self.context][REGISTER_GLOBAL as usize];
        self.registers[context][REGISTER_INTERRUPT as usize] = interrupt;

        // Switch to new context.
        self.context = context;
    }

    fn pop_interrupt(&mut self) {
        if let Some(Interrupt(value)) = self.pending_interrupts.pop() {
            self.switch_context(value);
        } else {
            self.switch_context(0);
        }
    }

    fn push_interrupt(&mut self, current: u32, next: u32) {
        let current_context = current & CONTEXT_MASK;
        let next_context = next & CONTEXT_MASK;
        if current_context < next_context {
            self.pending_interrupts.push(Interrupt(next));
            self.registers[next_context as usize][REGISTER_PROGRAM_COUNTER as usize] = 0;
        } else if current_context > next_context {
            self.pending_interrupts.push(Interrupt(current));
            self.switch_context(next);
        } else {
            todo!("Interrupt {} overlap.", current_context)
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            condition: [false; CONTEXT_COUNT],
            registers: [[0; REGISTER_COUNT]; CONTEXT_COUNT],
            context: 0,
            pending_interrupts: BinaryHeap::default(),
        }
    }
}

impl Index<u32> for Cpu {
    type Output = u32;

    fn index(&self, index: u32) -> &Self::Output {
        &self.registers[self.context][index as usize]
    }
}
