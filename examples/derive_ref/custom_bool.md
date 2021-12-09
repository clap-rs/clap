*Jump to [source](custom_bool.rs)*

Example of overriding the magic `bool` behavior

```bash
$ custom_bool --help
clap [..]

A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    custom_bool[EXE] [OPTIONS] --foo <FOO> <BOOM>

ARGS:
    <BOOM>    

OPTIONS:
        --bar <BAR>    [default: false]
        --foo <FOO>    
    -h, --help         Print help information
    -V, --version      Print version information
$ custom_bool
? failed
error: The following required arguments were not provided:
    --foo <FOO>
    <BOOM>

USAGE:
    custom_bool [OPTIONS] --foo <FOO> <BOOM>

For more information try --help
$ custom_bool --foo true false
[examples/derive_ref/custom_bool.rs:31] opt = Opt {
    foo: true,
    bar: false,
    boom: false,
}
$ custom_bool --foo true --bar true false
[examples/derive_ref/custom_bool.rs:31] opt = Opt {
    foo: true,
    bar: true,
    boom: false,
}
```
