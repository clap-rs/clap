*Jump to [source](git_derive.rs)*

**This requires enabling the `derive` feature flag.**

Git is an example of several common subcommand patterns.

Help:
```bash
$ git_derive
? failed
git 

A fictional versioning CLI

USAGE:
    git_derive[EXE] <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add      adds things
    clone    Clones repos
    help     Print this message or the help of the given subcommand(s)
    push     pushes things
$ git_derive help
git 

A fictional versioning CLI

USAGE:
    git_derive[EXE] <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add      adds things
    clone    Clones repos
    help     Print this message or the help of the given subcommand(s)
    push     pushes things
$ git_derive help add
git_derive[EXE]-add 

adds things

USAGE:
    git_derive[EXE] add <PATH>...

ARGS:
    <PATH>...    Stuff to add

OPTIONS:
    -h, --help    Print help information
```

A basic argument:
```bash
$ git_derive add
? failed
git_derive[EXE]-add 

adds things

USAGE:
    git_derive[EXE] add <PATH>...

ARGS:
    <PATH>...    Stuff to add

OPTIONS:
    -h, --help    Print help information
$ git_derive add Cargo.toml Cargo.lock
Adding ["Cargo.toml", "Cargo.lock"]
```

External subcommands:
```bash
$ git_derive custom-tool arg1 --foo bar
Calling out to "custom-tool" with ["arg1", "--foo", "bar"]
```
