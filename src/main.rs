use std::path::PathBuf;
use structopt::StructOpt;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

/// Progressively generate an image based on a target
#[derive(Debug, StructOpt)]
struct Opt {
    /// The target image
    #[structopt(short, long, parse(from_os_str))]
    target: PathBuf,

    /// The output image filename
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// The input image filename, if any
    #[structopt(short, long, default_value = "", parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let options = Opt::from_args();
    println!("Done.");

    let img = ImageBuffer::from_fn(512, 512, |x, y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    img.save("output.png").unwrap();
}
