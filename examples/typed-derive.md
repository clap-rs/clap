*Jump to [source](typed-derive.rs)*

**This requires enabling the `derive` feature flag.**

```console
$ typed-derive --help
clap 

USAGE:
    typed-derive[EXE] [OPTIONS]

OPTIONS:
    -D <DEFINES>        
    -h, --help          Print help information

$ typed-derive -D Foo=10 -D Alice=30
Args { defines: [("Foo", 10), ("Alice", 30)] }

$ typed-derive -D Foo
? failed
error: Invalid value "Foo" for '-D <DEFINES>': invalid KEY=value: no `=` found in `Foo`

For more information try --help

$ typed-derive -D Foo=Bar
? failed
error: Invalid value "Foo=Bar" for '-D <DEFINES>': invalid digit found in string

For more information try --help

```
