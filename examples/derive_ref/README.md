# Derive Reference

1. [Overview](#overview)
2. [Raw Attributes](#raw-attributes)
3. [Magic Attributes](#magic-attributes)
    1. [App Attributes](#app-attributes)
    2. [Arg Attributes](#arg-attributes)
    3. [Arg Types](#arg-types)
    4. [Arg Enum Attributes](#arg-enum-attributes)
    5. [Possible Value Attributes](#possible-value-attributes)
    6. [Doc Comments](#doc-comments)

## Overview

To derive `clap` types, you need to enable the `derive` feature flag.

See [demo.rs](../demo.rs) and [demo.md](../demo.md) for a brief example.

Let's start by breaking down what can go where:
```rust
use clap::{Parser, Args, Subcommand, ArgEnum};

/// Doc comment
#[derive(Parser)]
#[clap(APP ATTRIBUTE)]
struct Cli {
    /// Doc comment
    #[clap(ARG ATTRIBUTE)]
    field: Type,

    #[clap(flatten)]
    delegate: Struct,

    #[clap(subcommand)]
    command: Command,
}

/// Doc comment
#[derive(Args)]
#[clap(PARENT APP ATTRIBUTE)]
struct Struct { 
    /// Doc comment
    #[clap(ARG ATTRIBUTE)]
    field: Type,
}

/// Doc comment
#[derive(Subcommand)]
#[clap(PARENT APP ATTRIBUTE)]
enum Command {
    /// Doc comment
    #[clap(APP ATTRIBUTE)]
    Variant1(Struct),

    /// Doc comment
    #[clap(APP ATTRIBUTE)]
    Variant2 {
        /// Doc comment
        #[clap(ARG ATTRIBUTE)]
        field: Type,
    }
}

/// Doc comment
#[derive(ArgEnum)]
#[clap(ARG ENUM ATTRIBUTE)]
enum Mode {
    /// Doc comment
    #[clap(POSSIBLE VALUE ATTRIBUTE)]
    Variant1,
}

fn main() {
    let cli = Cli::parse();
}
```

- `Parser` parses arguments into a `struct` (arguments) or `enum` (subcommands).
- `Args` allows defining a set of re-usable arguments that get merged into their parent container.
- `Subcommand` defines available subcommands.
- `ArgEnum` allows parsing a value directly into an `enum`, erroring on unsupported values.

See also the [tutorial](../tutorial_derive/README.md) and [examples](../README.md).

## Raw Attributes

**Raw attributes** are forwarded directly to the underlying `clap` builder.  Any
`App`, `Arg`, or `PossibleValue` method can be used as an attribute.

Raw attributes come in two different syntaxes:
```rust
#[clap(
    global = true, // name = arg form, neat for one-arg methods
    required_if_eq("out", "file") // name(arg1, arg2, ...) form.
)]
```

- `method = arg` can only be used for methods which take only one argument.
- `method(arg1, arg2)` can be used with any method.

As long as `method_name` is not one of the magical methods - it will be
translated into a mere method call.

## Magic Attributes

**Magic attributes** have post-processing done to them, whether that is
- Providing of defaults
- Special behavior is triggered off of it

### App Attributes

These correspond to a `clap::App` which is used for both top-level parsers and
when defining subcommands.

In addition to the raw attributes, the following magic attributes are supported:
- `name  = <expr>`: `clap::App::name`
  - When not present: [crate `name`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field) (`Parser` container), variant name (`Subcommand` variant)
- `version [= <expr>]`: `clap::App::version`
  - When not present: no version set
  - Without `<expr>`: defaults to [crate `version`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)
- `author [= <expr>]`: `clap::App::author`
  - When not present: no author set
  - Without `<expr>`: defaults to [crate `authors`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-authors-field)
- `about [= <expr>]`: `clap::App::about`
  - When not present: [Doc comment summary](#doc-comments)
  - Without `<expr>`: [crate `description`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-description-field) (`Parser` container)
    - **TIP:** When a doc comment is also present, you most likely want to add
      `#[clap(long_about = None)]` to clear the doc comment so only `about`
      gets shown with both `-h` and `--help`.
- `long_about = <expr>`: `clap::App::long_about`
  - When not present: [Doc comment](#doc-comments) if there is a blank line, else nothing
- `verbatim_doc_comment`: Minimizes pre-processing when converting doc comments to `about` / `long_about`
- `help_heading`: `clap::App::help_heading`
  - When `flatten`ing `Args`, this is scoped to just the args in this struct and any struct `flatten`ed into it
- `rename_all = <expr>`: Override default field / variant name case conversion for `App::name` / `Arg::name`
  - When not present: `kebab-case`
  - Available values: `camelCase`, `kebab-case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, `snake_case`, `lower`, `UPPER`, `verbatim`
- `rename_all_env = <expr>`: Override default field name case conversion for env variables for  `clap::Arg::env`
  - When not present: `SCREAMING_SNAKE_CASE`
  - Available values: `camelCase`, `kebab-case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, `snake_case`, `lower`, `UPPER`, `verbatim`

And for `Subcommand` variants:
- `skip`: Ignore this variant
- `flatten`: Delegates to the variant for more subcommands (must implement `Subcommand`)
- `subcommand`: Nest subcommands under the current set of subcommands (must implement `Subcommand`)
- `external_subcommand`: `clap::AppSettings::AllowExternalSubcommand`
  - Variant must be either `Variant(Vec<String>)` or `Variant(Vec<OsString>)`

### Arg Attributes

These correspond to a `clap::Arg`.

In addition to the raw attributes, the following magic attributes are supported:
- `name = <expr>`: `clap::Arg::new`
  - When not present: case-converted field name is used
- `help = <expr>`: `clap::Arg::help`
  - When not present: [Doc comment summary](#doc-comments)
- `long_help = <expr>`: `clap::Arg::long_help`
  - When not present: [Doc comment](#doc-comments) if there is a blank line, else nothing
- `verbatim_doc_comment`: Minimizes pre-processing when converting doc comments to `help` / `long_help`
- `short [= <char>]`: `clap::Arg::short`
  - When not present: no short set
  - Without `<char>`: defaults to first character in the case-converted field name
- `long [= <str>]`: `clap::Arg::long`
  - When not present: no long set
  - Without `<str>`: defaults to the case-converted field name
- `env [= <str>]`: `clap::Arg::env`
  - When not present: no env set
  - Without `<str>`: defaults to the case-converted field name
- `flatten`: Delegates to the field for more arguments (must implement `Args`)
  - Only `help_heading` can be used with `flatten`.  See
    [clap-rs/clap#3269](https://github.com/clap-rs/clap/issues/3269) for why
    arg attributes are not generally supported.
  - **Tip:** Though we do apply a flattened `Args`'s Parent App Attributes, this
    makes reuse harder. Generally prefer putting the app attributes on the `Parser`
    or on the flattened field.
- `subcommand`: Delegates definition of subcommands to the field (must implement `Subcommand`)
  - When `Option<T>`, the subcommand becomes optional
- `from_global`: Read a `clap::Arg::global` argument (raw attribute), regardless of what subcommand you are in 
- `parse(<kind> [= <function>])` `clap::Arg::validator`
- `arg_enum`: Parse the value using the `ArgEnum` trait
- `skip [= <expr>]`: Ignore this field, filling in with `<expr>`
  - Without `<expr>`: fills the field with `Default::default()`
- `default_value = <str>`: `clap::Arg::default_value` and `clap::Arg::required(false)`
- `default_value_t [= <expr>]`: `clap::Arg::default_value` and `clap::Arg::required(false)`
  - Requires `std::fmt::Display` or `#[clap(arg_enum)]`
  - Without `<expr>`, relies on `Default::default()`

### Arg Types

`clap` assumes some intent based on the type used:

| Type                | Effect                               | Implies                                                          |
|---------------------|--------------------------------------|------------------------------------------------------------------|
| `bool`              | flag                                 | `#[clap(parser(from_flag))]`                                     |
| `Option<T>`         | optional argument                    | `.takes_value(true).required(false)`                             |
| `Option<Option<T>>` | optional value for optional argument | `.takes_value(true).required(false).min_values(0).max_values(1)` |
| `T`                 | required argument                    | `.takes_value(true).required(!has_default)`                      |
| `Vec<T>`            | `0..` occurrences of argument        | `.takes_value(true).required(false).multiple_occurrences(true)`  |
| `Option<Vec<T>>`    | `0..` occurrences of argument        | `.takes_value(true).required(false).multiple_occurrences(true)`  |

Notes:
- For custom type behavior, you can override the implied attributes/settings and/or set additional ones
  - For example, see [custom-bool](./custom-bool.md)
- `Option<Vec<T>>` will be `None` instead of `vec![]` if no arguments are provided.
  - This gives the user some flexibility in designing their argument, like with `min_values(0)`

You can then support your custom type with `#[clap(parse(<kind> [= <function>]))]`:

| `<kind>`                 | Signature                             | Default `<function>`            |
|--------------------------|---------------------------------------|---------------------------------|
| `from_str`               | `fn(&str) -> T`                       | `::std::convert::From::from`    |
| `try_from_str` (default) | `fn(&str) -> Result<T, E>`            | `::std::str::FromStr::from_str` |
| `from_os_str`            | `fn(&OsStr) -> T`                     | `::std::convert::From::from`    |
| `try_from_os_str`        | `fn(&OsStr) -> Result<T, OsString>`   | (no default function)           |
| `from_occurrences`       | `fn(u64) -> T`                        | `value as T`                    |
| `from_flag`              | `fn(bool) -> T`                       | `::std::convert::From::from`    |

Notes:
- `from_os_str`:
  - Implies `arg.takes_value(true).allow_invalid_utf8(true)`
- `try_from_os_str`:
  - Implies `arg.takes_value(true).allow_invalid_utf8(true)`
- `from_occurrences`:
  - Implies `arg.takes_value(false).multiple_occurrences(true)`
  - Reads from `clap::ArgMatches::occurrences_of` rather than a `value_of` function
- `from_flag`
  - Implies `arg.takes_value(false)`
  - Reads from `clap::ArgMatches::is_present` rather than a `value_of` function

**Warning:**
- To support non-UTF8 paths, you must use `parse(from_os_str)`, otherwise
  `clap` will use `clap::ArgMatches::value_of` with `PathBuf::FromStr`.

### Arg Enum Attributes

- `rename_all = <expr>`: Override default field / variant name case conversion for `PossibleValue::new`
  - When not present: `kebab-case`
  - Available values: `camelCase`, `kebab-case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, `snake_case`, `lower`, `UPPER`, `verbatim`

### Possible Value Attributes

These correspond to a `clap::PossibleValue`.

- `name = <expr>`: `clap::PossibleValue::new`
  - When not present: case-converted field name is used
- `help = <expr>`: `clap::PossibleValue::help`
  - When not present: [Doc comment summary](#doc-comments)

### Doc Comments

In clap, help messages for the whole binary can be specified
via [`App::about`] and [`App::long_about`] while help messages
for individual arguments can be specified via [`Arg::help`] and [`Arg::long_help`]".

`long_*` variants are used when user calls the program with
`--help` and "short" variants are used with `-h` flag.

```rust
# use clap::Parser;

#[derive(Parser)]
#[clap(about = "I am a program and I work, just pass `-h`", long_about = None)]
struct Foo {
    #[clap(short, help = "Pass `-h` and you'll see me!")]
    bar: String,
}
```

For convenience, doc comments can be used instead of raw methods
(this example works exactly like the one above):

```rust
# use clap::Parser;

#[derive(Parser)]
/// I am a program and I work, just pass `-h`
struct Foo {
    /// Pass `-h` and you'll see me!
    bar: String,
}
```

**NOTE:** Attributes have priority over doc comments!

**Top level doc comments always generate `App::about/long_about` calls!**
If you really want to use the `App::about/long_about` methods (you likely don't),
use the `about` / `long_about` attributes to override the calls generated from
the doc comment.  To clear `long_about`, you can use
`#[clap(long_about = None)]`.

**TIP:** Set `#![deny(missing_docs)]` to catch missing `--help` documentation at compile time.

#### Pre-processing

```rust
# use clap::Parser;
#[derive(Parser)]
/// Hi there, I'm Robo!
///
/// I like beeping, stumbling, eating your electricity,
/// and making records of you singing in a shower.
/// Pay up, or I'll upload it to youtube!
struct Robo {
    /// Call my brother SkyNet.
    ///
    /// I am artificial superintelligence. I won't rest
    /// until I'll have destroyed humanity. Enjoy your
    /// pathetic existence, you mere mortals.
    #[clap(long)]
    kill_all_humans: bool,
}
```

A doc comment consists of three parts:
- Short summary
- A blank line (whitespace only)
- Detailed description, all the rest

The summary corresponds with `App::about` / `Arg::help`.  When a blank line is
present, the whole doc comment will be passed to `App::long_about` /
`Arg::long_help`.  Or in other words, a doc may result in just a `App::about` /
`Arg::help` or `App::about` / `Arg::help` and `App::long_about` /
`Arg::long_help`

In addition, when `verbatim_doc_comment` is not present, `clap` applies some preprocessing, including:

- Strip leading and trailing whitespace from every line, if present.

- Strip leading and trailing blank lines, if present.

- Interpret each group of non-empty lines as a word-wrapped paragraph.

  We replace newlines within paragraphs with spaces to allow the output
  to be re-wrapped to the terminal width.

- Strip any excess blank lines so that there is exactly one per paragraph break.

- If the first paragraph ends in exactly one period,
  remove the trailing period (i.e. strip trailing periods but not trailing ellipses).

Sometimes you don't want this preprocessing to apply, for example the comment contains
some ASCII art or markdown tables, you would need to preserve LFs along with
blank lines and the leading/trailing whitespace. When you pass use the
`verbatim_doc_comment` magic attribute, you  preserve
them.

**Note:** Keep in mind that `verbatim_doc_comment` will *still*
- Remove one leading space from each line, even if this attribute is present,
  to allow for a space between `///` and the content.
- Remove leading and trailing blank lines
