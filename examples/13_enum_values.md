Building on [11_only_specific_values](11_only_specific_values.md), we will create an enum with 4 values, assign a positional argument
that accepts only one of those values, and use clap to parse the argument.

```bash
$ 13_enum_values Foo
Found a Foo
$ 13_enum_values Bar
Found a Bar
```

Anything else will error, guiding the user to a valid value:
```bash
$ 13_enum_values Alice
? failed
error: "Alice" isn't a valid value for '<type>'
	[possible values: Bar, Baz, Foo, Qux]

USAGE:
    13_enum_values <type>

For more information try --help
```

Valid values also get shown in the help:
```bash
$ 13_enum_values --help
myapp 

USAGE:
    13_enum_values <type>

ARGS:
    <type>    The type to use [possible values: Foo, Bar, Baz, Qux]

OPTIONS:
    -h, --help    Print help information
```
