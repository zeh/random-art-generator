use image::{DynamicImage, GenericImageView, Pixel, Rgb, RgbImage};
use painter::{Painter};
use std::time::{Instant};

pub mod painter;

const LUMA_R: f64 = 0.2126;
const LUMA_G: f64 = 0.7152;
const LUMA_B: f64 = 0.0722;

type Callback = fn(generator: &Generator, success: bool);

/// A definition for the image generation. This will contain all data needed for a generation process.
pub struct Generator {
	target: RgbImage,
	current: RgbImage,
}

impl Generator {
	pub fn from_image(target_image: DynamicImage) -> Generator {
		let target = target_image.to_rgb();
		let current = RgbImage::new(target_image.dimensions().0, target_image.dimensions().1);
		Generator {
			target: target,
			current: current,
		}
	}

	pub fn from_image_and_matrix(target_image: DynamicImage, matrix: [f64; 12]) -> Generator {
		let target = target_image.to_rgb();
		let current = RgbImage::new(target_image.dimensions().0, target_image.dimensions().1);
		Generator {
			target: Generator::color_transform(&target, matrix),
			current: current,
		}
	}

	pub fn prepopulate_with_image(&mut self, current_image: DynamicImage) {
		self.current = current_image.to_rgb();
	}

	pub fn prepopulate_with_color(&mut self, r: u8, g: u8, b: u8) {
		let dimensions = self.current.dimensions();
		self.current = RgbImage::from_pixel(dimensions.0, dimensions.1, Rgb([r, g, b]))
	}

	pub fn process(&mut self, attempts: u32, painter: impl Painter, cb: Option<Callback>) {
		let mut successful_attempts = 0;
		let mut discarded_attempts = 0;

		let mut new_candidate;
		let mut new_diff;
		let mut curr_diff = Generator::diff(&self.current, &self.target);

		println!("Starting attempts; initial difference from target is {:.2}%.", curr_diff * 100.0);

		let mut used;

		let time_started = Instant::now();
		let mut time_started_paint;
		let mut time_elapsed_paint = 0;
		let mut time_started_diff;
		let mut time_elapsed_diff = 0;
		let mut time_started_attempt;
		let mut time_elapsed_attempt = 0;

		for i in 0..attempts {
			time_started_attempt = Instant::now();
			used = false;

			time_started_paint = Instant::now();
			new_candidate = painter.paint(&self.current);
			time_elapsed_paint += time_started_paint.elapsed().as_micros();

			time_started_diff = Instant::now();
			new_diff = Generator::diff(&new_candidate, &self.target);
			time_elapsed_diff += time_started_diff.elapsed().as_micros();

			if new_diff < curr_diff {
				successful_attempts += 1;
				self.current = new_candidate;
				curr_diff = new_diff;
				used = true;
			} else {
				discarded_attempts += 1;
			}

			match cb {
				Some(cb) => (cb)(&self, used),
				None => (),
			}

			time_elapsed_attempt += time_started_attempt.elapsed().as_micros();

			if used {
				print!("Attempt {}/{} is useful;", i + 1, attempts);
				println!(" new difference is {:.2}%", new_diff * 100.0);
				//println!(" ({}ms paint, {}ms diff)", time_elapsed_paint as f64, time_elapsed_diff);
			}
		}

		let time_elapsed = time_started.elapsed().as_secs_f32();
		let atts = attempts as f64 * 1000.0;

		let final_diff = Generator::diff(&self.current, &self.target);
		println!("Finished in {:.3}s ({:.3}ms avg per attempt).", time_elapsed, time_elapsed_attempt as f64 / atts);
		println!("Attempt took an average of {:.3}ms for painting, and {:.3}ms for diffing.", time_elapsed_paint as f64 / atts, time_elapsed_diff as f64 / atts);
		println!("Used {} attempts, and discarded {}.", successful_attempts, discarded_attempts);
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

		let skip_step = 1;

		for (p_a, p_b) in samples_a.chunks_exact(3).zip(samples_b.chunks_exact(3)).step_by(skip_step) {
			diff_sum_r += (p_a[0] as i32 - p_b[0] as i32).abs();
			diff_sum_g += (p_a[1] as i32 - p_b[1] as i32).abs();
			diff_sum_b += (p_a[2] as i32 - p_b[2] as i32).abs();
		}

		let lr = LUMA_R / 255.0;
		let lg = LUMA_G / 255.0;
		let lb = LUMA_B / 255.0;
		let diff_sum = diff_sum_r as f64 * lr + diff_sum_g as f64 * lg + diff_sum_b as f64 * lb;

 		diff_sum / (num_pixels as f64 / skip_step as f64)
	}

	pub fn color_transform(image: &RgbImage, matrix: [f64; 12]) -> RgbImage {
		let mut transformed_image = image.clone();
		for (_x, _y, pixel) in transformed_image.enumerate_pixels_mut() {
			let channels = pixel.channels();
			let o_r = channels[0] as f64;
			let o_g = channels[1] as f64;
			let o_b = channels[2] as f64;
			let n_r = ((o_r * matrix[0] + o_g * matrix[1] + o_b * matrix[ 2] + matrix[ 3]).round()).max(0.0).min(255.0) as u8;
			let n_g = ((o_r * matrix[4] + o_g * matrix[5] + o_b * matrix[ 6] + matrix[ 7]).round()).max(0.0).min(255.0) as u8;
			let n_b = ((o_r * matrix[8] + o_g * matrix[9] + o_b * matrix[10] + matrix[11]).round()).max(0.0).min(255.0) as u8;
			*pixel = image::Rgb([n_r, n_g, n_b]);
		}
		transformed_image
	}
}
