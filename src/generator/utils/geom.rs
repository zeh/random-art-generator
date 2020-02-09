#[inline(always)]
pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
	let x = x1 - x2;
	let y = y1 - y2;
	(x * x + y * y).sqrt()
}
