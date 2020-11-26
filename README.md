<div align="center"><img width="500" height="250" src="docs/logo.png" alt="Random Art Generator">

# Random Art Generator

![GitHub](https://img.shields.io/github/license/zeh/random-art-generator)
</div>

Random Art Generator is a command-line application to produce generative art.

When passed a _target_ image, it will start painting elements on a blank canvas, essentially creating new images at random. Resulting images that are _mathematically closer_ to the target image are kept, and used as a new canvas; results that aren't get discarded. This repeats until the desired number of attempts or generations is achieved.

Technically known as [hill climbing](https://en.wikipedia.org/wiki/Hill_climbing), this process allows the creation of new images that progressively converge into the target image, with a style all its own depending on the chosen painting algorithm and its arguments.

This application is written in [Rust](https://www.rust-lang.org/), and downloadable binaries are currently available for macOS, Linux, and Windows. A webasm target - so it can be used as a JavaScript library - is planned for later.

## Downloading Random Art Generator

Check the [releases](https://github.com/zeh/random-art-generator/releases) page to download the latest stable executables. Each package contains a single command-line executable ("rag") that can be used.

Alternatively, check the [beta build actions](https://github.com/zeh/random-art-generator/actions?query=workflow%3A%22Beta+release%22+is%3Asuccess), click any of the builds, and download the executable file built for your system. New beta builds are made on every merge to the `dev` branch.

## Running Random Art Generator

Basic example with general parameters:

```shell
rag target.jpg [--input input.jpg] [--output output.png] [--max-tries 50] --generations 10 [--background-color ff0022] [--scale 2.0] [--painter rects|circles|strokes] [--painter-alpha 0.1-0.2 1.0]
```

Circles painter example with specific parameters:

```shell
rag target.jpg --generations 10 --painter circles [--painter-alpha 0.1-0.2 1.0] [--painter-radius 1-100] [--painter-radius-bias -3]
```

Rects painter example with specific parameters:

```shell
rag target.jpg --generations 10 --painter rects [--painter-alpha 0.1-0.2 1.0] [--painter-width 1-100] [--painter-width-bias -3] [--painter-height 1-100] [--painter-height-bias -3]
```

Command line parameters and flags are still being added. See [arguments](docs/arguments.md) for current documentation, or run `rag --help`.

More in-depth explanations will be available soon.

## Running the latest code

Check out the `dev` branch. Once Rust is installed, the code can be compiled and ran by [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) with `cargo run -- (other parameters)`. For example:

```shell
cargo run -- target.jpg [--input input.jpg] [--output output.png] [--max-tries 50] --generations 10 [--background-color ff0022] [--scale 2.0] [--painter rects|circles|strokes] [--painter-alpha 0.1-0.2 1.0]
```

## Testing

Non-exhaustive unit tests also exist.

```shell
cargo test
```

## Create release binary

A self-compiled release binary can be created with Cargo as well:

```shell
cargo build --release
```

## License

[MIT](LICENSE).
