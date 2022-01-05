*Jump to [source](keyvalue-derive.rs)*

**This requires enabling the `derive` feature flag.**

```console
$ keyvalue-derive --help
clap 

USAGE:
    keyvalue-derive[EXE] [OPTIONS]

OPTIONS:
    -D <DEFINES>        
    -h, --help          Print help information

$ keyvalue-derive -D Foo=10 -D Alice=30
Args { defines: [("Foo", 10), ("Alice", 30)] }

$ keyvalue-derive -D Foo
? failed
error: Invalid value for '-D <DEFINES>': invalid KEY=value: no `=` found in `Foo`

For more information try --help

$ keyvalue-derive -D Foo=Bar
? failed
error: Invalid value for '-D <DEFINES>': invalid digit found in string

For more information try --help

```
