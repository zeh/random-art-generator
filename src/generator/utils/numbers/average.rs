#[derive(Clone, Debug, PartialEq)]
pub struct AverageNumber {
	max_length: usize,
	length: usize,
	position: usize,
	is_dirty: bool,
	value: f64,
	values: Vec<f64>,
}

impl AverageNumber {
    #[allow(dead_code)]
	pub fn new(max_length: usize) -> AverageNumber {
		AverageNumber {
			max_length,
			length: 0,
			position: 0,
			is_dirty: true,
			value: 0.0,
			values: Vec::<f64>::new(),
		}
	}

    #[allow(dead_code)]
	pub fn put(&mut self, value: f64) {
		if self.position < self.length {
			self.values[self.position] = value;
		} else {
			self.values.push(value);
			self.length += 1;
		}
		self.position = (self.position + 1) % self.max_length;
		self.is_dirty = true;
	}

	#[allow(dead_code)]
	pub fn get(&mut self) -> Result<f64, &str> {
		if self.is_dirty {
			// Calculates the median value
			if self.length == 0 {
				return Err("Trying to get average number from empty list");
			} else {
				let mut total: f64 = 0.0;
				for i in 0..self.length {
					total += self.values[(((self.position as i64 - 1 - i as i64) + self.max_length as i64)
						% self.max_length as i64) as usize] as f64;
				}
				self.value = total / self.length as f64;
			}
			self.is_dirty = false;
		}
		Ok(self.value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_average_number_1() {
		let mut avg1 = AverageNumber::new(1);

		assert_eq!(avg1.get().is_err(), true);

		avg1.put(10.0);
		assert_eq!(avg1.get().unwrap(), 10.0);
		avg1.put(20.0);
		assert_eq!(avg1.get().unwrap(), 20.0);
		avg1.put(24.0);
		assert_eq!(avg1.get().unwrap(), 24.0);
	}

	#[test]
	fn test_average_number_2() {
		let mut avg2 = AverageNumber::new(2);

		assert_eq!(avg2.get().is_err(), true);

		avg2.put(10.0);
		assert_eq!(avg2.get().unwrap(), 10.0);
		avg2.put(20.0);
		assert_eq!(avg2.get().unwrap(), 15.0);
		avg2.put(30.0);
		assert_eq!(avg2.get().unwrap(), 25.0);
		avg2.put(40.0);
		assert_eq!(avg2.get().unwrap(), 35.0);
	}

	#[test]
	fn test_average_number_4() {
		let mut avg4 = AverageNumber::new(4);

		assert_eq!(avg4.get().is_err(), true);

		avg4.put(10.0);
		assert_eq!(avg4.get().unwrap(), 10.0);
		avg4.put(20.0);
		assert_eq!(avg4.get().unwrap(), 15.0);
		avg4.put(24.0);
		assert_eq!(avg4.get().unwrap(), 18.0);
		avg4.put(22.0);
		assert_eq!(avg4.get().unwrap(), 19.0);
		avg4.put(22.0);
		assert_eq!(avg4.get().unwrap(), 22.0);
	}
}
