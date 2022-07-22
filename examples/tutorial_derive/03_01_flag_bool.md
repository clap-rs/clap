```console
$ 03_01_flag_bool --help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    03_01_flag_bool[EXE] [OPTIONS]

OPTIONS:
    -v, --verbose    
    -h, --help       Print help information
    -V, --version    Print version information

$ 03_01_flag_bool
verbose: false

$ 03_01_flag_bool --verbose
verbose: true

$ 03_01_flag_bool --verbose --verbose
verbose: true

```
