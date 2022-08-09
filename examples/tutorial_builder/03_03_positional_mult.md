```console
$ 03_03_positional_mult --help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    03_03_positional_mult[EXE] [name]...

ARGS:
    <name>...    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

$ 03_03_positional_mult
name: None

$ 03_03_positional_mult bob
name: Some("bob")

```
