```console
$ 03_02_option_mult_derive --help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    03_02_option_mult_derive[EXE] [OPTIONS]

OPTIONS:
    -n, --name <NAME>    
    -h, --help           Print help information
    -V, --version        Print version information

$ 03_02_option_mult_derive
name: []

$ 03_02_option_mult_derive --name bob
name: ["bob"]

$ 03_02_option_mult_derive --name=bob
name: ["bob"]

$ 03_02_option_mult_derive -n bob
name: ["bob"]

$ 03_02_option_mult_derive -n=bob
name: ["bob"]

$ 03_02_option_mult_derive -nbob
name: ["bob"]

```
