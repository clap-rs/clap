**This requires enabling the [`derive` feature flag][crate::_features].**

Git is an example of several common subcommand patterns.

Help:
```console
$ git-derive
? failed
git 
A fictional versioning CLI

Usage:
    git-derive[EXE] <SUBCOMMAND>

Options:
    -h, --help    Print help information

Subcommands:
    clone    Clones repos
    push     pushes things
    add      adds things
    stash    
    help     Print this message or the help of the given subcommand(s)

$ git-derive help
git 
A fictional versioning CLI

Usage:
    git-derive[EXE] <SUBCOMMAND>

Options:
    -h, --help    Print help information

Subcommands:
    clone    Clones repos
    push     pushes things
    add      adds things
    stash    
    help     Print this message or the help of the given subcommand(s)

$ git-derive help add
git-add 
adds things

Usage:
    git-derive[EXE] add <PATH>...

Arguments:
    <PATH>...    Stuff to add

Options:
    -h, --help    Print help information

```

A basic argument:
```console
$ git-derive add
? failed
git-add 
adds things

Usage:
    git-derive[EXE] add <PATH>...

Arguments:
    <PATH>...    Stuff to add

Options:
    -h, --help    Print help information

$ git-derive add Cargo.toml Cargo.lock
Adding ["Cargo.toml", "Cargo.lock"]

```

Default subcommand:
```console
$ git-derive stash -h
git-stash 

Usage:
    git-derive[EXE] stash [OPTIONS]
    git-derive[EXE] stash <SUBCOMMAND>

Options:
    -m, --message <MESSAGE>    
    -h, --help                 Print help information

Subcommands:
    push     
    pop      
    apply    
    help     Print this message or the help of the given subcommand(s)

$ git-derive stash push -h
git-stash-push 

Usage:
    git-derive[EXE] stash push [OPTIONS]

Options:
    -m, --message <MESSAGE>    
    -h, --help                 Print help information

$ git-derive stash pop -h
git-stash-pop 

Usage:
    git-derive[EXE] stash pop [STASH]

Arguments:
    <STASH>    

Options:
    -h, --help    Print help information

$ git-derive stash -m "Prototype"
Pushing StashPush { message: Some("Prototype") }

$ git-derive stash pop
Popping None

$ git-derive stash push -m "Prototype"
Pushing StashPush { message: Some("Prototype") }

$ git-derive stash pop
Popping None

```

External subcommands:
```console
$ git-derive custom-tool arg1 --foo bar
Calling out to "custom-tool" with ["arg1", "--foo", "bar"]

```
