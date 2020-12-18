use getrandom::getrandom;

pub struct Rng {
	seed: u32,
	value: u32,
}

impl Rng {
	/// Generate a new Prando pseudo-random number generator. Uses a pseudo-random seed.
	pub fn new() -> Rng {
		let mut seed_buffer = [0u8; 4];
		getrandom(&mut seed_buffer).expect("Generating seed");
		// Not very elegant way to convert a [u8] to [u32]
		let mut seed: u32 = 0;
		for i in 0..seed_buffer.len() {
			seed |= (seed_buffer[i] as u32) << i * 8;
		}

		Rng::from_seed(seed)
	}

	/// Generate a new Prando pseudo-random number generator.
	///
	/// @param seed - A number that determines which pseudo-random number sequence will be created.
	pub fn from_seed(seed: u32) -> Rng {
		let mut rng = Rng {
			seed,
			value: 0,
		};
		rng.reset();
		rng
	}

	fn xorshift(mut value: u32) -> u32 {
		// Xorshift*32
		// Based on George Marsaglia's work: http://www.jstatsoft.org/v08/i14/paper
		value ^= value.wrapping_shl(13);
		value ^= value.wrapping_shr(17);
		value ^= value.wrapping_shl(5);
		value
	}

	fn recalculate(&mut self) {
		self.value = Rng::xorshift(self.value);
	}

	/// Reset the pseudo-random number sequence back to its starting seed. Further calls to next()
	/// will then produce the same sequence of numbers it had produced before. This is equivalent to
	/// creating a new instance with the same seed as another Prando instance.
	///
	/// Example:
	/// let rng = Rng::new_from_seed(12345678);
	/// println!(rng.next()); // 0.6177754114889017
	/// println!(rng.next()); // 0.5784605181725837
	/// rng.reset();
	/// println!(rng.next()); // 0.6177754114889017 again
	/// println!(rng.next()); // 0.5784605181725837 again
	pub fn reset(&mut self) {
		self.value = self.seed;
	}

	/// Skips ahead in the sequence of numbers that are being generated. This is equivalent to
	/// calling next() a specified number of times, but faster since it doesn't need to map the
	/// new random numbers to a range and return it.
	#[allow(dead_code)]
	pub fn skip(&mut self, mut iterations: u32) {
		while iterations > 0 {
			self.recalculate();
			iterations -= 1;
		}
	}

	/// Generates a pseudo-random number between 0 (inclusive) and u32 max (exclusive).
	///
	/// @return The generated pseudo-random number.
	pub fn next(&mut self) -> u32 {
		self.recalculate();
		self.value
	}

	/// Generates a pseudo-random number between a lower (inclusive) and a higher (exclusive) bounds.
	///
	/// @param min - The minimum number that can be randomly generated.
	/// @param pseudo_max - The maximum number that can be randomly generated (exclusive).
	/// @return The generated pseudo-random number.
	#[allow(dead_code)]
	pub fn next_u32_range(&mut self, min: u32, pseudo_max: u32) -> u32 {
		self.next_f64_range(min as f64, pseudo_max as f64) as u32
	}

	/// Generates a pseudo-random number between 0 (inclusive) and 1 (exclusive).
	///
	/// @return The generated pseudo-random number.
	#[allow(dead_code)]
	pub fn next_f64(&mut self) -> f64 {
		self.next() as f64 / (!0u32 as f64)
	}

	/// Generates a pseudo-random number between a lower (inclusive) and a higher (exclusive) bounds.
	///
	/// @param min - The minimum number that can be randomly generated.
	/// @param pseudo_max - The maximum number that can be randomly generated (exclusive).
	/// @return The generated pseudo-random number.
	#[allow(dead_code)]
	pub fn next_f64_range(&mut self, min: f64, pseudo_max: f64) -> f64 {
		if min == pseudo_max {
			return min;
		}
		self.next_f64() * (pseudo_max - min) + min
	}

