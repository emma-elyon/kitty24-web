mod cpu;

use std::f32::consts::TAU;

use common::*;
use cpu::*;

const BITS: usize = 24;
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
}

impl VirtualMachine {
    /// Run the virtual machine for one frame.
    pub fn run(&mut self) {
        for y in 0..HEIGHT {
            let cycle = y * CYCLES_PER_SCANLINE;
            for x in 0..WIDTH {
                self.step(CYCLES_PER_PIXEL);
                // This is a video cycle, update the pixel.
                let color_index = (x + y * WIDTH) * 4;
                self.video[color_index + 0] = 127 * x as u8 + self.ram[0];
                self.video[color_index + 1] = 0;
                self.video[color_index + 2] = 127 * y as u8 - self.ram[0];
                self.video[color_index + 3] = 255;
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
        // let cycle = cycle + cycles_after_first_sample
        // cycle = at next frame (ideally)

        // self.frames += self.audio.len();
        self.ram[0] += 1;
    }

    /// Step the virtual machine for `cycles`.
    fn step(&mut self, cycles: usize) {
        for _ in 0..cycles {
            let program_counter = self.cpu[REGISTER_PROGRAM_COUNTER];
            // We *can* run into an unchecked unwrap here, in case the program counter is at the RAM boundary.
            let [a, b, c]: [u8; 3] = self.ram
                [program_counter as usize..program_counter as usize + 3]
                .try_into()
                .unwrap();
            let instruction = u32::from_be_bytes([a, b, c, 0]);
            let c = instruction & 0b1_00000_000000_000000_000000;
            let c = c != 0;
            if !c || self.cpu.condition {
                let op = instruction & 0b0_11111_000000_000000_000000;
                let op = op >> 18;
                let op: Op = op.into();
                use Op::*;
                match op {
                    Let | Lethi => self.l(op, instruction),
                    Load | Store => self.m(op, instruction),
                    Ori | Nori | Andi | Xori => self.i(op, instruction),
                }
            }
        }
    }

    /// Sample audio channels for output buffer.
    fn sample(&mut self, cycle: usize) {
        let sample_index = cycle / CYCLES_PER_SAMPLE;
        let increment = self.ram[0] as f32 * INCREMENT;
        self.sin_phase += increment;
        self.sin_phase %= TAU;
        self.audio[sample_index] =
            0.25 * (self.sin_phase.sin() + self.ram[0] as f32 / 255.0).signum();
    }

    /// Execute immediate instruction.
    fn i(&mut self, op: Op, instruction: u32) {
        let r = instruction & 0_77_00_00;
        let r = r >> 12;
        let s = instruction & 0_00_77_00;
        let s = s >> 6;
        let u = instruction & 0_00_00_77;
        use Op::*;
        match op {
            Ori => self.cpu.set(r, s | u),
            Nori => self.cpu.set(r, !(s | u)),
            Andi => self.cpu.set(r, s & u),
            Xori => self.cpu.set(r, s ^ u),
            _ => todo!(),
        }
    }

    /// Execute let instruction.
    fn l(&mut self, op: Op, instruction: u32) {
        let r = instruction & 0_77_00_00;
        let r = r >> 12;
        let u = instruction & 0_00_77_77;
        match op {
            Op::Let => self.cpu.set(r, u),
            Op::Lethi => {
                let lo = self.cpu[r] & 0_77_77;
                let u = u << 12;
                self.cpu.set(r, lo | u)
            }
            _ => unreachable!(),
        }
    }

    fn m(&mut self, op: Op, instruction: u32) {
        let r = instruction & 0_77_00_00;
        let r = r >> 12;
        let s = instruction & 0_00_77_00;
        let s = s >> 6;
        let i = instruction & 0_00_00_77;
        // let i = ((i as u8) << 2) as i8 as i32;
        let i = (i << 2) as i8 as i32 >> 2;
        use Op::*;
        match op {
            Load => {
                let address = s as i32 + i;
                // TODO: Add underflow test.
                let value = self.ram[address as usize] as u32;
                self.cpu.set(r, value)
            }
            Store => {
                let address = r as i32 + i;
                // TODO: Add underflow test.
                self.ram[address as usize] = self.cpu[s] as u8;
            }
            _ => unreachable!(),
        }
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self {
            ram: vec![0; MEMORY_SIZE],
            audio: vec![0.0; SAMPLES_PER_FRAME],
            sin_phase: 0.0,
            video: vec![0; WIDTH * HEIGHT * 4],
            cpu: Cpu::default(),
        }
    }
}
