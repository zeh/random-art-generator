use std::convert::TryInto;

use image::{imageops, GrayImage, Pixel, RgbImage};

#[cfg(test)]
use image::Rgb;

const LUMA_R: f64 = 0.2126;
const LUMA_G: f64 = 0.7152;
const LUMA_B: f64 = 0.0722;

#[inline(always)]
pub fn blend_pixel(bottom: &[u8], top: &[u8], alpha: f64) -> [u8; 3] {
	// Return early if no need to blend
	if alpha == 1.0f64 {
		return [top[0], top[1], top[2]];
	} else if alpha == 0.0f64 {
		return [bottom[0], bottom[1], bottom[2]];
	}

	// Blend pixels
	let alpha_n: f64 = 1.0f64 - alpha;
	let nr: u8 = (top[0] as f64 * alpha + bottom[0] as f64 * alpha_n).round() as u8;
	let ng: u8 = (top[1] as f64 * alpha + bottom[1] as f64 * alpha_n).round() as u8;
	let nb: u8 = (top[2] as f64 * alpha + bottom[2] as f64 * alpha_n).round() as u8;
	[nr, ng, nb]
}

pub fn diff(a: &RgbImage, b: &RgbImage) -> f64 {
	let w = a.dimensions().0;
	let h = a.dimensions().1;
	let num_pixels = w * h;

	let mut diff_sum_r: i32 = 0;
	let mut diff_sum_g: i32 = 0;
	let mut diff_sum_b: i32 = 0;

	let samples_a = a.as_flat_samples().samples;
	let samples_b = b.as_flat_samples().samples;

	let skip_step = 1;

	for (p_a, p_b) in samples_a.chunks_exact(3).zip(samples_b.chunks_exact(3)).step_by(skip_step) {
		diff_sum_r += (p_a[0] as i32 - p_b[0] as i32).abs();
		diff_sum_g += (p_a[1] as i32 - p_b[1] as i32).abs();
		diff_sum_b += (p_a[2] as i32 - p_b[2] as i32).abs();
	}

	let lr = LUMA_R / 255.0;
	let lg = LUMA_G / 255.0;
	let lb = LUMA_B / 255.0;
	let diff_sum = diff_sum_r as f64 * lr + diff_sum_g as f64 * lg + diff_sum_b as f64 * lb;

	diff_sum / (num_pixels as f64 / skip_step as f64)
}

pub fn color_transform(image: &RgbImage, matrix: [f64; 12]) -> RgbImage {
	let mut transformed_image = image.clone();
	for (_x, _y, pixel) in transformed_image.enumerate_pixels_mut() {
		let channels = pixel.channels();
		let o_r = channels[0] as f64;
		let o_g = channels[1] as f64;
		let o_b = channels[2] as f64;
		let n_r = ((o_r * matrix[0] + o_g * matrix[1] + o_b * matrix[2] + matrix[3]).round())
			.max(0.0)
			.min(255.0) as u8;
		let n_g = ((o_r * matrix[4] + o_g * matrix[5] + o_b * matrix[6] + matrix[7]).round())
			.max(0.0)
			.min(255.0) as u8;
		let n_b = ((o_r * matrix[8] + o_g * matrix[9] + o_b * matrix[10] + matrix[11]).round())
			.max(0.0)
			.min(255.0) as u8;
		*pixel = image::Rgb([n_r, n_g, n_b]);
	}
	transformed_image
}

pub fn scale_image(image: &RgbImage, scale: f64) -> RgbImage {
	let width = (image.dimensions().0 as f64 * scale).round() as u32;
	let height = (image.dimensions().1 as f64 * scale).round() as u32;
	resize_image(image, width, height)
}

pub fn resize_image(image: &RgbImage, width: u32, height: u32) -> RgbImage {
	imageops::resize(image, width, height, imageops::FilterType::CatmullRom)
}

