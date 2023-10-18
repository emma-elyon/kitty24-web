use std::f32::consts::TAU;

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

pub struct VirtualMachine {
    ram: Vec<u8>,
	frames: usize,
    pub audio: Vec<f32>,
    pub video: Vec<u8>,
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
					let sample_index = cycle / CYCLES_PER_SAMPLE;
					let i = self.frames + sample_index;
					self.audio[sample_index] = (220.0 * TAU * i as f32 / SAMPLE_RATE as f32).sin();
				}
            }

			// Keep updating audio samples in horizontal blank.
			let cycle = cycle + WIDTH * CYCLES_PER_PIXEL;
			// cycle = beginning of HBLANK
			let cycles_until_first_sample = cycle % CYCLES_PER_SAMPLE;
			self.step(cycles_until_first_sample);
			let cycle = cycle + cycles_until_first_sample;
			// cycle = at first sample cycle.
			let sample_index = cycle / CYCLES_PER_SAMPLE;
			let i = self.frames + sample_index;
			self.audio[sample_index] = (220.0 * TAU * i as f32 / SAMPLE_RATE as f32).sin();
			let cycles_after_first_sample = CYCLES_PER_HORIZONTAL_BLANK - cycles_until_first_sample;
			let extra_sample_count = cycles_after_first_sample / CYCLES_PER_SAMPLE;
			for extra_sample in 1..=extra_sample_count {
				self.step(CYCLES_PER_SAMPLE);
				let cycle = cycle + CYCLES_PER_SAMPLE * extra_sample;
				let sample_index = cycle / CYCLES_PER_SAMPLE;
				let i = self.frames + sample_index;
				self.audio[sample_index] = (220.0 * TAU * i as f32 / SAMPLE_RATE as f32).sin();
			}
			// let cycle = cycle + cycles_after_first_sample;
			// cycle = beginning of VBLANK (ideally)
        }

		// Keep updating audio samples in vertical blank.
		let cycle = HEIGHT * CYCLES_PER_SCANLINE;
		// cycle = beginning of VBLANK
		let cycles_until_first_sample = cycle % CYCLES_PER_SAMPLE;
		self.step(cycles_until_first_sample);
		let cycle = cycle + cycles_until_first_sample;
		// cycle = at first sample cycle
		let sample_index = cycle / CYCLES_PER_SAMPLE;
		let i = self.frames + sample_index;
		self.audio[sample_index] = (220.0 * TAU * i as f32 / SAMPLE_RATE as f32).sin();
		let cycles_after_first_sample = CYCLES_PER_VERTICAL_BLANK - cycles_until_first_sample;
		let extra_sample_count = cycles_after_first_sample / CYCLES_PER_SAMPLE;
		for extra_sample in 1..=extra_sample_count {
			self.step(CYCLES_PER_SAMPLE);
			let cycle = cycle + CYCLES_PER_SAMPLE * extra_sample;
			// cycle = at extra sample cycle
			let sample_index = cycle / CYCLES_PER_SAMPLE;
			let i = self.frames + sample_index;
			self.audio[sample_index] = (220.0 * TAU * i as f32 / SAMPLE_RATE as f32).sin();
		}
		// let cycle = cycle + cycles_after_first_sample
		// cycle = at next frame (ideally)

		self.frames += self.audio.len();
        self.ram[0] += 1;
    }

	fn step(&mut self, _cycles: usize) {
	}
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self {
            ram: vec![0; MEMORY_SIZE],
            audio: vec![0.0; SAMPLES_PER_FRAME],
            video: vec![0; WIDTH * HEIGHT * 4],
			frames: 0,
        }
    }
}
