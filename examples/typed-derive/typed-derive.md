**This requires enabling the [`derive` feature flag][crate::_features].**

## Implicit [`Arg::value_parser`][crate::Arg::value_parser]

Help:
```console
$ typed-derive implicit --help
Usage: typed-derive implicit [OPTIONS]

Options:
  -O <OPTIMIZATION>      Implicitly using `std::str::FromStr`
  -I <DIR>               Allow invalid UTF-8 paths
      --bind <BIND>      Handle IP addresses
      --sleep <SLEEP>    Allow human-readable durations
  -h, --help             Print help

```

Optimization-level (number)
```console
$ typed-derive implicit -O 1
Implicit(ImplicitParsers { optimization: Some(1), include: None, bind: None, sleep: None })

$ typed-derive implicit -O plaid
? failed
error: invalid value 'plaid' for '-O <OPTIMIZATION>': invalid digit found in string

For more information, try '--help'.

```

Include (path)
```console
$ typed-derive implicit -I../hello
Implicit(ImplicitParsers { optimization: None, include: Some("../hello"), bind: None, sleep: None })

```

IP Address
```console
$ typed-derive implicit --bind 192.0.0.1
Implicit(ImplicitParsers { optimization: None, include: None, bind: Some(192.0.0.1), sleep: None })

$ typed-derive implicit --bind localhost
? failed
error: invalid value 'localhost' for '--bind <BIND>': invalid IP address syntax

For more information, try '--help'.

```

Time
```console
$ typed-derive implicit --sleep 10s
Implicit(ImplicitParsers { optimization: None, include: None, bind: None, sleep: Some(10s) })

$ typed-derive implicit --sleep forever
? failed
error: invalid value 'forever' for '--sleep <SLEEP>': failed to parse "forever" in the "friendly" format: parsing a friendly duration requires it to start with a unit value (a decimal integer) after an optional sign, but no integer was found

For more information, try '--help'.

```

## Built-in [`TypedValueParser`][crate::builder::TypedValueParser]

Help:
```console
$ typed-derive builtin --help
Usage: typed-derive builtin [OPTIONS]

Options:
      --port <PORT>            Support for discrete numbers [default: 22] [possible values: 22, 80]
      --log-level <LOG_LEVEL>  Support enums from a foreign crate that don't implement `ValueEnum` [default: info] [possible values: trace, debug, info, warn, error]
  -h, --help                   Print help

```

Discrete numbers
```console
$ typed-derive builtin --port 22
Builtin(BuiltInParsers { port: 22, log_level: Info })

$ typed-derive builtin --port 80
Builtin(BuiltInParsers { port: 80, log_level: Info })

$ typed-derive builtin --port
? failed
error: a value is required for '--port <PORT>' but none was supplied
  [possible values: 22, 80]

For more information, try '--help'.

$ typed-derive builtin --port 3000
? failed
error: invalid value '3000' for '--port <PORT>'
  [possible values: 22, 80]

For more information, try '--help'.

```

Enums from crates that can't implement `ValueEnum`
```console
$ typed-derive builtin --log-level debug
Builtin(BuiltInParsers { port: 22, log_level: Debug })

$ typed-derive builtin --log-level error
Builtin(BuiltInParsers { port: 22, log_level: Error })

$ typed-derive builtin --log-level
? failed
error: a value is required for '--log-level <LOG_LEVEL>' but none was supplied
  [possible values: trace, debug, info, warn, error]

For more information, try '--help'.

$ typed-derive builtin --log-level critical
? failed
error: invalid value 'critical' for '--log-level <LOG_LEVEL>'
  [possible values: trace, debug, info, warn, error]

For more information, try '--help'.

```

## Custom [`TypedValueParser`][crate::builder::TypedValueParser]

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
