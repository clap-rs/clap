*Jump to [source](custom-bool.rs)*

Example of overriding the magic `bool` behavior

```console
$ custom-bool --help
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage:
    custom-bool[EXE] [OPTIONS] --foo <FOO> <BOOM>

Arguments:
    <BOOM>    [possible values: true, false]

Options:
        --foo <FOO>    [possible values: true, false]
        --bar <BAR>    [default: false]
    -h, --help         Print help information
    -V, --version      Print version information

$ custom-bool
? failed
error: The following required arguments were not provided:
    --foo <FOO>
    <BOOM>

Usage:
    custom-bool[EXE] --foo <FOO> <BOOM>

For more information try --help

$ custom-bool --foo true false
[examples/derive_ref/custom-bool.rs:31] opt = Opt {
    foo: true,
    bar: false,
    boom: false,
}

$ custom-bool --foo true --bar true false
[examples/derive_ref/custom-bool.rs:31] opt = Opt {
    foo: true,
    bar: true,
    boom: false,
}

```
