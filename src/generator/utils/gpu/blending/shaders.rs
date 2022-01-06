use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::generator::utils::color::BlendingMode;
use crate::generator::utils::gpu::context::GPUContext;

use super::shader::BlendingShader;

pub struct BlendingShaders {
	shaders: HashMap<BlendingMode, BlendingShader>,
}

impl BlendingShaders {
	pub fn new(context: &GPUContext) -> Self {
		let mut shaders = HashMap::<BlendingMode, BlendingShader>::new();
		BlendingMode::iter().for_each(|blending_mode| {
			let shader = BlendingShader::new(context, blending_mode as u32);
			shaders.insert(blending_mode, shader);
		});

		BlendingShaders {
			shaders,
		}
	}

	pub fn get_from_blending_mode(&self, blending_mode: &BlendingMode) -> &BlendingShader {
		self.shaders.get(blending_mode).expect("creating shader for blending mode")
	}
}
