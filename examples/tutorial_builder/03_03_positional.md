```console
$ 03_03_positional --help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    03_03_positional[EXE] [name]

ARGS:
    <name>    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

$ 03_03_positional
name: None

$ 03_03_positional bob
name: Some("bob")

```
