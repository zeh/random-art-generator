use std::collections::HashMap;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;

use image::{DynamicImage, Rgb, RgbImage};

use painter::Painter;
use utils::benchmark::TimerBenchmark;
use utils::formatting::format_time;
use utils::image::{color_transform as image_color_transform, diff as image_diff, scale as image_scale};
use utils::numbers::AverageNumber;
use utils::terminal;

pub mod painter;
pub mod utils;

pub enum ProcessResult {
	// Image generated and sent along with its diff value
	Ok(RgbImage, f64),
	// Image generated, but we now its diff is not better than the current one, so we don't send anything
	Ignore,
	// Could not generate image because of an error
	Error(String),
}

pub struct ProcessCallbackResult {
	pub is_success: bool,
	pub is_final: bool,
	pub num_tries: u32,
	pub num_generations: u32,
	pub diff: f64,
	pub time_elapsed: f32,
	pub metadata: HashMap<String, String>,
}

pub struct GeneratorBenchmarks {
	paint: TimerBenchmark,
	diff: TimerBenchmark,
	generation: TimerBenchmark,
	result_callback: TimerBenchmark,
	whole_try: TimerBenchmark,
	total: TimerBenchmark,
}

type ProcessCallback = fn(generator: &Generator, result: ProcessCallbackResult);

/// A definition for the image generation. This will contain all data needed for a generation process.
pub struct Generator {
	target: RgbImage,
	current: RgbImage,
}

fn print_benchmark(bench: &TimerBenchmark, label: &str) {
	println!(
		"[BENCH] {:}: min {:.3}ms, avg {:.3}ms, median {:.3}, max {:.3}ms",
		label,
		bench.min_ms(),
		bench.average_ms(),
		bench.median_ms(),
		bench.max_ms()
	);
}

impl Generator {
	pub fn from_image(target_image: DynamicImage, scale: f64) -> Generator {
		let mut target = target_image.to_rgb8();
		if scale != 1.0f64 {
			target = image_scale(&target, scale);
		}
		let current = RgbImage::new(target.dimensions().0, target.dimensions().1);
		Generator {
			target,
			current,
		}
	}

	pub fn from_image_and_matrix(target_image: DynamicImage, scale: f64, matrix: [f64; 12]) -> Generator {
		let mut target = target_image.to_rgb8();
		if scale != 1.0f64 {
			target = image_scale(&target, scale);
		}
		let current = RgbImage::new(target.dimensions().0, target.dimensions().1);
		Generator {
			target: image_color_transform(&target, matrix),
			current,
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
		target_tries: u32,
		target_generations: u32,
		target_diff: f64,
		should_benchmark: bool,
		candidates: usize,
		painter: impl Painter + Send + Sync + 'static,
		cb: Option<ProcessCallback>,
	) {
		let mut curr_diff = image_diff(&self.current, &self.target);

		println!("Starting tries; initial difference from target is {:.2}%.", curr_diff * 100.0);

		let mut used;

		let mut benchmarks = GeneratorBenchmarks {
			paint: TimerBenchmark::new(),
			diff: TimerBenchmark::new(),
			generation: TimerBenchmark::new(),
			result_callback: TimerBenchmark::new(),
			whole_try: TimerBenchmark::new(),
			total: TimerBenchmark::new(),
		};

		let mut curr_tries: u32 = 0;
		let mut curr_generations: u32 = 0;

		let mut total_processes: u32 = 0;

		let arc_painter = Arc::new(painter);
		let arc_target = Arc::new(self.target.clone());

		let mut time_elapsed_try_avg = AverageNumber::new(100);
		let mut time_elapsed_generation_avg = AverageNumber::new(50);
		let mut time_elapsed_diff_pct_avg = AverageNumber::new(50);

		println!("First try...");

		benchmarks.total.start();
		benchmarks.generation.start();

		let mut diff_last_generation = curr_diff;
		let mut time_last_print = Instant::now();

		loop {
			benchmarks.whole_try.start();
			used = false;

			if should_benchmark || candidates == 1 {
				// Simple path with no concurrency
				benchmarks.paint.start();
				let new_candidate =
					arc_painter.paint(&self.current, total_processes, &self.target).expect("painting");
				benchmarks.paint.stop();

				benchmarks.diff.start();
				let new_diff = image_diff(&new_candidate, &self.target);
				benchmarks.diff.stop();

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
						let result = match thread_painter.paint(
							&thread_current,
							total_processes.wrapping_add(candidate as u32),
							&thread_target,
						) {
							Ok(new_candidate) => {
								let new_diff = image_diff(&new_candidate, &thread_target);

								// Only report candidates that are actually better than the current diff,
								// to minimize the back-and-forth of data. To be fair, however, this doesn't
								// seem to to do much in terms of performance.
								if new_diff < curr_diff {
									ProcessResult::Ok(new_candidate, new_diff)
								} else {
									ProcessResult::Ignore
								}
							}
							Err(err) => ProcessResult::Error(err.to_owned()),
						};
						tx1.send(result).unwrap();
					});
				}

				drop(tx);

				for result in rx {
					match result {
						ProcessResult::Ok(new_candidate, new_diff) => {
							if new_diff < curr_diff {
								self.current = new_candidate;
								curr_diff = new_diff;
								used = true;
							}
						}
						ProcessResult::Ignore => {}
						ProcessResult::Error(err) => {
							panic!("{}", err);
						}
					}
				}

				total_processes = total_processes.wrapping_add(candidates as u32);
			}

