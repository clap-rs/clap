Positional arguments are those values after the program name which are not preceded by any
identifier (such as "myapp some_file"). Positionals support many of the same options as
flags, as well as a few additional ones.

Let's look at their help:
```bash
$ 06_positional_args --help
MyApp 

USAGE:
    06_positional_args[EXE] <input> [config]

ARGS:
    <input>     the input file to use
    <config>    the config file to use

OPTIONS:
    -h, --help    Print help information
```

First, we see that the first argument is required:
```
$ 06_positional_args
? failed
error: The following required arguments were not provided:
    <input>
    <config>

USAGE:
    06_positional_args[EXE] <input> [config]

For more information try --help
```

That first argument causes the second to be required:
```
$ 06_positional_args input.txt
? failed
error: The following required arguments were not provided:
    <config>

USAGE:
    06_positional_args[EXE] <input> <config> [config]

For more information try --help
```

Everything works now that we specify both:
```
$ 06_positional_args input.txt config.toml
An input file was specified
Doing work with input.txt and config.toml
```
