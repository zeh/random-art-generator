use std::collections::HashMap;

use image::{Pixel, Rgb, RgbImage};

use crate::generator::utils::color::BlendingMode;
use crate::generator::utils::geom::find_target_draw_rect;
use crate::generator::utils::pixel::{blend, blend_linear};
use crate::generator::utils::random::{
	get_random_entry_weighted, get_random_range, get_random_ranges_bias_weighted,
	get_random_size_ranges_bias_weighted, get_rng,
};
use crate::generator::utils::units::{Margins, SizeUnit, WeightedValue};
use crate::generator::{
	painter::Painter,
	utils::{image::get_pixel_interpolated, random::get_random_color},
};

pub struct RectPainter {
	pub options: Options,
}

pub struct Options {
	pub blending_mode: Vec<WeightedValue<BlendingMode>>,
	pub alpha: Vec<WeightedValue<(f64, f64)>>,
	pub alpha_bias: f64,
	pub width: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	pub height: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	pub width_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub height_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub color_seed: f64,
	pub rng_seed: u32,
	pub margins: Margins<SizeUnit>,
}

impl RectPainter {
	pub fn new() -> Self {
		let options = Options {
			blending_mode: vec![WeightedValue {
				value: BlendingMode::default(),
				weight: 1.0,
			}],
			alpha: vec![WeightedValue {
				value: (1.0, 1.0),
				weight: 1.0,
			}],
			alpha_bias: 0.0,
			width: vec![WeightedValue {
				value: (SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0)),
				weight: 1.0,
			}],
			width_bias: 0.0,
			height: vec![WeightedValue {
				value: (SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0)),
				weight: 1.0,
			}],
			height_bias: 0.0,
			color_seed: 0.0,
			rng_seed: 0,
			margins: Margins::<SizeUnit> {
				top: SizeUnit::Pixels(0),
				right: SizeUnit::Pixels(0),
				bottom: SizeUnit::Pixels(0),
				left: SizeUnit::Pixels(0),
			},
		};

		RectPainter {
			options,
		}
	}
}

impl Painter for RectPainter {
	fn paint(&self, canvas: &RgbImage, iteration: u32, seed_map: &RgbImage) -> Result<RgbImage, &str> {
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
		let seed_color = get_pixel_interpolated(seed_map, (x1 + x2) as f64 / 2.0, (y1 + y2) as f64 / 2.0);
		let color = blend_linear(&random_color, &seed_color, self.options.color_seed);
		let alpha = get_random_ranges_bias_weighted(&mut rng, &self.options.alpha, self.options.alpha_bias);

		// Decide on blending mode
		let blending_mode = get_random_entry_weighted(&mut rng, &self.options.blending_mode);

		// Finally, paint
		let mut painted_canvas = canvas.clone();
		for x in x1..x2 {
			for y in y1..y2 {
				let new_pixel =
					Rgb(blend(painted_canvas.get_pixel(x, y).channels(), &color, alpha, &blending_mode));
				painted_canvas.put_pixel(x, y, new_pixel);
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
