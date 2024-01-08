`find` is an example of position-sensitive flags

```console
$ find --help
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage: find[EXE] [OPTIONS]

Options:
  -h, --help     Print help
  -V, --version  Print version

TESTS:
      --empty [<empty>]  File is empty and is either a regular file or a directory [default: false]
                         [possible values: true, false]
      --name <name>      Base of file name (the path with the leading directories removed) matches
                         shell pattern pattern

OPERATORS:
  -o, --or [<or>]    expr2 is not evaluate if exp1 is true [default: false] [possible values: true,
                     false]
  -a, --and [<and>]  Same as `expr1 expr1` [default: false] [possible values: true, false]

$ find --empty -o --name .keep
[
    (
        "empty",
        Bool(
            true,
        ),
    ),
    (
        "or",
        Bool(
            true,
        ),
    ),
    (
        "name",
        String(
            ".keep",
        ),
    ),
]

$ find --empty -o --name .keep -o --name foo
[
    (
        "empty",
        Bool(
            true,
        ),
    ),
    (
        "or",
        Bool(
            true,
        ),
    ),
    (
        "name",
        String(
            ".keep",
        ),
    ),
    (
        "or",
        Bool(
            true,
        ),
    ),
    (
        "name",
        String(
            "foo",
        ),
    ),
]

```

