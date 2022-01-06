use std::convert::TryInto;

use image::{imageops, GenericImageView, ImageBuffer, Pixel, RgbImage, RgbaImage};

use crate::generator::utils::pixel;

#[cfg(test)]
use image::Rgb;

pub fn color_transform(image: &RgbImage, matrix: [f64; 12]) -> RgbImage {
	let mut transformed_image = image.clone();
	for (_x, _y, pixel) in transformed_image.enumerate_pixels_mut() {
		*pixel = image::Rgb(pixel::color_matrix(pixel.channels(), matrix));
	}
	transformed_image
}

pub fn scale<I: GenericImageView>(
	image: &I,
	scale: f64,
) -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
where
	I::Pixel: 'static,
	<I::Pixel as Pixel>::Subpixel: 'static,
{
	let width = (image.dimensions().0 as f64 * scale).round() as u32;
	let height = (image.dimensions().1 as f64 * scale).round() as u32;
	resize(image, width, height)
}

pub fn resize<I: GenericImageView>(
	image: &I,
	width: u32,
	height: u32,
) -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
where
	I::Pixel: 'static,
	<I::Pixel as Pixel>::Subpixel: 'static,
{
	imageops::resize(image, width, height, imageops::FilterType::CatmullRom)
}

pub fn get_pixel_interpolated(image: &RgbImage, x: f64, y: f64) -> [u8; 3] {
	// Quick path if in a round pixel
	let width: f64 = image.width() as f64;
	let height: f64 = image.height() as f64;
	let xx = f64::max(0.0f64, f64::min(width - 1.0, x));
	let yy = f64::max(0.0f64, f64::min(height - 1.0, y));
	let xf = xx.fract();
	let yf = yy.fract();
	if xf == 0f64 && yf == 0f64 {
		return image
			.get_pixel(xx as u32, yy as u32)
			.channels()
			.to_owned()
			.try_into()
			.expect("converting pixels to array");
	}

	// Otherwise, do bilinear interpolation
	let x1 = xx.floor();
	let x2 = xx.ceil();
	let y1 = yy.floor();
	let y2 = yy.ceil();
	let color_tl = image.get_pixel(x1 as u32, y1 as u32).channels();
	let color_tr = image.get_pixel(x2 as u32, y1 as u32).channels();
	let color_bl = image.get_pixel(x1 as u32, y2 as u32).channels();
	let color_br = image.get_pixel(x2 as u32, y2 as u32).channels();
	let color_t = pixel::blend_linear(color_tl, color_tr, xf);
	let color_b = pixel::blend_linear(color_bl, color_br, xf);
	return pixel::blend_linear(&color_t, &color_b, yf);
}

pub fn convert_rgba8_image_to_rgb8(input: &RgbaImage) -> RgbImage {
	let (width, height) = input.dimensions();
	let input: &Vec<u8> = input.as_raw();
	let mut output_data = vec![0u8; (width * height * 3) as usize];

	let mut i = 0;
	for chunk in input.chunks_exact(4) {
		output_data[i..i + 3].copy_from_slice(&chunk[0..3]);
		i += 3;
	}

	ImageBuffer::from_raw(width as u32, height as u32, output_data).unwrap()
}

