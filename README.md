# Random Art Generator

This is a command-line application that generates images based on an existing target image.

It works by employing a painter algorithm that creates new images at random (based on a set of parameters), keeping images that have higher similarity to the target and discarding ones that aren't. This process allows the creation of new images that progressively converge into the target image.

This application is written in [Rust](https://www.rust-lang.org/). Currently, only the source code is provided, as no downloadable and executable binaries are created. This will likely change in the future, as both macOS/Windows/Linux applications and Webasm targets are planned.

## Run

Since the application is provided as source code, it need to be compiled and ran by [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```shell
cargo run -- [--input input.jpg] --target target.jpg [--output output.png] --attempts 10
```

Command line switches are still being added. For a full list of the currently available switches and a brief explanation, run:

```shell
cargo run -- --help
```

Or check [the struct source code](https://github.com/zeh/art-generator/blob/master/src/main.rs#L13).

## Create release binary

A self-compiled release binary can be created with Cargo as well:

```shell
cargo build --release
```
