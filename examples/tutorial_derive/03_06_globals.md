```console
$ 03_06_globals_derive help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

USAGE:
    03_06_globals_derive[EXE] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    --lowercase      Converts any file names to lowercase.
    -V, --version    Print version information

SUBCOMMANDS:
    add     Adds files to myapp
    help    Print this message or the help of the given subcommand(s)

$ 03_06_globals_derive help add
clap-add [..]
Adds files to myapp

USAGE:
    03_06_globals_derive[EXE] add [NAME]

ARGS:
    <NAME>    

OPTIONS:
    -h, --help       Print help information
    --lowercase      Converts any file names to lowercase.
    -V, --version    Print version information

$ 03_06_globals_derive add bob
'myapp add' was used, name is: Some("bob")

```

The `--lowercase` argument can be used at any level, top-level or in any subcommand:
```console
$ 03_06_globals_derive add MyFileName
'myapp add' was used, name is: Some("MyFileName")

$ 03_06_globals_derive --lowercase add MyFileName
'myapp add' was used, name is: Some("myfilename")

$ 03_06_globals_derive add MyFileName --lowercase
'myapp add' was used, name is: Some("myfilename")
```
