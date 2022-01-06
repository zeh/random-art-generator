use super::context::GPUContext;

pub struct SamplerInfo {
	pub sampler: wgpu::Sampler,
}

pub fn create_sampler(context: &GPUContext) -> SamplerInfo {
	let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
		address_mode_u: wgpu::AddressMode::ClampToEdge,
		address_mode_v: wgpu::AddressMode::ClampToEdge,
		address_mode_w: wgpu::AddressMode::ClampToEdge,
		//mag_filter: wgpu::FilterMode::Linear,
		mag_filter: wgpu::FilterMode::Nearest,
		min_filter: wgpu::FilterMode::Nearest,
		mipmap_filter: wgpu::FilterMode::Nearest,
		//lod_min_clamp: -100.0,
		//lod_max_clamp: 100.0,
		//compare: Some(wgpu::CompareFunction::LessEqual),
		..Default::default()
	});

	SamplerInfo {
		sampler,
	}
}
