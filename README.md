# Art Generator

A command-line application to progressively generate a new image based on an existing target image.

## Run

```shell
cargo run -- [--input input.jpg] --target target.jpg [--output output.png] [--iterations 1] [--candidates 10]
```

## Create release binary

```shell
cargo build --release
```
