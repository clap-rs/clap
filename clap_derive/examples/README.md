# Collection of examples "how to use `clap_derive`"

### [Help on the bottom](after_help.rs)

How to append a postscript to the help message generated.

### [At least N](at_least_two.rs)

How to require presence of at least N values, like `val1 val2 ... valN ... valM`.

### [Basic](basic.rs)

A basic example how to use `clap_derive`.

### [Deny missing docs](deny_missing_docs.rs)

**This is not an example but a test**, it should be moved to `tests` folder
as soon as [this](https://github.com/rust-lang/rust/issues/24584) is fixed (if ever).

### [Doc comments](doc_comments.rs)

How to use doc comments in place of `help/long_help`.

### [Enums as arguments](enum_in_args.rs)

How to use `arg_enum!` with `clap_derive`.

### [Arguments of subcommands in separate `struct`](enum_tuple.rs)

How to extract subcommands' args into external structs.

### [Environment variables](env.rs)

How to use environment variable fallback an how it interacts with `default_value`.

### [Advanced](example.rs)

Somewhat complex example of usage of `clap_derive`.

### [Flatten](flatten.rs)

How to use `#[clap(flatten)]`

### [Git](git.rs)

Pseudo-`git` example, shows how to use subcommands and how to document them.

### [Groups](group.rs)

Using `clap::Arg::group` with `clap`.

### [`key=value` pairs](keyvalue.rs)

How to parse `key=value` pairs.

### [`--no-*` flags](negative_flag.rs)

How to add `no-thing` flag which is `true` by default and `false` if passed.

### [No version](no_version.rs)

How to completely remove version.

### [Rename all](rename_all.rs)

How `#[clap(rename_all)]` works.

### [Skip](skip.rs)

How to use `#[clap(skip)]`.

### [Aliases](subcommand_aliases.rs)

How to use aliases

### [`true` or `false`](true_or_false.rs)

How to express "`"true"` or `"false"` argument.
