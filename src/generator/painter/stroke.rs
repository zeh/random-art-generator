use std::collections::HashMap;

use image::{GrayImage, Pixel, Rgb, RgbImage};

use crate::generator::utils::geom::find_target_draw_rect;
use crate::generator::utils::image::blend_pixel;
use crate::generator::utils::random::{
	get_noise_value, get_random_noise_sequence, get_random_range, get_random_ranges_bias_weighted,
	get_random_size_ranges_bias_weighted, get_rng,
};
use crate::generator::utils::units::{Margins, SizeUnit, WeightedValue};
use crate::generator::{
	painter::Painter,
	utils::{image::get_pixel_interpolated, random::get_random_color},
};

#[derive(Clone)]
pub struct StrokePainter {
	pub options: Options,
}

#[derive(Clone)]
pub struct Options {
	pub alpha: Vec<WeightedValue<(f64, f64)>>,
	pub alpha_bias: f64,
	pub width: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	pub height: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	pub width_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub height_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub wave_height: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	pub wave_height_bias: f64,
	pub wave_length: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	pub wave_length_bias: f64,
	pub anti_alias: bool,
	pub color_seed: f64,
	pub rng_seed: u32,
	pub margins: Margins<SizeUnit>,
}

impl StrokePainter {
	pub fn new() -> StrokePainter {
		let options = Options {
			alpha: vec![WeightedValue {
				value: (1.0, 1.0),
				weight: 1.0,
			}],
			alpha_bias: 0.0f64,
			width: vec![WeightedValue {
				value: (SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0)),
				weight: 1.0,
			}],
			width_bias: 0.0f64,
			height: vec![WeightedValue {
				value: (SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0)),
				weight: 1.0,
			}],
			height_bias: 0.0f64,
			wave_height: vec![WeightedValue {
				value: (SizeUnit::Fraction(0.01), SizeUnit::Fraction(0.01)),
				weight: 1.0,
			}],
			wave_height_bias: 0.0f64,
			wave_length: vec![WeightedValue {
				value: (SizeUnit::Fraction(0.5), SizeUnit::Fraction(0.5)),
				weight: 1.0,
			}],
			wave_length_bias: 0.0f64,
			anti_alias: true,
			color_seed: 0.0f64,
			rng_seed: 0,
			margins: Margins::<SizeUnit> {
				top: SizeUnit::Pixels(0),
				right: SizeUnit::Pixels(0),
				bottom: SizeUnit::Pixels(0),
				left: SizeUnit::Pixels(0),
			},
		};

		StrokePainter {
			options,
		}
	}
}

