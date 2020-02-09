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
