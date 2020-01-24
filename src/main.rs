use std::path::PathBuf;
use structopt::StructOpt;

use image::GenericImageView;

mod generator;

/// Progressively generate an image based on a target
#[derive(Debug, StructOpt)]
struct Opt {
    /// The target image
    #[structopt(short, long, parse(from_os_str))]
    target: PathBuf,

    /*
    TODO: these are disabled for now, until they are actually implemented

    /// The output image filename
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// The input image filename, if any
    #[structopt(short, long, default_value = "", parse(from_os_str))]
    input: PathBuf,
    */
}

fn main() {
	let options = Opt::from_args();
	println!("Done.");

    let target_file = options.target.as_path();
    let target_result = image::open(target_file);
    let target_image = match target_result {
        Ok(content) => { content },
        Err(_) => { panic!("Cannot open file {:?}, exiting", target_file); }
    };

    println!("Using target image of {:?} with dimensions of {:?}.", target_file, target_image.dimensions());

    let gen = generator::create(target_image);
    gen.process();
    gen.save();
}
