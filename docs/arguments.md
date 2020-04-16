# RAG Arguments

Being a command-line application, RAG takes several command-line arguments to control the image it generates.

The general format for running RAG is as follow:

```shell
rag target-file-image [FLAGS] [OPTIONS]
```

Running with a single `-h` or `--help` argument will show you all arguments available, and a brief explanation of each:

```shell
rag -h
```

A longer list of all arguments will be added later. For now, this can be ran to generate their documentation:

```shell
cargo doc --bin random-art-generator --no-deps
```

Or, check [the struct source code](https://github.com/zeh/art-generator/blob/master/src/main.rs#L17) for more insight into each argument.
