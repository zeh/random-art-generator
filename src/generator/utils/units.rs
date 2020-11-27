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

#[derive(Clone, Debug, PartialEq)]
pub struct Margins<T> {
	pub top: T,
	pub right: T,
	pub bottom: T,
	pub left: T,
}

impl Margins<SizeUnit> {
	pub fn to_pixels(&self, width: u32, height: u32) -> Margins<i64> {
		Margins::<i64> {
			top: self.top.to_pixels(height),
			right: self.right.to_pixels(width),
			bottom: self.bottom.to_pixels(height),
			left: self.left.to_pixels(width),
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

	#[test]
	fn test_margins() {
		let ms = Margins::<SizeUnit> {
			top: SizeUnit::Fraction(0.5),
			right: SizeUnit::Fraction(0.123),
			bottom: SizeUnit::Fraction(0.987),
			left: SizeUnit::Fraction(0.0),
		};

		assert_eq!(
			ms.to_pixels(200, 300),
			Margins::<i64> {
				top: 150,
				right: 25,
				bottom: 296,
				left: 0,
			}
		);
		assert_eq!(
			ms.to_pixels(100, 100),
			Margins::<i64> {
				top: 50,
				right: 12,
				bottom: 99,
				left: 0,
			}
		);
	}
}
