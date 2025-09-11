Help:
```console
$ typed-derive fn-parser --help
Usage: typed-derive fn-parser [OPTIONS]

Options:
  -D <DEFINES>      Hand-written parser for tuples
  -h, --help        Print help

```

Defines (key-value pairs)
```console
$ typed-derive fn-parser -D Foo=10 -D Alice=30
FnParser(FnParser { defines: [("Foo", 10), ("Alice", 30)] })

$ typed-derive fn-parser -D Foo
? failed
error: invalid value 'Foo' for '-D <DEFINES>': invalid KEY=value: no `=` found in `Foo`

For more information, try '--help'.

$ typed-derive fn-parser -D Foo=Bar
? failed
error: invalid value 'Foo=Bar' for '-D <DEFINES>': invalid digit found in string

For more information, try '--help'.

```
