use image::RgbImage;

use crate::generator::painter::Painter;
use crate::generator::utils::geom::find_target_draw_rect;
use crate::generator::utils::gpu::context::GPUContext;
use crate::generator::utils::pixel::blend_linear;
use crate::generator::utils::random::rng::Rng;
use crate::generator::utils::random::{
	get_random_range, get_random_ranges_bias_weighted, get_random_size_ranges_bias_weighted,
};
use crate::generator::utils::units::{Margins, SizeUnit, WeightedValue};
use crate::generator::utils::{image::get_pixel_interpolated, random::get_random_color};

use shader::Shader;

use super::PaintParameters;

pub mod shader;

pub struct RectPainter {
	pub options: Options,
	pub shader: Shader,
}

pub struct Options {
	pub width: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	pub height: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	pub width_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub height_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub rotation: Vec<WeightedValue<(f64, f64)>>,
	pub anti_alias: bool,
	pub color_seed: f64,
	pub margins: Margins<SizeUnit>,
}

impl RectPainter {
	pub fn new(context: &GPUContext) -> Self {
		let options = Options {
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
			rotation: vec![WeightedValue {
				value: (0.0, 0.0),
				weight: 1.0,
			}],
			anti_alias: true,
			color_seed: 0.0,
			margins: Margins::<SizeUnit> {
				top: SizeUnit::Pixels(0),
				right: SizeUnit::Pixels(0),
				bottom: SizeUnit::Pixels(0),
				left: SizeUnit::Pixels(0),
			},
		};

		RectPainter {
			options,
			shader: Shader::new(context),
		}
	}
}

impl Painter for RectPainter {
	fn get_paint_parameters(
		&self,
		context: &GPUContext,
		rng: &mut Rng,
		painted_texture_size: &wgpu::Extent3d,
		painted_texture_view: &wgpu::TextureView,
		seed_map: &RgbImage,
	) -> Result<PaintParameters, &str> {
		let image_area = (painted_texture_size.width, painted_texture_size.height);
		let target_area = match find_target_draw_rect(image_area, &self.options.margins) {
			Ok(rect) => rect,
			Err(err) => return Err(err),
		};
		let target_visible_area =
			(image_area.0.min(target_area.width as u32), image_area.1.min(target_area.height as u32));

		// Find random dimensions for rect to be painted
		let rect_w = get_random_size_ranges_bias_weighted(
			rng,
			&self.options.width,
			self.options.width_bias,
			target_visible_area.0,
		);
		let rect_h = get_random_size_ranges_bias_weighted(
			rng,
			&self.options.height,
			self.options.height_bias,
			target_visible_area.1,
		);

		// Rotate
		let rotation = get_random_ranges_bias_weighted(rng, &self.options.rotation, 0.0);
		let rotation_rad = rotation.to_radians();

		// Find correct needed distance from borders
		let rotation_right = (0.0f64).to_radians();
		let rotation_down = (90.0f64).to_radians();
		let space_down = (
			(rotation_down + rotation_rad).cos() * rect_h * 0.5,
			(rotation_down + rotation_rad).sin() * rect_h * 0.5,
		);
		let space_right = (
			(rotation_right + rotation_rad).cos() * rect_w * 0.5,
			(rotation_right + rotation_rad).sin() * rect_w * 0.5,
		);
		let space_bottom_left = (space_down.0 - space_right.0, space_down.1 - space_right.1);
		let space_bottom_right = (space_down.0 + space_right.0, space_down.1 + space_right.1);
		let space_x = space_bottom_left.0.abs().max(space_bottom_right.0.abs());
		let space_y = space_bottom_left.1.abs().max(space_bottom_right.1.abs());

		// Distribute along the axis too
		let rect_x = get_random_range(
			rng,
			target_area.x as f64 + space_x,
			(target_area.x + target_area.width) as f64 - space_x,
		);
		let rect_y = get_random_range(
			rng,
			target_area.y as f64 + space_y,
			(target_area.y + target_area.height) as f64 - space_y,
		);

		// Determine color
		let random_color = get_random_color(rng);
		let seed_color = get_pixel_interpolated(seed_map, rect_x, rect_y);
		let color = blend_linear(&random_color, &seed_color, self.options.color_seed);

		// Finally, serve parameters
		Ok(PaintParameters {
			pipeline: &self.shader.pipeline,
			bind_groups: self.shader.get_bind_groups(
				&context,
				rect_x,
				rect_y,
				rect_w,
				rect_h,
				rotation,
				color,
				self.options.anti_alias,
				&painted_texture_view,
			),
		})
	}
}
