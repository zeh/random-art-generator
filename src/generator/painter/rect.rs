use image::{Pixel, Rgb, RgbImage};
use rand::{Rng, thread_rng};

use crate::generator::utils::image::{blend_pixel};
use crate::generator::utils::random::{get_random_range, get_random_ranges};
use crate::generator::painter::{Painter};

pub struct RectPainter {
	pub options: Options,
}

pub struct Options {
	pub alpha: Vec<(f64, f64)>,
	min_width: f64,
	max_width: f64,
	min_height: f64,
	max_height: f64,
	width_distribution: f64,
	height_distribution: f64,
}

impl RectPainter {
	pub fn new() -> RectPainter {
		let options = Options {
			alpha: vec![(1.0, 1.0)],
			min_width: 0.0,
			max_width: 1.0,
			min_height: 0.0,
			max_height: 1.0,
			width_distribution: 3.0, // Cubic
			height_distribution: 3.0, // Cubic
		};

		RectPainter {
			options,
		}
	}
}

impl Painter for RectPainter {
	fn paint(&self, canvas: &RgbImage) -> RgbImage {
		let mut rng = thread_rng();

		let image_w = canvas.dimensions().0 as f64;
		let image_h = canvas.dimensions().1 as f64;

		// Find dimensions in the 0-1 range
		let random_w: f64 = get_random_range(&mut rng, 0.0f64, 1.0f64).powf(self.options.width_distribution);
		let random_h: f64 = get_random_range(&mut rng, 0.0f64, 1.0f64).powf(self.options.height_distribution);

		// Lerp dimensions into pixels
		let rect_w: f64 = (self.options.min_width + random_w * (self.options.max_width - self.options.min_width)) * image_w;
		let rect_h: f64 = (self.options.min_height + random_h * (self.options.max_height - self.options.min_height)) * image_h;

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

		// Finally, paint
		let mut painted_canvas = canvas.clone();
		for x in x1..x2 {
			for y in y1..y2 {
				let new_pixel = Rgb(
					blend_pixel(
						painted_canvas.get_pixel(x, y).channels(),
						top_pixel_channels,
						alpha
					)
				);
				painted_canvas.put_pixel(x, y, new_pixel);
			}
		}

		painted_canvas
	}
}
