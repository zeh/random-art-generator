#[inline(always)]
pub fn blend(bottom: &[u8], top: &[u8], alpha: f64) -> [u8; 3] {
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

#[inline(always)]
pub fn color_matrix(pixel: &[u8], matrix: [f64; 12]) -> [u8; 3] {
	let rgb = [pixel[0] as f64, pixel[1] as f64, pixel[2] as f64];
	[
		color_matrix_channel(rgb, [matrix[0], matrix[1], matrix[2]], matrix[3]),
		color_matrix_channel(rgb, [matrix[4], matrix[5], matrix[6]], matrix[7]),
		color_matrix_channel(rgb, [matrix[8], matrix[9], matrix[10]], matrix[11]),
	]
}

#[inline(always)]
fn color_matrix_channel(rgb: [f64; 3], rgb_mul: [f64; 3], offset: f64) -> u8 {
	let result = rgb[0] * rgb_mul[0] + rgb[1] * rgb_mul[1] + rgb[2] * rgb_mul[2] + offset;
	result.round().max(0.0).min(255.0) as u8
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_blend() {
		assert_eq!(blend(&[0, 0, 0], &[255, 128, 0], 0.0), [0, 0, 0]);
		assert_eq!(blend(&[0, 0, 0], &[255, 128, 0], 0.1), [26, 13, 0]);
		assert_eq!(blend(&[0, 0, 0], &[255, 128, 0], 0.5), [128, 64, 0]);
		assert_eq!(blend(&[0, 0, 0], &[255, 128, 0], 1.0), [255, 128, 0]);
		assert_eq!(blend(&[128, 128, 128], &[255, 128, 0], 0.0), [128, 128, 128]);
		assert_eq!(blend(&[128, 128, 128], &[255, 128, 0], 0.1), [141, 128, 115]);
		assert_eq!(blend(&[128, 128, 128], &[255, 128, 0], 0.5), [192, 128, 64]);
		assert_eq!(blend(&[128, 128, 128], &[255, 128, 0], 1.0), [255, 128, 0]);
		assert_eq!(blend(&[255, 255, 255], &[255, 128, 0], 0.0), [255, 255, 255]);
		assert_eq!(blend(&[255, 255, 255], &[255, 128, 0], 0.1), [255, 242, 230]);
		assert_eq!(blend(&[255, 255, 255], &[255, 128, 0], 0.5), [255, 192, 128]);
		assert_eq!(blend(&[255, 255, 255], &[255, 128, 0], 1.0), [255, 128, 0]);
		assert_eq!(blend(&[0, 128, 255], &[0, 10, 20], 0.0), [0, 128, 255]);
		assert_eq!(blend(&[0, 128, 255], &[0, 10, 20], 0.1), [0, 116, 232]);
		assert_eq!(blend(&[0, 128, 255], &[0, 10, 20], 0.5), [0, 69, 138]);
		assert_eq!(blend(&[0, 128, 255], &[0, 10, 20], 1.0), [0, 10, 20]);
	}

	#[test]
	fn test_color_matrix() {
		let white = [255u8, 255u8, 255u8];
		let black = [0u8, 0u8, 0u8];
		let red = [255u8, 0u8, 0u8];
		let green = [0u8, 255u8, 0u8];
		let blue = [0u8, 0u8, 255u8];

		let identity_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_matrix(&white, identity_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, identity_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, identity_mtx), [255u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, identity_mtx), [0u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&blue, identity_mtx), [0u8, 0u8, 255u8]);

		let red_filter_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
		assert_eq!(color_matrix(&white, red_filter_mtx), [255u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&black, red_filter_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, red_filter_mtx), [255u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, red_filter_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&blue, red_filter_mtx), [0u8, 0u8, 0u8]);

		let green_filter_mtx = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
		assert_eq!(color_matrix(&white, green_filter_mtx), [0u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&black, green_filter_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, green_filter_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, green_filter_mtx), [0u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&blue, green_filter_mtx), [0u8, 0u8, 0u8]);

		let blue_filter_mtx = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_matrix(&white, blue_filter_mtx), [0u8, 0u8, 255u8]);
		assert_eq!(color_matrix(&black, blue_filter_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, blue_filter_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, blue_filter_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&blue, blue_filter_mtx), [0u8, 0u8, 255u8]);

		let red_fill_mtx = [1.0, 0.0, 0.0, 255.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_matrix(&white, red_fill_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, red_fill_mtx), [255u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, red_fill_mtx), [255u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, red_fill_mtx), [255u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&blue, red_fill_mtx), [255u8, 0u8, 255u8]);

		let green_fill_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 255.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_matrix(&white, green_fill_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, green_fill_mtx), [0u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&red, green_fill_mtx), [255u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&green, green_fill_mtx), [0u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&blue, green_fill_mtx), [0u8, 255u8, 255u8]);

		let blue_fill_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 255.0];
		assert_eq!(color_matrix(&white, blue_fill_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, blue_fill_mtx), [0u8, 0u8, 255u8]);
		assert_eq!(color_matrix(&red, blue_fill_mtx), [255u8, 0u8, 255u8]);
		assert_eq!(color_matrix(&green, blue_fill_mtx), [0u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&blue, blue_fill_mtx), [0u8, 0u8, 255u8]);

		let red_drain_mtx = [1.0, 0.0, 0.0, -255.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_matrix(&white, red_drain_mtx), [0u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, red_drain_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, red_drain_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, red_drain_mtx), [0u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&blue, red_drain_mtx), [0u8, 0u8, 255u8]);

		let green_drain_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -255.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_matrix(&white, green_drain_mtx), [255u8, 0u8, 255u8]);
		assert_eq!(color_matrix(&black, green_drain_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, green_drain_mtx), [255u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, green_drain_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&blue, green_drain_mtx), [0u8, 0u8, 255u8]);

		let blue_drain_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -255.0];
		assert_eq!(color_matrix(&white, blue_drain_mtx), [255u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&black, blue_drain_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, blue_drain_mtx), [255u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, blue_drain_mtx), [0u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&blue, blue_drain_mtx), [0u8, 0u8, 0u8]);

		let r2g_mtx = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_matrix(&white, r2g_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, r2g_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, r2g_mtx), [255u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&green, r2g_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&blue, r2g_mtx), [0u8, 0u8, 255u8]);

		let r2b_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0];
		assert_eq!(color_matrix(&white, r2b_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, r2b_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, r2b_mtx), [255u8, 0u8, 255u8]);
		assert_eq!(color_matrix(&green, r2b_mtx), [0u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&blue, r2b_mtx), [0u8, 0u8, 0u8]);

		let g2r_mtx = [0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_matrix(&white, g2r_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, g2r_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, g2r_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, g2r_mtx), [255u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&blue, g2r_mtx), [0u8, 0u8, 255u8]);

		let g2b_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
		assert_eq!(color_matrix(&white, g2b_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, g2b_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, g2b_mtx), [255u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, g2b_mtx), [0u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&blue, g2b_mtx), [0u8, 0u8, 0u8]);

		let b2r_mtx = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_matrix(&white, b2r_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, b2r_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, b2r_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, b2r_mtx), [0u8, 255u8, 0u8]);
		assert_eq!(color_matrix(&blue, b2r_mtx), [255u8, 0u8, 255u8]);

		let b2g_mtx = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
		assert_eq!(color_matrix(&white, b2g_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, b2g_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, b2g_mtx), [255u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&green, b2g_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&blue, b2g_mtx), [0u8, 255u8, 255u8]);

		let naive_gray_mtx =
			[0.3333, 0.3333, 0.3333, 0.0, 0.3333, 0.3333, 0.3333, 0.0, 0.3333, 0.3333, 0.3333, 0.0];
		assert_eq!(color_matrix(&white, naive_gray_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, naive_gray_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, naive_gray_mtx), [85u8, 85u8, 85u8]);
		assert_eq!(color_matrix(&green, naive_gray_mtx), [85u8, 85u8, 85u8]);
		assert_eq!(color_matrix(&blue, naive_gray_mtx), [85u8, 85u8, 85u8]);

		let luma_gray_mtx =
			[0.2126, 0.7152, 0.0722, 0.0, 0.2126, 0.7152, 0.0722, 0.0, 0.2126, 0.7152, 0.0722, 0.0];
		assert_eq!(color_matrix(&white, luma_gray_mtx), [255u8, 255u8, 255u8]);
		assert_eq!(color_matrix(&black, luma_gray_mtx), [0u8, 0u8, 0u8]);
		assert_eq!(color_matrix(&red, luma_gray_mtx), [54u8, 54u8, 54u8]);
		assert_eq!(color_matrix(&green, luma_gray_mtx), [182u8, 182u8, 182u8]);
		assert_eq!(color_matrix(&blue, luma_gray_mtx), [18u8, 18u8, 18u8]);
	}

	#[test]
	fn test_color_matrix_channel() {
		let white = [255.0, 255.0, 255.0];
		let black = [0.0, 0.0, 0.0];
		let red = [255.0, 0.0, 0.0];
		let green = [0.0, 255.0, 0.0];
		let blue = [0.0, 0.0, 255.0];

		assert_eq!(color_matrix_channel(white, [1.0, 0.0, 0.0], 0.0), 255);
		assert_eq!(color_matrix_channel(white, [0.0, 1.0, 0.0], 0.0), 255);
		assert_eq!(color_matrix_channel(white, [0.0, 0.0, 1.0], 0.0), 255);
		assert_eq!(color_matrix_channel(white, [0.0, 0.0, 0.0], 0.0), 0);
		assert_eq!(color_matrix_channel(white, [0.2, 0.3, 0.4], -1.0), 229);
		assert_eq!(color_matrix_channel(white, [0.2, 0.3, 0.4], 0.0), 230);
		assert_eq!(color_matrix_channel(white, [0.2, 0.3, 0.4], 1.0), 231);
		assert_eq!(color_matrix_channel(white, [0.2, 0.3, 0.4], -255.0), 0);
		assert_eq!(color_matrix_channel(white, [0.2, 0.3, 0.4], 255.0), 255);

		assert_eq!(color_matrix_channel(white, [1.0, 1.0, 1.0], 128.0), 255);
		assert_eq!(color_matrix_channel(white, [0.0, 0.0, 0.0], 128.0), 128);
		assert_eq!(color_matrix_channel(white, [1.0, 0.0, 0.0], -128.0), 127);
		assert_eq!(color_matrix_channel(white, [0.0, 0.0, 0.0], 128.0), 128);

		assert_eq!(color_matrix_channel(black, [1.0, 0.0, 0.0], 0.0), 0);
		assert_eq!(color_matrix_channel(black, [0.0, 1.0, 0.0], 0.0), 0);
		assert_eq!(color_matrix_channel(black, [0.0, 0.0, 1.0], 0.0), 0);
		assert_eq!(color_matrix_channel(black, [0.0, 0.0, 0.0], 0.0), 0);
		assert_eq!(color_matrix_channel(black, [0.0, 0.0, 0.0], -255.0), 0);
		assert_eq!(color_matrix_channel(black, [0.0, 0.0, 0.0], -128.0), 0);
		assert_eq!(color_matrix_channel(black, [0.0, 0.0, 0.0], 128.0), 128);
		assert_eq!(color_matrix_channel(black, [0.0, 0.0, 0.0], 255.0), 255);
		assert_eq!(color_matrix_channel(black, [0.0, 0.0, 0.0], 256.0), 255);

		assert_eq!(color_matrix_channel(red, [1.0, 0.0, 0.0], 0.0), 255);
		assert_eq!(color_matrix_channel(red, [0.0, 1.0, 0.0], 0.0), 0);
		assert_eq!(color_matrix_channel(red, [0.0, 0.0, 1.0], 0.0), 0);

		assert_eq!(color_matrix_channel(green, [1.0, 0.0, 0.0], 0.0), 0);
		assert_eq!(color_matrix_channel(green, [0.0, 1.0, 0.0], 0.0), 255);
		assert_eq!(color_matrix_channel(green, [0.0, 0.0, 1.0], 0.0), 0);

		assert_eq!(color_matrix_channel(blue, [1.0, 0.0, 0.0], 0.0), 0);
		assert_eq!(color_matrix_channel(blue, [0.0, 1.0, 0.0], 0.0), 0);
		assert_eq!(color_matrix_channel(blue, [0.0, 0.0, 1.0], 0.0), 255);
	}
}
