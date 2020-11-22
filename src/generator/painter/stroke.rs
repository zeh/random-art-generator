use image::{Pixel, Rgb, RgbImage};
use rand::{thread_rng, Rng};

use crate::generator::painter::Painter;
use crate::generator::utils::image::blend_pixel;
use crate::generator::utils::random::{
	get_noise_value, get_random_noise_sequence, get_random_range, get_random_ranges,
	get_random_size_ranges_bias,
};
use crate::generator::utils::units::SizeUnit;

#[derive(Clone)]
pub struct StrokePainter {
	pub options: Options,
}

#[derive(Clone)]
pub struct Options {
	pub alpha: Vec<(f64, f64)>,
	pub width: Vec<(SizeUnit, SizeUnit)>,
	pub height: Vec<(SizeUnit, SizeUnit)>,
	pub width_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub height_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub wave_height: Vec<(SizeUnit, SizeUnit)>,
	pub wave_height_bias: f64,
	pub wave_length: Vec<(SizeUnit, SizeUnit)>,
	pub wave_length_bias: f64,
	pub anti_alias: bool,
}

impl StrokePainter {
	pub fn new() -> StrokePainter {
		let options = Options {
			alpha: vec![(1.0, 1.0)],
			width: vec![(SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0))],
			width_bias: 0.0f64,
			height: vec![(SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0))],
			height_bias: 0.0f64,
			wave_height: vec![(SizeUnit::Fraction(0.01), SizeUnit::Fraction(0.01))],
			wave_height_bias: 0.0f64,
			wave_length: vec![(SizeUnit::Fraction(0.5), SizeUnit::Fraction(0.5))],
			wave_length_bias: 0.0f64,
			anti_alias: true,
		};

		StrokePainter {
			options,
		}
	}
}

impl Painter for StrokePainter {
	fn paint(&self, canvas: &RgbImage) -> RgbImage {
		let mut rng = thread_rng();

		let image_w_i = canvas.dimensions().0;
		let image_h_i = canvas.dimensions().1;
		let image_w = image_w_i as f64;
		let image_h = image_h_i as f64;

		// Find random dimensions
		let rect_w: f64 =
			get_random_size_ranges_bias(&mut rng, &self.options.width, self.options.width_bias, image_w_i);
		let rect_h: f64 =
			get_random_size_ranges_bias(&mut rng, &self.options.height, self.options.height_bias, image_h_i);

		// Distribute along the axis too
		let rect_x: f64 = get_random_range(&mut rng, 0.0f64, 1.0f64) * (image_w - rect_w);
		let rect_y: f64 = get_random_range(&mut rng, 0.0f64, 1.0f64) * (image_h - rect_h);

		// Found final, round positions
		let x1 = rect_x.round().max(0.0).min(image_w) as u32;
		let x2 = (rect_x + rect_w).round().max(0.0).min(image_w) as u32;
		let y1 = rect_y.round().max(0.0).min(image_w) as u32;
		let y2 = (rect_y + rect_h).round().max(0.0).min(image_h) as u32;

		// Determine color
		let r = rng.gen_range(0u8, 255u8);
		let g = rng.gen_range(0u8, 255u8);
		let b = rng.gen_range(0u8, 255u8);
		let top_pixel = Rgb([r, g, b]);
		let top_pixel_channels = top_pixel.channels();
		let alpha = get_random_ranges(&mut rng, &self.options.alpha);

		// Determine waviness
		let wave_height: f64 = get_random_size_ranges_bias(
			&mut rng,
			&self.options.wave_height,
			self.options.wave_height_bias,
			image_h_i,
		);
		let wave_length: f64 = get_random_size_ranges_bias(
			&mut rng,
			&self.options.wave_length,
			self.options.wave_length_bias,
			image_w_i,
		);

		let mut painted_canvas = canvas.clone();

		// Finally, paint
		if wave_height == 0.0 || wave_length == 0.0 {
			// Fast path, no waviness
			for x in x1..x2 {
				for y in y1..y2 {
					let new_pixel = Rgb(blend_pixel(
						painted_canvas.get_pixel(x, y).channels(),
						top_pixel_channels,
						alpha,
					));
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
			let x2_safe = (x2 + margin_ceil).min(image_w as u32);
			let y1_safe = (y1 as i64 - margin_ceil as i64).max(0) as u32;
			let y2_safe = (y2 + margin_ceil).min(image_h as u32);

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
						top_pixel_channels,
						if self.options.anti_alias {
							alpha_x * alpha_y * alpha
						} else {
							if alpha_x * alpha_y >= 0.5f64 {
								1.0f64
							} else {
								0.0f64
							}
						}
					));
					painted_canvas.put_pixel(x, y, new_pixel);
				}
			}
		}

		painted_canvas
	}
}
