use color_processing::Color;

pub fn parse_color(src: &str) -> Result<(u8, u8, u8), &str> {
	match Color::new_string(src) {
		Some(color) => {
			let rgb = color.get_rgba();
			let r = (rgb.0 * 255.0).round() as u8;
			let g = (rgb.1 * 255.0).round() as u8;
			let b = (rgb.2 * 255.0).round() as u8;
			Ok((r, g, b))
		},
		None => {
			Err("Cannot parse color string")
		}
	}
}

pub fn parse_color_matrix(src: &str) -> Result<[f64; 12], &str> {
	let matrix_vec = src
		.split(',')
		.collect::<Vec<&str>>()
		.iter()
		.map(|&e| e.parse::<f64>()
			.expect("Cannot convert matrix element to float")) // TODO: this should return an Err() instead
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