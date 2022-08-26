```console
$ 03_05_default_values_derive --help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage:
    03_05_default_values_derive[EXE] [NAME]

Arguments:
    <NAME>    [default: alice]

Options:
    -h, --help       Print help information
    -V, --version    Print version information

$ 03_05_default_values_derive
name: "alice"

$ 03_05_default_values_derive bob
name: "bob"

```
