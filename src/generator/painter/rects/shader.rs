use crate::generator::utils::gpu::buffer::create_uniform_buffer;
use crate::generator::utils::gpu::context::GPUContext;
use crate::generator::utils::gpu::pipeline::create_compute_pipeline_with_layouts;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniform {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
	pub rotation: f32,
	pub corner_radius: f32,
	pub color_r: f32,
	pub color_g: f32,
	pub color_b: f32,
	pub anti_alias: u32,
}

impl Uniform {
	pub fn new() -> Self {
		Uniform {
			x: 0.0,
			y: 0.0,
			width: 0.0,
			height: 0.0,
			rotation: 0.0,
			corner_radius: 0.0,
			color_r: 0.0,
			color_g: 0.0,
			color_b: 0.0,
			anti_alias: 1,
		}
	}
}

pub struct Shader {
	pub pipeline: wgpu::ComputePipeline,
}

impl Shader {
	pub fn create_uniform() -> Uniform {
		Uniform::new()
	}

	pub fn new(context: &GPUContext) -> Self {
		let label = "Rects";
		let shader_source = include_str!("shader.wgsl");

		// Bind layouts
		let bind_group_layouts = [
			&context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				label: None,
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::COMPUTE,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
			}),
			&context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				label: None,
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::COMPUTE,
					ty: wgpu::BindingType::StorageTexture {
						access: wgpu::StorageTextureAccess::WriteOnly,
						format: wgpu::TextureFormat::Rgba8Unorm,
						view_dimension: wgpu::TextureViewDimension::D2,
					},
					count: None,
				}],
			}),
		];

		let pipeline =
			create_compute_pipeline_with_layouts(context, &bind_group_layouts, shader_source, label);

		// Alternative, automatic
		// let pipeline = create_compute_pipeline(&device, include_str!("shader.wgsl"));
		// let bind_group_layout = pipeline.get_bind_group_layout(0);

		Shader {
			pipeline,
		}
	}

	pub fn get_bind_groups(
		&self,
		context: &GPUContext,
		x: f64,
		y: f64,
		width: f64,
		height: f64,
		rotation: f64,
		corner_radius: f64,
		color: [u8; 3],
		anti_alias: bool,
		texture_output: &wgpu::TextureView,
	) -> [wgpu::BindGroup; 2] {
		let bind_group_0 = self.create_bind_group_0(
			context,
			x,
			y,
			width,
			height,
			rotation,
			corner_radius,
			color,
			anti_alias,
		);
		let bind_group_1 = self.create_bind_group_1(context, texture_output);
		[bind_group_0, bind_group_1]
	}

	fn create_bind_group_0(
		&self,
		context: &GPUContext,
		x: f64,
		y: f64,
		width: f64,
		height: f64,
		rotation: f64,
		corner_radius: f64,
		color: [u8; 3],
		anti_alias: bool,
	) -> wgpu::BindGroup {
		let mut uniform = Self::create_uniform();
		uniform.x = x as f32;
		uniform.y = y as f32;
		uniform.width = width as f32;
		uniform.height = height as f32;
		uniform.rotation = rotation as f32;
		uniform.corner_radius = corner_radius as f32;
		uniform.color_r = color[0] as f32 / 255.0;
		uniform.color_g = color[1] as f32 / 255.0;
		uniform.color_b = color[2] as f32 / 255.0;
		uniform.anti_alias = if anti_alias {
			1
		} else {
			0
		};
		let uniform_buffer = create_uniform_buffer(context, uniform, "Rects");

		let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: None,
			layout: &self.pipeline.get_bind_group_layout(0),
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: uniform_buffer.as_entire_binding(),
			}],
		});

		// TODO: this might need to be written later, this is ahead of time
		context.queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));

		bind_group
	}

	fn create_bind_group_1(
		&self,
		context: &GPUContext,
		texture_output: &wgpu::TextureView,
	) -> wgpu::BindGroup {
		let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: None,
			layout: &self.pipeline.get_bind_group_layout(1),
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: wgpu::BindingResource::TextureView(texture_output),
			}],
		});

		bind_group
	}
}
