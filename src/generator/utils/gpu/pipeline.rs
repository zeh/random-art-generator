use super::context::GPUContext;

pub fn create_compute_pipeline_with_layouts(
	context: &GPUContext,
	bind_group_layouts: &[&wgpu::BindGroupLayout],
	shader_source: &str,
	label: &str,
) -> wgpu::ComputePipeline {
	let device = &context.device;

	let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
		label: Some(format!("Shader module for {}", label).as_str()),
		source: wgpu::ShaderSource::Wgsl(shader_source.into()),
	});

	let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
		label: Some(format!("Compute pipeline layout module for {}", label).as_str()),
		bind_group_layouts,
		push_constant_ranges: &[],
	});

	let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
		label: Some(format!("Compute pipeline for {}", label).as_str()),
		layout: Some(&compute_pipeline_layout),
		module: &shader_module,
		entry_point: "cs_main",
	});

	compute_pipeline
}

// TODO: use?
#[allow(dead_code)]
pub fn create_compute_pipeline(
	context: &GPUContext,
	shader_source: &str,
	label: &str,
) -> wgpu::ComputePipeline {
	let device = &context.device;

	let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
		label: Some(format!("Compute shader module for {}", label).as_str()),
		source: wgpu::ShaderSource::Wgsl(shader_source.into()),
	});

	// Bind group layouts will be inferred by the shader module when the compute pipeline is created,
	// so we don't need bind group layout or a compute pipeline layout
	let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
		label: Some(format!("Compute pipeline for {}", label).as_str()),
		layout: None,
		module: &shader_module,
		entry_point: "cs_main",
	});

	compute_pipeline
}
