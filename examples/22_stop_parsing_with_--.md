You can use `--` to escape further arguments.

Let's see what this looks like in the help:
```bash
$ 22_stop_parsing_with_-- --help
myprog 

USAGE:
    22_stop_parsing_with_-- [OPTIONS] [-- <slop>...]

ARGS:
    <slop>...    

OPTIONS:
    -f              
    -h, --help      Print help information
    -p <pea>        
```

Here is a baseline without any arguments:
```bash
$ 22_stop_parsing_with_--
-f used: false
-p's value: None
'slops' values: None
```

Notice that we can't pass positional arguments before `--`:
```bash
$ 22_stop_parsing_with_-- foo bar
? failed
error: Found argument 'foo' which wasn't expected, or isn't valid in this context

USAGE:
    22_stop_parsing_with_-- [OPTIONS] [-- <slop>...]

For more information try --help
```

But you can after:
```bash
$ 22_stop_parsing_with_-- -f -p=bob -- sloppy slop slop
-f used: true
-p's value: Some("bob")
'slops' values: Some(["sloppy", "slop", "slop"])
```

As mentioned, the parser will directly pass everything through:
```bash
$ 22_stop_parsing_with_-- -- -f -p=bob sloppy slop slop
-f used: false
-p's value: None
'slops' values: Some(["-f", "-p=bob", "sloppy", "slop", "slop"])
```
