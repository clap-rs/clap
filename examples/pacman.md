*Jump to [source](pacman.rs)*

[`pacman`](https://wiki.archlinux.org/index.php/pacman) defines subcommands via flags.

Here, `-S` is a short flag subcommand:
```console
$ pacman -S package
Installing package...

```

Here `--sync` is a long flag subcommand:
```console
$ pacman --sync package
Installing package...

```

Now the short flag subcommand (`-S`) with a long flag:
```console
$ pacman -S --search name
Searching for name...

```

And the various forms of short flags that work:
```console
$ pacman -S -s name
Searching for name...

$ pacman -Ss name
Searching for name...

```
*(users can "stack" short subcommands with short flags or with other short flag subcommands)*

**NOTE:** Keep in mind that subcommands, flags, and long flags are *case sensitive*: `-Q` and `-q` are different flags/subcommands. For example, you can have both `-Q` subcommand and `-q` flag, and they will be properly disambiguated.
Let's make a quick program to illustrate.
