In order to use YAML to define your CLI you must compile clap with the "yaml" feature because
it's **not** included by default.

In order to do this, ensure your Cargo.toml looks like one of the following:

```toml
[dependencies]
clap = { features = ["yaml"] }
```

__OR__

```toml
[dependencies.clap]
features = ["yaml"]
```

This then works like any other clap app:
```bash
$ 17_yaml --help
yaml_app 1.0

Kevin K. <kbknapp@gmail.com>

an example using a .yaml file to build a CLI

USAGE:
    17_yaml [OPTIONS] [pos] [SUBCOMMAND]

ARGS:
    <pos>    example positional argument from yaml [possible values: fast, slow]

OPTIONS:
    -F <flag>                      demo flag argument
    -h, --help                     Print help information
        --max-vals <maxvals>...    you can only supply a max of 3 values for me!
        --min-vals <minvals>...    you must supply at least two values to satisfy me
        --mode <mode>              shows an option with specific values [possible values: vi, emacs]
        --mult-vals <two>          demos an option which has two named values
    -o, --option <opt>             example option argument from yaml
    -V, --version                  Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    subcmd    demos subcommands from yaml
$ 17_yaml
? failed
yaml_app 1.0

Kevin K. <kbknapp@gmail.com>

an example using a .yaml file to build a CLI

USAGE:
    17_yaml [OPTIONS] [pos] [SUBCOMMAND]

ARGS:
    <pos>    example positional argument from yaml [possible values: fast, slow]

OPTIONS:
    -F <flag>                      demo flag argument
    -h, --help                     Print help information
        --max-vals <maxvals>...    you can only supply a max of 3 values for me!
        --min-vals <minvals>...    you must supply at least two values to satisfy me
        --mode <mode>              shows an option with specific values [possible values: vi, emacs]
        --mult-vals <two>          demos an option which has two named values
    -o, --option <opt>             example option argument from yaml
    -V, --version                  Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    subcmd    demos subcommands from yaml
$ 17_yaml --mode vi
You are using vi
$ 17_yaml --mode joe
? failed
error: "joe" isn't a valid value for '--mode <mode>'
	[possible values: emacs, vi]

USAGE:
    17_yaml --mode <mode>

For more information try --help
```
