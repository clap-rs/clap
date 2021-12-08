*Jump to [source](keyvalue_derive.rs)*

```bash
$ keyvalue_derive --help
clap 

USAGE:
    keyvalue_derive[EXE] [OPTIONS]

OPTIONS:
    -D <DEFINES>        
    -h, --help          Print help information
$ keyvalue_derive -D Foo=10 -D Alice=30
Args { defines: [("Foo", 10), ("Alice", 30)] }
$ keyvalue_derive -D Foo
? failed
error: Invalid value for '-D <DEFINES>': invalid KEY=value: no `=` found in `Foo`

For more information try --help
$ keyvalue_derive -D Foo=Bar
? failed
error: Invalid value for '-D <DEFINES>': invalid digit found in string

For more information try --help
```
