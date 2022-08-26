For more on creating a custom subcommand, see [the cargo
book](https://doc.rust-lang.org/cargo/reference/external-tools.html#custom-subcommands).
The crate [`clap-cargo`](https://github.com/crate-ci/clap-cargo) can help in
mimicking cargo's interface.

The help looks like:
```console
$ cargo-example-derive --help
cargo 

Usage:
    cargo <SUBCOMMAND>

Options:
    -h, --help    Print help information

Subcommands:
    example-derive    A simple to use, efficient, and full-featured Command Line Argument Parser
    help              Print this message or the help of the given subcommand(s)

$ cargo-example-derive example-derive --help
cargo-example-derive [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage:
    cargo example-derive [OPTIONS]

Options:
        --manifest-path <MANIFEST_PATH>    
    -h, --help                             Print help information
    -V, --version                          Print version information

```

Then to directly invoke the command, run:
```console
$ cargo-example-derive example-derive
None

$ cargo-example-derive example-derive --manifest-path Cargo.toml
Some("Cargo.toml")

```
