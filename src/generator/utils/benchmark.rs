use std::time::Instant;

#[cfg(test)]
use std::time::Duration;

#[cfg(test)]
use std::thread;

#[derive(Clone)]
pub struct TimerBenchmark {
	values_ns: Vec<u128>,
	start_time: Option<Instant>,
}

#[allow(dead_code)]
impl TimerBenchmark {
	pub fn new() -> TimerBenchmark {
		TimerBenchmark {
			values_ns: Vec::<u128>::new(),
			start_time: None,
		}
	}

	pub fn start(&mut self) {
		assert!(self.start_time.is_none(), "cannot start a measurement twice");
		self.start_time = Some(Instant::now());
	}

	pub fn stop(&mut self) {
		assert!(self.start_time.is_some(), "cannot stop a measurement that never started");
		let duration = self.start_time.unwrap().elapsed().as_nanos();
		self.values_ns.push(duration);
		self.start_time = None;
	}

	pub fn clear(&mut self) {
		self.values_ns.clear();
		self.start_time = None;
	}

	pub fn len(&self) -> usize {
		self.values_ns.len()
	}

	pub fn average_ms(&self) -> f64 {
		let len = self.len();
		return if len == 0 {
			0.0
		} else {
			let total: f64 = self.values_ns.iter().copied().map(|x| x as f64).sum();
			Self::ns_to_ms((total / len as f64).round() as u128)
		};
	}

	pub fn median_ms(&self) -> f64 {
		let len = self.len();
		return if len == 0 {
			0.0
		} else if len % 2 == 1 {
			let mut sorted_values = self.values_ns.clone();
			sorted_values.sort();
			Self::ns_to_ms(sorted_values[(len - 1) / 2])
		} else {
			let mut sorted_values = self.values_ns.clone();
			sorted_values.sort();
			Self::ns_to_ms(
				((sorted_values[len / 2 - 1] as f64 + sorted_values[len / 2] as f64) / 2.0).round() as u128,
			)
		};
	}

	pub fn min_ms(&self) -> f64 {
		Self::ns_to_ms(self.values_ns.iter().min().copied().unwrap_or(0))
	}

	pub fn max_ms(&self) -> f64 {
		Self::ns_to_ms(self.values_ns.iter().max().copied().unwrap_or(0))
	}

	pub fn last_ms(&self) -> f64 {
		assert!(self.values_ns.len() > 0, "cannot read last time of empty benchmark");
		Self::ns_to_ms(self.values_ns.last().copied().unwrap())
	}

	pub fn current_ms(&self) -> f64 {
		assert!(self.is_started(), "cannot read current time of stopped benchmark");
		Self::ns_to_ms(self.start_time.unwrap().elapsed().as_nanos())
	}

	pub fn is_started(&self) -> bool {
		self.start_time.is_some()
	}

	#[inline(always)]
	fn ns_to_ms(ns: u128) -> f64 {
		ns as f64 / 1_000_000.0
	}

	#[cfg(test)]
	pub fn mock_insert(&mut self, duration_ns: u128) {
		self.values_ns.push(duration_ns);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_timer_benchmark_empty() {
		let bench = TimerBenchmark::new();
		assert_eq!(bench.len(), 0);
		assert_eq!(bench.average_ms(), 0.0);
		assert_eq!(bench.median_ms(), 0.0);
		assert_eq!(bench.min_ms(), 0.0);
		assert_eq!(bench.max_ms(), 0.0);
	}

	#[test]
	fn test_timer_benchmark_time() {
		let mut bench = TimerBenchmark::new();
		bench.start();
		bench.stop();
		assert_eq!(bench.len(), 1);

		// Approximations for time
		assert_eq!(bench.average_ms() >= 0.0, true);
		assert_eq!(bench.median_ms() >= 0.0, true);
		assert_eq!(bench.min_ms() >= 0.0, true);
		assert_eq!(bench.max_ms() >= 0.0, true);
		assert_eq!(bench.last_ms() >= 0.0, true);

		assert_eq!(bench.average_ms() < 1000.0, true);
		assert_eq!(bench.median_ms() < 1000.0, true);
		assert_eq!(bench.min_ms() < 1000.0, true);
		assert_eq!(bench.max_ms() < 1000.0, true);
		assert_eq!(bench.last_ms() < 1000.0, true);

		bench.clear();
		assert_eq!(bench.len(), 0);

		bench.start();
		thread::sleep(Duration::from_millis(30));
		bench.stop();
		assert_eq!(bench.len(), 1);

		assert_eq!(bench.is_started(), false);
		bench.start();
		thread::sleep(Duration::from_millis(30));
		assert_eq!(bench.is_started(), true);
		assert_eq!(bench.current_ms() >= 30.0, true);
		assert_eq!(bench.current_ms() < 1000.0, true);
		bench.stop();

		assert_eq!(bench.len(), 2);

		// Approximations for time
		assert_eq!(bench.average_ms() >= 30.0, true);
		assert_eq!(bench.median_ms() >= 30.0, true);
		assert_eq!(bench.min_ms() >= 30.0, true);
		assert_eq!(bench.max_ms() >= 30.0, true);
		assert_eq!(bench.last_ms() >= 30.0, true);

		assert_eq!(bench.average_ms() < 1000.0, true);
		assert_eq!(bench.median_ms() < 1000.0, true);
		assert_eq!(bench.min_ms() < 1000.0, true);
		assert_eq!(bench.max_ms() < 1000.0, true);
		assert_eq!(bench.last_ms() < 1000.0, true);
	}

	#[test]
	fn test_timer_benchmark_inserted_single() {
		let mut bench = TimerBenchmark::new();
		bench.mock_insert(2_120_000);
		assert_eq!(bench.len(), 1);
		assert_eq!(bench.average_ms(), 2.12);
		assert_eq!(bench.median_ms(), 2.12);
		assert_eq!(bench.min_ms(), 2.12);
		assert_eq!(bench.max_ms(), 2.12);
		assert_eq!(bench.last_ms(), 2.12);
	}

	#[test]
	fn test_timer_benchmark_inserted_multiple() {
		let mut bench = TimerBenchmark::new();
		bench.mock_insert(2_120_000);
		bench.mock_insert(0_100_000);
		bench.mock_insert(3_500_000);
		bench.mock_insert(3_912_332);
		bench.mock_insert(1_012_100);
		assert_eq!(bench.len(), 5);
		assert_eq!(bench.average_ms(), 2.128886);
		assert_eq!(bench.median_ms(), 2.12);
		assert_eq!(bench.min_ms(), 0.1);
		assert_eq!(bench.max_ms(), 3.912332);
		assert_eq!(bench.last_ms(), 1.0121);

		let mut bench = TimerBenchmark::new();
		bench.mock_insert(0_100_000);
		bench.mock_insert(3_500_000);
		bench.mock_insert(3_912_332);
		bench.mock_insert(1_012_100);
		assert_eq!(bench.len(), 4);
		assert_eq!(bench.average_ms(), 2.131108);
		assert_eq!(bench.median_ms(), 2.25605);
		assert_eq!(bench.min_ms(), 0.1);
		assert_eq!(bench.max_ms(), 3.912332);
		assert_eq!(bench.last_ms(), 1.0121);
	}
}
