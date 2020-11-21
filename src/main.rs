use std::path::PathBuf;
use structopt::StructOpt;

use image::GenericImageView;

use generator::painter::circle::CirclePainter;
use generator::painter::rect::RectPainter;
use generator::painter::stroke::StrokePainter;
use generator::utils::parsing::{parse_color, parse_color_matrix, parse_float_pair, parse_size_pair};
use generator::utils::units::SizeUnit;
use generator::Generator;

mod generator;

/// Progressively generate an image based on a target
#[derive(Debug, StructOpt)]
struct Opt {
	/// The target image
	#[structopt(parse(from_os_str))]
	target: PathBuf,

	/// Integer; maximum number of iterations (successful or nor) to run (0 = no maximum)
	#[structopt(short = "t", long, default_value = "0", required_if("generations", "0"))]
	max_tries: u32,

	/// Integer; minimum number of generations (successful tries) required (0 = no minimum)
	#[structopt(short, long, default_value = "0", required_if("max_tries", "0"))]
	generations: u32,

	/// String; the output image filename
	#[structopt(short, long, default_value = "output.png", parse(from_os_str))]
	output: PathBuf,

	/// String; the input image filename, if any
	#[structopt(short, long, parse(from_os_str))]
	input: Option<PathBuf>,

	/// The color to be used as the default background for the new image, as a string in the typical HTML color formats.
	///
	/// Some examples of valid parameters:
	///
	/// * `white`
	/// * `'#ff0'`
	/// * `'#4C4C4C'`
	/// * `'rgb(76, 76, 76)'`
	/// * `'cmyk(0%, 0%, 0%, 70%)'`
	/// * `'hsl(0, 0%, 29.8%)'`
	///
	/// Notice that in some cases, the terminal might have trouble with parameters starting with the character `#` or containing spaces,
	/// hence why quotes might be required for the value.
	///
	/// Additionally, to pass hexadecimal arguments, the following syntax also works:
	///
	/// * `ff0`
	/// * `4C4C4C`
	///
	/// This argument is parsed by the [color_processing](https://docs.rs/color_processing) crate.
	#[structopt(long, default_value = "000000", parse(try_from_str = parse_color))]
	background_color: (u8, u8, u8),

	/// Comma-separated number array; a 3x4 color matrix to be applied to the target image
	///
	/// This is in the format "r_from_r,r_from_g,r_from_b,r_offset,g_from_r,g_from_b,...". For example:
	/// * Identity is "1,0,0,0,0,1,0,0,0,0,1,0"
	/// * Grayscale is "0.33,0.59,0.11,0,0.33,0.59,0.11,0,0.33,0.59,0.11,0"
	/// * Sepia is "0.393,0.769,0.686,0,0.349,0.686,0.168,0,0.272,0.534,0.131,0"
	/// * Polaroid is "1.438,0.122,-0.016,-8,-0.062,1.378,-0.016,-13,-0.062,-0.122,1.483,-5"
	#[structopt(long, parse(try_from_str = parse_color_matrix))]
	target_color_matrix: Option<[f64; 12]>,

	/// Number; the new size of the output image, as a scale of the target image
	#[structopt(short, long, default_value = "1")]
	scale: f64,

	/// String; painter to be used ("circles", "strokes", "rects")
	#[structopt(short, long, possible_values = &["circles", "strokes", "rects"], default_value = "rects")]
	painter: String,

	/// List of number ranges; the alphas to be used at random. Examples: "1.0", "0.1", "0.1-0.2", "0.1-0.2 0.3 0.5 0.9-1.0"
	#[structopt(long, default_value = "1", parse(try_from_str = parse_float_pair))]
	painter_alpha: Vec<(f64, f64)>,

	/// List of size ranges; the radius when applicable
	#[structopt(long, default_value = "0%-50%", parse(try_from_str = parse_size_pair))]
	painter_radius: Vec<(SizeUnit, SizeUnit)>,

	/// Number; bias for radius (0.0 = normal, -1.0 = quad bias towards small, 1.0 = quad bias towards large)
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_radius_bias: f64,

	/// List of size ranges; width when applicable
	#[structopt(long, default_value = "0%-100%", parse(try_from_str = parse_size_pair))]
	painter_width: Vec<(SizeUnit, SizeUnit)>,

	/// Number; bias for width (0.0 = normal, -1.0 = quad bias towards small, 1.0 = quad bias towards large)
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_width_bias: f64,

