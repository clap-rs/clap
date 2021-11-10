Using `Arg::default_value`, rather than `Option::unwrap_or`, gives clap more information it can use with our argument.

For example, let's look at the help:
```bash
$ 10_default_values --help
myapp 

does awesome things

USAGE:
    10_default_values [OPTIONS] [INPUT]

ARGS:
    <INPUT>    The input file to use [default: input.txt]

OPTIONS:
    -c <CONFIG>        The config file to use
    -h, --help         Print help information
```
`<INPUT>`'s description says what the default is while `-c`'s does not.

Otherwise, they'll work the same:
```bash
$ 10_default_values
The input file is: input.txt
The config file is: config.json
$ 10_default_values other.txt -c other.toml
The input file is: other.txt
The config file is: other.toml
```
