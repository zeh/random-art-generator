use std::env;
use std::path::PathBuf;
use std::string::ToString;

use image::GenericImageView;
use structopt::StructOpt;

use generator::painter::{circle::CirclePainter, rect::RectPainter, stroke::StrokePainter};
use generator::utils::color::BlendingMode;
use generator::utils::files;
use generator::utils::parsing::{
	parse_color, parse_color_matrix, parse_scale, parse_size_margins, parse_weighted_blending_mode,
	parse_weighted_float_pair, parse_weighted_size_pair,
};
use generator::utils::random::get_random_seed;
use generator::utils::units::{Margins, SizeUnit, WeightedValue};
use generator::{Generator, ProcessCallbackResult};

mod generator;

/// Progressively generate an image based on a target
#[derive(Debug, StructOpt)]
struct Opt {
	/// The target image.
	///
	/// The painting algorithms will try matching this image, without copying directly from it.
	#[structopt(parse(from_os_str))]
	target: PathBuf,

	/// Maximum number of image generation tries (successful or nor) to run.
	///
	/// On each try, the painter algorithm tries creating an image that is closer to the target image (with several "candidates" per try).
	///
	/// The more complex the result image gets, the harder it is to create an improved image, so it's common to have many unsuccessful tries. Use this option to set a maximum number of tries.
	///
	/// Using a limited number of tries can give a predicted time for completion, but also gives an unpredictable number of successful paints. Use the `-g`/`--generations` parameter to control the number of desired paints instead.
	///
	/// Set to `0` if no limit is desired.
	#[structopt(short = "t", long, default_value = "0", required_if("generations", "0"))]
	max_tries: u32,

	/// Number of successful generations desired.
	///
	/// This is equivalent to the number of times the result image is expected to be successfully painted. In general, the higher the number of generations, the closer the image will be to the target image.
	///
	/// Set to `0` if no limit is desired.
	#[structopt(short, long, default_value = "0", required_if("max_tries", "0"))]
	generations: u32,

	/// Number of parallel image painting candidates per try.
	///
	/// In general, the higher the number of candidates, the better the resulting images, at a cost of higher CPU usage.
	///
	/// When set to `0`, this uses the number of available cores in the CPU.
	#[structopt(short, long, default_value = "0")]
	candidates: usize,

	/// Expected difference score to reach, indicating the desired difference from the new generated image to the target image. New candidates are generated continuously until the resulting difference is below this threshold.
	///
	/// On each successfull image generation, the new diff value is generated by calculating the color difference of every pixel. For example, for a completely white target image, a completely black image has 100% difference, while a gray image would have 50% difference.
	///
	/// Be aware that the lower the target difference, the longer the time taken for full generation, sometimes exponentially so. Very low diff numbers (e.g. 10% and lower) might be virtually impossible to reach, or take an unordinate amount of time.
	#[structopt(short, long, default_value = "0", parse(try_from_str = parse_scale))]
	diff: f64,

	/// Amount of color from the original target image to use as a "seed" when deciding on what color to use when painting a new candidate. With this set to `0`; the algorithm will try painting with a completely random new color; with this set to `1`, the algorithm will use the color already found in the target color; and everything in between is a blend of the two.
	///
	/// Using a higher color seed number causes the algorithm to generate valid candidates much faster, and thus create a new image that is closer to the target in shorter time. It does decrease the randomness of the output image, and could in some ways be seen as "cheating" as the algorithm isn't painting blindly anymore.
	///
	/// Possible values: `0`..`1`
	#[structopt(long, default_value = "0", parse(try_from_str = parse_scale))]
	color_seed: f64,

	/// Outputs benchmark results.
	///
	/// With this flag, the application will gather some benchmark metrics and output them after it runs. This is useful to measure efficiency of the algorithm as it evolves.
	///
	/// It's recommended to use the same `--candidates` and `--rng-seed` value across different runs, for consistent results.
	#[structopt(long)]
	benchmark: bool,

