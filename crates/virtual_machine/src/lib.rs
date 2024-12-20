use std::f32::consts::TAU;

mod cpu;
mod io;

use common::*;
use cpu::*;
use io::*;

use crate::io::COMPOSITE_MODE;

const BITS: usize = 24;
const MASK: usize = 2_usize.pow(BITS as u32) - 1;
const MEMORY_SIZE: usize = 2_usize.pow(BITS as u32);
pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 180;
const TOTAL_WIDTH: usize = 640;
const TOTAL_HEIGHT: usize = 360;
const BLANK_WIDTH: usize = TOTAL_WIDTH - WIDTH;
const BLANK_HEIGHT: usize = TOTAL_HEIGHT - HEIGHT;
const FRAME_RATE: usize = 60;
const SAMPLE_RATE: usize = 48000;
const CLOCK_RATE: usize = 24 * FRAME_RATE * SAMPLE_RATE;
const CYCLES_PER_FRAME: usize = CLOCK_RATE / FRAME_RATE;
const CYCLES_PER_SCANLINE: usize = CYCLES_PER_FRAME / TOTAL_HEIGHT;
const CYCLES_PER_PIXEL: usize = CYCLES_PER_SCANLINE / TOTAL_WIDTH;
const CYCLES_PER_SAMPLE: usize = CLOCK_RATE / SAMPLE_RATE;
const CYCLES_PER_HORIZONTAL_BLANK: usize = BLANK_WIDTH * CYCLES_PER_PIXEL;
const CYCLES_PER_VERTICAL_BLANK: usize = BLANK_HEIGHT * CYCLES_PER_SCANLINE;
const SAMPLES_PER_FRAME: usize = SAMPLE_RATE / FRAME_RATE;
const INCREMENT: f32 = TAU / SAMPLE_RATE as f32;

pub struct VirtualMachine {
    ram: Vec<u8>,
    pub audio: Vec<f32>,
    sin_phase: f32,
    pub video: Vec<u8>,
    cpu: Cpu,
    pub error_message: Vec<u8>,
}

impl VirtualMachine {
    /// Create a new virtual machine with the given ROM.
    pub fn new(rom: Vec<u8>) -> Self {
        // TODO: Fix nop (no longer 0x000000)
        let mut ram = vec![0; MEMORY_SIZE];
        ram.splice(0..rom.len(), rom);
        Self {
            ram,
            audio: vec![0.0; SAMPLES_PER_FRAME],
            sin_phase: 0.0,
            video: vec![0; WIDTH * HEIGHT * 4],
            cpu: Cpu::default(),
            error_message: vec![],
        }
    }

    pub fn error(message: String) -> Self {
        let error_message = message.as_bytes().to_vec();
        let ram = vec![0; MEMORY_SIZE];
        Self {
            ram,
            audio: vec![0.0; SAMPLES_PER_FRAME],
            sin_phase: 0.0,
            video: vec![0; WIDTH * HEIGHT * 4],
            cpu: Cpu::default(),
            error_message,
        }
    }

    pub fn registers(&self) -> [u32; REGISTER_COUNT] {
        self.cpu.registers()
    }