	/// List of size ranges; height when applicable
	#[structopt(long, default_value = "0%-100%", parse(try_from_str = parse_size_pair))]
	painter_height: Vec<(SizeUnit, SizeUnit)>,

	/// Number; bias for height (0.0 = normal, -1.0 = quad bias towards small, 1.0 = quad bias towards large)
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_height_bias: f64,

	/// Flag; disables anti-alias where possible
	#[structopt(long)]
	painter_disable_anti_alias: bool,

	/// List of size ranges; waviness when applicable
	#[structopt(long, default_value = "0.5%", parse(try_from_str = parse_size_pair))]
	painter_wave_height: Vec<(SizeUnit, SizeUnit)>,

	/// Number; bias for waviness (0.0 = normal, -1.0 = quad bias towards small, 1.0 = quad bias towards large)
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_wave_height_bias: f64,

	/// List of size ranges; waviness when applicable
	#[structopt(long, default_value = "400%", parse(try_from_str = parse_size_pair))]
	painter_wave_length: Vec<(SizeUnit, SizeUnit)>,

	/// Number; bias for waviness (0.0 = normal, -1.0 = quad bias towards small, 1.0 = quad bias towards large)
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_wave_length_bias: f64,
}

fn get_options() -> Opt {
	return Opt::from_args();
}

fn on_tried(generator: &Generator, success: bool) {
	if success {
		// TODO: a bit repetitive, investigate how to add properties to callbacks
		let options = get_options();
		let output_file = options.output.as_path();
		generator.get_current().save(output_file).expect("Cannot write to output file {:?}, exiting");
	}
}

fn main() {
	let options = get_options();

	// Target
	let target_file = options.target.as_path();
	let target_image = image::open(target_file).expect("Cannot open target file {:?}, exiting");

	println!("Using target image of {:?} with dimensions of {:?}.", target_file, target_image.dimensions());

	// Create Generator
	let mut gen = match options.target_color_matrix {
		Some(color_matrix) => {
			// Target has a color matrix, parse it first
			generator::Generator::from_image_and_matrix(target_image, options.scale, color_matrix)
		}
		None => {
			// No color matrix needed, generate with the image
			generator::Generator::from_image(target_image, options.scale)
		}
	};

	// Set input
	match options.input {
		Some(input) => {
			let input_file = input.as_path();
			let input_image = image::open(input_file).expect("Cannot open input file {:?}, exiting");

			println!(
				"Using input image of {:?} with dimensions of {:?}.",
				input_file,
				input_image.dimensions()
			);

			gen.prepopulate_with_image(input_image);
		}
		None => {
			let color = options.background_color;
			gen.prepopulate_with_color(color.0, color.1, color.2);
		}
	}

	// Set output
	let output_file = options.output.as_path();
	println!("Using output image of {:?}.", output_file);

	// Process everything
	// TODO: use actual enums here and use a single object from trait (can't seen to make it work)
	// TODO: error out on passed painter options that are unused?
	match &options.painter[..] {
		"circles" => {
			let mut painter = CirclePainter::new();
			painter.options.alpha = options.painter_alpha;
			painter.options.radius = options.painter_radius;
			painter.options.radius_bias = options.painter_radius_bias;
			painter.options.anti_alias = !options.painter_disable_anti_alias;
			gen.process(options.max_tries, options.generations, painter, Some(on_tried));
		}
		"rects" => {
			let mut painter = RectPainter::new();
			painter.options.alpha = options.painter_alpha;
			painter.options.width = options.painter_width;
			painter.options.width_bias = options.painter_width_bias;
			painter.options.height = options.painter_height;
			painter.options.height_bias = options.painter_height_bias;
			gen.process(options.max_tries, options.generations, painter, Some(on_tried));
		}
		"strokes" => {
			let mut painter = StrokePainter::new();
			painter.options.alpha = options.painter_alpha;
			painter.options.width = options.painter_width;
			painter.options.width_bias = options.painter_width_bias;
			painter.options.height = options.painter_height;
			painter.options.height_bias = options.painter_height_bias;
			painter.options.wave_height = options.painter_wave_height;
			painter.options.wave_height_bias = options.painter_wave_height_bias;
			painter.options.wave_length = options.painter_wave_length;
			painter.options.wave_length_bias = options.painter_wave_length_bias;
			painter.options.anti_alias = !options.painter_disable_anti_alias;
			gen.process(options.max_tries, options.generations, painter, Some(on_tried));
		}
		_ => unreachable!(),
	}
	gen.get_current().save(output_file).expect("Cannot write to output file {:?}, exiting");
}