	/// Disables writing image metadata.
	///
	/// By default, the output image file includes metadata with the software name and version, all generation statistics, and original command line arguments used, including original file names passed. With this flag set, nothing is written.
	#[structopt(long)]
	no_metadata: bool,

	/// The filename for the result image to be saved to.
	///
	/// On each successful generation, this file is rewritten with the results. Extensions such as `.png` or `.jpg` are allowed. More formats might be supported in the future.
	///
	/// If the destination file already exists, it is overwritten without warning.
	#[structopt(short, long, default_value = "output.png", parse(from_os_str))]
	output: PathBuf,

	/// The filename for an input image, if any.
	///
	/// When present, the input image that serves as the starting image before anything is painted atop it. The `--background-color` parameter is also ignored.
	#[structopt(short, long, parse(from_os_str))]
	input: Option<PathBuf>,

	/// The seed to use for the pseudorandom number generator.
	///
	/// This should be an unsigned 32-but integer number (that is, between and `0` and `4294967295`, inclusive). If `0` is passed, the seed iself is randomized.
	///
	/// When generating new candidates, the application tries creating new images in a randomized, but *deterministic*, fashion. This means that as long as all inputs - target images, parameters - are the same, the end result will always be the same.
	///
	/// To allow for that but also an element of unpredictability, it uses a *random seed* which is a number that determines the point in the random number sequence where new random numbers will come from.
	///
	/// In other words, re-running the application repeated times with the same value for `--rng-seed` should always produce the same result. This is useful in case a particularly interesting image is generated; in this case, it's worth re-running the application with a higher `--scale` value, to create a larger image (e.g. for printing).
	///
	/// When no seed is passed, one is chosen at random, and both printed during the program's output, and added to the generated file's metadata, in case one wants to recreate the result image.
	#[structopt(long, default_value = "0")]
	rng_seed: u32,

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
	/// This value is parsed by the [color_processing](https://docs.rs/color_processing) crate.
	#[structopt(long, default_value = "000000", parse(try_from_str = parse_color))]
	background_color: (u8, u8, u8),

	/// Color matrix to be applied to the target image before using it.
	///
	/// This allows one to change how colors in the target image are perceived by the system when determining whether a newly painted candidate is close to the target or not. While editing the target image on an image editor (by changing its colors) prior to running Random Art Generator has the same effect, running a color matrix transformation as part of the application can help automate the generation process.
	///
	/// This is a (somewhat) typical 3x4 matrix for color transformations between RGB channels, in the format:
	///
	/// ```
	/// r_from_r, r_from_g, r_from_b, r_offset,
	/// g_from_r, g_from_g, g_from_b, g_offset,
	/// b_from_r, b_from_g, b_from_b, b_offset
	/// ```
	///
	/// For example, the *identity* matrix (equivalent to no change) is `1,0,0,0,0,1,0,0, 0,0,1,0`.
	///
	/// Other examples:
	///
	/// * Grayscale is "0.33,0.59,0.11,0,0.33,0.59,0.11,0,0.33,0.59,0.11,0"
	/// * Sepia is "0.393,0.769,0.686,0,0.349,0.686,0.168,0,0.272,0.534,0.131,0"
	/// * Polaroid is "1.438,0.122,-0.016,-8,-0.062,1.378,-0.016,-13,-0.062,-0.122,1.483,-5"
	#[structopt(long, parse(try_from_str = parse_color_matrix))]
	target_color_matrix: Option<[f64; 12]>,

	/// Save the output file more frequently.
	///
	/// The default behavior for the application is to only write the final output file when the target generations, tries, or diff are achieved. With this flag, the output file will be saved frequently, on every successful generation.
	///
	/// This is useful if one expects to be interrupting the writing process in the middle.
	#[structopt(long)]
	save_often: bool,

	/// The new size of the output image, as a scale of the target image.
	///
	/// This is useful if one wants the result image to be either smaller or larger than the target image.
	///
	/// Larger images tend to take more time to generate. It's useful to try and generate smaller images (or of the same size as the target) when trying out parameters, and once one is happy with the results, regenerate the image using a larger scale and the same random number generator seed used in the original result (via `--rng-seed`).
	#[structopt(short, long, default_value = "1")]
	scale: f64,

