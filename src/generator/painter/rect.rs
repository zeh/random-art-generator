use image::{Pixel, Rgb, RgbImage};
use rand::{thread_rng, Rng};

use crate::generator::painter::Painter;
use crate::generator::utils::image::blend_pixel;
use crate::generator::utils::random::{
	get_random_range, get_random_ranges_bias, get_random_size_ranges_bias,
};
use crate::generator::utils::units::SizeUnit;

#[derive(Clone)]
pub struct RectPainter {
	pub options: Options,
}

#[derive(Clone)]
pub struct Options {
	pub alpha: Vec<(f64, f64)>,
	pub alpha_bias: f64,
	pub width: Vec<(SizeUnit, SizeUnit)>,
	pub height: Vec<(SizeUnit, SizeUnit)>,
	pub width_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub height_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
}

impl RectPainter {
	pub fn new() -> RectPainter {
		let options = Options {
			alpha: vec![(1.0, 1.0)],
			alpha_bias: 0.0f64,
			width: vec![(SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0))],
			width_bias: 0.0f64,
			height: vec![(SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0))],
			height_bias: 0.0f64,
		};

		RectPainter {
			options,
		}
	}
}

impl Painter for RectPainter {
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
		let alpha = get_random_ranges_bias(&mut rng, &self.options.alpha, self.options.alpha_bias);

		// Finally, paint
		let mut painted_canvas = canvas.clone();
		for x in x1..x2 {
			for y in y1..y2 {
				let new_pixel =
					Rgb(blend_pixel(painted_canvas.get_pixel(x, y).channels(), top_pixel_channels, alpha));
				painted_canvas.put_pixel(x, y, new_pixel);
			}
		}

		painted_canvas
	}
}
