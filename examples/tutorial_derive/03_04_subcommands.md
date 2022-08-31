```console
$ 03_04_subcommands_derive help
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage:
    03_04_subcommands_derive[EXE] <COMMAND>

Commands:
    add     Adds files to myapp
    help    Print this message or the help of the given subcommand(s)

Options:
    -h, --help       Print help information
    -V, --version    Print version information

$ 03_04_subcommands_derive help add
clap-add [..]
Adds files to myapp

Usage:
    03_04_subcommands_derive[EXE] add [NAME]

Arguments:
    [NAME]    

Options:
    -h, --help       Print help information
    -V, --version    Print version information

$ 03_04_subcommands_derive add bob
'myapp add' was used, name is: Some("bob")

```

Because we used `command: Commands` instead of `command: Option<Commands>`:
```console
$ 03_04_subcommands_derive
? failed
clap [..]
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage:
    03_04_subcommands_derive[EXE] <COMMAND>

Commands:
    add     Adds files to myapp
    help    Print this message or the help of the given subcommand(s)

Options:
    -h, --help       Print help information
    -V, --version    Print version information

```

Because we added `#[clap(propagate_version = true)]`:
```console
$ 03_04_subcommands_derive --version
clap [..]

$ 03_04_subcommands_derive add --version
clap-add [..]

```
