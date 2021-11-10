Args describe a possible valid argument which may be supplied by the user at runtime. There
are three different types of arguments (flags, options, and positional) as well as a fourth
special type of argument, called Subcommands (which will be discussed separately).

# Help and Version

`clap` automatically generates a help and version flag for you, unless you specify your
own. By default help uses "-h" and "--help", and version uses "-V" and "--version". You can
safely override "-V" and "-h" to your own arguments, and "--help" and "--version" will still
be automatically generated for you.

```bash
$ 03_args --help
MyApp 

USAGE:
    03_args [OPTIONS] <input> [output]

ARGS:
    <input>     the input file to use
    <output>    Supply an output file to use

OPTIONS:
    -c, --config <config>    sets the config file to use
    -d                       turn on debugging information
    -h, --help               Print help information
    -i, --int <IFACE>        Set an interface to use
        --license            display the license file
```
