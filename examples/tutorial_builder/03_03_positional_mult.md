```console
$ 03_03_positional_mult --help
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage: 03_03_positional_mult[EXE] [name]...

Arguments:
  [name]...  

Options:
  -h, --help     Print help information
  -V, --version  Print version information

$ 03_03_positional_mult
names: []

$ 03_03_positional_mult bob
names: ["bob"]

$ 03_03_positional_mult bob john
names: ["bob", "john"]

```
