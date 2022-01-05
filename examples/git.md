*Jump to [source](git.rs)*

Git is an example of several common subcommand patterns.

Help:
```console
$ git
? failed
git 
A fictional versioning CLI

USAGE:
    git[EXE] <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add      adds things
    clone    Clones repos
    help     Print this message or the help of the given subcommand(s)
    push     pushes things

$ git help
git 
A fictional versioning CLI

USAGE:
    git[EXE] <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add      adds things
    clone    Clones repos
    help     Print this message or the help of the given subcommand(s)
    push     pushes things

$ git help add
git[EXE]-add 
adds things

USAGE:
    git[EXE] add <PATH>...

ARGS:
    <PATH>...    Stuff to add

OPTIONS:
    -h, --help    Print help information

```

A basic argument:
```console
$ git add
? failed
git[EXE]-add 
adds things

USAGE:
    git[EXE] add <PATH>...

ARGS:
    <PATH>...    Stuff to add

OPTIONS:
    -h, --help    Print help information

$ git add Cargo.toml Cargo.lock
Adding ["Cargo.toml", "Cargo.lock"]

```

External subcommands:
```console
$ git custom-tool arg1 --foo bar
Calling out to "custom-tool" with ["arg1", "--foo", "bar"]

```