	/// Blending mode(s) to be used when overlaying new candidates, either as a single entry, or as a list. The blending modes follow some of the classic Photoshop blending modes.
	///
	/// Use this option with caution. Some monotonic blending modes (`screen`, `multiply`, etc) might cause the image generation to never finish. For example, with a complete white base image, it's impossible for it to be altered further with the `screen` blending mode.
	///
	/// Possible values: `normal`, `multiply`, `screen`, `overlay`, `darken`, `lighten`, `color-dodge`, `color-burn`, `hard-light`, `soft-light`, `difference`, `exclusion`
	#[structopt(long, default_value = "normal", default_value = "normal", parse(try_from_str = parse_weighted_blending_mode))]
	blending_mode: Vec<WeightedValue<BlendingMode>>,

	/// Painter to be used.
	///
	/// This determines how new candidates will be painted when trying to approximate the target image. A selection of basic painters currently exist.
	///
	/// Painters can be further configured with other `--painter-*` arguments.
	///
	/// Possible values: `rects`, `circles`, `strokes`
	#[structopt(short, long, possible_values = &["circles", "strokes", "rects"], default_value = "rects")]
	painter: String,

	/// Opacity to use when painting new images.
	///
	/// This can be either a single value between `0.0` (fully transparent) and `1.0` (fully opaque), or a range in the same scale for randomized values (e.g. `0.1-0.9`).
	///
	/// The argument is a list, so it can also feature more than one value (or ranges, or a mix of values or ranges), in which case one new entry is randomly picked for each new paint.
	#[structopt(long, default_value = "1", parse(try_from_str = parse_weighted_float_pair))]
	painter_alpha: Vec<WeightedValue<(f64, f64)>>,

	/// Bias for distribution in `--painter-alpha` ranges.
	///
	/// A bias of 0.0 means a normal, linear distribution; -1.0 = quad bias towards range start; 1.0 = quad bias towards range end.
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_alpha_bias: f64,

	/// Radius to use when painting elements, when applicable.
	///
	/// This applies when `--painter` is set to `circles`. In case a percentage value is passed, it is relative to either the width or height of the result image (whichever is smaller).
	///
	/// The argument is a list, so it can also feature more than one value (or ranges, or a mix of values or ranges), in which case one new entry is randomly picked for each new paint.
	#[structopt(long, default_value = "0%-50%", parse(try_from_str = parse_weighted_size_pair))]
	painter_radius: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,

	/// Bias for distribution in `--painter-radius` ranges.
	///
	/// A bias of 0.0 means a normal, linear distribution; -1.0 = quad bias towards range start; 1.0 = quad bias towards range end.
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_radius_bias: f64,

	/// Width to use when painting elements.
	///
	/// This applies when `--painter` is set to `rects` or `strokes`.
	///
	/// The argument is a list, so it can also feature more than one value (or ranges, or a mix of values or ranges), in which case one new entry is randomly picked for each new paint.
	#[structopt(long, default_value = "0%-100%", parse(try_from_str = parse_weighted_size_pair))]
	painter_width: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,

	/// Bias for distribution in `--painter-width` ranges.
	///
	/// A bias of 0.0 means a normal, linear distribution; -1.0 = quad bias towards range start; 1.0 = quad bias towards range end.
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_width_bias: f64,

	/// Height to use when painting elements.
	///
	/// This applies when `--painter` is set to `rects` or `strokes`.
	///
	/// The argument is a list, so it can also feature more than one value (or ranges, or a mix of values or ranges), in which case one new entry is randomly picked for each new paint.
	#[structopt(long, default_value = "0%-100%", parse(try_from_str = parse_weighted_size_pair))]
	painter_height: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,