			if used {
				curr_generations += 1;

				// Update time stats for generation
				benchmarks.generation.stop();
				time_elapsed_generation_avg.put(benchmarks.generation.last_ms());

				// Update time stats for diff
				let diff_change = diff_last_generation - curr_diff;
				let diff_time = benchmarks.generation.last_ms();
				let diff_time_per_pct = diff_time / diff_change;
				time_elapsed_diff_pct_avg.put(diff_time_per_pct);
				diff_last_generation = curr_diff;
			}

			curr_tries += 1;

			let finished = (target_tries > 0 && curr_tries == target_tries)
				|| (target_generations > 0 && curr_generations == target_generations)
				|| (target_diff > 0.0 && curr_diff <= target_diff);

			if !finished && !benchmarks.generation.is_started() {
				benchmarks.generation.start();
			}

			if let Some(process_callback) = cb {
				benchmarks.result_callback.start();
				process_callback(
					&self,
					ProcessCallbackResult {
						is_success: used,
						is_final: finished,
						num_tries: curr_tries,
						num_generations: curr_generations,
						diff: curr_diff,
						time_elapsed: benchmarks.total.current_ms() as f32 / 1000.0,
						metadata: arc_painter.get_metadata(),
					},
				);
				benchmarks.result_callback.stop();
			}

			// Update time stats for tries
			benchmarks.whole_try.stop();
			time_elapsed_try_avg.put(benchmarks.whole_try.last_ms() as f64);

			// Only output log if the generation succeeded, or if enough time has passed
			if used || time_last_print.elapsed().as_secs() >= 1 {
				terminal::cursor_up();
				terminal::erase_line_to_end();

				// Tries block
				if target_tries > 0 {
					let remaining = target_tries - curr_tries;
					let time_left = if curr_tries > 0 {
						format_time((remaining as f64 * time_elapsed_try_avg.get().unwrap()).max(0.0))
					} else {
						"∞".to_string()
					};
					print!("Try {}/{} ({} left): ", curr_tries, target_tries, time_left,);
				} else {
					print!("Try {}: ", curr_tries);
				}

				// Generations block
				if target_generations > 0 {
					let remaining = target_generations - curr_generations;
					let time_left = if curr_generations > 0 {
						format_time((remaining as f64 * time_elapsed_generation_avg.get().unwrap()).max(0.0))
					} else {
						"∞".to_string()
					};
					print!(
						"{}/{} generations so far ({} left), ",
						curr_generations, target_generations, time_left,
					);
				} else {
					print!("{} generations so far, ", curr_generations);
				}

				// Diff block
				if target_diff > 0.0 {
					let remaining = curr_diff - target_diff;
					let time_left =
						format_time((remaining as f64 * time_elapsed_diff_pct_avg.get().unwrap()).max(0.0));
					println!(
						"new difference is {:.2}%/{:.2}% ({} left)",
						curr_diff * 100.0,
						target_diff * 100.0,
						time_left,
					);
				} else {
					println!("new difference is {:.2}%", curr_diff * 100.0);
				}

				time_last_print = Instant::now();
			}

			if finished {
				// Requirements reached, can stop trying
				break;
			}
		}

		benchmarks.total.stop();

		let final_diff = image_diff(&self.current, &self.target);
		println!(
			"Finished {} tries in {:.3}s ({:.3}ms avg per try), using {} candidate threads.",
			curr_tries,
			benchmarks.total.last_ms() / 1000.0,
			benchmarks.whole_try.average_ms(),
			candidates
		);
		if candidates == 1 {
			println!(
				"Tries took an average of {:.3}ms for painting, and {:.3}ms for diffing, using a single thread.",
				benchmarks.paint.average_ms(),
				benchmarks.diff.average_ms()
			);

			if should_benchmark {
				print_benchmark(&benchmarks.paint, "paint");
				print_benchmark(&benchmarks.diff, "diff");
				print_benchmark(&benchmarks.result_callback, "result_callback");
				print_benchmark(&benchmarks.whole_try, "whole_try");
			}
		}
		println!(
			"Produced {} generations, a {:.2}% success rate.",
			curr_generations,
			curr_generations as f64 / curr_tries as f64 * 100.0
		);
		println!("The final difference from target is {:.2}%.", final_diff * 100.0);
	}

	pub fn get_current(&self) -> RgbImage {
		self.current.clone()
	}
}
