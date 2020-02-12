use image::{Pixel, Rgb, RgbImage};
use rand::{Rng, thread_rng};

use crate::generator::painter::{Painter};
use crate::generator::utils::geom::{distance};
use crate::generator::utils::image::{blend_pixel};
use crate::generator::utils::random::{get_random_range};

pub struct CirclePainter {
	options: Options,
}

struct Options {
	min_radius: f64,
	max_radius: f64,
	min_alpha: f64,
	max_alpha: f64,
	radius_distribution: f64,
	anti_alias: bool,
}

impl CirclePainter {
	pub fn new() -> CirclePainter {
		let options = Options {
			min_radius: 0.0,
			max_radius: 0.5,
			min_alpha: 0.9,
			max_alpha: 1.0,
			radius_distribution: 3.0, // Cubic
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

		let image_w = canvas.dimensions().0 as f64;
		let image_h = canvas.dimensions().1 as f64;

		// Find dimensions in the 0-1 range
		let random_r: f64 = get_random_range(&mut rng, 0.0f64, 1.0f64).powf(self.options.radius_distribution);

		// Lerp dimensions into pixels
		let radius: f64 = (self.options.min_radius + random_r * (self.options.max_radius - self.options.min_radius)) * image_w;

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
		let alpha: f64 = get_random_range(&mut rng, self.options.min_alpha, self.options.max_alpha);

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
							if abs >= 0.5f64 { 1.0f64 } else { 0.0f64 }
						}
					};
					let new_pixel = Rgb(
						blend_pixel(
							painted_canvas.get_pixel(x, y).channels(),
							top_pixel_channels,
							new_alpha * alpha
						)
					);
					painted_canvas.put_pixel(x, y, new_pixel);
				}
			}
		}

		painted_canvas
	}
}
