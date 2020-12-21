pub fn format_time(ms: f64) -> String {
	let seconds = ms / 1000.0;
	let minutes = seconds / 60.0;
	let hours = minutes / 60.0;

	let r_seconds = (seconds % 60.0).floor();
	let r_minutes = (minutes % 60.0).floor();
	let r_hours = hours.floor();

	if hours >= 1.0 {
		format!("{:.0}h {:02.0}m {:02.0}s", r_hours, r_minutes, r_seconds).to_owned()
	} else if minutes >= 1.0 {
		format!("{:.0}m {:02.0}s", r_minutes, r_seconds).to_owned()
	} else {
		format!("{:.0}s", r_seconds).to_owned()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_format_time() {
		let s = 1000.0;
		let m = s * 60.0;
		let h = m * 60.0;
		assert_eq!(format_time(0.0), "0s");
		assert_eq!(format_time(1.0), "0s");
		assert_eq!(format_time(999.0), "0s");
		assert_eq!(format_time(s), "1s");
		assert_eq!(format_time(s * 59.0), "59s");
		assert_eq!(format_time(m), "1m 00s");
		assert_eq!(format_time(m * 50.0 + s * 10.0), "50m 10s");
		assert_eq!(format_time(h), "1h 00m 00s");
		assert_eq!(format_time(h * 20.0 + m * 59.0 + s * 33.0), "20h 59m 33s");
	}
}
