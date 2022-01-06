use image::{RgbImage, RgbaImage};

use crate::generator::utils::image::{convert_rgb8_image_to_rgba8, convert_rgba8_image_to_rgb8};

use super::buffer::{
	convert_buffer_data_to_rgba_image, get_gpu_texture_data_rgba, put_gpu_texture_data_rgba,
};
use super::context::GPUContext;
use super::encoder::{
	add_encoder_pass_copy_buffer_to_texture, add_encoder_pass_copy_texture_to_buffer,
	add_encoder_pass_copy_texture_to_texture, create_encoder,
};

pub struct TextureInfo {
	pub texture_descriptor: wgpu::TextureDescriptor<'static>,
	pub texture: wgpu::Texture,
	pub texture_view: wgpu::TextureView,
}

pub struct TextureBufferInfo {
	pub buffer: wgpu::Buffer,
	pub padded_bytes_per_row: u32,
	pub unpadded_bytes_per_row: u32,
}

pub fn create_texture(context: &GPUContext, width: u32, height: u32, label: &'static str) -> TextureInfo {
	let texture_descriptor = wgpu::TextureDescriptor {
		label: Some(label),
		size: wgpu::Extent3d {
			width,
			height,
			depth_or_array_layers: 1,
		},
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: wgpu::TextureFormat::Rgba8Unorm,
		usage: wgpu::TextureUsages::COPY_SRC
			| wgpu::TextureUsages::TEXTURE_BINDING
			| wgpu::TextureUsages::STORAGE_BINDING
			// TODO: used for input textures only, check if it has any adverse effect
			| wgpu::TextureUsages::COPY_DST,
	};

	let texture = context.device.create_texture(&texture_descriptor);

	let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

	TextureInfo {
		texture_descriptor,
		texture,
		texture_view,
	}
}

pub fn create_texture_buffer(
	context: &GPUContext,
	width: u32,
	height: u32,
	label: &str,
) -> TextureBufferInfo {
	// Texture -> buffer copies need to be aligned, so we have two counts
	let pixel_size = std::mem::size_of::<[u8; 4]>() as u32;
	let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
	let unpadded_bytes_per_row = pixel_size * width;
	let padding = (align - unpadded_bytes_per_row % align) % align;
	let padded_bytes_per_row = unpadded_bytes_per_row + padding;

	// Create a buffer to copy the texture
	let buffer_size = (padded_bytes_per_row * height) as wgpu::BufferAddress;
	let buffer_desc = wgpu::BufferDescriptor {
		label: Some(label),
		size: buffer_size,
		usage: wgpu::BufferUsages::COPY_DST
			| wgpu::BufferUsages::MAP_READ
			// TODO: map_write and copy_src are for input textures, check if it has any adverse effect
			| wgpu::BufferUsages::MAP_WRITE
			| wgpu::BufferUsages::COPY_SRC,
		mapped_at_creation: false,
	};

	TextureBufferInfo {
		buffer: context.device.create_buffer(&buffer_desc),
		padded_bytes_per_row,
		unpadded_bytes_per_row,
	}
}

pub fn create_texture_from_image_rgb(
	context: &GPUContext,
	from_image: &RgbImage,
	label: &'static str,
) -> TextureInfo {
	create_texture_from_image_rgba(context, &convert_rgb8_image_to_rgba8(from_image), label)
}

pub fn create_texture_from_image_rgba(
	context: &GPUContext,
	from_image: &RgbaImage,
	label: &'static str,
) -> TextureInfo {
	let (width, height) = from_image.dimensions();
	let to_texture_info = create_texture(context, width, height, label);
	let from_image_raw_data = from_image.as_raw();

	let transport_buffer =
		create_texture_buffer(context, width, height, format!("Texture transport to {}", label).as_str());

	put_gpu_texture_data_rgba(
		context,
		from_image_raw_data,
		&transport_buffer.buffer,
		transport_buffer.padded_bytes_per_row,
		transport_buffer.unpadded_bytes_per_row,
	);

	let mut encoder = create_encoder(context, "Texture from image");

	add_encoder_pass_copy_buffer_to_texture(
		&mut encoder,
		&transport_buffer.buffer,
		transport_buffer.padded_bytes_per_row,
		height,
		&to_texture_info.texture,
		to_texture_info.texture_descriptor.size,
	);

	context.queue.submit(Some(encoder.finish()));

	to_texture_info
}

pub fn create_image_from_texture(
	context: &GPUContext,
	from_texture: &wgpu::Texture,
	from_texture_size: wgpu::Extent3d,
	label: &str,
) -> RgbImage {
	let transport_buffer = create_texture_buffer(
		context,
		from_texture_size.width,
		from_texture_size.height,
		format!("Image transport to {}", label).as_str(),
	);

	let mut encoder = create_encoder(context, "Image from texture");

	add_encoder_pass_copy_texture_to_buffer(
		&mut encoder,
		from_texture,
		from_texture_size,
		&transport_buffer.buffer,
		transport_buffer.padded_bytes_per_row,
	);

	context.queue.submit(Some(encoder.finish()));

	let data = get_gpu_texture_data_rgba(
		context,
		&transport_buffer.buffer,
		transport_buffer.padded_bytes_per_row,
		transport_buffer.unpadded_bytes_per_row,
	);

	let image_rgba =
		convert_buffer_data_to_rgba_image(data, from_texture_size.width, from_texture_size.height);
	let image_rgb = convert_rgba8_image_to_rgb8(&image_rgba);

	image_rgb
}

pub fn copy_textures_to_textures(
	context: &GPUContext,
	from_textures: Vec<&wgpu::Texture>,
	from_textures_sizes: Vec<wgpu::Extent3d>,
	to_textures: Vec<&wgpu::Texture>,
) {
	assert_eq!(
		from_textures.len(),
		from_textures_sizes.len(),
		"number of origin textures does not match number of texture sizes"
	);
	assert_eq!(
		from_textures.len(),
		to_textures.len(),
		"number of origin textures does not match number of destination textures"
	);

	let mut encoder = create_encoder(context, "Texture to texture");

	for i in 0..from_textures.len() {
		add_encoder_pass_copy_texture_to_texture(
			&mut encoder,
			from_textures[i],
			from_textures_sizes[i],
			to_textures[i],
		);
	}

	context.queue.submit(Some(encoder.finish()));
}
