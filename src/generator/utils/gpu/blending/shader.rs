use crate::generator::utils::gpu::buffer::create_uniform_buffer;
use crate::generator::utils::gpu::context::GPUContext;
use crate::generator::utils::gpu::pipeline::create_compute_pipeline_with_layouts;
use crate::generator::utils::gpu::sampler::{create_sampler, SamplerInfo};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniform {
	pub blend_type: u32,
	pub opacity: f32,
}

impl Uniform {
	pub fn new() -> Self {
		Uniform {
			blend_type: 0,
			opacity: 0.0,
		}
	}
}

pub struct BlendingShader {
	pub blend_type: u32,
	pub pipeline: wgpu::ComputePipeline,
	pub in_sampler: SamplerInfo,
}

impl BlendingShader {
	pub fn create_uniform() -> Uniform {
		Uniform::new()
	}

	pub fn new(context: &GPUContext, blend_type: u32) -> Self {
		let label = "Blending";
		let shader_source = include_str!("shader.wgsl");

		// Internal inputs
		let in_sampler = create_sampler(context);

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
				entries: &[
					wgpu::BindGroupLayoutEntry {
						binding: 0,
						visibility: wgpu::ShaderStages::COMPUTE,
						// ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
						ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 1,
						visibility: wgpu::ShaderStages::COMPUTE,
						ty: wgpu::BindingType::Texture {
							multisampled: false,
							view_dimension: wgpu::TextureViewDimension::D2,
							sample_type: wgpu::TextureSampleType::Float {
								filterable: true,
							},
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 2,
						visibility: wgpu::ShaderStages::COMPUTE,
						ty: wgpu::BindingType::Texture {
							multisampled: false,
							view_dimension: wgpu::TextureViewDimension::D2,
							sample_type: wgpu::TextureSampleType::Float {
								filterable: true,
							},
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 3,
						visibility: wgpu::ShaderStages::COMPUTE,
						ty: wgpu::BindingType::StorageTexture {
							access: wgpu::StorageTextureAccess::WriteOnly,
							format: wgpu::TextureFormat::Rgba8Unorm,
							view_dimension: wgpu::TextureViewDimension::D2,
						},
						count: None,
					},
				],
			}),
		];

		let pipeline =
			create_compute_pipeline_with_layouts(context, &bind_group_layouts, shader_source, label);

		// Alternative, automatic
		// let pipeline = create_compute_pipeline(&device, shader_source);
		// let bind_group_layout = pipeline.get_bind_group_layout(0);

		BlendingShader {
			blend_type,
			pipeline,
			in_sampler,
		}
	}

	pub fn get_bind_groups(
		&self,
		context: &GPUContext,
		opacity: f32,
		texture_bottom: &wgpu::TextureView,
		texture_top: &wgpu::TextureView,
		texture_output: &wgpu::TextureView,
	) -> [wgpu::BindGroup; 2] {
		let bind_group_0 = self.create_bind_group_0(context, opacity);
		let bind_group_1 = self.create_bind_group_1(context, texture_bottom, texture_top, texture_output);
		[bind_group_0, bind_group_1]
	}

	fn create_bind_group_0(&self, context: &GPUContext, opacity: f32) -> wgpu::BindGroup {
		let mut uniform = Self::create_uniform();
		uniform.blend_type = self.blend_type;
		uniform.opacity = opacity;
		let uniform_buffer = create_uniform_buffer(context, uniform, "Blending");

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
		texture_bottom: &wgpu::TextureView,
		texture_top: &wgpu::TextureView,
		texture_output: &wgpu::TextureView,
	) -> wgpu::BindGroup {
		let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: None,
			layout: &self.pipeline.get_bind_group_layout(1),
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::Sampler(&self.in_sampler.sampler),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::TextureView(texture_bottom),
				},
				wgpu::BindGroupEntry {
					binding: 2,
					resource: wgpu::BindingResource::TextureView(texture_top),
				},
				wgpu::BindGroupEntry {
					binding: 3,
					resource: wgpu::BindingResource::TextureView(texture_output),
				},
			],
		});

		bind_group
	}
}
