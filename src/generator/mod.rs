use std::collections::HashMap;
use std::time::Instant;

use image::{DynamicImage, Rgb, RgbImage};

use crate::generator::painter::{PaintParameters, Painter};
use crate::generator::utils::gpu::diff::calculate_diff_from_textures;

use self::utils::benchmark::TimerBenchmark;
use self::utils::color::BlendingMode;
use self::utils::formatting::format_time;
use self::utils::gpu::blending::shader::BlendingShader;
use self::utils::gpu::blending::shaders::BlendingShaders;
use self::utils::gpu::context::GPUContext;
use self::utils::gpu::diff::calculate_total_diff_from_buffer;
use self::utils::gpu::diff::shader::DiffShader;
use self::utils::gpu::encoder::add_encoder_pass_compute;
use self::utils::gpu::encoder::create_encoder;
use self::utils::gpu::texture::TextureInfo;
use self::utils::gpu::texture::{copy_textures_to_textures, create_image_from_texture};
use self::utils::gpu::texture::{create_texture, create_texture_from_image_rgb};
use self::utils::image::{color_transform as image_color_transform, scale as image_scale};
use self::utils::numbers::AverageNumber;
use self::utils::random::{get_random_entry_weighted, get_random_ranges_bias_weighted, get_rng};
use self::utils::terminal;
use self::utils::units::WeightedValue;

pub mod painter;
pub mod utils;

pub struct GeneratorBenchmarks {
	prepare: TimerBenchmark,
	paint: TimerBenchmark,
	paint_queue: TimerBenchmark,
	paint_diff_buffers: TimerBenchmark,
	generation: TimerBenchmark,
	result_callback: TimerBenchmark,
	whole_try: TimerBenchmark,
	total: TimerBenchmark,
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

type ProcessCallback = fn(
	context: &GPUContext,
	generator: &Generator,
	texture: &wgpu::Texture,
	texture_size: wgpu::Extent3d,
	result: ProcessCallbackResult,
);

pub struct PaintCandidate {
	pub painted_texture: TextureInfo,
	pub composited_texture: TextureInfo,
	pub composited_diff_buffer: Option<wgpu::Buffer>,
	pub composited_diff: f64,
}

impl PaintCandidate {
	pub fn new(context: &GPUContext, width: u32, height: u32) -> Self {
		let painted_texture = create_texture(context, width, height, "Painted");
		let composited_texture = create_texture(context, width, height, "Composited");

		PaintCandidate {
			painted_texture,
			composited_texture,
			composited_diff_buffer: None,
			composited_diff: 0.0,
		}
	}
}

fn print_benchmark(bench: &TimerBenchmark, label: &str) {
	match bench.len() {
		0 => println!("[BENCH] {:>20}: N/A", label),
		1 => println!("[BENCH] {:>20}: {:.3}ms", label, bench.last_ms()),
		_ => println!(
			"[BENCH] {:>20} ({}x): min {:.3}ms, avg {:.3}ms, median {:.3}, max {:.3}ms",
			label,
			bench.len(),
			bench.min_ms(),
			bench.average_ms(),
			bench.median_ms(),
			bench.max_ms()
		),
	}
}

/// A definition for the image generation. This will contain all data needed for a generation process.
pub struct Generator {
	target: RgbImage,
	input: RgbImage,
}

impl Generator {
	pub fn from_image(target_image: DynamicImage, scale: f64) -> Generator {
		let mut target = target_image.to_rgb8();
		if scale != 1.0f64 {
			target = image_scale(&target, scale);
		}
		let input = RgbImage::new(target.dimensions().0, target.dimensions().1);
		Generator {
			target,
			input,
		}
	}

	pub fn from_image_and_matrix(target_image: DynamicImage, scale: f64, matrix: [f64; 12]) -> Generator {
		let mut target = target_image.to_rgb8();
		if scale != 1.0f64 {
			target = image_scale(&target, scale);
		}
		let input = RgbImage::new(target.dimensions().0, target.dimensions().1);
		Generator {
			target: image_color_transform(&target, matrix),
			input,
		}
	}

	pub fn prepopulate_with_image(&mut self, current_image: DynamicImage) {
		self.input = current_image.to_rgb8();
	}

	pub fn prepopulate_with_color(&mut self, r: u8, g: u8, b: u8) {
		let dimensions = self.input.dimensions();
		self.input = RgbImage::from_pixel(dimensions.0, dimensions.1, Rgb([r, g, b]))
	}

