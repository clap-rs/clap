```console
$ 03_01_flag_count --help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    03_01_flag_count[EXE] [OPTIONS]

OPTIONS:
    -v, --verbose...    
    -h, --help          Print help information
    -V, --version       Print version information

$ 03_01_flag_count
verbose: 0

$ 03_01_flag_count --verbose
verbose: 1

$ 03_01_flag_count --verbose --verbose
verbose: 2

```
