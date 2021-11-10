Of the three argument types, flags are the most simple. Flags are simple switches which can
be either "on" or "off"

`clap` also supports multiple occurrences of flags, the common example is "verbosity" where a
user could want a little information with "-v" or tons of information with "-v -v" or "-vv"

Let's look at their help:
```bash
$ 05_flag_args --help
MyApp 

USAGE:
    05_flag_args[EXE] [OPTIONS] [output]

ARGS:
    <output>    sets an output file

OPTIONS:
    -a, --awesome          turns up the awesome
    -c, --config <FILE>    sets a custom config file
    -h, --help             Print help information
```

By default, nothing happens:
```bash
$ 05_flag_args
Nothing is awesome
```

Note that `--awesome` places requirements on how other flags are used:
```bash
$ 05_flag_args --awesome
? failed
error: The following required arguments were not provided:
    --config <FILE>

USAGE:
    05_flag_args[EXE] --config <FILE> --awesome

For more information try --help
$ 05_flag_args output.txt --config file.toml --awesome
? failed
error: The argument '--awesome' cannot be used with '<output>'

USAGE:
    05_flag_args[EXE] --config <FILE> <output>

For more information try --help
```

You can then add `--awesome` as many times as you like:
```bash
$ 05_flag_args --config file.toml --awesome
Awesomeness is turned on
Some things are awesome
$ 05_flag_args --config file.toml --awesome --awesome
Awesomeness is turned on
Lots of things are awesome
$ 05_flag_args --config file.toml -aaaaaaaaaaaaaaaaaa
Awesomeness is turned on
EVERYTHING is awesome!
```
