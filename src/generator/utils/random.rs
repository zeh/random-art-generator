use rand::{Rng, rngs};

#[inline(always)]
pub fn get_random_range(rng: &mut rngs::ThreadRng, min: f64, max: f64) -> f64 {
	if min == max {
		return min;
	};
	rng.gen_range(min, max)
}
