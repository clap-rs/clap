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
    clone    Clones repos
    push     pushes things
    add      adds things
    stash    
    help     Print this message or the help of the given subcommand(s)

$ git help
git 
A fictional versioning CLI

USAGE:
    git[EXE] <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    clone    Clones repos
    push     pushes things
    add      adds things
    stash    
    help     Print this message or the help of the given subcommand(s)

$ git help add
git-add 
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
git-add 
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

Default subcommand:
```console
$ git stash -h
git-stash 

USAGE:
    git[EXE] stash [OPTIONS]
    git[EXE] stash <SUBCOMMAND>

OPTIONS:
    -m, --message <MESSAGE>    
    -h, --help                 Print help information

SUBCOMMANDS:
    push     
    pop      
    apply    
    help     Print this message or the help of the given subcommand(s)

$ git stash push -h
git-stash-push 

USAGE:
    git[EXE] stash push [OPTIONS]

OPTIONS:
    -m, --message <MESSAGE>    
    -h, --help                 Print help information

$ git stash pop -h
git-stash-pop 

USAGE:
    git[EXE] stash pop [STASH]

ARGS:
    <STASH>    

OPTIONS:
    -h, --help    Print help information

$ git stash -m "Prototype"
Pushing Some("Prototype")

$ git stash pop
Popping None

$ git stash push -m "Prototype"
Pushing Some("Prototype")

$ git stash pop
Popping None

```

External subcommands:
```console
$ git custom-tool arg1 --foo bar
Calling out to "custom-tool" with ["arg1", "--foo", "bar"]

```
