use image::RgbImage;

use super::utils::gpu::context::GPUContext;
use super::utils::random::rng::Rng;

pub mod circles;
pub mod rects;

pub trait Painter {
	fn get_paint_parameters(
		&self,
		context: &GPUContext,
		rng: &mut Rng,
		painted_texture_size: &wgpu::Extent3d,
		painted_texture_view: &wgpu::TextureView,
		seed_map: &RgbImage,
	) -> Result<PaintParameters, &str>;
}

pub struct PaintParameters<'a> {
	pub pipeline: &'a wgpu::ComputePipeline,
	pub bind_groups: [wgpu::BindGroup; 2],
}
