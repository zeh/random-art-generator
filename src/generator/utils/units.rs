#[derive(Clone, Debug, PartialEq)]
pub enum SizeUnit {
	Fraction(f64),
	Pixels(i64),
}

impl SizeUnit {
	pub fn to_pixels(&self, total_size: u32) -> i64 {
		match self {
			Self::Fraction(value) => (*value * total_size as f64).round() as i64,
			Self::Pixels(value) => *value,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_size_unit() {
		// Fraction
		assert_eq!(SizeUnit::Fraction(0.0).to_pixels(100), 0);
		assert_eq!(SizeUnit::Fraction(0.5).to_pixels(100), 50);
		assert_eq!(SizeUnit::Fraction(0.494).to_pixels(100), 49);
		assert_eq!(SizeUnit::Fraction(0.178).to_pixels(100), 18);

		// Pixels
		assert_eq!(SizeUnit::Pixels(10).to_pixels(100), 10);
		assert_eq!(SizeUnit::Pixels(20).to_pixels(200), 20);
		assert_eq!(SizeUnit::Pixels(30).to_pixels(100), 30);
		assert_eq!(SizeUnit::Pixels(40).to_pixels(200), 40);
	}
}
