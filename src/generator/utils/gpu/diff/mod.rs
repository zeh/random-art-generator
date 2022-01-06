use crate::generator::utils::color::{LUMA_B, LUMA_G, LUMA_R};

use super::buffer::get_gpu_buffer_data;
use super::context::GPUContext;
use super::encoder::{add_encoder_pass_compute, create_encoder};
use super::texture::TextureInfo;

use self::shader::DiffShader;

pub mod shader;

#[cfg(test)]
pub mod tests;

// Given a [u8] buffer coming from the diff shader, calculate the actual diff value in the 0..1 range.
pub fn calculate_total_diff_from_buffer(
	context: &GPUContext,
	out_buffer: &wgpu::Buffer,
	width: u32,
	height: u32,
) -> f64 {
	const LUMA_R_PRE: f64 = LUMA_R / 255.0;
	const LUMA_G_PRE: f64 = LUMA_G / 255.0;
	const LUMA_B_PRE: f64 = LUMA_B / 255.0;

	// Get diff data
	let data = get_gpu_buffer_data(&context, &out_buffer);

	// TODO: use something like bytemuck::cast_slice(&[etc]) to convert buffer back?

	// Convert [[u8; 4]] to Vec<u32>
	let diffs = data
		.chunks_exact(4)
		.map(|chunks| {
			(chunks[0] as u32) | (chunks[1] as u32) << 8 | (chunks[2] as u32) << 16 | (chunks[3] as u32) << 24
		})
		.collect::<Vec<_>>();

	// Converts [[u32], [u32], u32]] to a sum of (u64, u64, u64)
	let diffs_all = diffs.chunks_exact(3).fold((0, 0, 0), |accum, chunks| {
		(accum.0 + (chunks[0] as u64), accum.1 + (chunks[1] as u64), accum.0 + (chunks[2] as u64))
	});

	// Average per pixel with luminosity applied
	((diffs_all.0 as f64 * LUMA_R_PRE)
		+ (diffs_all.1 as f64 * LUMA_G_PRE)
		+ (diffs_all.2 as f64 * LUMA_B_PRE))
		/ (width * height) as f64
}

/// This creates an encoder, runs a diff compute pass, and immediately returns the value.
/// It's meant to be used when a quick diff step is needed.
pub fn calculate_diff_from_textures(
	context: &GPUContext,
	diff_shader: &DiffShader,
	texture_a: &TextureInfo,
	texture_b: &TextureInfo,
) -> f64 {
	let mut encoder = create_encoder(&context, "Quick diff encoder");
	encoder.push_debug_group("Calculate");

	let width = texture_a.texture_descriptor.size.width;
	let height = texture_a.texture_descriptor.size.height;

	let diff_out_buffer = diff_shader.create_results_buffer(context, width, height);
	let diff_bind_groups = diff_shader.get_bind_groups(
		context,
		&texture_a.texture_view,
		&texture_b.texture_view,
		&diff_out_buffer,
	);
	add_encoder_pass_compute(&mut encoder, &diff_shader.pipeline, &diff_bind_groups, width, height, "diff");

	encoder.pop_debug_group();
	context.queue.submit(Some(encoder.finish()));

	calculate_total_diff_from_buffer(context, &diff_out_buffer, width, height)
}
