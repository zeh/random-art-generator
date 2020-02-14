use rand::{Rng, rngs};

#[inline(always)]
pub fn get_random_range(rng: &mut rngs::ThreadRng, min: f64, max: f64) -> f64 {
	if min == max {
		return min;
	};
	rng.gen_range(min, max)
}

#[inline(always)]
pub fn get_random_range_bias(rng: &mut rngs::ThreadRng, min: f64, max: f64, bias: f64) -> f64 {
	if min == max {
		return min;
	};
	let mut r = rng.gen_range(0.0f64, 1.0f64);
	if bias < 0.0f64 {
		r = r.powf(-bias + 1.0f64);
	} else if bias > 0.0f64 {
		r = 1.0f64 - (1.0f64 - r).powf(bias + 1.0f64);
	}
	min + r * (max - min)
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

pub fn get_random_ranges_bias(rng: &mut rngs::ThreadRng, ranges: &Vec<(f64, f64)>, bias: f64) -> f64 {
	let range: (f64, f64) = ranges[get_random_int(rng, 0, ranges.len() as u32) as usize];
	get_random_range_bias(rng, range.0, range.1, bias)
}
