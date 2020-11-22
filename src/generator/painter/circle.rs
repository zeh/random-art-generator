use image::{Pixel, Rgb, RgbImage};
use rand::{thread_rng, Rng};

use crate::generator::painter::Painter;
use crate::generator::utils::geom::distance;
use crate::generator::utils::image::blend_pixel;
use crate::generator::utils::random::{get_random_range, get_random_ranges, get_random_size_ranges_bias};
use crate::generator::utils::units::SizeUnit;

#[derive(Clone)]
pub struct CirclePainter {
	pub options: Options,
}

#[derive(Clone)]
pub struct Options {
	pub alpha: Vec<(f64, f64)>,
	pub radius: Vec<(SizeUnit, SizeUnit)>,
	pub radius_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub anti_alias: bool,
}

impl CirclePainter {
	pub fn new() -> CirclePainter {
		let options = Options {
			alpha: vec![(1.0, 1.0)],
			radius: vec![(SizeUnit::Fraction(0.0), SizeUnit::Fraction(0.5))],
			radius_bias: 0.0f64,
			anti_alias: true,
		};

		CirclePainter {
			options,
		}
	}
}

impl Painter for CirclePainter {
	fn paint(&self, canvas: &RgbImage) -> RgbImage {
		let mut rng = thread_rng();

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
		let r = rng.gen_range(0u8, 255u8);
		let g = rng.gen_range(0u8, 255u8);
		let b = rng.gen_range(0u8, 255u8);
		let top_pixel = Rgb([r, g, b]);
		let top_pixel_channels = top_pixel.channels();
		let alpha = get_random_ranges(&mut rng, &self.options.alpha);

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
						top_pixel_channels,
						new_alpha * alpha,
					));
					painted_canvas.put_pixel(x, y, new_pixel);
				}
			}
		}

		painted_canvas
	}
}
