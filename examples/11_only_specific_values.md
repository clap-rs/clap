If you have arguments of specific values you want to test for, you can use the
.possible_values() method of Arg

This allows you specify the valid values for that argument. If the user does not use one of
those specific values, they will receive a graceful exit with error message informing them
of the mistake, and what the possible valid values are

For this example, assume you want one positional argument of either "fast" or "slow"
i.e. the only possible ways to run the program are "myprog fast" or "myprog slow"

```bash
$ 11_only_specific_values fast
Hare
$ 11_only_specific_values slow
Tortoise
```

Anything else will error, guiding the user to a valid value:
```bash
$ 11_only_specific_values medium
? failed
error: "medium" isn't a valid value for '<MODE>'
	[possible values: fast, slow]

USAGE:
    11_only_specific_values <MODE>

For more information try --help
```

Valid values also get shown in the help:
```bash
$ 11_only_specific_values --help
myapp 

does awesome things

USAGE:
    11_only_specific_values <MODE>

ARGS:
    <MODE>    What mode to run the program in [possible values: fast, slow]

OPTIONS:
    -h, --help    Print help information
```

For integrating this with enums, see [13_enum_values](13_enum_values.md)
