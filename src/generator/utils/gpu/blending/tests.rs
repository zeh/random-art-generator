use std::io::Cursor;

use image::io::Reader as ImageReader;
use image::{RgbImage, RgbaImage};

use crate::generator::utils::color::BlendingMode;
use crate::generator::utils::gpu::blending::blend_textures_to_image;
use crate::generator::utils::gpu::context::GPUContext;
use crate::generator::utils::gpu::texture::{create_texture_from_image_rgba, TextureInfo};

use super::shader::BlendingShader;

#[test]
fn test_blend() {
	// Create needed instances
	let context = GPUContext::new(false, false);

	// Bottom is just a gradient
	let bottom_bytes = include_bytes!("test_bottom.png");
	let bt = create_texture_from_image_rgba(
		&context,
		&get_rgba_image_from_bytes(bottom_bytes),
		"Test blend image - bottom",
	);

	// Top is a combination of gradient/opacity, repeated brightness_segments times
	let top_bytes = include_bytes!("test_top.png");
	let tp = create_texture_from_image_rgba(
		&context,
		&get_rgba_image_from_bytes(top_bytes),
		"Test blend image - bottom",
	);

	// Test all modes
	test_blend_img(&context, &bt, &tp, BlendingMode::Normal, include_bytes!("test_blend_normal.png"));
	test_blend_img(&context, &bt, &tp, BlendingMode::Multiply, include_bytes!("test_blend_multiply.png"));
	test_blend_img(&context, &bt, &tp, BlendingMode::Screen, include_bytes!("test_blend_screen.png"));
	test_blend_img(&context, &bt, &tp, BlendingMode::Overlay, include_bytes!("test_blend_overlay.png"));
	test_blend_img(&context, &bt, &tp, BlendingMode::Darken, include_bytes!("test_blend_darken.png"));
	test_blend_img(&context, &bt, &tp, BlendingMode::Lighten, include_bytes!("test_blend_lighten.png"));
	test_blend_img(
		&context,
		&bt,
		&tp,
		BlendingMode::ColorDodge,
		include_bytes!("test_blend_color_dodge.png"),
	);
	test_blend_img(&context, &bt, &tp, BlendingMode::ColorBurn, include_bytes!("test_blend_color_burn.png"));
	test_blend_img(&context, &bt, &tp, BlendingMode::HardLight, include_bytes!("test_blend_hard_light.png"));
	test_blend_img(&context, &bt, &tp, BlendingMode::SoftLight, include_bytes!("test_blend_soft_light.png"));
	test_blend_img(&context, &bt, &tp, BlendingMode::Difference, include_bytes!("test_blend_difference.png"));
	test_blend_img(&context, &bt, &tp, BlendingMode::Exclusion, include_bytes!("test_blend_exclusion.png"));
}

fn get_rgb_image_from_bytes(bytes: &[u8]) -> RgbImage {
	ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap().to_rgb8()
}

fn get_rgba_image_from_bytes(bytes: &[u8]) -> RgbaImage {
	ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap().to_rgba8()
}

fn test_blend_img(
	context: &GPUContext,
	bottom: &TextureInfo,
	top: &TextureInfo,
	blending_mode: BlendingMode,
	bytes: &[u8],
) {
	println!("Testing {}", blending_mode);
	let shader = BlendingShader::new(context, blending_mode as u32);
	let result = blend_textures_to_image(&context, &shader, 1.0, bottom, top);
	let expected = get_rgb_image_from_bytes(&bytes);
	assert_eq!(result.as_raw(), expected.as_raw());
}
