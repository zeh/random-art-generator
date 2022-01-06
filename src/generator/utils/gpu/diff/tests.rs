use image::{Rgb, RgbImage};

use crate::generator::utils::color::{LUMA_B, LUMA_G, LUMA_R};
use crate::generator::utils::gpu::context::GPUContext;
use crate::generator::utils::gpu::diff::calculate_diff_from_textures;
use crate::generator::utils::gpu::texture::create_texture_from_image_rgb;

use super::shader::DiffShader;

#[test]
fn test_diff_simple() {
	diff_images(8, 8);
}

#[test]
fn test_diff_small() {
	diff_images(66, 67);
}

#[test]
fn test_diff_rect() {
	diff_images(2, 88);
}

#[test]
fn test_diff_large() {
	diff_images(1024, 1024);
}

fn diff_images(width: u32, height: u32) {
	// Create needed instances
	let context = GPUContext::new(false, false);
	let diff_shader = DiffShader::new(&context);

	let white_img = create_texture_from_image_rgb(
		&context,
		&RgbImage::from_fn(width, height, |_x, _y| Rgb([255u8, 255u8, 255u8])),
		"Test diff image - white",
	);
	let black_img = create_texture_from_image_rgb(
		&context,
		&RgbImage::from_fn(width, height, |_x, _y| Rgb([0u8, 0u8, 0u8])),
		"Test diff image - black",
	);
	let half_black_img = create_texture_from_image_rgb(
		&context,
		&RgbImage::from_fn(width, height, |x, _y| {
			if x % 2 == 0 {
				Rgb([0u8, 0u8, 0u8])
			} else {
				Rgb([255u8, 255u8, 255u8])
			}
		}),
		"Test diff image - half-black",
	);
	let red_img = create_texture_from_image_rgb(
		&context,
		&RgbImage::from_fn(width, height, |_x, _y| Rgb([255u8, 0u8, 0u8])),
		"Test diff image - red",
	);
	let green_img = create_texture_from_image_rgb(
		&context,
		&RgbImage::from_fn(width, height, |_x, _y| Rgb([0u8, 255u8, 0u8])),
		"Test diff image - green",
	);
	let blue_img = create_texture_from_image_rgb(
		&context,
		&RgbImage::from_fn(width, height, |_x, _y| Rgb([0u8, 0u8, 255u8])),
		"Test diff image - blue",
	);

	assert_eq!(
		simpler_float(calculate_diff_from_textures(&context, &diff_shader, &white_img, &white_img)),
		simpler_float(0.0)
	);
	assert_eq!(
		simpler_float(calculate_diff_from_textures(&context, &diff_shader, &white_img, &black_img)),
		simpler_float(1.0)
	);
	assert_eq!(
		simpler_float(calculate_diff_from_textures(&context, &diff_shader, &white_img, &half_black_img)),
		simpler_float(0.5)
	);
	assert_eq!(
		simpler_float(calculate_diff_from_textures(&context, &diff_shader, &black_img, &half_black_img)),
		simpler_float(0.5)
	);

	// Luma-based differences
	assert_eq!(
		simpler_float(calculate_diff_from_textures(&context, &diff_shader, &white_img, &red_img)),
		simpler_float(LUMA_G + LUMA_B)
	);
	assert_eq!(
		simpler_float(calculate_diff_from_textures(&context, &diff_shader, &black_img, &red_img)),
		simpler_float(LUMA_R)
	);
	assert_eq!(
		simpler_float(calculate_diff_from_textures(&context, &diff_shader, &white_img, &green_img)),
		simpler_float(LUMA_R + LUMA_B)
	);
	assert_eq!(
		simpler_float(calculate_diff_from_textures(&context, &diff_shader, &black_img, &green_img)),
		simpler_float(LUMA_G)
	);
	assert_eq!(
		simpler_float(calculate_diff_from_textures(&context, &diff_shader, &white_img, &blue_img)),
		simpler_float(LUMA_R + LUMA_G)
	);
	assert_eq!(
		simpler_float(calculate_diff_from_textures(&context, &diff_shader, &black_img, &blue_img)),
		simpler_float(LUMA_B)
	);
}

fn simpler_float(value: f64) -> f64 {
	(value * 10000.0).round() / 10000.0
}