	pub fn process(
		&mut self,
		context: GPUContext,
		rng_seed: u32,
		target_tries: u32,
		target_generations: u32,
		target_diff: f64,
		should_benchmark: bool,
		blending_mode: Vec<WeightedValue<BlendingMode>>,
		blending_opacity: Vec<WeightedValue<(f64, f64)>>,
		blending_opacity_bias: f64,
		num_candidates: usize,
		painter: impl Painter,
		cb: Option<ProcessCallback>,
	) {
		let mut benchmarks = GeneratorBenchmarks {
			prepare: TimerBenchmark::new(),
			paint: TimerBenchmark::new(),
			paint_queue: TimerBenchmark::new(),
			paint_diff_buffers: TimerBenchmark::new(),
			generation: TimerBenchmark::new(),
			result_callback: TimerBenchmark::new(),
			whole_try: TimerBenchmark::new(),
			total: TimerBenchmark::new(),
		};

		benchmarks.prepare.start();

		let (target_width, target_height) = self.target.dimensions();
		let target_texture = create_texture_from_image_rgb(&context, &self.target, "Target");
		let current_texture = create_texture_from_image_rgb(&context, &self.input, "Current");

		let blending_shaders = BlendingShaders::new(&context);
		let diff_shader = DiffShader::new(&context);

		// Calculate initial diff
		let mut curr_diff =
			calculate_diff_from_textures(&context, &diff_shader, &target_texture, &current_texture);

		println!("Starting tries; initial difference from target is {:.2}%.", curr_diff * 100.0);

		let mut used;

		let mut curr_tries: u32 = 0;
		let mut curr_generations: u32 = 0;

		let mut total_processes: u32 = 0;

		let mut paint_candidates = Vec::<PaintCandidate>::new();
		for _ in 0..num_candidates {
			paint_candidates.push(PaintCandidate::new(&context, target_width, target_height));
		}

		let mut time_elapsed_try_avg = AverageNumber::new(100);
		let mut time_elapsed_generation_avg = AverageNumber::new(50);
		let mut time_elapsed_diff_pct_avg = AverageNumber::new(50);

		println!("First try...");

		benchmarks.total.start();
		benchmarks.generation.start();

		let mut diff_last_generation = curr_diff;
		let mut time_last_print = Instant::now();

		benchmarks.prepare.stop();

		loop {
			benchmarks.whole_try.start();
			used = false;

			benchmarks.paint.start();
			benchmarks.paint_queue.start();

			let mut encoder = create_encoder(&context, format!("Encoder: try #{}", curr_tries).as_str());
			encoder.push_debug_group(format!("Try: #{}", curr_tries).as_str());

			for candidate in 0..num_candidates {
				encoder.push_debug_group(format!("Candidate: #{}", candidate).as_str());

				// TODO: reuse the same RNG later, instead of trying to advance manually?
				let mut rng = get_rng(rng_seed, total_processes.wrapping_add(candidate.try_into().unwrap()));

				// Basic parameters
				let paint_parameters = painter
					.get_paint_parameters(
						&context,
						&mut rng,
						&paint_candidates[candidate].painted_texture.texture_descriptor.size,
						&paint_candidates[candidate].painted_texture.texture_view,
						&self.target,
					)
					.expect("creating paint parameters");

				// Additional parameters
				let blending_opacity =
					get_random_ranges_bias_weighted(&mut rng, &blending_opacity, blending_opacity_bias);
				let blending_mode = get_random_entry_weighted(&mut rng, &blending_mode);
				let blending_shader = blending_shaders.get_from_blending_mode(blending_mode);

				// Finally, enqueue painting, blending, and diffing new candidate
				self.paint_new_candidate(
					&context,
					&mut encoder,
					&target_texture,
					&current_texture,
					&mut paint_candidates[candidate],
					&paint_parameters,
					&blending_shader,
					blending_opacity,
					&diff_shader,
				);

				encoder.pop_debug_group();
			}

			encoder.pop_debug_group();
			benchmarks.paint_queue.stop();

			// Dispatch all
			context.queue.submit(Some(encoder.finish()));

			// Calculate diffs
			benchmarks.paint_diff_buffers.start();

			// TODO: simply/queue this
			for candidate in 0..num_candidates {
				paint_candidates[candidate].composited_diff = calculate_total_diff_from_buffer(
					&context,
					&paint_candidates[candidate]
						.composited_diff_buffer
						.as_ref()
						.expect("calculating diff from buffer"),
					target_width,
					target_height,
				);
			}

			benchmarks.paint_diff_buffers.stop();
			benchmarks.paint.stop();

			// Decide the best diff of all results
			let best_paint_candidate = paint_candidates
				.iter_mut()
				.reduce(|best, item| {
					if item.composited_diff < best.composited_diff {
						item
					} else {
						best
					}
				})
				.expect("finding best candidate diff");

			let new_diff = best_paint_candidate.composited_diff;

			if new_diff < curr_diff {
				self.set_current_from_candidate(&context, &current_texture, best_paint_candidate);
				curr_diff = new_diff;
				used = true;
			}

			total_processes = total_processes.wrapping_add(num_candidates as u32);

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
					&context,
					&self,
					&current_texture.texture,
					current_texture.texture_descriptor.size,
					ProcessCallbackResult {
						is_success: used,
						is_final: finished,
						num_tries: curr_tries,
						num_generations: curr_generations,
						diff: curr_diff,
						time_elapsed: benchmarks.total.current_ms() as f32 / 1000.0,
						metadata: self.get_metadata(rng_seed),
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
					print!("Try {}/{} ({} left): ", curr_tries, target_tries, time_left);
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

		println!(
			"Finished {} tries in {:.3}s ({:.3}ms avg per try), using {} candidate threads.",
			curr_tries,
			benchmarks.total.last_ms() / 1000.0,
			benchmarks.whole_try.average_ms(),
			num_candidates
		);

		if num_candidates == 1 {
			println!(
				"Tries took an average of {:.3}ms for painting, using a single thread.",
				benchmarks.paint.average_ms(),
			);
		}

		if should_benchmark {
			print_benchmark(&benchmarks.prepare, "prepare");
			print_benchmark(&benchmarks.paint, "paint");
			print_benchmark(&benchmarks.paint_queue, "paint_queue");
			print_benchmark(&benchmarks.paint_diff_buffers, "paint_diff_buffers");
			print_benchmark(&benchmarks.result_callback, "result_callback");
			print_benchmark(&benchmarks.whole_try, "whole_try");
		}

		println!(
			"Produced {} generations, a {:.2}% success rate.",
			curr_generations,
			curr_generations as f64 / curr_tries as f64 * 100.0
		);
		println!("The final difference from target is {:.2}%.", curr_diff * 100.0);
	}

	fn paint_new_candidate(
		&self,
		context: &GPUContext,
		encoder: &mut wgpu::CommandEncoder,
		target_texture: &TextureInfo,
		current_texture: &TextureInfo,
		paint_candidate: &mut PaintCandidate,
		paint_parameters: &PaintParameters,
		blending_shader: &BlendingShader,
		blending_opacity: f64,
		diff_shader: &DiffShader,
	) {
		let width = paint_candidate.painted_texture.texture_descriptor.size.width;
		let height = paint_candidate.painted_texture.texture_descriptor.size.height;

		// Add paint pipeline pass
		add_encoder_pass_compute(
			encoder,
			&paint_parameters.pipeline,
			&paint_parameters.bind_groups,
			width,
			height,
			"paint",
		);

		// Set uniforms for painting
		// TODO: we might not need this yet (moved to shader anyway)
		// context.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));

		// Add blending pipeline pass
		let blending_bind_groups = blending_shader.get_bind_groups(
			context,
			blending_opacity as f32,
			&current_texture.texture_view,
			&paint_candidate.painted_texture.texture_view,
			&paint_candidate.composited_texture.texture_view,
		);
		add_encoder_pass_compute(
			encoder,
			&blending_shader.pipeline,
			&blending_bind_groups,
			width,
			height,
			"blending",
		);

		// Add diff pipeline pass
		let diff_out_buffer = diff_shader.create_results_buffer(context, width, height);
		let diff_bind_groups = diff_shader.get_bind_groups(
			context,
			&target_texture.texture_view,
			&paint_candidate.composited_texture.texture_view,
			&diff_out_buffer,
		);
		add_encoder_pass_compute(encoder, &diff_shader.pipeline, &diff_bind_groups, width, height, "diff");

		paint_candidate.composited_diff_buffer = Some(diff_out_buffer);
	}

	fn set_current_from_candidate(
		&self,
		context: &GPUContext,
		current_texture: &TextureInfo,
		paint_candidate: &mut PaintCandidate,
	) {
		copy_textures_to_textures(
			context,
			vec![&paint_candidate.composited_texture.texture],
			vec![paint_candidate.composited_texture.texture_descriptor.size],
			vec![&current_texture.texture],
		);
	}

	fn get_metadata(&self, rng_seed: u32) -> HashMap<String, String> {
		let mut data = HashMap::new();
		data.insert(String::from("RNG seed"), format!("{}", rng_seed));
		data
	}

	pub fn get_image(
		&self,
		context: &GPUContext,
		texture: &wgpu::Texture,
		texture_size: wgpu::Extent3d,
	) -> RgbImage {
		create_image_from_texture(context, texture, texture_size, "Output")
	}
}