    /// Run the virtual machine for one frame.
    pub fn run(&mut self) {
        for y in 0..HEIGHT {
            let cycle = y * CYCLES_PER_SCANLINE;
            for x in 0..WIDTH {
                self.step(CYCLES_PER_PIXEL);
                // This is a video cycle, update the pixel.
                let color_index = (x + y * WIDTH) * 4;
                let ram_index = FRAMEBUFFER + color_index;
                self.video[color_index + 0] = self.ram[ram_index + 0];
                self.video[color_index + 1] = self.ram[ram_index + 1];
                self.video[color_index + 2] = self.ram[ram_index + 2];
                self.video[color_index + 3] = self.ram[ram_index + 3];
                let cycle = cycle + x * CYCLES_PER_PIXEL;
                if cycle % CYCLES_PER_SAMPLE == 0 {
                    // This is an audio cycle as well, update the sample.
                    self.sample(cycle);
                }
            }

            // Keep updating audio samples in horizontal blank.
            let cycle = cycle + WIDTH * CYCLES_PER_PIXEL;
            // cycle = beginning of HBLANK
            let cycles_until_first_sample =
                CYCLES_PER_SAMPLE - (cycle + CYCLES_PER_SAMPLE - 1) % CYCLES_PER_SAMPLE;

            self.step(cycles_until_first_sample);
            let cycle = cycle + cycles_until_first_sample;
            // cycle = at first sample cycle.
            self.sample(cycle);
            let cycles_after_first_sample = CYCLES_PER_HORIZONTAL_BLANK - cycles_until_first_sample;
            let extra_sample_count = cycles_after_first_sample / CYCLES_PER_SAMPLE;
            for extra_sample in 1..=extra_sample_count {
                self.step(CYCLES_PER_SAMPLE);
                let cycle = cycle + CYCLES_PER_SAMPLE * extra_sample;
                self.sample(cycle);
            }
            // let cycle = cycle + cycles_after_first_sample;
            // cycle = beginning of VBLANK (ideally)
        }

        self.cpu.set(REGISTER_INTERRUPT, INTERRUPT_VBLANK);

        // Keep updating audio samples in vertical blank.
        let cycle = HEIGHT * CYCLES_PER_SCANLINE;
        // cycle = beginning of VBLANK
        let cycles_until_first_sample =
            CYCLES_PER_SAMPLE - (cycle + CYCLES_PER_SAMPLE - 1) % CYCLES_PER_SAMPLE;
        self.step(cycles_until_first_sample);
        let cycle = cycle + cycles_until_first_sample;
        // cycle = at first sample cycle
        self.sample(cycle);
        let cycles_after_first_sample = CYCLES_PER_VERTICAL_BLANK - cycles_until_first_sample - 1;
        let extra_sample_count = cycles_after_first_sample / CYCLES_PER_SAMPLE;
        for extra_sample in 1..=extra_sample_count {
            self.step(CYCLES_PER_SAMPLE);
            let cycle = cycle + CYCLES_PER_SAMPLE * extra_sample;
            // cycle = at extra sample cycle
            self.sample(cycle);
        }
        // TODO: self.step(cycles_after_last_sample)
        // let cycle = cycle + cycles_after_first_sample
        // cycle = at next frame (ideally)
    }

    /// Step the virtual machine for `cycles`.
    fn step(&mut self, cycles: usize) {
        for _ in 0..cycles {
            let program_counter = self.cpu[REGISTER_PROGRAM_COUNTER];
            self.cpu.set(REGISTER_PROGRAM_COUNTER, program_counter + 3);
            // TODO: Avoid overflow.
            let instruction = u32::from_be_bytes([
                0,
                self.ram[program_counter as usize + 0],
                self.ram[program_counter as usize + 1],
                self.ram[program_counter as usize + 2],
            ]);
            let c = instruction & 0b1_00000_000000_000000_000000;
            let c = c != 0;
            if !c || self.cpu.condition() {
                let op = instruction & 0b0_11111_000000_000000_000000;
                let op = op >> 18;
                let op: Op = op.into();
                use Op::*;
                match op {
                    Shri | Shli | Slessi | Load | Load2 | Load3 | Store | Store2 | Store3 | Ori
                    | Nori | Andi | Xori | Lessi | Addi | Subi | Muli => {
                        self.i(op, instruction);
                    }
                    Let | Lethi => {
                        self.l(op, instruction);
                    }
                    Ashr | Rol | Shr | Shl | Sless | Or | Nor | And | Xor | Less | Add | Sub
                    | Mul => {
                        self.r(op, instruction);
                    }
                }
            }
        }
    }

    /// Sample audio channels for output buffer.
    fn sample(&mut self, cycle: usize) {
        let sample_index = cycle / CYCLES_PER_SAMPLE;
        let midi = self.ram[0xFA0003] as f32 + self.ram[0xFA0004] as f32 / 256.0;
        let frequency = 2.0_f32.powf((midi - 69.0) / 12.0) * 440.0;
        let increment = frequency * INCREMENT;
        self.sin_phase += increment;
        self.sin_phase %= TAU;
        self.audio[sample_index] = 0.125 * (self.sin_phase - TAU / 2.0).signum();
    }

