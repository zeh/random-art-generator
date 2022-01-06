use crate::generator::utils::gpu::buffer::create_storage_buffer;
use crate::generator::utils::gpu::context::GPUContext;
use crate::generator::utils::gpu::pipeline::create_compute_pipeline_with_layouts;
use crate::generator::utils::gpu::sampler::{create_sampler, SamplerInfo};

pub struct DiffShader {
	pub pipeline: wgpu::ComputePipeline,
	pub in_sampler: SamplerInfo,
}

impl DiffShader {
	pub fn new(context: &GPUContext) -> Self {
		let label = "Diff";
		let shader_source = include_str!("shader.wgsl");

		// Internal inputs
		let in_sampler = create_sampler(context);

		// Bind layouts
		let bind_group_layouts =
			[&context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				label: None,
				entries: &[
					wgpu::BindGroupLayoutEntry {
						binding: 0,
						visibility: wgpu::ShaderStages::COMPUTE,
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
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Storage {
								read_only: false,
							},
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
				],
			})];

		let pipeline =
			create_compute_pipeline_with_layouts(context, &bind_group_layouts, shader_source, label);

		DiffShader {
			pipeline,
			in_sampler,
		}
	}

	pub fn create_results_buffer(&self, context: &GPUContext, width: u32, height: u32) -> wgpu::Buffer {
		let num_entries_needed = f64::ceil((width * height) as f64 / Self::get_max_pixel_stride() as f64);
		let data = vec![0u32; num_entries_needed as usize * 3];
		create_storage_buffer(context, &data, "Diff results")
	}

	pub fn get_bind_groups(
		&self,
		context: &GPUContext,
		texture_a: &wgpu::TextureView,
		texture_b: &wgpu::TextureView,
		buffer_output: &wgpu::Buffer,
	) -> [wgpu::BindGroup; 1] {
		let bind_group_0 = self.create_bind_group_0(context, texture_a, texture_b, buffer_output);
		[bind_group_0]
	}

	fn create_bind_group_0(
		&self,
		context: &GPUContext,
		texture_a: &wgpu::TextureView,
		texture_b: &wgpu::TextureView,
		buffer_output: &wgpu::Buffer,
	) -> wgpu::BindGroup {
		let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: None,
			layout: &self.pipeline.get_bind_group_layout(0),
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::Sampler(&self.in_sampler.sampler),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::TextureView(texture_a),
				},
				wgpu::BindGroupEntry {
					binding: 2,
					resource: wgpu::BindingResource::TextureView(texture_b),
				},
				wgpu::BindGroupEntry {
					binding: 3,
					resource: buffer_output.as_entire_binding(),
				},
			],
		});

		bind_group
	}

	// See the shader code for info on this stride
	fn get_max_pixel_stride() -> u32 {
		16_777_216
	}
}
