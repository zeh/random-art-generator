use std::collections::HashMap;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;

use image::{DynamicImage, Rgb, RgbImage};

use painter::Painter;
use utils::image::{color_transform, diff, scale_image};
use utils::terminal;

pub mod painter;
pub mod utils;

type ProcessCallback = fn(
	generator: &Generator,
	is_success: bool,
	is_final: bool,
	num_tries: u32,
	num_generations: u32,
	diff: f64,
	time_elapsed: f32,
	metadata: HashMap<String, String>,
);

/// A definition for the image generation. This will contain all data needed for a generation process.
pub struct Generator {
	target: RgbImage,
	current: RgbImage,
}

impl Generator {
	pub fn from_image(target_image: DynamicImage, scale: f64) -> Generator {
		let mut target = target_image.to_rgb8();
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
		let mut target = target_image.to_rgb8();
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
		self.current = current_image.to_rgb8();
	}

	pub fn prepopulate_with_color(&mut self, r: u8, g: u8, b: u8) {
		let dimensions = self.current.dimensions();
		self.current = RgbImage::from_pixel(dimensions.0, dimensions.1, Rgb([r, g, b]))
	}

	pub fn process(
		&mut self,
		tries: u32,
		generations: u32,
		candidates: usize,
		painter: impl Painter + Send + Sync + 'static,
		cb: Option<ProcessCallback>,
	) {
		let mut curr_diff = diff(&self.current, &self.target);

		println!("Starting iterations; initial difference from target is {:.2}%.", curr_diff * 100.0);

		let mut used;

		let time_started = Instant::now();
		let mut time_elapsed_paint = 0;
		let mut time_elapsed_diff = 0;
		let mut time_started_iteration;
		let mut time_elapsed_iteration = 0;

		let mut total_tries: u32 = 0;
		let mut total_gen: u32 = 0;

		let mut total_processes: u64 = 0;

		let arc_painter = Arc::new(painter);
		let arc_target = Arc::new(self.target.clone());

		println!("First try...");

		loop {
			time_started_iteration = Instant::now();
			used = false;

			if candidates == 1 {
				// Simple path with no concurrency
				let time_started_paint = Instant::now();
				let new_candidate = arc_painter.paint(&self.current, total_processes, &self.target).expect("painting");
				time_elapsed_paint += time_started_paint.elapsed().as_micros();

				let time_started_diff = Instant::now();
				let new_diff = diff(&new_candidate, &self.target);
				time_elapsed_diff += time_started_diff.elapsed().as_micros();

				if new_diff < curr_diff {
					self.current = new_candidate;
					curr_diff = new_diff;
					used = true;
				}

				total_processes = total_processes.wrapping_add(1);
			} else {
				// Complex path with concurrency
				// This can sometimes be slower because of all the image cloning done,
				// and will use more memory, but on the aggregate reaches successfull
				// generations in about 25% of the original time
				let (tx, rx) = mpsc::channel();

				for candidate in 0..candidates {
					let tx1 = mpsc::Sender::clone(&tx);
					let thread_painter = Arc::clone(&arc_painter);
					let thread_current = self.current.clone();
					let thread_target = Arc::clone(&arc_target);

					thread::spawn(move || {
						let new_candidate = thread_painter.paint(
							&thread_current,
							total_processes.wrapping_add(candidate as u64),
							&thread_target,
						).expect("painting");
						let new_diff = diff(&new_candidate, &thread_target);

						// Only report candidates that are actually better than the current diff,
						// to minimize the back-and-forth of data. To be fair, however, this doesn't
						// seem to to do much in terms of performance.
						if new_diff < curr_diff {
							tx1.send((new_candidate, new_diff)).unwrap();
						}
					});
				}

				drop(tx);

				for (new_candidate, new_diff) in rx {
					if new_diff < curr_diff {
						self.current = new_candidate;
						curr_diff = new_diff;
						used = true;
					}
				}

				total_processes = total_processes.wrapping_add(candidates as u64);
			}

			if used {
				total_gen += 1;
			}

			total_tries += 1;

			let finished =
				(tries > 0 && total_tries == tries) || (generations > 0 && total_gen == generations);

			if cb.is_some() {
				(cb.unwrap())(
					&self,
					used,
					finished,
					total_tries,
					total_gen,
					curr_diff,
					time_started.elapsed().as_secs_f32(),
					arc_painter.get_metadata(),
				);
			}

			time_elapsed_iteration += time_started_iteration.elapsed().as_micros();

			// Only output log if the generation succeeded
			if used {
				terminal::cursor_up();
				terminal::erase_line_to_end();

				// Tries block
				if tries > 0 {
					print!("Try {}/{} is useful; ", total_tries, tries);
				} else {
					print!("Try {} is useful; ", total_tries);
				}

				// Generations block
				if generations > 0 {
					print!("{}/{} generations so far, ", total_gen, generations);
				} else {
					print!("{} generations so far, ", total_gen);
				}

				// Diff block
				println!("new difference is {:.2}%", curr_diff * 100.0);
			}

			if finished {
				// Requirements reached, can stop iterations
				break;
			}
		}

		let time_elapsed = time_started.elapsed().as_secs_f32();
		let atts = total_tries as f64 * 1000.0;

		let final_diff = diff(&self.current, &self.target);
		println!(
			"Finished {} tries in {:.3}s ({:.3}ms avg per try), using {} candidate threads.",
			total_tries,
			time_elapsed,
			time_elapsed_iteration as f64 / atts,
			candidates
		);
		if candidates == 1 {
			println!(
				"Tries took an average of {:.3}ms for painting, and {:.3}ms for diffing, using a single thread.",
				time_elapsed_paint as f64 / atts,
				time_elapsed_diff as f64 / atts
			);
		}
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
