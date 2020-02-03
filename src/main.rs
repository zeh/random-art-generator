use std::path::PathBuf;
use structopt::StructOpt;

use image::GenericImageView;

use generator::{Generator};
use generator::painter::{Painter, RectPainter};

mod generator;

/// Progressively generate an image based on a target
#[derive(Debug, StructOpt)]
struct Opt {
	/// Number of iterations to run
	#[structopt(long, default_value = "10")]
	iterations: u32,

	/// The target image
	#[structopt(short, long, parse(from_os_str))]
	target: PathBuf,

	/// The output image filename
	#[structopt(short, long, default_value = "output.png", parse(from_os_str))]
	output: PathBuf,

	/// The input image filename, if any
	#[structopt(short, long, default_value = "", parse(from_os_str))]
	input: PathBuf,

	/// A 3x4 color matrix (as a comma-separated number list) to be applied to the target image in the format "r_from_r,r_from_g,r_from_b,r_offset,g_from_r,g_from_b,...". Identity is "1,0,0,0,0,1,0,0,0,0,1,0"
	#[structopt(long)]
	target_color_matrix: Option<String>,
}

fn get_options() -> Opt {
	return Opt::from_args();
}

fn on_iteration(generator: &Generator, success: bool) {
	if success {
		// TODO: a bit repetitive, investigate how to add properties to callbacks
		let options = get_options();
		let output_file = options.output.as_path();
		generator.get_current().save(output_file)
			.expect("Cannot write to output file {:?}, exiting");
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
		Some(m) => {
			// Target has a color matrix, parse it first
			let matrix_vec = m
				.split(',')
				.collect::<Vec<&str>>()
				.iter()
				.map(|&e| e.parse::<f64>()
				.expect("Cannot convert matrix to numbers"))
				.collect::<Vec<f64>>();
			assert_eq!(matrix_vec.len(), 12, "Matrix length must be 12");

			// Convert matrix vector to array
			let mut matrix_arr = [0f64; 12];
			for (place, element) in matrix_arr.iter_mut().zip(matrix_vec.iter()) {
				*place = *element;
			}

			// Finally, generate it with the matrix
			generator::Generator::from_image_and_matrix(target_image, matrix_arr)
		},
		None => {
			// No color matrix needed, generate with the image
			generator::Generator::from_image(target_image)
		},
	};

	// Set input
	match options.input.to_str() {
		Some(input_str) => {
			if input_str.len() > 0 {
				let input_file = options.input.as_path();
				let input_image = image::open(input_file)
					.expect("Cannot open input file {:?}, exiting");

				println!("Using input image of {:?} with dimensions of {:?}.", input_file, input_image.dimensions());

				gen.prepopulate(input_image);
			}
		},
		None => {},
	}

	// Set output
	let output_file = options.output.as_path();
	println!("Using output image of {:?}.", output_file);

	// Process everything
	let painter = RectPainter::new();
	gen.process(options.iterations, painter, Some(on_iteration));
	gen.get_current().save(output_file)
		.expect("Cannot write to output file {:?}, exiting");
}
