use rand::{rngs, Rng};
use std::f64::consts::PI;

use crate::generator::utils::units::SizeUnit;

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

pub fn get_random_size_range_bias(
	rng: &mut rngs::ThreadRng,
	min: &SizeUnit,
	max: &SizeUnit,
	bias: f64,
	pixel_size: u32,
) -> f64 {
	let min_pixels = min.to_pixels(pixel_size);
	let max_pixels = max.to_pixels(pixel_size);
	return get_random_range_bias(rng, min_pixels as f64, max_pixels as f64, bias);
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

pub fn get_random_size_ranges_bias(
	rng: &mut rngs::ThreadRng,
	ranges: &Vec<(SizeUnit, SizeUnit)>,
	bias: f64,
	pixel_size: u32,
) -> f64 {
	let range: &(SizeUnit, SizeUnit) = &ranges[get_random_int(rng, 0, ranges.len() as u32) as usize];
	get_random_size_range_bias(rng, &range.0, &range.1, bias, pixel_size)
}

pub fn get_random_noise_sequence(rng: &mut rngs::ThreadRng, min: f64, max: f64) -> [f64; 256] {
	let mut sequence = [0f64; 256];
	for i in 0..256 {
		sequence[i] = rng.gen_range(min, max);
	}
	return sequence;
}

#[inline(always)]
pub fn get_noise_value(noise: [f64; 256], position: f64) -> f64 {
	let pp = if position < 0.0 {
		1.0 - position.abs()
	} else {
		position
	};
	let pp = pp.fract() * 256.0f64;
	let p1 = pp.floor() as usize;
	let p2 = (p1 + 1) % 256;

	let v1 = noise[p1];
	let v2 = noise[p2];

	// Phase
	let f = pp.fract();

	// Remap phase for smoothstep
	let f = (1.0 - (f * PI).cos()) * 0.5;

	v1 + (v2 - v1) * f
}