    /// Perform composition operation specified in IO registers.
    fn composite(&mut self, mode: u32) {
        let source_address = u32::from_be_bytes([
            0,
            self.ram[COMPOSITE_SRC_ADDR + 0],
            self.ram[COMPOSITE_SRC_ADDR + 1],
            self.ram[COMPOSITE_SRC_ADDR + 2],
        ]);
        let source_width = u32::from_be_bytes([
            0,
            self.ram[COMPOSITE_SRC_WIDTH + 0],
            self.ram[COMPOSITE_SRC_WIDTH + 1],
            self.ram[COMPOSITE_SRC_WIDTH + 2],
        ]);
        let source_height = u32::from_be_bytes([
            0,
            self.ram[COMPOSITE_SRC_HEIGHT + 0],
            self.ram[COMPOSITE_SRC_HEIGHT + 1],
            self.ram[COMPOSITE_SRC_HEIGHT + 2],
        ]);
        let source_stride = u32::from_be_bytes([
            0,
            self.ram[COMPOSITE_SRC_STRIDE + 0],
            self.ram[COMPOSITE_SRC_STRIDE + 1],
            self.ram[COMPOSITE_SRC_STRIDE + 2],
        ]);
        let destination_p0 = u32::from_be_bytes([
            0,
            self.ram[COMPOSITE_DST_P0 + 0],
            self.ram[COMPOSITE_DST_P0 + 1],
            self.ram[COMPOSITE_DST_P0 + 2],
        ]);
        let destination_p1 = u32::from_be_bytes([
            0,
            self.ram[COMPOSITE_DST_P1 + 0],
            self.ram[COMPOSITE_DST_P1 + 1],
            self.ram[COMPOSITE_DST_P1 + 2],
        ]);
        let destination_p2 = u32::from_be_bytes([
            0,
            self.ram[COMPOSITE_DST_P2 + 0],
            self.ram[COMPOSITE_DST_P2 + 1],
            self.ram[COMPOSITE_DST_P2 + 2],
        ]);
        let destination_p3 = u32::from_be_bytes([
            0,
            self.ram[COMPOSITE_DST_P3 + 0],
            self.ram[COMPOSITE_DST_P3 + 1],
            self.ram[COMPOSITE_DST_P3 + 2],
        ]);
        let destination_address = FRAMEBUFFER;
        // Assuming points go clockwise top-left to bottom-left
        let x0 = destination_p0 % WIDTH as u32;
        let y0 = destination_p0 / WIDTH as u32;
        let x1 = destination_p1 % WIDTH as u32;
        let y1 = destination_p1 / WIDTH as u32;
        let x2 = destination_p2 % WIDTH as u32;
        let y2 = destination_p2 / WIDTH as u32;
        let x3 = destination_p3 % WIDTH as u32;
        let y3 = destination_p3 / WIDTH as u32;
        let min_x = x0.min(x1).min(x2).min(x3);
        let max_x = x0.max(x1).max(x2).max(x3);
        let min_y = y0.min(y1).min(y2).min(y3);
        let max_y = y0.max(y1).max(y2).max(y3);
    }

    /// Execute immediate instruction.
    fn i(&mut self, op: Op, instruction: u32) {
        let r = instruction & 0o77_00_00;
        let r = r >> 12;
        let s = instruction & 0o00_77_00;
        let s = s >> 6;
        let s = self.cpu[s];
        let u = instruction & 0o00_00_77;
        use Op::*;
        match op {
            Shri => {
                self.cpu.set(r, s >> u);
            }
            Shli => {
                self.cpu.set(r, s << u);
            }
            Slessi => {
                let t = ((s << 8) as i32) < (u << 8) as i32;
                self.cpu.set(r, t as u32);
                self.cpu.set_condition(s == u);
            }
            Load => {
                let i = (u << 2) as i8 as i32 >> 2;
                let address = s as i32 + i;
                // TODO: Add overflow/underflow test.
                let value = self.ram[address as usize] as u32;
                self.cpu.set(r, value)
            }
            Load2 => {
                let i = (u << 2) as i8 as i32 >> 2;
                let address = s as i32 + i;
                // TODO: Add overflow/underflow test.
                let value = u32::from_be_bytes([
                    0,
                    0,
                    self.ram[address as usize + 0],
                    self.ram[address as usize + 1],
                ]);
                self.cpu.set(r, value)
            }
            Load3 => {
                let i = (u << 2) as i8 as i32 >> 2;
                let address = s as i32 + i;
                // TODO: Add overflow/underflow test.
                let value = u32::from_be_bytes([
                    0,
                    self.ram[address as usize + 0],
                    self.ram[address as usize + 1],
                    self.ram[address as usize + 2],
                ]);
                self.cpu.set(r, value)
            }
            Store => {
                let i = (u << 2) as i8 as i32 >> 2;
                let address = self.cpu[r] as i32 + i;
                let address = address as usize;
                // TODO: Add overflow/underflow test.
                self.ram[address] = s as u8;
            }
            Store2 => {
                let i = (u << 2) as i8 as i32 >> 2;
                let address = self.cpu[r] as i32 + i;
                // TODO: Add overflow/underflow test.
                let [_, _, a, b] = s.to_be_bytes();
                self.ram[address as usize + 0] = a;
                self.ram[address as usize + 1] = b;
            }
            Store3 => {
                let i = (u << 2) as i8 as i32 >> 2;
                let address = self.cpu[r] as i32 + i;
                let address = address as usize;
                // TODO: Add overflow/underflow test.
                let [_, a, b, c] = s.to_be_bytes();
                self.ram[address + 0] = a;
                self.ram[address + 1] = b;
                self.ram[address + 2] = c;
                match address {
                    COMPOSITE_MODE => self.composite(s),
                    _ => {}
                }
            }
            Ori => {
                self.cpu.set(r, s | u);
                self.cpu.set_condition(s | u == 0);
            }
            Nori => {
                self.cpu.set(r, !(s | u));
                self.cpu.set_condition(!(s | u) & MASK as u32 == 0);
            }
            Andi => {
                self.cpu.set(r, s & u);
                self.cpu.set_condition(s & u == 0);
            }
            Xori => {
                self.cpu.set(r, s ^ u);
                self.cpu.set_condition(s ^ u == 0);
            }
            Lessi => {
                self.cpu.set(r, (s < u) as u32);
                self.cpu.set_condition(s == u);
            }
            Addi => {
                let (add, overflow) = s.overflowing_add(u);
                self.cpu.set(r, add);
                self.cpu.set_condition(overflow || 0xFFFFFF < s + u);
            }
            Subi => {
                let (sub, overflow) = s.overflowing_sub(u);
                self.cpu.set(r, sub);
                self.cpu.set_condition(overflow || 0 > s as i32 - u as i32);
            }
            Muli => {
                let (mul, overflow) = s.overflowing_mul(u);
                self.cpu.set(r, mul);
                self.cpu.set_condition(overflow || 0xFFFFFF < s as u64 * u as u64);
            }
            _ => unreachable!(),
        }
    }

