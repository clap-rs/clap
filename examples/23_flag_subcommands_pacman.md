This feature allows users of the app to pass subcommands in the fashion of short or long flags.
You may be familiar with it if you ever used [`pacman`](https://wiki.archlinux.org/index.php/pacman).
Some made up examples of what flag subcommands are:

Here, `-S` is a short flag subcommand:
```bash
$ 23_flag_subcommands_pacman -S package
Installing package...
```

Here `--sync` is a long flag subcommand:
```bash
$ 23_flag_subcommands_pacman --sync package
Installing package...
```

Now the short flag subcommand (`-S`) with a long flag:
```bash
$ 23_flag_subcommands_pacman -S --search name
Searching for name...
```

And the various forms of short flags that work:
```
$ 23_flag_subcommands_pacman -S -s name
Searching for name...
$ 23_flag_subcommands_pacman -Ss name
Searching for name...
```
*(users can "stack" short subcommands with short flags or with other short flag subcommands)*

**NOTE:** Keep in mind that subcommands, flags, and long flags are *case sensitive*: `-Q` and `-q` are different flags/subcommands. For example, you can have both `-Q` subcommand and `-q` flag, and they will be properly disambiguated.
Let's make a quick program to illustrate.
