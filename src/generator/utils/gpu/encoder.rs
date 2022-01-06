use std::num::NonZeroU32;

use super::context::GPUContext;
use super::workgroups::{get_workgroup_count_depth, get_workgroup_count_height, get_workgroup_count_width};

pub fn create_encoder(context: &GPUContext, label: &str) -> wgpu::CommandEncoder {
	context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
		label: Some(label),
	})
}

pub fn add_encoder_pass_compute(
	encoder: &mut wgpu::CommandEncoder,
	pipeline: &wgpu::ComputePipeline,
	bind_groups: &[wgpu::BindGroup],
	width: u32,
	height: u32,
	label: &str,
) {
	encoder.push_debug_group(format!("Compute: {}", label).as_str());
	{
		let mut compute_pass = encoder.begin_compute_pass(&Default::default());

		compute_pass.push_debug_group("Pipeline");
		compute_pass.set_pipeline(&pipeline);
		compute_pass.pop_debug_group();

		compute_pass.push_debug_group("Bind groups");
		for i in 0..bind_groups.len() {
			compute_pass.set_bind_group(i as u32, &bind_groups[i], &[]);
		}
		compute_pass.pop_debug_group();

		compute_pass.push_debug_group("Dispatch");
		compute_pass.dispatch(
			get_workgroup_count_width(width),
			get_workgroup_count_height(height),
			get_workgroup_count_depth(1),
		);
		compute_pass.pop_debug_group();
	}
	encoder.pop_debug_group();
}

pub fn add_encoder_pass_copy_buffer_to_texture(
	encoder: &mut wgpu::CommandEncoder,
	from_buffer: &wgpu::Buffer,
	from_bytes_per_row: u32,
	from_rows_per_image: u32,
	to_texture: &wgpu::Texture,
	to_texture_size: wgpu::Extent3d,
) {
	encoder.push_debug_group("Copy: buffer to texture");
	encoder.copy_buffer_to_texture(
		wgpu::ImageCopyBuffer {
			buffer: from_buffer,
			layout: wgpu::ImageDataLayout {
				offset: 0,
				bytes_per_row: NonZeroU32::new(from_bytes_per_row),
				rows_per_image: NonZeroU32::new(from_rows_per_image),
			},
		},
		to_texture.as_image_copy(),
		// wgpu::ImageCopyTexture {
		// 	texture: to_texture,
		// 	aspect: wgpu::TextureAspect::All,
		// 	mip_level: 0,
		// 	origin: wgpu::Origin3d::ZERO,
		// },
		to_texture_size,
	);
	encoder.pop_debug_group();
}

pub fn add_encoder_pass_copy_texture_to_buffer(
	encoder: &mut wgpu::CommandEncoder,
	from_texture: &wgpu::Texture,
	from_texture_size: wgpu::Extent3d,
	to_buffer: &wgpu::Buffer,
	to_bytes_per_row: u32,
) {
	encoder.push_debug_group("Copy: texture to buffer");
	encoder.copy_texture_to_buffer(
		from_texture.as_image_copy(),
		// wgpu::ImageCopyTexture {
		// 	texture: from_texture,
		// 	aspect: wgpu::TextureAspect::All,
		// 	mip_level: 0,
		// 	origin: wgpu::Origin3d::ZERO,
		// },
		wgpu::ImageCopyBuffer {
			buffer: to_buffer,
			layout: wgpu::ImageDataLayout {
				offset: 0,
				bytes_per_row: NonZeroU32::new(to_bytes_per_row),
				rows_per_image: NonZeroU32::new(from_texture_size.height),
			},
		},
		from_texture_size,
	);
	encoder.pop_debug_group();
}

pub fn add_encoder_pass_copy_texture_to_texture(
	encoder: &mut wgpu::CommandEncoder,
	from_texture: &wgpu::Texture,
	from_texture_size: wgpu::Extent3d,
	to_texture: &wgpu::Texture,
) {
	encoder.push_debug_group("Copy: texture to texture");
	{
		encoder.copy_texture_to_texture(
			from_texture.as_image_copy(),
			to_texture.as_image_copy(),
			from_texture_size,
		);
	}
	encoder.pop_debug_group();
}
