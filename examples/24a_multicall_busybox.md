Example of a busybox-style multicall program

See the documentation for clap::AppSettings::Multicall for rationale.

This example omits every command except true and false,
which are the most trivial to implement,
```bash,ignore
$ busybox true
? 0
$ busybox false
? 1
```
*Note: without the links setup, we can't demonostrate the multicall behavior*

But includes the `--install` option as an example of why it can be useful
for the main program to take arguments that aren't applet subcommands.
```bash,ignore
$ busybox --install
? failed
...
```

Though users must pass something:
```bash,ignore
$ busybox
? failed
busybox 

USAGE:
    busybox[EXE] [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help                 Print help information
        --install <install>    Install hardlinks for all subcommands in path

SUBCOMMANDS:
    false    does nothing unsuccessfully
    help     Print this message or the help of the given subcommand(s)
    true     does nothing successfully
```
