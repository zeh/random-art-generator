use color_processing::Color;

use crate::generator::utils::units::SizeUnit;

pub fn parse_color(src: &str) -> Result<(u8, u8, u8), &str> {
	match Color::new_string(src) {
		Some(color) => {
			let rgb = color.get_rgba();
			let r = (rgb.0 * 255.0).round() as u8;
			let g = (rgb.1 * 255.0).round() as u8;
			let b = (rgb.2 * 255.0).round() as u8;
			Ok((r, g, b))
		}
		None => Err("Cannot parse color string"),
	}
}

pub fn parse_color_matrix(src: &str) -> Result<[f64; 12], &str> {
	let matrix_vec = src
		.split(',')
		.collect::<Vec<&str>>()
		.iter()
		.map(|&e| e.parse::<f64>().expect("Cannot convert matrix element to float")) // TODO: this should return an Err() instead
		.collect::<Vec<f64>>();
	if matrix_vec.len() == 12 {
		// Convert matrix vector to array
		let mut matrix_arr = [0f64; 12];
		for (place, element) in matrix_arr.iter_mut().zip(matrix_vec.iter()) {
			*place = *element;
		}
		Ok(matrix_arr)
	} else {
		Err("Matrix length must be 12")
	}
}

// Parses "1.0", "0.9-1.0" into (1.0, 1.0), (0.9, 1.0)
pub fn parse_float_pair(src: &str) -> Result<(f64, f64), &str> {
	match src.find('-') {
		Some(_) => {
			// A pair
			let mut arr = src
				.split('-')
				.collect::<Vec<&str>>()
				.iter()
				.map(|&e| e.parse::<f64>().expect("Cannot convert matrix element to float")) // TODO: this should return an Err() instead
				.collect::<Vec<f64>>();
			if arr.len() == 2 {
				Ok((arr.remove(0), arr.remove(0)))
			} else {
				Err("Float range length must be 2")
			}
		}
		None => {
			// A single number
			let num = src.parse::<f64>().unwrap();
			Ok((num, num))
		}
	}
}

pub fn parse_size(src: &str) -> Result<SizeUnit, &str> {
	if src.ends_with("%") {
		match src[..src.len() - 1].parse::<f64>() {
			Ok(value) => Ok(SizeUnit::Fraction(value / 100.0f64)),
			_ => Err("Could not parse fraction value"),
		}
	} else {
		match src.parse::<f64>() {
			Ok(value) => Ok(SizeUnit::Pixels(value.round() as i32)),
			_ => Err("Could not parse pixel value"),
		}
	}
}