	/// Bias for distribution in `--painter-height` ranges.
	///
	/// A bias of 0.0 means a normal, linear distribution; -1.0 = quad bias towards range start; 1.0 = quad bias towards range end.
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_height_bias: f64,

	/// Disables calculating antialias on edges when painting new elements.
	///
	/// This makes rendering faster in some cases, but can produce jagged edges, and is therefore not recommended.
	///
	/// The one exception is when creating artwork meant to be printed. In that case, antialiased edges can produce dithering artifacts during the printing process; it is better to create an aliased result at a higher resolution instead (using `--scale`) to match the printer's resolution.
	#[structopt(long)]
	painter_disable_anti_alias: bool,

	/// Height of paint waves, when applicable.
	///
	/// This applies when `--painter` is set to `strokes`. In case a percentage value is passed, it is always relative to the width of the result image.
	///
	/// In the `strokes` painter, *waves* are the deformations that occur on the edges of each painted element. The waves have a *height* (their strength, perpendicular to the edge itself) and a *length* (the size of an entire wave along the direction of the edge). This length encompasses a set of different waves (rather than just one wave), to create a noise-like pattern. The bigger the length, the gentler the wave looks, similarly to producing a sound wave of lower frequency.
	///
	/// The argument is a list, so it can also feature more than one value (or ranges, or a mix of values or ranges), in which case one new entry is randomly picked for each new paint.
	#[structopt(long, default_value = "0.5%", parse(try_from_str = parse_weighted_size_pair))]
	painter_wave_height: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,

	/// Bias for distribution in `--painter-wave-height` ranges.
	///
	/// A bias of 0.0 means a normal, linear distribution; -1.0 = quad bias towards range start; 1.0 = quad bias towards range end.
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_wave_height_bias: f64,

	/// Length of paint waves, when applicable.
	///
	/// This applies when `--painter` is set to `strokes`. In case a percentage value is passed, it is always relative to the height of the result image.
	///
	/// In the `strokes` painter, *waves* are the deformations that occur on the edges of each painted element. The waves have a *height* (their strength, perpendicular to the edge itself) and a *length* (the size of an entire wave along the direction of the edge). The higher the wave, the stronger they look.
	///
	/// The argument is a list, so it can also feature more than one value (or ranges, or a mix of values or ranges), in which case one new entry is randomly picked for each new paint.
	#[structopt(long, default_value = "400%", parse(try_from_str = parse_weighted_size_pair))]
	painter_wave_length: Vec<WeightedValue<(SizeUnit, SizeUnit)>>,

	/// Bias for distribution in `--painter-wave-length` ranges.
	///
	/// A bias of 0.0 means a normal, linear distribution; -1.0 = quad bias towards range start; 1.0 = quad bias towards range end.
	#[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
	painter_wave_length_bias: f64,

	/// Margins for the output image.
	///
	/// This can either be a single size for all margins, or a comma-separated list of 2..4 items denoting the margins for each specific side (similar to how margins are written in CSS).
	///
	/// When a percentage unit is used, they refer to the maximum width or height of the image. Values higher than the image size (in pixels or in percentages higher than `100%`) are allowed, in which case they cause the paint algorithm to bleed out of the image space.
	#[structopt(long, default_value = "0", allow_hyphen_values = true, parse(try_from_str = parse_size_margins))]
	margins: Margins<SizeUnit>,
}

fn get_options() -> Opt {
	return Opt::from_args();
}

fn get_command_line(options: &Opt) -> String {
	// Creates string with arguments only
	let args = &env::args().collect::<Vec<String>>()[1..];
	let mut args_str = args.join(" ");

	// Replaces filenames
	args_str = args_str.replace(options.target.to_str().as_ref().unwrap(), "target.img");
	args_str = args_str.replace(options.output.to_str().as_ref().unwrap(), "output.img");
	if let Some(input) = options.input.as_ref() {
		args_str = args_str.replace(input.to_str().as_ref().unwrap(), "input.img");
	}

	// Output with fake executable name
	"[rag] ".to_string() + &args_str
}

