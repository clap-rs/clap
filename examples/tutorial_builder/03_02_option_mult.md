```console
$ 03_02_option_mult --help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    03_02_option_mult[EXE] [OPTIONS]

OPTIONS:
    -n, --name <name>    
    -h, --help           Print help information
    -V, --version        Print version information

$ 03_02_option_mult
name: None

$ 03_02_option_mult --name bob
name: Some("bob")

$ 03_02_option_mult --name=bob
name: Some("bob")

$ 03_02_option_mult -n bob
name: Some("bob")

$ 03_02_option_mult -n=bob
name: Some("bob")

$ 03_02_option_mult -nbob
name: Some("bob")

```
