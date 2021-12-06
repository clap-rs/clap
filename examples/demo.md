*Jump to [source](demo.rs)*

Used to validate README.md's content
```bash
$ demo --help
clap [..]



A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    demo[EXE] [OPTIONS] <INPUT>

ARGS:
    <INPUT>    Some input. Because this isn't an Option<T> it's required to be used

OPTIONS:
    -c, --config <PATH>    Sets a custom config file. Could have been an Option<T> with no default
                           too [default: default.toml]
    -h, --help             Print help information
    -m, --mode <MODE>      What mode to run the program in [default: slow] [possible values: fast,
                           slow]
    -v, --verbose          A level of verbosity, and can be used multiple times
    -V, --version          Print version information
```
