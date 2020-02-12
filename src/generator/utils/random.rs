use rand::{Rng, rngs};

#[inline(always)]
pub fn get_random_range(rng: &mut rngs::ThreadRng, min: f64, max: f64) -> f64 {
	if min == max {
		return min;
	};
	rng.gen_range(min, max)
}

#[inline(always)]
pub fn get_random_int(rng: &mut rngs::ThreadRng, min: u32, max: u32) -> u32 {
	if min == max {
		return min;
	};
	rng.gen_range(min, max)
}

pub fn get_random_ranges(rng: &mut rngs::ThreadRng, ranges: &Vec<(f64, f64)>) -> f64 {
	let range: (f64, f64) = ranges[get_random_int(rng, 0, ranges.len() as u32) as usize];
	get_random_range(rng, range.0, range.1)
}
