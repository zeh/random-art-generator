pub struct GPUContext {
	pub adapter: wgpu::Adapter,
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
}

impl GPUContext {
	pub fn new(is_verbose: bool, use_low_power: bool) -> Self {
		pollster::block_on(Self::create_context_async(is_verbose, use_low_power))
	}

	async fn create_context_async(is_verbose: bool, use_low_power: bool) -> GPUContext {
		// Much of this is based on:
		// https://github.com/gfx-rs/wgpu/blob/master/wgpu/examples/hello-compute/main.rs

		// Instantiates instance of WebGPU
		let instance = wgpu::Instance::new(wgpu::Backends::all());

		// Instantiates the general connection to the GPU
		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: if use_low_power {
					wgpu::PowerPreference::LowPower
				} else {
					wgpu::PowerPreference::HighPerformance
				},
				compatible_surface: None,
				..wgpu::RequestAdapterOptions::default()
			})
			.await
			.unwrap();

		let max_limits = adapter.limits();

		// Instantiates the feature specific connection to the GPU, defining some parameters
		let (device, queue) = {
			// Create a list of limits tro try, from more desired to least desired,
			// so we can use the highest texture dimension possible
			let mut limits_to_try = Vec::<wgpu::Limits>::new();

			// Built-in defaults
			limits_to_try.push(wgpu::Limits {
				max_texture_dimension_2d: max_limits.max_texture_dimension_2d,
				max_compute_workgroup_size_x: max_limits.max_compute_workgroup_size_x,
				max_compute_workgroup_size_y: max_limits.max_compute_workgroup_size_y,
				max_compute_workgroup_size_z: max_limits.max_compute_workgroup_size_z,
				max_compute_invocations_per_workgroup: max_limits.max_compute_invocations_per_workgroup,
				..wgpu::Limits::default()
			});
			limits_to_try.push(wgpu::Limits::downlevel_defaults());
			limits_to_try.push(wgpu::Limits::downlevel_webgl2_defaults());

			// Try all until something works
			let mut device_and_queue: Option<(wgpu::Device, wgpu::Queue)> = None;
			for limits in limits_to_try {
				let result = adapter
					.request_device(
						&wgpu::DeviceDescriptor {
							label: Some("Descriptor request with varying limits"),
							features: wgpu::Features::MAPPABLE_PRIMARY_BUFFERS,
							limits,
						},
						None,
					)
					.await;

				if result.is_ok() {
					device_and_queue = Some(result.unwrap());
					break;
				}
			}

			if device_and_queue.is_none() {
				panic!("Could not create proper device and queue for the adapter");
			}

			device_and_queue.unwrap()
		};

		let info = adapter.get_info();
		let limits = device.limits();

		if is_verbose {
			println!(
				"[GPU] GPU initialized; using adapter \"{}\" (type \"{:?}\") via \"{:?}\".",
				info.name, info.device_type, info.backend
			);
			println!(
				"[GPU] Max texture size is {}, max workgroup size is {}x{}x{} (with {} invocations).",
				limits.max_texture_dimension_2d,
				limits.max_compute_workgroup_size_x,
				limits.max_compute_workgroup_size_y,
				limits.max_compute_workgroup_size_z,
				limits.max_compute_invocations_per_workgroup
			);
		}

		GPUContext {
			adapter,
			queue,
			device,
		}
	}
}
