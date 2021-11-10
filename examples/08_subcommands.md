Subcommands function exactly like sub-Apps, because that's exactly what they are. Each
instance of a Subcommand can have its own version, author(s), Args, and even its own
subcommands.

Just like Apps, each subcommand will get its own "help" and "version" flags automatically
generated. Also, like Apps, you can override "-V" or "-h" safely and still get "--help" and
"--version" auto generated.

**NOTE:** If you specify a subcommand for your App, clap will also autogenerate a "help"
subcommand along with "-h" and "--help" (applies to sub-subcommands as well).

```bash
$ 08_subcommands help
MyApp 1.0

USAGE:
    08_subcommands [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add     Adds files to myapp
    help    Print this message or the help of the given subcommand(s)
$ 08_subcommands help add
08_subcommands-add 0.1

Kevin K.

Adds files to myapp

USAGE:
    08_subcommands add <input>

ARGS:
    <input>    the file to add

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
$ 08_subcommands add help
'myapp add' was used, input is: help
```

```bash
$ 08_subcommands --version
MyApp 1.0
```

Without any subcommand:
```bash
$ 08_subcommands
No subcommand was used
```

And with:
```bash
$ 08_subcommands add input.txt
'myapp add' was used, input is: input.txt
```
