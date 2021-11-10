Option arguments are those that take an additional value, such as "-c value". In clap they
support three types of specification, those with short() as "-o some", or those with long()
as "--option value" or "--option=value"

Options also support a multiple setting, which is discussed in the example below.

Let's look at their help:
```bash
$ 07_option_args --help
MyApp 

USAGE:
    07_option_args[EXE] [OPTIONS] --input <input> [output]

ARGS:
    <output>    the output file to use

OPTIONS:
    -c, --config <FILE>    the config file to use
    -h, --help             Print help information
    -i, --input <input>    the input file to use
```

First, we see that `--input` is required:
```bash
$ 07_option_args
? failed
error: The following required arguments were not provided:
    --config <FILE>
    --input <input>

USAGE:
    07_option_args[EXE] [OPTIONS] --input <input> [output]

For more information try --help
```

But `--input` also requires `--config`:
```bash
$ 07_option_args --input input.txt --input another.txt
? failed
error: The following required arguments were not provided:
    --config <FILE>

USAGE:
    07_option_args[EXE] [OPTIONS] --input <input> --config <FILE> [output]

For more information try --help
```

Everything works now that we specify both:
```bash
$ 07_option_args --input input.txt --input another.txt --config config.toml
An input file was specified
An input file: input.txt
An input file: input.txt
An input file: another.txt
The "input" argument was used 2 times
```

But we can't mix this with output:
```bash
$ 07_option_args --input input.txt --input another.txt --config config.toml output.txt
? failed
error: The argument '<output>' cannot be used with '--input <input>'

USAGE:
    07_option_args[EXE] --input <input> --input <input> --config <FILE>

For more information try --help
```

That requires passing it in by itself:
```bash
$ 07_option_args output.txt
The "input" argument was used 0 times
```
