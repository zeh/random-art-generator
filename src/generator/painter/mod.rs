use image::{Pixel, Rgb, RgbImage};
use rand::{Rng, rngs, thread_rng};

fn get_random(rng: &mut rngs::ThreadRng, min: f64, max: f64) -> f64 {
	if min == max {
		return min;
	};
	rng.gen_range(min, max)
}

pub trait Painter {
	fn new() -> Self;
	fn paint(&self, canvas: &RgbImage) -> RgbImage;
}

struct Options {
	min_width: f64,
	max_width: f64,
	min_height: f64,
	max_height: f64,
	min_alpha: f64,
	max_alpha: f64,
	width_distribution: f64,
	height_distribution: f64,
}

pub struct RectPainter {
	options: Options,
}

impl Painter for RectPainter {
	fn new() -> RectPainter {
		let options = Options {
			min_width: 0.0,
			max_width: 1.0,
			min_height: 0.0,
			max_height: 1.0,
			min_alpha: 1.0,
			max_alpha: 1.0,
			width_distribution: 3.0, // Cubic
			height_distribution: 3.0, // Cubic
		};

		RectPainter {
			options,
		}
	}

	fn paint(&self, canvas: &RgbImage) -> RgbImage {
		let mut rng = thread_rng();

		let image_w = canvas.dimensions().0 as f64;
		let image_h = canvas.dimensions().1 as f64;

		// Find dimensions in the 0-1 range
		let random_w: f64 = get_random(&mut rng, 0.0f64, 1.0f64).powf(self.options.width_distribution);
		let random_h: f64 = get_random(&mut rng, 0.0f64, 1.0f64).powf(self.options.height_distribution);

		// Lerp dimensions into pixels
		let rect_w: f64 = (self.options.min_width + random_w * (self.options.max_width - self.options.min_width)) * image_w;
		let rect_h: f64 = (self.options.min_height + random_h * (self.options.max_height - self.options.min_height)) * image_h;

		// Distribute along the axis too
		let rect_x: f64 = get_random(&mut rng, 0.0f64, 1.0f64) * (image_w - rect_w);
		let rect_y: f64 = get_random(&mut rng, 0.0f64, 1.0f64) * (image_h - rect_h);

		// Found final, round positions
		let x1 = rect_x.round().max(0.0).min(image_w) as u32;
		let x2 = (rect_x + rect_w).round().max(0.0).min(image_w) as u32;
		let y1 = rect_y.round().max(0.0).min(image_w) as u32;
		let y2 = (rect_y + rect_h).round().max(0.0).min(image_h) as u32;

		// Determine color
		let r = rng.gen_range(0u8, 255u8);
		let g = rng.gen_range(0u8, 255u8);
		let b = rng.gen_range(0u8, 255u8);
		let pixel = Rgb([r, g, b]);
		let alpha: f64 = get_random(&mut rng, self.options.min_alpha, self.options.max_alpha);
		let alpha_n: f64 = 1.0 - alpha;

		// Finally, paint
		let mut painted_canvas = canvas.clone();
		for x in x1..x2 {
			for y in y1..y2 {
				let new_pixel;
				if alpha == 1.0f64 {
					// No need to blend
					new_pixel = pixel;
				} else {
					// Blend pixels
					let old_pixel = painted_canvas.get_pixel(x, y).channels();
					let nr: u8 = (r as f64 * alpha + old_pixel[0] as f64 * alpha_n).round() as u8;
					let ng: u8 = (g as f64 * alpha + old_pixel[1] as f64 * alpha_n).round() as u8;
					let nb: u8 = (b as f64 * alpha + old_pixel[2] as f64 * alpha_n).round() as u8;
					new_pixel = Rgb([nr, ng, nb]);
				}
				painted_canvas.put_pixel(x, y, new_pixel);
			}
		}

		painted_canvas
	}
}