pub fn convert_rgb8_image_to_rgba8(input: &RgbImage) -> RgbaImage {
	let (width, height) = input.dimensions();
	let input: &Vec<u8> = input.as_raw();
	let mut output_data = vec![0u8; (width * height * 4) as usize];

	let mut i = 0;
	for chunk in input.chunks_exact(3) {
		output_data[i..i + 3].copy_from_slice(&chunk[0..3]);
		output_data[i + 3] = 255u8;
		i += 4;
	}

	ImageBuffer::from_raw(width as u32, height as u32, output_data).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_color_transform() {
		let white_img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([255u8, 255u8, 255u8]));
		let black_img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([0u8, 0u8, 0u8]));
		let r_img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([255u8, 0u8, 0u8]));
		let g_img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([0u8, 255u8, 0u8]));
		let b_img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([0u8, 0u8, 255u8]));

		let identity_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_transform(&white_img, identity_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, identity_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, identity_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, identity_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 0u8]));
		assert_eq!(color_transform(&b_img, identity_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 255u8]));

		let red_filter_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
		assert_eq!(color_transform(&white_img, red_filter_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 0u8]));
		assert_eq!(color_transform(&black_img, red_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, red_filter_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, red_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&b_img, red_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));

		// Further tests are performed in pixel::test_color_matrix()
	}

	#[test]
	fn test_scale() {
		let img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([255u8, 255u8, 255u8]));
		assert_eq!(scale(img, 2.0).dimensions(), (16, 16));
		assert_eq!(scale(img, 0.5).dimensions(), (4, 4));
		assert_eq!(scale(img, 1.01).dimensions(), (8, 8));
	}

	#[test]
	fn test_get_pixel_interpolated() {
		let img = &RgbImage::from_raw(
			3,
			3,
			vec![
				0u8, 0u8, 0u8, 255u8, 255u8, 255u8, 255u8, 0u8, 0u8, 255u8, 0u8, 0u8, 255u8, 255u8, 255u8,
				255u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 255u8, 255u8, 255u8, 0u8, 128u8,
			],
		)
		.unwrap();

		// Fast path
		assert_eq!(get_pixel_interpolated(img, 0f64, 0f64), [0u8, 0u8, 0u8]);
		assert_eq!(get_pixel_interpolated(img, 1f64, 0f64), [255u8, 255u8, 255u8]);
		assert_eq!(get_pixel_interpolated(img, 1f64, 2f64), [0u8, 255u8, 255u8]);

		assert_eq!(get_pixel_interpolated(img, 8f64, 2f64), [255u8, 0u8, 128u8]);
		assert_eq!(get_pixel_interpolated(img, 1f64, 8f64), [0u8, 255u8, 255u8]);
		assert_eq!(get_pixel_interpolated(img, 8f64, 8f64), [255u8, 0u8, 128u8]);

		// Linearly interpolated
		assert_eq!(get_pixel_interpolated(img, 0.25f64, 0f64), [64u8, 64u8, 64u8]);
		assert_eq!(get_pixel_interpolated(img, 0.5f64, 0f64), [128u8, 128u8, 128u8]);
		assert_eq!(get_pixel_interpolated(img, 0f64, 0.25f64), [64u8, 0u8, 0u8]);
		assert_eq!(get_pixel_interpolated(img, 0f64, 0.5f64), [128u8, 0u8, 0u8]);
		assert_eq!(get_pixel_interpolated(img, 1.5f64, 1f64), [255u8, 128u8, 128u8]);
		assert_eq!(get_pixel_interpolated(img, 2f64, 1.5f64), [255u8, 0u8, 64u8]);

		assert_eq!(get_pixel_interpolated(img, 8f64, 1.5f64), [255u8, 0u8, 64u8]);
		assert_eq!(get_pixel_interpolated(img, 1.5f64, 8f64), [128u8, 128u8, 192u8]);

		// Bilinearly interpolated
		assert_eq!(get_pixel_interpolated(img, 0.5f64, 0.5f64), [192u8, 128u8, 128u8]);
		assert_eq!(get_pixel_interpolated(img, 1.5f64, 1.5f64), [192u8, 128u8, 160u8]);
		assert_eq!(get_pixel_interpolated(img, 0.33f64, 1.78f64), [56u8, 84u8, 84u8]);

		assert_eq!(get_pixel_interpolated(img, 9.1f64, 8.2f64), [255u8, 0u8, 128u8]);
	}

	#[test]
	fn test_convert_rgb8_and_rgba8() {
		let img_rgb8 = &RgbImage::from_raw(
			3,
			4,
			vec![
				0, 0, 0, 255, 255, 255, 255, 0, 0, 255, 0, 0, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255,
				255, 0, 128, 1, 2, 3, 100, 200, 201, 0, 0, 2,
			],
		)
		.unwrap();

		let img_rgba8 = &RgbaImage::from_raw(
			3,
			4,
			vec![
				0, 0, 0, 255, 255, 255, 255, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 255, 255, 255, 255, 0,
				0, 255, 0, 0, 0, 255, 0, 255, 255, 255, 255, 0, 128, 255, 1, 2, 3, 255, 100, 200, 201, 255,
				0, 0, 2, 255,
			],
		)
		.unwrap();

		assert_eq!(convert_rgb8_image_to_rgba8(img_rgb8).as_raw(), img_rgba8.as_raw());
		assert_eq!(convert_rgb8_image_to_rgba8(img_rgb8).dimensions(), img_rgba8.dimensions());

		assert_eq!(convert_rgba8_image_to_rgb8(img_rgba8).as_raw(), img_rgb8.as_raw());
		assert_eq!(convert_rgba8_image_to_rgb8(img_rgba8).dimensions(), img_rgb8.dimensions());
	}
}