	/// Generates a pseudo-random boolean.
	///
	/// @return A value of true or false.
	#[allow(dead_code)]
	pub fn next_bool(&mut self) -> bool {
		self.next_f64() > 0.5f64
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic_and_reset() {
		let mut rng = Rng::new();
		let num1 = rng.next();
		let num2 = rng.next();

		assert_ne!(num1, num2);

		rng.reset();
		assert_eq!(rng.next(), num1);
		assert_eq!(rng.next(), num2);
	}

	#[test]
	fn test_seed() {
		let mut rng = Rng::from_seed(12345678);
		let num1 = rng.next();
		let num2 = rng.next();

		assert_ne!(num1, num2);

		// Pre-generated values
		assert_eq!(num1, 506005380);
		assert_eq!(num2, 3857352168);

		rng.reset();
		assert_eq!(rng.next(), num1);
		assert_eq!(rng.next(), num2);

		let mut rng2 = Rng::from_seed(97342);
		assert_ne!(rng2.next(), num1);
		assert_ne!(rng2.next(), num2);
	}

	#[test]
	fn test_skip() {
		let seed = 1234337;

		let mut rng = Rng::from_seed(seed);
		let num1 = rng.next();
		for _ in 0..10 {
			rng.next();
		}
		let num2 = rng.next();

		let mut rng2 = Rng::from_seed(seed);
		assert_eq!(rng2.next(), num1);
		rng2.skip(10);
		assert_eq!(rng2.next(), num2);
		rng2.reset();
		rng2.skip(11);
		assert_eq!(rng2.next(), num2);
	}

	#[test]
	fn test_generate_integers() {
		let mut rng = Rng::from_seed(1337);

		let num1 = rng.next_u32_range(5, 15);
		let num2 = rng.next_u32_range(5, 15);

		assert_ne!(num2, num1);

		// Pre-generated values
		assert_eq!(num1, 5);
		assert_eq!(num2, 13);

		rng.reset();
		assert_eq!(rng.next_u32_range(5, 15), num1);
		assert_eq!(rng.next_u32_range(5, 15), num2);

		// Within range
		let mut any_lower = false;
		let mut any_higher = false;

		for _ in 0..100 {
			let f = rng.next_u32_range(2, 42);
			if f < 2 {
				any_lower = true;
			}
			if f >= 42 {
				any_higher = true;
			}
		}

		assert_eq!(any_lower, false);
		assert_eq!(any_higher, false);
	}

	#[test]
	fn test_generate_bools() {
		let mut rng = Rng::from_seed(31339);

		// Pre-generated values
		assert_eq!(rng.next_bool(), true);
		assert_eq!(rng.next_bool(), false);
		assert_eq!(rng.next_bool(), false);

		rng.reset();
		assert_eq!(rng.next_bool(), true);
		assert_eq!(rng.next_bool(), false);
		assert_eq!(rng.next_bool(), false);
	}

	#[test]
	fn test_randomize_evenly() {
		let mut totals = Vec::<u32>::new();
		let numbers_created: u32 = 60000;
		let slots: u32 = 10;
		let mut rng = Rng::from_seed(924576);

		while totals.len() < slots as usize {
			totals.push(0);
		}

		for _ in 0..numbers_created {
			let pos: usize = rng.next_u32_range(0, slots) as usize;
			totals[pos] = totals[pos] + 1;
		}

		let expected_total_per_range = numbers_created as f64 / slots as f64;
		for i in 0..totals.len() {
			// Percentage off the expected amount
			let delta = (totals[i] as f64 - expected_total_per_range) / expected_total_per_range as f64;
			// Max 2% deviation from expected
			assert!(delta < 0.02);
			assert!(delta > -0.02);
		}
	}

	#[test]
	fn test_full_sequence() {
		// Should allow a minimum 2^32-2 sequence
		let mut rng = Rng::from_seed(237622);
		let num = rng.next();

		// Create a sequence
		let mut repeat_pos: u64 = 0;
		let l = 2u64.pow(32);
		for i in 0..l {
			if rng.next() == num {
				repeat_pos = i;
				break;
			}
		}

		assert!(repeat_pos > 2u64.pow(32) - 3);
	}
}
