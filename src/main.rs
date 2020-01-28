use std::path::PathBuf;
use structopt::StructOpt;

use image::GenericImageView;

mod generator;

/// Progressively generate an image based on a target
#[derive(Debug, StructOpt)]
struct Opt {
    /// Number of candidates per iteration
    #[structopt(long, default_value = "10")]
    candidates: u32,

    /// Number of iterations to run
    #[structopt(long, default_value = "1")]
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
}

fn main() {
	let options = Opt::from_args();

    // Target
    let target_file = options.target.as_path();
    let target_image = image::open(target_file)
        .expect("Cannot open target file {:?}, exiting");

    println!("Using target image of {:?} with dimensions of {:?}.", target_file, target_image.dimensions());

    // Start generating
    let mut gen = generator::Generator::from(target_image);

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
    gen.process(options.candidates, options.iterations);
    gen.finalize().save(output_file)
        .expect("Cannot write to output file {:?}, exiting");
}
