*Jump to [source](escaped_positional.rs)*

**This requires enabling the `cargo` feature flag.**

You can use `--` to escape further arguments.

Let's see what this looks like in the help:
```bash
$ escaped_positional --help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    escaped_positional[EXE] [OPTIONS] [-- <SLOP>...]

ARGS:
    <SLOP>...    

OPTIONS:
    -f               
    -h, --help       Print help information
    -p <PEAR>        
    -V, --version    Print version information
```

Here is a baseline without any arguments:
```bash
$ escaped_positional
-f used: false
-p's value: None
'slops' values: []
```

Notice that we can't pass positional arguments before `--`:
```bash
$ escaped_positional foo bar
? failed
error: Found argument 'foo' which wasn't expected, or isn't valid in this context

USAGE:
    escaped_positional[EXE] [OPTIONS] [-- <SLOP>...]

For more information try --help
```

But you can after:
```bash
$ escaped_positional -f -p=bob -- sloppy slop slop
-f used: true
-p's value: Some("bob")
'slops' values: ["sloppy", "slop", "slop"]
```

As mentioned, the parser will directly pass everything through:
```bash
$ escaped_positional -- -f -p=bob sloppy slop slop
-f used: false
-p's value: None
'slops' values: ["-f", "-p=bob", "sloppy", "slop", "slop"]
```
