use image::{Pixel, Rgb, RgbImage};

use crate::generator::utils::geom::distance;
use crate::generator::utils::image::blend_pixel;
use crate::generator::utils::random::{
	get_random_range, get_random_ranges_bias, get_random_size_ranges_bias, get_rng,
};
use crate::generator::utils::units::SizeUnit;
use crate::generator::{
	painter::Painter,
	utils::{image::get_pixel_interpolated, random::get_random_color},
};

#[derive(Clone)]
pub struct CirclePainter {
	pub options: Options,
}

#[derive(Clone)]
pub struct Options {
	pub alpha: Vec<(f64, f64)>,
	pub alpha_bias: f64,
	pub radius: Vec<(SizeUnit, SizeUnit)>,
	pub radius_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub anti_alias: bool,
	pub color_seed: f64,
	pub rng_seed: u128,
}

impl CirclePainter {
	pub fn new() -> CirclePainter {
		let options = Options {
			alpha: vec![(1.0, 1.0)],
			alpha_bias: 0.0f64,
			radius: vec![(SizeUnit::Fraction(0.0), SizeUnit::Fraction(0.5))],
			radius_bias: 0.0f64,
			anti_alias: true,
			color_seed: 0.0f64,
			rng_seed: 0u128,
		};

		CirclePainter {
			options,
		}
	}
}

impl Painter for CirclePainter {
	fn paint(&self, canvas: &RgbImage, iteration: u64, seed_map: &RgbImage) -> RgbImage {
		let mut rng = get_rng(self.options.rng_seed, iteration);

		let image_w_i = canvas.dimensions().0;
		let image_h_i = canvas.dimensions().1;
		let image_w = image_w_i as f64;
		let image_h = image_h_i as f64;

		// Find random radius
		let radius: f64 =
			get_random_size_ranges_bias(&mut rng, &self.options.radius, self.options.radius_bias, image_w_i);

		// Distribute along the axis too
		let circle_x: f64 = get_random_range(&mut rng, 0.0f64, 1.0f64) * (image_w - radius * 2.0f64);
		let circle_y: f64 = get_random_range(&mut rng, 0.0f64, 1.0f64) * (image_h - radius * 2.0f64);

		// Found final, round positions
		let cx = circle_x + radius;
		let cy = circle_y + radius;
		let x1 = (cx - radius).floor().max(0.0).min(image_w) as u32;
		let y1 = (cy - radius).floor().max(0.0).min(image_h) as u32;
		let x2 = (cx + radius).ceil().max(0.0).min(image_w) as u32;
		let y2 = (cy + radius).ceil().max(0.0).min(image_h) as u32;

		// Determine color
		let random_color = get_random_color(&mut rng);
		let seed_color = get_pixel_interpolated(seed_map, cx, cy);
		let color = blend_pixel(&random_color, &seed_color, self.options.color_seed);
		let alpha = get_random_ranges_bias(&mut rng, &self.options.alpha, self.options.alpha_bias);

		// Finally, paint
		let mut painted_canvas = canvas.clone();
		for x in x1..x2 {
			for y in y1..y2 {
				let dist = distance(cx, cy, x as f64, y as f64);
				if dist <= radius {
					let abs = radius - dist;
					let new_alpha = if abs > 1.0f64 {
						1.0f64
					} else {
						if self.options.anti_alias {
							abs
						} else {
							if abs >= 0.5f64 {
								1.0f64
							} else {
								0.0f64
							}
						}
					};
					let new_pixel = Rgb(blend_pixel(
						painted_canvas.get_pixel(x, y).channels(),
						&color,
						new_alpha * alpha,
					));
					painted_canvas.put_pixel(x, y, new_pixel);
				}
			}
		}

		painted_canvas
	}
}