impl Painter for StrokePainter {
	fn paint(
		&self,
		canvas: &RgbImage,
		iteration: u32,
		seed_map: &RgbImage,
		focus_map: &GrayImage,
	) -> Result<RgbImage, &str> {
		let mut rng = get_rng(self.options.rng_seed, iteration);

		let image_area = canvas.dimensions();
		let target_area = match find_target_draw_rect(image_area, &self.options.margins) {
			Ok(rect) => rect,
			Err(err) => return Err(err),
		};
		let target_visible_area =
			(image_area.0.min(target_area.width as u32), image_area.1.min(target_area.height as u32));

		// Find random dimensions for rect to be painted
		let rect_w = get_random_size_ranges_bias_weighted(
			&mut rng,
			&self.options.width,
			self.options.width_bias,
			target_visible_area.0,
		);
		let rect_h = get_random_size_ranges_bias_weighted(
			&mut rng,
			&self.options.height,
			self.options.height_bias,
			target_visible_area.1,
		);

		// Distribute along the axis too
		let rect_x = get_random_range(
			&mut rng,
			target_area.x as f64,
			(target_area.x + target_area.width) as f64 - rect_w,
		);
		let rect_y = get_random_range(
			&mut rng,
			target_area.y as f64,
			(target_area.y + target_area.height) as f64 - rect_h,
		);

		// Find final, round positions
		let x1 = rect_x.round().max(0.0).min(image_area.0 as f64) as u32;
		let x2 = (rect_x + rect_w).round().max(0.0).min(image_area.0 as f64) as u32;
		let y1 = rect_y.round().max(0.0).min(image_area.1 as f64) as u32;
		let y2 = (rect_y + rect_h).round().max(0.0).min(image_area.1 as f64) as u32;

		// Determine color
		let random_color = get_random_color(&mut rng);
		let seed_color =
			get_pixel_interpolated(seed_map, (x1 + x2) as f64 / 2.0f64, (y1 + y2) as f64 / 2.0f64);
		let color = blend_pixel(&random_color, &seed_color, self.options.color_seed);
		let alpha = get_random_ranges_bias_weighted(&mut rng, &self.options.alpha, self.options.alpha_bias);

		// Determine waviness
		let wave_height = get_random_size_ranges_bias_weighted(
			&mut rng,
			&self.options.wave_height,
			self.options.wave_height_bias,
			target_visible_area.0 as u32,
		);
		let wave_length = get_random_size_ranges_bias_weighted(
			&mut rng,
			&self.options.wave_length,
			self.options.wave_length_bias,
			target_visible_area.1 as u32,
		);

		let mut painted_canvas = canvas.clone();

		// Finally, paint
		if wave_height == 0.0 || wave_length == 0.0 {
			// Fast path, no waviness
			for x in x1..x2 {
				for y in y1..y2 {
					let new_pixel =
						Rgb(blend_pixel(painted_canvas.get_pixel(x, y).channels(), &color, alpha));
					painted_canvas.put_pixel(x, y, new_pixel);
				}
			}
		} else {
			// Slow path, waviness
			let margins: f64 = wave_height / 2.0;
			let margin_ceil: u32 = margins.ceil() as u32;

			let noise = get_random_noise_sequence(&mut rng, -margins, margins);
			let noise_freq = wave_length;

			let x1_safe = (x1 as i64 - margin_ceil as i64).max(0) as u32;
			let x2_safe = (x2 + margin_ceil).min(image_area.0 as u32);
			let y1_safe = (y1 as i64 - margin_ceil as i64).max(0) as u32;
			let y2_safe = (y2 + margin_ceil).min(image_area.1 as u32);

			for x in x1_safe..x2_safe {
				for y in y1_safe..y2_safe {
					let alpha_x = if x >= x1 + margin_ceil && x < x2 - margin_ceil {
						// Inner box
						1.0f64
					} else {
						// // Part of margin
						let noise_x = get_noise_value(noise, y as f64 / noise_freq);

						let offset_x1 = x as f64 - (x1 as f64 + noise_x);
						let alpha_x1 = if offset_x1 > 0.5 {
							1.0f64
						} else if offset_x1 < -0.5 {
							0.0f64
						} else {
							offset_x1 + 0.5f64
						};

						let offset_x2 = (x2 as f64 + noise_x) - x as f64;
						let alpha_x2 = if offset_x2 > 0.5 {
							1.0f64
						} else if offset_x2 < -0.5 {
							0.0f64
						} else {
							offset_x2 + 0.5f64
						};

						alpha_x1 * alpha_x2
					};

					let alpha_y = if y >= y1 + margin_ceil && y < y2 - margin_ceil {
						// Inner box
						1.0f64
					} else {
						// // Part of margin
						let noise_y = get_noise_value(noise, x as f64 / noise_freq);

						let offset_y1 = y as f64 - (y1 as f64 + noise_y);
						let alpha_y1 = if offset_y1 > 0.5 {
							1.0f64
						} else if offset_y1 < -0.5 {
							0.0f64
						} else {
							offset_y1 + 0.5f64
						};

						let offset_y2 = (y2 as f64 + noise_y) - y as f64;
						let alpha_y2 = if offset_y2 > 0.5 {
							1.0f64
						} else if offset_y2 < -0.5 {
							0.0f64
						} else {
							offset_y2 + 0.5f64
						};

						alpha_y1 * alpha_y2
					};

					let new_pixel = Rgb(blend_pixel(
						painted_canvas.get_pixel(x, y).channels(),
						&color,
						if self.options.anti_alias {
							alpha_x * alpha_y * alpha
						} else {
							if alpha_x * alpha_y >= 0.5f64 {
								1.0f64
							} else {
								0.0f64
							}
						},
					));
					painted_canvas.put_pixel(x, y, new_pixel);
				}
			}
		}

		Ok(painted_canvas)
	}

	fn get_metadata(&self) -> HashMap<String, String> {
		let mut data = HashMap::new();
		data.insert(String::from("RNG seed"), format!("{}", &self.options.rng_seed));
		data
	}
}