pub fn grayscale_image(image: &RgbImage) -> GrayImage {
	imageops::grayscale(image)
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
	let color_t = blend_pixel(color_tl, color_tr, xf);
	let color_b = blend_pixel(color_bl, color_br, xf);
	return blend_pixel(&color_t, &color_b, yf);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_blend_pixel() {
		assert_eq!(blend_pixel(&[0, 0, 0], &[255, 128, 0], 0.0), [0, 0, 0]);
		assert_eq!(blend_pixel(&[0, 0, 0], &[255, 128, 0], 0.1), [26, 13, 0]);
		assert_eq!(blend_pixel(&[0, 0, 0], &[255, 128, 0], 0.5), [128, 64, 0]);
		assert_eq!(blend_pixel(&[0, 0, 0], &[255, 128, 0], 1.0), [255, 128, 0]);
		assert_eq!(blend_pixel(&[128, 128, 128], &[255, 128, 0], 0.0), [128, 128, 128]);
		assert_eq!(blend_pixel(&[128, 128, 128], &[255, 128, 0], 0.1), [141, 128, 115]);
		assert_eq!(blend_pixel(&[128, 128, 128], &[255, 128, 0], 0.5), [192, 128, 64]);
		assert_eq!(blend_pixel(&[128, 128, 128], &[255, 128, 0], 1.0), [255, 128, 0]);
		assert_eq!(blend_pixel(&[255, 255, 255], &[255, 128, 0], 0.0), [255, 255, 255]);
		assert_eq!(blend_pixel(&[255, 255, 255], &[255, 128, 0], 0.1), [255, 242, 230]);
		assert_eq!(blend_pixel(&[255, 255, 255], &[255, 128, 0], 0.5), [255, 192, 128]);
		assert_eq!(blend_pixel(&[255, 255, 255], &[255, 128, 0], 1.0), [255, 128, 0]);
		assert_eq!(blend_pixel(&[0, 128, 255], &[0, 10, 20], 0.0), [0, 128, 255]);
		assert_eq!(blend_pixel(&[0, 128, 255], &[0, 10, 20], 0.1), [0, 116, 232]);
		assert_eq!(blend_pixel(&[0, 128, 255], &[0, 10, 20], 0.5), [0, 69, 138]);
		assert_eq!(blend_pixel(&[0, 128, 255], &[0, 10, 20], 1.0), [0, 10, 20]);
	}

	#[test]
	fn test_diff() {
		let white_img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([255u8, 255u8, 255u8]));
		let black_img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([0u8, 0u8, 0u8]));
		let half_black_img = &RgbImage::from_fn(8, 8, |x, _y| {
			if x % 2 == 0 {
				Rgb([0u8, 0u8, 0u8])
			} else {
				Rgb([255u8, 255u8, 255u8])
			}
		});
		let red_img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([255u8, 0u8, 0u8]));
		let green_img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([0u8, 255u8, 0u8]));
		let blue_img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([0u8, 0u8, 255u8]));

		assert_eq!(diff(&white_img, &white_img), 0.0);
		assert_eq!(diff(&white_img, &black_img), 1.0);
		assert_eq!(diff(&white_img, &half_black_img), 0.5);
		assert_eq!(diff(&black_img, &half_black_img), 0.5);

		// Luma-based differences
		// TODO: this might change later once luma is a parameter
		assert_eq!(diff(&white_img, &red_img), LUMA_G + LUMA_B);
		assert_eq!(diff(&black_img, &red_img), LUMA_R);
		assert_eq!(diff(&white_img, &green_img), LUMA_R + LUMA_B);
		assert_eq!(diff(&black_img, &green_img), LUMA_G);
		assert_eq!(diff(&white_img, &blue_img), LUMA_R + LUMA_G);
		assert_eq!(diff(&black_img, &blue_img), LUMA_B);
	}

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

		let green_filter_mtx = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
		assert_eq!(color_transform(&white_img, green_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 0u8]));
		assert_eq!(color_transform(&black_img, green_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, green_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, green_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 0u8]));
		assert_eq!(color_transform(&b_img, green_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));

		let blue_filter_mtx = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_transform(&white_img, blue_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 255u8]));
		assert_eq!(color_transform(&black_img, blue_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, blue_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, blue_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&b_img, blue_filter_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 255u8]));

		let red_fill_mtx = [1.0, 0.0, 0.0, 255.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_transform(&white_img, red_fill_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, red_fill_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, red_fill_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, red_fill_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 0u8]));
		assert_eq!(color_transform(&b_img, red_fill_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 255u8]));

		let green_fill_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 255.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_transform(&white_img, green_fill_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, green_fill_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 0u8]));
		assert_eq!(color_transform(&r_img, green_fill_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 0u8]));
		assert_eq!(color_transform(&g_img, green_fill_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 0u8]));
		assert_eq!(color_transform(&b_img, green_fill_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 255u8]));

		let blue_fill_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 255.0];
		assert_eq!(color_transform(&white_img, blue_fill_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, blue_fill_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 255u8]));
		assert_eq!(color_transform(&r_img, blue_fill_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 255u8]));
		assert_eq!(color_transform(&g_img, blue_fill_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 255u8]));
		assert_eq!(color_transform(&b_img, blue_fill_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 255u8]));

		let red_drain_mtx = [1.0, 0.0, 0.0, -255.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_transform(&white_img, red_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, red_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, red_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, red_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 0u8]));
		assert_eq!(color_transform(&b_img, red_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 255u8]));

		let green_drain_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -255.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_transform(&white_img, green_drain_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 255u8]));
		assert_eq!(color_transform(&black_img, green_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, green_drain_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, green_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&b_img, green_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 255u8]));

		let blue_drain_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -255.0];
		assert_eq!(color_transform(&white_img, blue_drain_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 0u8]));
		assert_eq!(color_transform(&black_img, blue_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, blue_drain_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, blue_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 0u8]));
		assert_eq!(color_transform(&b_img, blue_drain_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));

		let r2g_mtx = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_transform(&white_img, r2g_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, r2g_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, r2g_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 0u8]));
		assert_eq!(color_transform(&g_img, r2g_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&b_img, r2g_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 255u8]));

		let r2b_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];
		assert_eq!(color_transform(&white_img, r2b_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, r2b_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, r2b_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 255u8]));
		assert_eq!(color_transform(&g_img, r2b_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 0u8]));
		assert_eq!(color_transform(&b_img, r2b_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));

		let g2r_mtx = [0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_transform(&white_img, g2r_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, g2r_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, g2r_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, g2r_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 0u8]));
		assert_eq!(color_transform(&b_img, g2r_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 255u8]));

		let g2b_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
		assert_eq!(color_transform(&white_img, g2b_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, g2b_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, g2b_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, g2b_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 255u8]));
		assert_eq!(color_transform(&b_img, g2b_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));

		let b2r_mtx = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_transform(&white_img, b2r_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, b2r_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, b2r_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, b2r_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 0u8]));
		assert_eq!(color_transform(&b_img, b2r_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 255u8]));

		let b2g_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_transform(&white_img, b2g_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, b2g_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, b2g_mtx).get_pixel(0, 0), &Rgb([255u8, 0u8, 0u8]));
		assert_eq!(color_transform(&g_img, b2g_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&b_img, b2g_mtx).get_pixel(0, 0), &Rgb([0u8, 255u8, 255u8]));

		let naive_gray_mtx =
			[0.3333, 0.3333, 0.3333, 0.0, 0.3333, 0.3333, 0.3333, 0.0, 0.3333, 0.3333, 0.3333, 0.0];
		assert_eq!(color_transform(&white_img, naive_gray_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, naive_gray_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, naive_gray_mtx).get_pixel(0, 0), &Rgb([85u8, 85u8, 85u8]));
		assert_eq!(color_transform(&g_img, naive_gray_mtx).get_pixel(0, 0), &Rgb([85u8, 85u8, 85u8]));
		assert_eq!(color_transform(&b_img, naive_gray_mtx).get_pixel(0, 0), &Rgb([85u8, 85u8, 85u8]));

		let luma_gray_mtx =
			[0.2126, 0.7152, 0.0722, 0.0, 0.2126, 0.7152, 0.0722, 0.0, 0.2126, 0.7152, 0.0722, 0.0];
		assert_eq!(color_transform(&white_img, luma_gray_mtx).get_pixel(0, 0), &Rgb([255u8, 255u8, 255u8]));
		assert_eq!(color_transform(&black_img, luma_gray_mtx).get_pixel(0, 0), &Rgb([0u8, 0u8, 0u8]));
		assert_eq!(color_transform(&r_img, luma_gray_mtx).get_pixel(0, 0), &Rgb([54u8, 54u8, 54u8]));
		assert_eq!(color_transform(&g_img, luma_gray_mtx).get_pixel(0, 0), &Rgb([182u8, 182u8, 182u8]));
		assert_eq!(color_transform(&b_img, luma_gray_mtx).get_pixel(0, 0), &Rgb([18u8, 18u8, 18u8]));
	}

	#[test]
	fn test_scale_image() {
		let img = &RgbImage::from_fn(8, 8, |_x, _y| Rgb([255u8, 255u8, 255u8]));
		assert_eq!(scale_image(img, 2.0).dimensions(), (16, 16));
		assert_eq!(scale_image(img, 0.5).dimensions(), (4, 4));
		assert_eq!(scale_image(img, 1.01).dimensions(), (8, 8));
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
}
