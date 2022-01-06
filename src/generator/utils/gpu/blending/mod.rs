#[cfg(test)]
use image::RgbImage;

#[cfg(test)]
use super::context::GPUContext;
#[cfg(test)]
use super::encoder::{add_encoder_pass_compute, create_encoder};
#[cfg(test)]
use super::texture::{create_image_from_texture, create_texture, TextureInfo};

#[cfg(test)]
use self::shader::BlendingShader;

pub mod shader;
pub mod shaders;

#[cfg(test)]
pub mod tests;

#[cfg(test)]
fn blend_textures(
	context: &GPUContext,
	blending_shader: &BlendingShader,
	blending_opacity: f64,
	texture_bottom: &TextureInfo,
	texture_top: &TextureInfo,
	texture_result: &TextureInfo,
) {
	let mut encoder = create_encoder(&context, "Quick blending encoder");
	encoder.push_debug_group("Calculate");

	let width = texture_bottom.texture_descriptor.size.width;
	let height = texture_bottom.texture_descriptor.size.height;

	let blending_bind_groups = blending_shader.get_bind_groups(
		context,
		blending_opacity as f32,
		&texture_bottom.texture_view,
		&texture_top.texture_view,
		&texture_result.texture_view,
	);
	add_encoder_pass_compute(
		&mut encoder,
		&blending_shader.pipeline,
		&blending_bind_groups,
		width,
		height,
		"blending",
	);

	encoder.pop_debug_group();
	context.queue.submit(Some(encoder.finish()));
}

/// This creates an encoder, runs a blend compute pass, and immediately returns the value.
/// It's meant to be used when a quick blend step is needed.
#[cfg(test)]
pub fn blend_textures_to_image(
	context: &GPUContext,
	blending_shader: &BlendingShader,
	blending_opacity: f64,
	texture_bottom: &TextureInfo,
	texture_top: &TextureInfo,
) -> RgbImage {
	let width = texture_bottom.texture_descriptor.size.width;
	let height = texture_bottom.texture_descriptor.size.height;
	let texture_result = create_texture(context, width, height, "Quick blend result");
	blend_textures(context, blending_shader, blending_opacity, texture_bottom, texture_top, &texture_result);

	create_image_from_texture(
		context,
		&texture_result.texture,
		texture_result.texture_descriptor.size,
		"Quick blend result image",
	)
}
