use image::RgbImage;

use crate::generator::painter::Painter;
use crate::generator::utils::geom::find_target_draw_rect;
use crate::generator::utils::gpu::context::GPUContext;
use crate::generator::utils::pixel::blend_linear;
use crate::generator::utils::random::rng::Rng;
use crate::generator::utils::random::{get_random_range, get_random_size_ranges_bias_weighted};
use crate::generator::utils::units::{Margins, SizeUnit, WeightedValue};
use crate::generator::utils::{image::get_pixel_interpolated, random::get_random_color};

use shader::Shader;

use super::PaintParameters;

pub mod shader;

pub struct CirclePainter {
	pub options: Options,
	pub shader: Shader,
}

pub struct Options {
	pub radius: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	pub radius_bias: f64, // 0 = normal; -1 = quad bias towards small, 1 = quad bias towards big, etc
	pub anti_alias: bool,
	pub color_seed: f64,
	pub margins: Margins<SizeUnit>,
}

impl CirclePainter {
	pub fn new(context: &GPUContext) -> Self {
		let options = Options {
			radius: vec![WeightedValue {
				value: (SizeUnit::Fraction(0.0), SizeUnit::Fraction(0.5)),
				weight: 1.0,
			}],
			radius_bias: 0.0,
			anti_alias: true,
			color_seed: 0.0,
			margins: Margins::<SizeUnit> {
				top: SizeUnit::Pixels(0),
				right: SizeUnit::Pixels(0),
				bottom: SizeUnit::Pixels(0),
				left: SizeUnit::Pixels(0),
			},
		};

		CirclePainter {
			options,
			shader: Shader::new(context),
		}
	}
}

impl Painter for CirclePainter {
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

		// Find random radius for the circle to be painted
		let max_dimension = target_visible_area.0.min(target_visible_area.1);
		let radius = get_random_size_ranges_bias_weighted(
			rng,
			&self.options.radius,
			self.options.radius_bias,
			max_dimension,
		);

		// Distribute along the axis too
		let circle_x = get_random_range(
			rng,
			target_area.x as f64 + radius,
			(target_area.x + target_area.width) as f64 - radius,
		);
		let circle_y = get_random_range(
			rng,
			target_area.y as f64 + radius,
			(target_area.y + target_area.height) as f64 - radius,
		);

		// Determine color
		let random_color = get_random_color(rng);
		let seed_color = get_pixel_interpolated(seed_map, circle_x, circle_y);
		let color = blend_linear(&random_color, &seed_color, self.options.color_seed);

		// Finally, serve parameters
		Ok(PaintParameters {
			pipeline: &self.shader.pipeline,
			bind_groups: self.shader.get_bind_groups(
				&context,
				circle_x,
				circle_y,
				radius,
				color,
				self.options.anti_alias,
				&painted_texture_view,
			),
		})
	}
}