    /// Execute let instruction.
    fn l(&mut self, op: Op, instruction: u32) {
        let r = instruction & 0o77_00_00;
        let r = r >> 12;
        let u = instruction & 0o00_77_77;
        match op {
            Op::Let => self.cpu.set(r, u),
            Op::Lethi => {
                let lo = self.cpu[r] & 0o77_77;
                let u = u << 12;
                self.cpu.set(r, lo | u);
            }
            _ => unreachable!(),
        }
    }

    fn r(&mut self, op: Op, instruction: u32) {
        let r = instruction & 0o77_00_00;
        let r = r >> 12;
        let s = instruction & 0o00_77_00;
        let s = s >> 6;
        let s = self.cpu[s];
        let t = instruction & 0o00_00_77;
        let t = self.cpu[t];
        use Op::*;
        match op {
            Ashr => {
                let s = (s << 8) as i32;
                let t = (t << 8) as i32;
                let s = s >> (t >> 8);
                let s = s as u32 >> 8;
                self.cpu.set(r, s)
            }
            Rol => {
                let t = (t << 8) as i32 >> 8;
                let t = t % BITS as i32;
                // let t = t.abs() % BITS as i32 * t.signum();
                if t.is_positive() {
                    let (roll_left, _overflow) = s.overflowing_shl(t as u32);
                    let (roll_right, _overflow) = (s as i32).overflowing_shr((24 - t) as u32);
                    self.cpu.set(r, roll_left | roll_right as u32);
                } else {
                    let (roll_right, _overflow) = (s as i32).overflowing_shr(-t as u32);
                    let (roll_left, _overflow) = s.overflowing_shl((24 + t) as u32);
                    self.cpu.set(r, roll_left | roll_right as u32);
                }
            }
            Shr => {
                self.cpu.set(r, s >> t);
            }
            Shl => {
                self.cpu.set(r, s << t);
            }
            Sless => {
                let u = ((s << 8) as i32) < (t << 8) as i32;
                self.cpu.set(r, u as u32);
                self.cpu.set_condition(s == t);
            }
            Or => {
                self.cpu.set(r, s | t);
                self.cpu.set_condition(s | t == 0);
            }
            Nor => {
                self.cpu.set(r, !(s | t));
                self.cpu.set_condition(!(s | t) & MASK as u32 == 0);
            }
            And => {
                self.cpu.set(r, s & t);
                self.cpu.set_condition(s & t == 0);
            }
            Xor => {
                self.cpu.set(r, s ^ t);
                self.cpu.set_condition(s ^ t == 0);
            }
            Less => {
                self.cpu.set(r, (s < t) as u32);
                self.cpu.set_condition(s == t);
            }
            Add => {
                let (add, overflow) = s.overflowing_add(t);
                self.cpu.set(r, add);
                self.cpu.set_condition(overflow || 0xFFFFFF < s + t);
            }
            Sub => {
                let (sub, overflow) = s.overflowing_sub(t);
                self.cpu.set(r, sub);
                self.cpu.set_condition(overflow || 0 > s as i32 - t as i32);
            }
            Mul => {
                let (mul, overflow) = s.overflowing_mul(t);
                self.cpu.set(r, mul);
                self.cpu.set_condition(overflow || 0xFFFFFF < s as u64 * t as u64);
            }
            _ => unreachable!(),
        }
    }
}
