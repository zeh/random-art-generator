use image::{Pixel, RgbImage};

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
		let n_r = ((o_r * matrix[0] + o_g * matrix[1] + o_b * matrix[ 2] + matrix[ 3]).round()).max(0.0).min(255.0) as u8;
		let n_g = ((o_r * matrix[4] + o_g * matrix[5] + o_b * matrix[ 6] + matrix[ 7]).round()).max(0.0).min(255.0) as u8;
		let n_b = ((o_r * matrix[8] + o_g * matrix[9] + o_b * matrix[10] + matrix[11]).round()).max(0.0).min(255.0) as u8;
		*pixel = image::Rgb([n_r, n_g, n_b]);
	}
	transformed_image
}
