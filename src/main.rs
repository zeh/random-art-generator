use std::path::PathBuf;
use structopt::StructOpt;

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
}
