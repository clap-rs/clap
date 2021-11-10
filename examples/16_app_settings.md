You can use `AppSettings` to change the application level behavior of clap. `app.setting()` function
takes `AppSettings` enum as argument. You can learn more about AppSettings in the
documentation, which also has examples on each setting.

This example will only show usage of one AppSettings setting. See documentation for more
information.

Something is required:
```bash
$ 16_app_settings
? failed
error: The following required arguments were not provided:
    <input>

USAGE:
    16_app_settings <input>
    16_app_settings <SUBCOMMAND>

For more information try --help
```

It can either be an argument:
```bash
$ 16_app_settings input.txt
The input file is: input.txt
```

Or the `test` subcommand:
```bash
$ 16_app_settings test
The 'test' subcommand was used
```

And see what this looks like in the help:
```bash
$ 16_app_settings --help
myapp 

USAGE:
    16_app_settings <input>
    16_app_settings <SUBCOMMAND>

ARGS:
    <input>    input file to use

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    test    does some testing
```
