use std::convert::TryInto;

use color_processing::Color;

use crate::generator::utils::units::{Margins, SizeUnit, WeightedValue};

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

pub fn parse_float(src: &str) -> Result<f64, &str> {
	src.parse::<f64>().or(Err("Could not parse float value"))
}

pub fn parse_float_list(src: &str, divider: char) -> Result<Vec<f64>, &str> {
	src.split(divider).collect::<Vec<&str>>().iter().map(|&e| parse_float(e)).collect()
}

// Parses "1.0", "0.9-1.0" into (1.0, 1.0), (0.9, 1.0)
pub fn parse_float_pair(src: &str) -> Result<(f64, f64), &str> {
	let values = parse_float_list(&src, '-')?;
	match values.len() {
		1 => Ok((values[0], values[0])),
		2 => Ok((values[0], values[1])),
		_ => Err("Float range must be 1-2"),
	}
}

pub fn parse_scale(src: &str) -> Result<f64, &str> {
	if src.ends_with("%") {
		match src[..src.len() - 1].parse::<f64>() {
			Ok(value) => Ok(value / 100.0),
			_ => Err("Could not parse scale percent value"),
		}
	} else {
		match src.parse::<f64>() {
			Ok(value) => Ok(value),
			_ => Err("Could not parse scale float value"),
		}
	}
}

pub fn parse_color_matrix(src: &str) -> Result<[f64; 12], &str> {
	let values = parse_float_list(&src, ',')?;
	match values.len() {
		12 => values.try_into().or(Err("Could not convert float list")) as Result<[f64; 12], &str>,
		_ => Err("Matrix length must be 12"),
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
			Ok(value) => Ok(SizeUnit::Pixels(value.round() as i64)),
			_ => Err("Could not parse pixel value"),
		}
	}
}

pub fn parse_size_list(src: &str, divider: char) -> Result<Vec<SizeUnit>, &str> {
	src.split(divider).collect::<Vec<&str>>().iter().map(|&e| parse_size(e)).collect()
}

// Parses "100%", "90%-100%", "10-20", "2" into pairs of SizeUnits
pub fn parse_size_pair(src: &str) -> Result<(SizeUnit, SizeUnit), &str> {
	let values = parse_size_list(&src, '-')?;
	match values.len() {
		1 => Ok((values[0].clone(), values[0].clone())),
		2 => Ok((values[0].clone(), values[1].clone())),
		_ => Err("Size range length must be 2"),
	}
}

pub fn parse_size_margins(src: &str) -> Result<Margins<SizeUnit>, &str> {
	let values = parse_size_list(src, ',')?;
	match values.len() {
		1 => Ok(Margins::<SizeUnit> {
			top: values[0].clone(),
			right: values[0].clone(),
			bottom: values[0].clone(),
			left: values[0].clone(),
		}),
		2 => Ok(Margins::<SizeUnit> {
			top: values[0].clone(),
			right: values[1].clone(),
			bottom: values[0].clone(),
			left: values[1].clone(),
		}),
		3 => Ok(Margins::<SizeUnit> {
			top: values[0].clone(),
			right: values[1].clone(),
			bottom: values[2].clone(),
			left: values[1].clone(),
		}),
		4 => Ok(Margins::<SizeUnit> {
			top: values[0].clone(),
			right: values[1].clone(),
			bottom: values[2].clone(),
			left: values[3].clone(),
		}),
		_ => Err("Margin list length must be 1-4"),
	}
}

/// Parses "*@n" into a string "*" with n weight. This is used so we can have pairs with weights.
pub fn parse_weight(src: &str) -> Result<(&str, f64), &str> {
	let values = src.split('@').collect::<Vec<&str>>();
	match values.len() {
		1 => Ok((src, 1.0)),
		2 => match parse_float(values[1]) {
			Ok(val) => Ok((values[0].clone(), val)),
			Err(err) => Err(err),
		},
		_ => Err("Value cannot contain more than one weight value"),
	}
}

/// Parses a size pair with a weight (e.f. "1-2@1", "10%", "5-10%@2") into a WeightedValue<>
pub fn parse_weighted_size_pair(src: &str) -> Result<WeightedValue<(SizeUnit, SizeUnit)>, &str> {
	match parse_weight(src) {
		Ok((src_pair, weight)) => match parse_size_pair(src_pair) {
			Ok(value) => Ok(WeightedValue {
				value,
				weight,
			}),
			Err(err) => Err(err),
		},
		Err(err) => Err(err),
	}
}