// Parses "100%", "90%-100%", "10-20", "2" into pairs of SizeUnits
pub fn parse_size_pair(src: &str) -> Result<(SizeUnit, SizeUnit), &str> {
	match src.find('-') {
		Some(_) => {
			// A pair
			let mut arr = src
				.split('-')
				.collect::<Vec<&str>>()
				.iter()
				.map(|&e| parse_size(e).expect("Cannot size element to unit")) // TODO: this should return an Err() instead
				.collect::<Vec<SizeUnit>>();
			if arr.len() == 2 {
				Ok((arr.remove(0), arr.remove(0)))
			} else {
				Err("Float range length must be 2")
			}
		}
		None => {
			// A single unit value
			let size = parse_size(src);
			match size {
				Ok(value) => Ok((value.clone(), value)),
				Err(error) => Err(error),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_color() {
		assert_eq!(parse_color("white"), Ok((255, 255, 255)));
		assert_eq!(parse_color("fff"), Ok((255, 255, 255)));
		assert_eq!(parse_color("ffffff"), Ok((255, 255, 255)));
		assert_eq!(parse_color("#ffffff"), Ok((255, 255, 255)));
		assert_eq!(parse_color("rgb(255, 255, 255)"), Ok((255, 255, 255)));
	}

	#[test]
	fn testparse_color_matrix() {
		assert_eq!(
			parse_color_matrix("1,2,3,4,5,6,7,8,9,10,11,12"),
			Ok([1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.])
		);
		assert_eq!(
			parse_color_matrix("1.1,2.2,3.3,4.4,5.5,6.6,7.7,8.8,9.9,10.0,11.1,12.2"),
			Ok([1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9, 10., 11.1, 12.2])
		);
	}

	#[test]
	fn test_parse_float_pair() {
		// Singles
		assert_eq!(parse_float_pair("0"), Ok((0.0f64, 0.0f64)));
		assert_eq!(parse_float_pair("0.0"), Ok((0.0f64, 0.0f64)));
		assert_eq!(parse_float_pair("0.5"), Ok((0.5f64, 0.5f64)));
		assert_eq!(parse_float_pair("1"), Ok((1.0f64, 1.0f64)));
		assert_eq!(parse_float_pair("1.0"), Ok((1.0f64, 1.0f64)));

		// Pairs
		assert_eq!(parse_float_pair("0-1"), Ok((0.0f64, 1.0f64)));
		assert_eq!(parse_float_pair("0.0-1"), Ok((0.0f64, 1.0f64)));
		assert_eq!(parse_float_pair("0-1.0"), Ok((0.0f64, 1.0f64)));
		assert_eq!(parse_float_pair("0.5-0.6"), Ok((0.5f64, 0.6f64)));
		assert_eq!(parse_float_pair("1-1"), Ok((1.0f64, 1.0f64)));
		assert_eq!(parse_float_pair("1.0-2.0"), Ok((1.0f64, 2.0f64)));
		assert_eq!(parse_float_pair("1-1.2"), Ok((1.0f64, 1.2f64)));
	}

	#[test]
	fn test_parse_size() {
		// Fraction
		assert_eq!(parse_size("0%"), Ok(SizeUnit::Fraction(0.0)));
		assert_eq!(parse_size("10%"), Ok(SizeUnit::Fraction(0.1)));
		assert_eq!(parse_size("100%"), Ok(SizeUnit::Fraction(1.0)));
		assert_eq!(parse_size("2000%"), Ok(SizeUnit::Fraction(20.0)));

		// Pixels
		assert_eq!(parse_size("0"), Ok(SizeUnit::Pixels(0)));
		assert_eq!(parse_size("0.1"), Ok(SizeUnit::Pixels(0)));
		assert_eq!(parse_size("190.01"), Ok(SizeUnit::Pixels(190)));
		assert_eq!(parse_size("333190.01"), Ok(SizeUnit::Pixels(333190)));
	}

	#[test]
	fn test_parse_size_pair() {
		// Singles, fraction
		assert_eq!(parse_size_pair("0%"), Ok((SizeUnit::Fraction(0.0), SizeUnit::Fraction(0.0))));
		assert_eq!(parse_size_pair("0.0%"), Ok((SizeUnit::Fraction(0.0), SizeUnit::Fraction(0.0))));
		assert_eq!(parse_size_pair("2.5%"), Ok((SizeUnit::Fraction(0.025), SizeUnit::Fraction(0.025))));
		assert_eq!(parse_size_pair("50%"), Ok((SizeUnit::Fraction(0.5), SizeUnit::Fraction(0.5))));
		assert_eq!(parse_size_pair("100%"), Ok((SizeUnit::Fraction(1.0), SizeUnit::Fraction(1.0))));

		// Singles, pixels
		assert_eq!(parse_size_pair("0"), Ok((SizeUnit::Pixels(0), SizeUnit::Pixels(0))));
		assert_eq!(parse_size_pair("0.0"), Ok((SizeUnit::Pixels(0), SizeUnit::Pixels(0))));
		assert_eq!(parse_size_pair("2.5"), Ok((SizeUnit::Pixels(3), SizeUnit::Pixels(3))));
		assert_eq!(parse_size_pair("100"), Ok((SizeUnit::Pixels(100), SizeUnit::Pixels(100))));

		// Pairs, fraction
		assert_eq!(parse_size_pair("0%-100%"), Ok((SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0))));
		assert_eq!(parse_size_pair("0.0%-100%"), Ok((SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0))));
		assert_eq!(parse_size_pair("0%-100.0%"), Ok((SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0))));
		assert_eq!(parse_size_pair("50%-60%"), Ok((SizeUnit::Fraction(0.5), SizeUnit::Fraction(0.6))));
		assert_eq!(parse_size_pair("100%-100%"), Ok((SizeUnit::Fraction(1.0), SizeUnit::Fraction(1.0))));
		assert_eq!(parse_size_pair("100%-200%"), Ok((SizeUnit::Fraction(1.0), SizeUnit::Fraction(2.0))));
		assert_eq!(parse_size_pair("100%-120%"), Ok((SizeUnit::Fraction(1.0), SizeUnit::Fraction(1.2))));

		// Pairs, pixels
		assert_eq!(parse_size_pair("0-100"), Ok((SizeUnit::Pixels(0), SizeUnit::Pixels(100))));
		assert_eq!(parse_size_pair("0.0-100"), Ok((SizeUnit::Pixels(0), SizeUnit::Pixels(100))));
		assert_eq!(parse_size_pair("0-100.0"), Ok((SizeUnit::Pixels(0), SizeUnit::Pixels(100))));
		assert_eq!(parse_size_pair("50.1-60"), Ok((SizeUnit::Pixels(50), SizeUnit::Pixels(60))));
		assert_eq!(parse_size_pair("100-100"), Ok((SizeUnit::Pixels(100), SizeUnit::Pixels(100))));
		assert_eq!(parse_size_pair("100-20000"), Ok((SizeUnit::Pixels(100), SizeUnit::Pixels(20000))));
		assert_eq!(parse_size_pair("100-120"), Ok((SizeUnit::Pixels(100), SizeUnit::Pixels(120))));
	}
}
