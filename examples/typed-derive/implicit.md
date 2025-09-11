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
