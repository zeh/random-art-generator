#[derive(Clone, Debug, PartialEq)]
pub struct SmoothNumber {
	divider: f64,
	value: f64,
	has_value: bool,
}

impl SmoothNumber {
    #[allow(dead_code)]
	pub fn new(divider: f64) -> SmoothNumber {
		SmoothNumber {
			divider,
			value: 0.0,
			has_value: false,
		}
	}

    #[allow(dead_code)]
	pub fn put(&mut self, new_value: f64) {
		if self.has_value {
			self.value -= (self.value - new_value) / self.divider;
		} else {
			self.value = new_value;
			self.has_value = true;
		}
	}

	#[allow(dead_code)]
	pub fn get(&mut self) -> Result<f64, &str> {
		if self.has_value {
			Ok(self.value)
		} else {
			Err("Trying to get smooth number from empty list")
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_smooth_number_2() {
		let mut smt2 = SmoothNumber::new(2.0);

		assert_eq!(smt2.get().is_err(), true);

		smt2.put(10.0);
		assert_eq!(smt2.get().unwrap(), 10.0);
		smt2.put(20.0);
		assert_eq!(smt2.get().unwrap(), 15.0);
		smt2.put(24.0);
		assert_eq!(smt2.get().unwrap(), 19.5);
	}

	#[test]
	fn test_smooth_number_4() {
		let mut smt4 = SmoothNumber::new(4.0);

		assert_eq!(smt4.get().is_err(), true);

		smt4.put(10.0);
		assert_eq!(smt4.get().unwrap(), 10.0);
		smt4.put(20.0);
		assert_eq!(smt4.get().unwrap(), 12.5);
		smt4.put(30.0);
		assert_eq!(smt4.get().unwrap(), 16.875);
		smt4.put(40.0);
		assert_eq!(smt4.get().unwrap(), 22.65625);
	}

	#[test]
	fn test_smooth_number_10() {
		let mut smt10 = SmoothNumber::new(10.0);

		assert_eq!(smt10.get().is_err(), true);

		smt10.put(10.0);
		assert_eq!(smt10.get().unwrap(), 10.0);
		smt10.put(20.0);
		assert_eq!(smt10.get().unwrap(), 11.0);
		smt10.put(24.0);
		assert_eq!(smt10.get().unwrap(), 12.3);
		smt10.put(22.0);
		assert_eq!(smt10.get().unwrap(), 13.270000000000001); // TODO: Maybe use epsilon for tests instead
		smt10.put(22.0);
		assert_eq!(smt10.get().unwrap(), 14.143);
	}
}
