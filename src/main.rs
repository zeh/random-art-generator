use std::path::PathBuf;
use structopt::StructOpt;

use color_processing::Color;
use image::GenericImageView;

use generator::{Generator};
use generator::painter::{Painter, RectPainter};

mod generator;

/// Progressively generate an image based on a target
#[derive(Debug, StructOpt)]
struct Opt {
	/// Minimum number of iterations (successful or nor) to run (0 = no minimum)
	#[structopt(short, long, default_value = "0", required_if("generations", "0"))]
	attempts: u32,

	/// Minimum number of generations (successful attempts) required (0 = no minimum)
	#[structopt(short, long, default_value = "0", required_if("attempts", "0"))]
	generations: u32,

	/// The target image
	#[structopt(short, long, parse(from_os_str))]
	target: PathBuf,

	/// The output image filename
	#[structopt(short, long, default_value = "output.png", parse(from_os_str))]
	output: PathBuf,

	/// The input image filename, if any
	#[structopt(short, long, parse(from_os_str))]
	input: Option<PathBuf>,

	/// The starting background color, if any, in hex rrggbb format
	#[structopt(long, default_value = "000000", parse(try_from_str = parse_color))]
	background_color: (u8, u8, u8),

	/// A 3x4 color matrix to be applied to the target image, as a comma-separated number list
	///
	/// This is in the format "r_from_r,r_from_g,r_from_b,r_offset,g_from_r,g_from_b,...". For example:
	/// * Identity is "1,0,0,0,0,1,0,0,0,0,1,0"
	/// * Grayscale is "0.33,0.33,0.33,0,0.59,0.59,0.59,0,0.11,0.11,0.11,0" (untested)
	/// * Sepia is "0.393,0.769,0.686,0,0.349,0.686,0.168,0,0.272,0.534,0.131,0" (untested)
	/// * Polaroid is "1.438,0.122,-0.016,-8,-0.062,1.378,-0.016,-13,-0.062,-0.122,1.483,-5" (untested)
	#[structopt(long, parse(try_from_str = parse_color_matrix))]
	target_color_matrix: Option<[f64; 12]>,
}

fn get_options() -> Opt {
	return Opt::from_args();
}

fn on_attempt(generator: &Generator, success: bool) {
	if success {
		// TODO: a bit repetitive, investigate how to add properties to callbacks
		let options = get_options();
		let output_file = options.output.as_path();
		generator.get_current().save(output_file)
			.expect("Cannot write to output file {:?}, exiting");
	}
}

fn parse_color(src: &str) -> Result<(u8, u8, u8), &str> {
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

fn parse_color_matrix(src: &str) -> Result<[f64; 12], &str> {
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

fn main() {
	let options = get_options();

	// Target
	let target_file = options.target.as_path();
	let target_image = image::open(target_file)
		.expect("Cannot open target file {:?}, exiting");

	println!("Using target image of {:?} with dimensions of {:?}.", target_file, target_image.dimensions());

	// Create Generator
	let mut gen = match options.target_color_matrix {
		Some(color_matrix) => {
			// Target has a color matrix, parse it first
			generator::Generator::from_image_and_matrix(target_image, color_matrix)
		},
		None => {
			// No color matrix needed, generate with the image
			generator::Generator::from_image(target_image)
		},
	};

	// Set input
	match options.input {
		Some(input) => {
			let input_file = input.as_path();
			let input_image = image::open(input_file)
				.expect("Cannot open input file {:?}, exiting");

			println!("Using input image of {:?} with dimensions of {:?}.", input_file, input_image.dimensions());

			gen.prepopulate_with_image(input_image);
		},
		None => {
			let color = options.background_color;
			gen.prepopulate_with_color(color.0, color.1, color.2);
		},
	}

	// Set output
	let output_file = options.output.as_path();
	println!("Using output image of {:?}.", output_file);

	// Process everything
	let painter = RectPainter::new();
	gen.process(options.attempts, options.generations, painter, Some(on_attempt));
	gen.get_current().save(output_file)
		.expect("Cannot write to output file {:?}, exiting");
}