/// Parses a float pair with a weight (e.f. "1-2@1", "10.2", "5.2-10@2") into a WeightedValue<>
pub fn parse_weighted_float_pair(src: &str) -> Result<WeightedValue<(f64, f64)>, &str> {
	match parse_weight(src) {
		Ok((src_pair, weight)) => match parse_float_pair(src_pair) {
			Ok(value) => Ok(WeightedValue {
				value,
				weight,
			}),
			Err(err) => Err(err),
		},
		Err(err) => Err(err),
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

		// Errors
		assert!(parse_color("").is_err());
		assert!(parse_color("foo").is_err());
	}

	#[test]
	fn test_parse_float() {
		assert_eq!(parse_float("0"), Ok(0.0f64));
		assert_eq!(parse_float("0.0"), Ok(0.0f64));
		assert_eq!(parse_float("0.5"), Ok(0.5f64));
		assert_eq!(parse_float("1"), Ok(1.0f64));
		assert_eq!(parse_float("13.244"), Ok(13.244f64));

		// Errors
		assert!(parse_float("").is_err());
		assert!(parse_float("foo").is_err());
	}

	#[test]
	fn test_parse_float_list() {
		assert_eq!(parse_float_list("0", ','), Ok(vec![0.0f64]));
		assert_eq!(parse_float_list("0,-2", ','), Ok(vec![0.0f64, -2.0f64]));
		assert_eq!(parse_float_list("0.0,-647,245.2,1", ','), Ok(vec![0.0f64, -647.0f64, 245.2f64, 1.0f64]));
		assert_eq!(parse_float_list("1-2-7.55", '-'), Ok(vec![1.0f64, 2.0f64, 7.55f64]));

		// Errors
		assert!(parse_float_list("", ',').is_err());
		assert!(parse_float_list("foo", ',').is_err());
		assert!(parse_float_list("1,2,foo", ',').is_err());
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

		// Errors
		assert!(parse_float_pair("").is_err());
		assert!(parse_float_pair("foo").is_err());
		assert!(parse_float_pair("1-foo").is_err());
		assert!(parse_float_pair("1-2-3").is_err());
	}

	#[test]
	fn test_parse_scale() {
		assert_eq!(parse_scale("0"), Ok(0.0));
		assert_eq!(parse_scale("0.0"), Ok(0.0));
		assert_eq!(parse_scale("0.5"), Ok(0.5));
		assert_eq!(parse_scale("1"), Ok(1.0));
		assert_eq!(parse_scale("13.244"), Ok(13.244));
		assert_eq!(parse_scale("0%"), Ok(0.0));
		assert_eq!(parse_scale("0.0%"), Ok(0.0));
		assert_eq!(parse_scale("0.5%"), Ok(0.005));
		assert_eq!(parse_scale("10%"), Ok(0.1));
		assert_eq!(parse_scale("50%"), Ok(0.5));
		assert_eq!(parse_scale("100%"), Ok(1.0));
		assert_eq!(parse_scale("133.24%"), Ok(1.3324));

		// Errors
		assert!(parse_float("").is_err());
		assert!(parse_float("%").is_err());
		assert!(parse_float("foo").is_err());
	}

	#[test]
	fn test_parse_color_matrix() {
		assert_eq!(
			parse_color_matrix("1,2,3,4,5,6,7,8,9,10,11,12"),
			Ok([1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.])
		);
		assert_eq!(
			parse_color_matrix("-1,2,3,4,5,6,-7,8,9,10,11,-12"),
			Ok([-1., 2., 3., 4., 5., 6., -7., 8., 9., 10., 11., -12.])
		);
		assert_eq!(
			parse_color_matrix("1.1,2.2,-3.3,4.4,5.5,6.6,7.7,8.8,9.9,10.0,-11.1,12.2"),
			Ok([1.1, 2.2, -3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9, 10., -11.1, 12.2])
		);

		// Errors
		assert!(parse_color_matrix("").is_err());
		assert!(parse_color_matrix("foo").is_err());
		assert!(parse_color_matrix("0.5,3,foo").is_err());
		assert!(parse_color_matrix("1,2,3,4").is_err());
	}

	#[test]
	fn test_parse_size() {
		// Fraction
		assert_eq!(parse_size("0%"), Ok(SizeUnit::Fraction(0.0)));
		assert_eq!(parse_size("10%"), Ok(SizeUnit::Fraction(0.1)));
		assert_eq!(parse_size("100%"), Ok(SizeUnit::Fraction(1.0)));
		assert_eq!(parse_size("2000%"), Ok(SizeUnit::Fraction(20.0)));
		assert_eq!(parse_size("-150%"), Ok(SizeUnit::Fraction(-1.5)));

		// Pixels
		assert_eq!(parse_size("0"), Ok(SizeUnit::Pixels(0)));
		assert_eq!(parse_size("0.1"), Ok(SizeUnit::Pixels(0)));
		assert_eq!(parse_size("-0.2"), Ok(SizeUnit::Pixels(0)));
		assert_eq!(parse_size("-9.1"), Ok(SizeUnit::Pixels(-9)));
		assert_eq!(parse_size("190.01"), Ok(SizeUnit::Pixels(190)));
		assert_eq!(parse_size("333190.01"), Ok(SizeUnit::Pixels(333190)));

		// Errors
		assert!(parse_size("").is_err());
		assert!(parse_size("foo").is_err());
	}

	#[test]
	fn test_parse_size_list() {
		assert_eq!(parse_size_list("0%", ','), Ok(vec![SizeUnit::Fraction(0.0)]));
		assert_eq!(parse_size_list("0%,2.5", ','), Ok(vec![SizeUnit::Fraction(0.0), SizeUnit::Pixels(3)]));
		assert_eq!(
			parse_size_list("10%-25.2-156.8%-140", '-'),
			Ok(vec![
				SizeUnit::Fraction(0.1),
				SizeUnit::Pixels(25),
				SizeUnit::Fraction(1.568),
				SizeUnit::Pixels(140)
			])
		);

		// Errors
		assert!(parse_size_list("", ',').is_err());
		assert!(parse_size_list("1,foo", ',').is_err());
		assert!(parse_size_list("bar", ',').is_err());
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

		// Mixed
		assert_eq!(parse_size_pair("12.3-60%"), Ok((SizeUnit::Pixels(12), SizeUnit::Fraction(0.6))));
		assert_eq!(parse_size_pair("2.3%-1230"), Ok((SizeUnit::Fraction(0.023), SizeUnit::Pixels(1230))));

		// Errors
		assert!(parse_size_pair("").is_err());
		assert!(parse_size_pair("foo").is_err());
		assert!(parse_size_pair("0-100-20").is_err());
	}

	#[test]
	fn test_parse_size_margins() {
		let u1pc = SizeUnit::Fraction(0.01);
		let u50pc = SizeUnit::Fraction(0.5);
		let u173_292pc = SizeUnit::Fraction(173.292);
		let u10px = SizeUnit::Pixels(10);
		let u55px = SizeUnit::Pixels(55);

		// All
		assert_eq!(
			parse_size_margins("1%"),
			Ok(Margins::<SizeUnit> {
				top: u1pc.clone(),
				right: u1pc.clone(),
				bottom: u1pc.clone(),
				left: u1pc.clone(),
			})
		);

		// V, H
		assert_eq!(
			parse_size_margins("50%,55"),
			Ok(Margins::<SizeUnit> {
				top: u50pc.clone(),
				right: u55px.clone(),
				bottom: u50pc.clone(),
				left: u55px.clone(),
			})
		);

		// T, H, B
		assert_eq!(
			parse_size_margins("10,17329.2%,1%"),
			Ok(Margins::<SizeUnit> {
				top: u10px.clone(),
				right: u173_292pc.clone(),
				bottom: u1pc.clone(),
				left: u173_292pc.clone(),
			})
		);

		// T, R, B, L
		assert_eq!(
			parse_size_margins("55,50%,1%,17329.2%"),
			Ok(Margins::<SizeUnit> {
				top: u55px.clone(),
				right: u50pc.clone(),
				bottom: u1pc.clone(),
				left: u173_292pc.clone(),
			})
		);

		// Errors
		assert!(parse_size_margins("").is_err());
		assert!(parse_size_margins("foo").is_err());
		assert!(parse_size_margins("1,2,3,4,5").is_err());
	}

	#[test]
	fn test_parse_weight() {
		assert_eq!(parse_weight(""), Ok(("", 1.0)));
		assert_eq!(parse_weight("0"), Ok(("0", 1.0)));
		assert_eq!(parse_weight("foo"), Ok(("foo", 1.0)));
		assert_eq!(parse_weight("0.2@2"), Ok(("0.2", 2.0)));
		assert_eq!(parse_weight("bar@100"), Ok(("bar", 100.0)));
		assert_eq!(parse_weight("50%@2000.57"), Ok(("50%", 2000.57)));
		assert_eq!(parse_weight("10-20%@0.123"), Ok(("10-20%", 0.123)));
		assert_eq!(parse_weight("-5.5@0.333"), Ok(("-5.5", 0.333)));

		// Errors
		assert!(parse_weight("1@1@1").is_err());
		assert!(parse_weight("2@1/1").is_err());
		assert!(parse_weight("3@a").is_err());
	}

	#[test]
	fn test_parse_weighted_size_pair() {
		assert_eq!(
			parse_weighted_size_pair("0%"),
			Ok(WeightedValue {
				value: (SizeUnit::Fraction(0.0), SizeUnit::Fraction(0.0)),
				weight: 1.0
			}),
		);
		assert_eq!(
			parse_weighted_size_pair("5"),
			Ok(WeightedValue {
				value: (SizeUnit::Pixels(5), SizeUnit::Pixels(5)),
				weight: 1.0
			}),
		);
		assert_eq!(
			parse_weighted_size_pair("0.0%-100%@2"),
			Ok(WeightedValue {
				value: (SizeUnit::Fraction(0.0), SizeUnit::Fraction(1.0)),
				weight: 2.0
			}),
		);
		assert_eq!(
			parse_weighted_size_pair("50.1-60%@5.5"),
			Ok(WeightedValue {
				value: (SizeUnit::Pixels(50), SizeUnit::Fraction(0.6)),
				weight: 5.5
			}),
		);
		assert_eq!(
			parse_weighted_size_pair("50%@100"),
			Ok(WeightedValue {
				value: (SizeUnit::Fraction(0.5), SizeUnit::Fraction(0.5)),
				weight: 100.0
			}),
		);
		assert_eq!(
			parse_weighted_size_pair("100%@1337"),
			Ok(WeightedValue {
				value: (SizeUnit::Fraction(1.0), SizeUnit::Fraction(1.0)),
				weight: 1337.0
			}),
		);
		assert_eq!(
			parse_weighted_size_pair("0-122@15.5"),
			Ok(WeightedValue {
				value: (SizeUnit::Pixels(0), SizeUnit::Pixels(122)),
				weight: 15.5
			}),
		);
		assert_eq!(
			parse_weighted_size_pair("1%-200@2"),
			Ok(WeightedValue {
				value: (SizeUnit::Fraction(0.01), SizeUnit::Pixels(200)),
				weight: 2.0
			}),
		);
		assert_eq!(
			parse_weighted_size_pair("6-15.56@3.9"),
			Ok(WeightedValue {
				value: (SizeUnit::Pixels(6), SizeUnit::Pixels(16)),
				weight: 3.9
			}),
		);
		assert_eq!(
			parse_weighted_size_pair("9-12%"),
			Ok(WeightedValue {
				value: (SizeUnit::Pixels(9), SizeUnit::Fraction(0.12)),
				weight: 1.0
			}),
		);
		assert_eq!(
			parse_weighted_size_pair("10%-2000.2"),
			Ok(WeightedValue {
				value: (SizeUnit::Fraction(0.1), SizeUnit::Pixels(2000)),
				weight: 1.0
			}),
		);

		// Errors
		assert!(parse_size_pair("@1").is_err());
		assert!(parse_size_pair("foo@2").is_err());
		assert!(parse_size_pair("0-100-20@1").is_err());
		assert!(parse_size_pair("0-100@1@2").is_err());
		assert!(parse_size_pair("0.2@1_2").is_err());
		assert!(parse_size_pair("10%@a").is_err());
	}

	#[test]
	fn test_parse_weighted_float_pair() {
		assert_eq!(
			parse_weighted_float_pair("0"),
			Ok(WeightedValue {
				value: (0.0, 0.0),
				weight: 1.0
			}),
		);
		assert_eq!(
			parse_weighted_float_pair("5"),
			Ok(WeightedValue {
				value: (5.0, 5.0),
				weight: 1.0
			}),
		);
		assert_eq!(
			parse_weighted_float_pair("0.0-100@2"),
			Ok(WeightedValue {
				value: (0.0, 100.0),
				weight: 2.0
			}),
		);
		assert_eq!(
			parse_weighted_float_pair("50.1-60@5.5"),
			Ok(WeightedValue {
				value: (50.1, 60.0),
				weight: 5.5
			}),
		);
		assert_eq!(
			parse_weighted_float_pair("1-1.2@100"),
			Ok(WeightedValue {
				value: (1.0, 1.2),
				weight: 100.0
			}),
		);

		// Errors
		assert!(parse_weighted_float_pair("").is_err());
		assert!(parse_weighted_float_pair("foo").is_err());
		assert!(parse_weighted_float_pair("1-2-3").is_err());
		assert!(parse_weighted_float_pair("@1").is_err());
		assert!(parse_weighted_float_pair("foo@2").is_err());
		assert!(parse_weighted_float_pair("0-100-20@1").is_err());
		assert!(parse_weighted_float_pair("0-100@1@2").is_err());
		assert!(parse_weighted_float_pair("0.2@1_2").is_err());
		assert!(parse_weighted_float_pair("10%@a").is_err());
	}
}
