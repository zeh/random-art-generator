use image::{ImageBuffer, Rgb, RgbImage, Rgba, RgbaImage};
use wgpu::util::DeviceExt;

use super::context::GPUContext;

pub fn create_uniform_buffer<A: bytemuck::Pod>(
	context: &GPUContext,
	uniform: A,
	label: &'static str,
) -> wgpu::Buffer {
	let label = format!("Uniform: {}", label);
	let uniform_cast = &[uniform];
	let buffer_init_descriptor = &wgpu::util::BufferInitDescriptor {
		label: Some(label.as_str()),
		contents: bytemuck::cast_slice(uniform_cast),
		usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
	};

	context.device.create_buffer_init(buffer_init_descriptor)
}

pub fn create_storage_buffer<A: bytemuck::Pod>(
	context: &GPUContext,
	contents: &Vec<A>,
	label: &'static str,
) -> wgpu::Buffer {
	let label = format!("Storage: {}", label);
	let buffer_init_descriptor = &wgpu::util::BufferInitDescriptor {
		label: Some(label.as_str()),
		contents: bytemuck::cast_slice(contents),
		usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ,
	};

	context.device.create_buffer_init(buffer_init_descriptor)
}

pub fn get_gpu_texture_data_rgba(
	context: &GPUContext,
	buffer: &wgpu::Buffer,
	padded_bytes_per_row: u32,
	unpadded_bytes_per_row: u32,
) -> Vec<u8> {
	pollster::block_on(get_gpu_texture_data_rgba_async(
		context,
		buffer,
		padded_bytes_per_row,
		unpadded_bytes_per_row,
	))
}

async fn get_gpu_texture_data_rgba_async(
	context: &GPUContext,
	buffer: &wgpu::Buffer,
	padded_bytes_per_row: u32,
	unpadded_bytes_per_row: u32,
) -> Vec<u8> {
	let buffer_slice = buffer.slice(..);
	let buffer_map_request = buffer_slice.map_async(wgpu::MapMode::Read);

	// Wait for the GPU to finish
	context.device.poll(wgpu::Maintain::Wait);
	buffer_map_request.await.expect("mapping GPU buffer data into CPU memory");
	let padded_data = buffer_slice.get_mapped_range();
	let data = padded_data
		.chunks_exact(padded_bytes_per_row as _)
		.map(|chunk| &chunk[..unpadded_bytes_per_row as _])
		.flatten()
		.map(|x| *x)
		.collect::<Vec<_>>();

	drop(padded_data);
	buffer.unmap();

	data
}

pub fn get_gpu_buffer_data(context: &GPUContext, buffer: &wgpu::Buffer) -> Vec<u8> {
	pollster::block_on(get_gpu_buffer_data_async(context, buffer))
}

async fn get_gpu_buffer_data_async(context: &GPUContext, buffer: &wgpu::Buffer) -> Vec<u8> {
	let buffer_slice = buffer.slice(..);
	let buffer_map_request = buffer_slice.map_async(wgpu::MapMode::Read);

	// Wait for the GPU to finish
	context.device.poll(wgpu::Maintain::Wait);
	buffer_map_request.await.expect("mapping GPU buffer data into CPU memory");
	let padded_data = buffer_slice.get_mapped_range();
	let data = padded_data.into_iter().map(|x| *x).collect::<Vec<_>>();

	drop(padded_data);
	buffer.unmap();

	data
}

pub fn put_gpu_texture_data_rgba(
	context: &GPUContext,
	from_data: &Vec<u8>,
	to_buffer: &wgpu::Buffer,
	padded_bytes_per_row: u32,
	unpadded_bytes_per_row: u32,
) {
	pollster::block_on(put_gpu_texture_data_rgba_async(
		context,
		from_data,
		to_buffer,
		padded_bytes_per_row,
		unpadded_bytes_per_row,
	))
}

async fn put_gpu_texture_data_rgba_async(
	context: &GPUContext,
	from_data: &Vec<u8>,
	to_buffer: &wgpu::Buffer,
	padded_bytes_per_row: u32,
	unpadded_bytes_per_row: u32,
) {
	let to_buffer_slice = to_buffer.slice(..);
	let to_buffer_map_request = to_buffer_slice.map_async(wgpu::MapMode::Write);

	let row_padding = vec![0u8; padded_bytes_per_row as usize - unpadded_bytes_per_row as usize];
	let row_padding_slice = &row_padding[..];

	// Wait for the GPU to finish
	context.device.poll(wgpu::Maintain::Wait);
	to_buffer_map_request.await.expect("mapping GPU buffer data from CPU memory");

	to_buffer_slice.get_mapped_range_mut().copy_from_slice(
		&from_data
			.chunks_exact(unpadded_bytes_per_row as _)
			.map(|chunk| [&chunk[..], row_padding_slice].concat())
			.flatten()
			.collect::<Vec<_>>(),
	);

	drop(from_data);
	to_buffer.unmap();
}

pub fn convert_buffer_data_to_rgba_image(data: Vec<u8>, width: u32, height: u32) -> RgbaImage {
	ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, data).unwrap()
}

// TODO: use?
#[allow(dead_code)]
pub fn convert_buffer_data_to_rgb_image(data: Vec<u8>, width: u32, height: u32) -> RgbImage {
	ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, data).unwrap()
}
