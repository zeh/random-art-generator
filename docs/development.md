# Development information

Random Art Generator is an experimental, personal project. As such, it doesn't sport any contribution guidelines, a formal roadmap, or public task list. Feel free to play around with the codebase, fork it, or learn from it if applicable. Contributions to the code, while welcome, should likely be restricted to non-breaking improvements or bug fixes.

Random Art Generator is licensed using the [MIT LICENSE](../LICENSE).

## Running

Being a Rust application, it's easy to hack around Random Art Generator.

Clone the repo, [install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html), and then simply run:

```shell
cargo run -- [parameters..]
```

## Testing

Non-exhaustive tests exist. Run with:

```shell
cargo test
```

## Misc

It is recommended to install [cargo-wgsl](https://github.com/PolyMeilex/cargo-wgsl) to help in the development of shaders. This will enable shader inspection in e.g. Visual Studio Code, and add a new `cargo-wgsl` command line tool.
