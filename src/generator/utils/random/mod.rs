use getrandom::getrandom;
use oorandom::Rand64;
use std::f64::consts::PI;

use crate::generator::utils::units::{SizeUnit, WeightedValue};

pub fn get_random_seed() -> u128 {
	let mut seed_buffer = [0u8; 8];
	getrandom(&mut seed_buffer).expect("Generating seed");
	// Not very elegant way to convert a [u8] to [u128]
	let mut seed = 0u128;
	for i in 0..seed_buffer.len() {
		seed |= (seed_buffer[i] as u128) << i * 8;
	}
	seed
}

pub fn get_rng(seed: u128, inc: u64) -> Rand64 {
	// Seeds close to each other produce very similar results, so we multiply them a bit
	Rand64::new(seed.wrapping_add((inc as u128) << 32))
}

#[inline(always)]
pub fn get_random_range(rng: &mut Rand64, min: f64, max: f64) -> f64 {
	if min == max {
		return min;
	};
	min + rng.rand_float() * (max - min)
}

#[inline(always)]
pub fn get_random_range_bias(rng: &mut Rand64, min: f64, max: f64, bias: f64) -> f64 {
	if min == max {
		return min;
	};
	let mut r = rng.rand_float();
	if bias < 0.0f64 {
		r = r.powf(-bias + 1.0f64);
	} else if bias > 0.0f64 {
		r = 1.0f64 - (1.0f64 - r).powf(bias + 1.0f64);
	}
	min + r * (max - min)
}

pub fn get_random_size_range_bias(
	rng: &mut Rand64,
	min: &SizeUnit,
	max: &SizeUnit,
	bias: f64,
	pixel_size: u32,
) -> f64 {
	let min_pixels = min.to_pixels(pixel_size);
	let max_pixels = max.to_pixels(pixel_size);
	return get_random_range_bias(rng, min_pixels as f64, max_pixels as f64, bias);
}

pub fn get_random_entry_weighted<'a, T>(rng: &mut Rand64, entries: &'a Vec<WeightedValue<T>>) -> &'a T {
	let total_weight = entries.iter().map(|r| r.weight).sum();
	let desired_position = get_random_range(rng, 0.0, total_weight);
	let mut acc = 0.0f64;
	&entries
		.iter()
		.find(|&r| {
			acc += r.weight;
			acc >= desired_position
		})
		.expect("finding weighted random value")
		.value
}

pub fn get_random_ranges_bias_weighted(
	rng: &mut Rand64,
	ranges: &Vec<WeightedValue<(f64, f64)>>,
	bias: f64,
) -> f64 {
	let range = get_random_entry_weighted(rng, ranges);
	get_random_range_bias(rng, range.0, range.1, bias)
}

pub fn get_random_size_ranges_bias_weighted(
	rng: &mut Rand64,
	ranges: &Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	bias: f64,
	pixel_size: u32,
) -> f64 {
	let range = get_random_entry_weighted(rng, ranges);
	get_random_size_range_bias(rng, &range.0, &range.1, bias, pixel_size)
}

pub fn get_random_noise_sequence(rng: &mut Rand64, min: f64, max: f64) -> [f64; 256] {
	let mut sequence = [0f64; 256];
	for i in 0..256 {
		sequence[i] = get_random_range(rng, min, max);
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

pub fn get_random_color(rng: &mut Rand64) -> [u8; 3] {
	[rng.rand_range(0..256) as u8, rng.rand_range(0..256) as u8, rng.rand_range(0..256) as u8]
}
