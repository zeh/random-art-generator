use image::{DynamicImage, GenericImageView, RgbImage};
use std::time::{Instant};
use painter::{Painter};

mod painter;

const LUMA_R: f64 = 0.2126;
const LUMA_G: f64 = 0.7152;
const LUMA_B: f64 = 0.0722;

/// A definition for the image generation. This will contain all data needed for a generation process.
pub struct Generator {
	target: RgbImage,
	current: RgbImage,
}

impl Generator {
	pub fn from(target_image: DynamicImage) -> Generator {
		let target = target_image.to_rgb();
		let current = RgbImage::new(target_image.dimensions().0, target_image.dimensions().1);
		Generator {
			target: target,
			current: current,
		}
	}

	pub fn prepopulate(&mut self, current_image: DynamicImage) {
		self.current = current_image.to_rgb();
	}

	pub fn process(&mut self, iterations: u32) {
		let painter = painter::RectPainter {};

		let mut improved_iterations = 0;
		let mut discarded_iterations = 0;

		let mut new_candidate;
		let mut new_diff;
		let mut curr_diff = Generator::diff(&self.current, &self.target);

		println!("Starting iterations; initial difference from target is {:.2}%.", curr_diff * 100.0);

		let mut time_start;
		let mut time_elapsed;
		let mut time_elapsed_paint;
		let mut time_elapsed_total: u128 = 0;

		let time_start_global = Instant::now();
		let mut used;

		for i in 0..iterations {
			time_start = Instant::now();
			used = false;

			new_candidate = painter.paint(&self.current);
			time_elapsed_paint = time_start.elapsed().as_millis();

			new_diff = Generator::diff(&new_candidate, &self.target);

			if new_diff < curr_diff {
				improved_iterations += 1;
				self.current = new_candidate;
				curr_diff = new_diff;
				used = true;
			} else {
				discarded_iterations += 1;
			}

			time_elapsed = time_start.elapsed().as_millis();

			if used {
				print!("Iteration {}/{} is useful;", i + 1, iterations);
				print!(" new difference is {:.2}%", new_diff * 100.0);
				println!(" ({}ms paint, {}ms total)", time_elapsed_paint, time_elapsed);
			}

			time_elapsed_total += time_elapsed;
		}

		let time_elapsed_total_2 = time_start_global.elapsed().as_millis();

		let final_diff = Generator::diff(&self.current, &self.target);
		println!("Finished in {}ms ({}ms avg per iteration), user time {}ms.", time_elapsed_total, time_elapsed_total / iterations as u128, time_elapsed_total_2);
		println!("Used {} iterations, and discarded {}.", improved_iterations, discarded_iterations);
		println!("The final difference from target is {:.2}%.", final_diff * 100.0);
	}

	pub fn get_current(&self) -> RgbImage {
		return self.current.clone();
	}

	pub fn diff(a: &RgbImage, b: &RgbImage) -> f64 {
		let w = a.dimensions().0;
		let h = a.dimensions().1;
		let num_pixels = w * h;

		let mut diff_sum_r: i32 = 0;
		let mut diff_sum_g: i32 = 0;
		let mut diff_sum_b: i32 = 0;

		let samples_a = a.as_flat_samples().samples;
		let samples_b = b.as_flat_samples().samples;
		let mut pos: usize = 0;

		for _ in 0..num_pixels {
			diff_sum_r += (samples_a[pos + 0] as i32 - samples_b[pos + 0] as i32).abs();
			diff_sum_g += (samples_a[pos + 1] as i32 - samples_b[pos + 1] as i32).abs();
			diff_sum_b += (samples_a[pos + 2] as i32 - samples_b[pos + 2] as i32).abs();
			pos += 3;
		}

		let lr = LUMA_R / 255.0;
		let lg = LUMA_G / 255.0;
		let lb = LUMA_B / 255.0;
		let diff_sum = diff_sum_r as f64 * lr + diff_sum_g as f64 * lg + diff_sum_b as f64 * lb;

 		diff_sum / (num_pixels as f64)
	}
}
