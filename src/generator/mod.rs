use image::{DynamicImage, Rgb, RgbImage};
use painter::Painter;
use std::time::Instant;

use crate::generator::utils::image::{color_transform, diff, scale_image};

pub mod painter;
pub mod utils;

type Callback = fn(generator: &Generator, success: bool);

/// A definition for the image generation. This will contain all data needed for a generation process.
pub struct Generator {
	target: RgbImage,
	current: RgbImage,
}

impl Generator {
	pub fn from_image(target_image: DynamicImage, scale: f64) -> Generator {
		let mut target = target_image.to_rgb();
		if scale != 1.0f64 {
			target = scale_image(&target, scale);
		}
		let current = RgbImage::new(target.dimensions().0, target.dimensions().1);
		Generator {
			target: target,
			current: current,
		}
	}

	pub fn from_image_and_matrix(target_image: DynamicImage, scale: f64, matrix: [f64; 12]) -> Generator {
		let mut target = target_image.to_rgb();
		if scale != 1.0f64 {
			target = scale_image(&target, scale);
		}
		let current = RgbImage::new(target.dimensions().0, target.dimensions().1);
		Generator {
			target: color_transform(&target, matrix),
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

	pub fn process(&mut self, tries: u32, generations: u32, painter: impl Painter, cb: Option<Callback>) {
		let mut new_candidate;
		let mut new_diff;
		let mut curr_diff = diff(&self.current, &self.target);

		println!("Starting iterations; initial difference from target is {:.2}%.", curr_diff * 100.0);

		let mut used;

		let time_started = Instant::now();
		let mut time_started_paint;
		let mut time_elapsed_paint = 0;
		let mut time_started_diff;
		let mut time_elapsed_diff = 0;
		let mut time_started_iteration;
		let mut time_elapsed_iteration = 0;

		let mut total_tries: u32 = 0;
		let mut total_gen: u32 = 0;

		loop {
			time_started_iteration = Instant::now();
			used = false;

			time_started_paint = Instant::now();
			new_candidate = painter.paint(&self.current);
			time_elapsed_paint += time_started_paint.elapsed().as_micros();

			time_started_diff = Instant::now();
			new_diff = diff(&new_candidate, &self.target);
			time_elapsed_diff += time_started_diff.elapsed().as_micros();

			if new_diff < curr_diff {
				total_gen += 1;
				self.current = new_candidate;
				curr_diff = new_diff;
				used = true;
			}

			match cb {
				Some(cb) => (cb)(&self, used),
				None => (),
			}

			total_tries += 1;

			time_elapsed_iteration += time_started_iteration.elapsed().as_micros();

			// Only output log if the generation succeeded
			if used {
				// Tries block
				if tries > 0 {
					print!("Try {}/{} is useful; ", total_tries + 1, tries);
				} else {
					print!("Try {} is useful; ", total_tries + 1);
				}

				// Generations block
				if generations > 0 {
					print!("{}/{} generations so far, ", total_gen, generations);
				} else {
					print!("{} generations so far, ", total_gen);
				}

				// Diff block
				println!("new difference is {:.2}%", new_diff * 100.0);
			}

			if (tries > 0 && total_tries == tries) || (generations > 0 && total_gen == generations) {
				// Requirements reached, can stop iterations
				break;
			}
		}

		let time_elapsed = time_started.elapsed().as_secs_f32();
		let atts = total_tries as f64 * 1000.0;

		let final_diff = diff(&self.current, &self.target);
		println!(
			"Finished {} tries in {:.3}s ({:.3}ms avg per try).",
			total_tries,
			time_elapsed,
			time_elapsed_iteration as f64 / atts
		);
		println!(
			"Tries took an average of {:.3}ms for painting, and {:.3}ms for diffing.",
			time_elapsed_paint as f64 / atts,
			time_elapsed_diff as f64 / atts
		);
		println!(
			"Produced {} generations, a {:.2}% success rate.",
			total_gen,
			total_gen as f64 / total_tries as f64 * 100.0
		);
		println!("The final difference from target is {:.2}%.", final_diff * 100.0);
	}

	pub fn get_current(&self) -> RgbImage {
		return self.current.clone();
	}
}
