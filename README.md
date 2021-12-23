<div align="center"><img width="500" height="250" src="docs/logo.png" alt="Random Art Generator">

# Random Art Generator

![GitHub](https://img.shields.io/github/license/zeh/random-art-generator)
</div>

<p align="center"><a href="docs/download.md">Download</a> | <a href="docs/running.md">Reference</a> | <a href="https://github.com/zeh/random-art-generator/releases">Changelog</a> | <a href="docs/development.md">Development</a> | <a href="LICENSE">License (MIT)</a></p>

Random Art Generator is a command-line application to produce generative art.

When passed a _target_ image, it will start painting elements on a blank canvas, essentially creating new images at random. Resulting images that are _mathematically closer_ to the target image are kept, and used as a new canvas; results that aren't get discarded. This repeats until the desired number of attempts or generations is achieved.

For example:

```shell
rag profile.jpg --generations 100
```

Technically known as [hill climbing](https://en.wikipedia.org/wiki/Hill_climbing), this process allows the creation of new images that progressively converge into the target image, with a style all its own depending on the chosen painting algorithm and its arguments.

This application is written in [Rust](https://www.rust-lang.org/), and downloadable binaries are currently available for macOS, Linux, and Windows. A webasm target - so it can be used as a JavaScript library - is planned for later.