fn on_processed(generator: &Generator, result: ProcessCallbackResult) {
	// Ignore unsuccessful generations
	if !result.is_success {
		return;
	}

	let options = get_options();

	// Only write the file if it's the final generation, or it's meant to save often
	if !result.is_final && !options.save_often {
		return;
	}

	let output_path = options.output.as_path();

	if options.no_metadata {
		// No metadata wanted, write the file directly
		files::write_image(generator.get_current(), output_path);
	} else {
		// Write the file with metadata

		// Define new metadata
		let mut comments = vec![
			format!(
				"Produced {} generations after {} tries; the final difference from target is {:.2}%.",
				result.num_generations,
				result.num_tries,
				result.diff * 100.0
			),
			format!("Command line: {}", get_command_line(&options)),
		];

		// Add painter-specific metadata
		for (key, value) in result.metadata {
			comments.push(format!("{}: {}", key, value));
		}

		files::write_image_with_metadata(generator.get_current(), output_path, comments);
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

	// Other options
	let candidates = if options.candidates > 0 {
		options.candidates
	} else {
		println!("Using auto {} candidates.", num_cpus::get());
		num_cpus::get()
	};

	let rng_seed = if options.rng_seed == 0 {
		get_random_seed()
	} else {
		options.rng_seed
	};
	println!("RNG seed is {}.", rng_seed);

	// Process everything
	// TODO: use actual enums here and use a single object from trait (can't seen to make it work)
	// TODO: error out on passed painter options that are unused?
	match &options.painter[..] {
		"circles" => {
			let mut painter = CirclePainter::new();
			painter.options.blending_mode = options.blending_mode;
			painter.options.alpha = options.painter_alpha;
			painter.options.alpha_bias = options.painter_alpha_bias;
			painter.options.radius = options.painter_radius;
			painter.options.radius_bias = options.painter_radius_bias;
			painter.options.anti_alias = !options.painter_disable_anti_alias;
			painter.options.color_seed = options.color_seed;
			painter.options.rng_seed = rng_seed;
			painter.options.margins = options.margins;
			gen.process(
				options.max_tries,
				options.generations,
				options.diff,
				options.benchmark,
				candidates,
				painter,
				Some(on_processed),
			);
		}
		"rects" => {
			let mut painter = RectPainter::new();
			painter.options.blending_mode = options.blending_mode;
			painter.options.alpha = options.painter_alpha;
			painter.options.alpha_bias = options.painter_alpha_bias;
			painter.options.width = options.painter_width;
			painter.options.width_bias = options.painter_width_bias;
			painter.options.height = options.painter_height;
			painter.options.height_bias = options.painter_height_bias;
			painter.options.color_seed = options.color_seed;
			painter.options.rng_seed = rng_seed;
			painter.options.margins = options.margins;
			gen.process(
				options.max_tries,
				options.generations,
				options.diff,
				options.benchmark,
				candidates,
				painter,
				Some(on_processed),
			);
		}
		"strokes" => {
			let mut painter = StrokePainter::new();
			painter.options.blending_mode = options.blending_mode;
			painter.options.alpha = options.painter_alpha;
			painter.options.alpha_bias = options.painter_alpha_bias;
			painter.options.width = options.painter_width;
			painter.options.width_bias = options.painter_width_bias;
			painter.options.height = options.painter_height;
			painter.options.height_bias = options.painter_height_bias;
			painter.options.wave_height = options.painter_wave_height;
			painter.options.wave_height_bias = options.painter_wave_height_bias;
			painter.options.wave_length = options.painter_wave_length;
			painter.options.wave_length_bias = options.painter_wave_length_bias;
			painter.options.anti_alias = !options.painter_disable_anti_alias;
			painter.options.color_seed = options.color_seed;
			painter.options.rng_seed = rng_seed;
			painter.options.margins = options.margins;
			gen.process(
				options.max_tries,
				options.generations,
				options.diff,
				options.benchmark,
				candidates,
				painter,
				Some(on_processed),
			);
		}
		_ => unreachable!(),
	}
}
