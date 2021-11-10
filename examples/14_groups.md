`ArgGroup`s are a family of related arguments and way for you to say, "Any of these arguments".
By placing arguments in a logical group, you can make easier requirement and exclusion rules
instead of having to list each individually, or when you want a rule to apply "any but not all"
arguments.

Perhaps the most common use of `ArgGroup`s is to require one and *only* one argument to be
present out of a given set. Imagine that you had multiple arguments, and you want one of them to
be required, but making all of them required isn't feasible because perhaps they conflict with
each other. For example, lets say that you were building an application where one could set a
given version number by supplying a string with an option argument, i.e. `--set-ver v1.2.3`, you
also wanted to support automatically using a previous version number and simply incrementing one
of the three numbers. So you create three flags `--major`, `--minor`, and `--patch`. All of
these arguments shouldn't be used at one time but you want to specify that *at least one* of
them is used. For this, you can create a group.
```bash
$ 14_groups
? failed
error: The following required arguments were not provided:
    <--set-ver <ver>|--major|--minor|--patch>

USAGE:
    14_groups[EXE] [OPTIONS] <--set-ver <ver>|--major|--minor|--patch> [INPUT_FILE]

For more information try --help
$ 14_groups --major
Version: 2.2.3
$ 14_groups --major --minor
? failed
error: The argument '--major' cannot be used with '--minor'

USAGE:
    14_groups[EXE] [OPTIONS] <--set-ver <ver>|--major|--minor|--patch> [INPUT_FILE]

For more information try --help
```

You can also do things such as name an ArgGroup as a confliction or requirement, meaning any
of the arguments that belong to that group will cause a failure if present, or must present
respectively.
```bash
$ 14_groups --major -c config.toml
? failed
error: The following required arguments were not provided:
    <INPUT_FILE|--spec-in <SPEC_IN>>

USAGE:
    14_groups[EXE] -c <config> <--set-ver <ver>|--major|--minor|--patch> <INPUT_FILE|--spec-in <SPEC_IN>>

For more information try --help
$ 14_groups --major -c config.toml --spec-in input.txt
Version: 2.2.3
Doing work using input input.txt and config config.toml
```
