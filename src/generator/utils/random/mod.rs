use std::f64::consts::PI;

use crate::generator::utils::units::{SizeUnit, WeightedValue};
use rng::Rng;

pub mod rng;

pub fn get_random_seed() -> u32 {
	Rng::new().next()
}

pub fn get_rng(seed: u32, iteration: u32) -> Rng {
	// Seeds close to each other produce very similar results, so we multiply them a bit
	Rng::from_seed(seed.wrapping_add(Rng::from_seed(iteration).next()))
}

#[inline(always)]
pub fn get_random_range(rng: &mut Rng, min: f64, pseudo_max: f64) -> f64 {
	rng.next_f64_range(min, pseudo_max)
}

#[inline(always)]
pub fn get_random_range_bias(rng: &mut Rng, min: f64, max: f64, bias: f64) -> f64 {
	if min == max {
		return min;
	};
	let mut r = rng.next_f64();
	if bias < 0.0f64 {
		r = r.powf(-bias + 1.0f64);
	} else if bias > 0.0f64 {
		r = 1.0f64 - (1.0f64 - r).powf(bias + 1.0f64);
	}
	min + r * (max - min)
}

pub fn get_random_size_range_bias(
	rng: &mut Rng,
	min: &SizeUnit,
	max: &SizeUnit,
	bias: f64,
	pixel_size: u32,
) -> f64 {
	let min_pixels = min.to_pixels(pixel_size);
	let max_pixels = max.to_pixels(pixel_size);
	return get_random_range_bias(rng, min_pixels as f64, max_pixels as f64, bias);
}

pub fn get_random_entry_weighted<'a, T>(rng: &mut Rng, entries: &'a Vec<WeightedValue<T>>) -> &'a T {
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
	rng: &mut Rng,
	ranges: &Vec<WeightedValue<(f64, f64)>>,
	bias: f64,
) -> f64 {
	let range = get_random_entry_weighted(rng, ranges);
	get_random_range_bias(rng, range.0, range.1, bias)
}

pub fn get_random_size_ranges_bias_weighted(
	rng: &mut Rng,
	ranges: &Vec<WeightedValue<(SizeUnit, SizeUnit)>>,
	bias: f64,
	pixel_size: u32,
) -> f64 {
	let range = get_random_entry_weighted(rng, ranges);
	get_random_size_range_bias(rng, &range.0, &range.1, bias, pixel_size)
}

pub fn get_random_noise_sequence(rng: &mut Rng, min: f64, max: f64) -> [f64; 256] {
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

pub fn get_random_color(rng: &mut Rng) -> [u8; 3] {
	[rng.next_u32_range(0, 256) as u8, rng.next_u32_range(0, 256) as u8, rng.next_u32_range(0, 256) as u8]
}
