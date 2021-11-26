# clap_derive

Parse command line argument by defining a struct.  It combines
[structopt](https://github.com/TeXitoi/structopt) and
[clap](https://crates.io/crates/clap) into a single experience. This crate is
used by clap, and not meant to be used directly by consumers.

## Documentation

Find it on [Docs.rs](https://docs.rs/clap_derive).  You can also check the
[examples](https://github.com/clap-rs/clap/tree/master/clap_derive/examples)
and the [changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md).

## Example

Add `clap` to your dependencies of your `Cargo.toml`:

```toml
[dependencies]
clap = "3"
```

And then, in your rust file:

```rust,should_panic
use std::path::PathBuf;
use clap::{Parser, ValueHint};

/// A basic example
#[derive(Parser, Debug)]
#[clap(name = "basic", version)]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    /// Set speed
    #[clap(short, long, default_value = "42")]
    speed: f64,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Output file
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    output: PathBuf,

    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[clap(short = 'c', long)]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[clap(short, long)]
    level: Vec<String>,

    /// Files to process
    #[clap(name = "FILE", parse(from_os_str), value_hint = ValueHint::AnyPath)]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::parse();
    println!("{:#?}", opt);
}
```

Using this example:

```bash
$ ./basic
error: The following required arguments were not provided:
    --output <output>

USAGE:
    basic --output <output> --speed <speed>

For more information try --help
$ ./basic --help
basic 0.3.0
Guillaume Pinot <texitoi@texitoi.eu>, others
A basic example

USAGE:
    basic [OPTIONS] --output <output> [--] [file]...

ARGS:
    <FILE>...    Files to process

OPTIONS:
    -c, --nb-cars <nb-cars>    Number of cars
    -d, --debug                Activate debug mode
    -h, --help                 Print help information
    -l, --level <level>...     admin_level to consider
    -o, --output <output>      Output file
    -s, --speed <speed>        Set speed [default: 42]
    -V, --version              Print version information
    -v, --verbose              Verbose mode (-v, -vv, -vvv, etc.)

ARGS:
    <file>...    Files to process
$ ./basic -o foo.txt
Opt {
    debug: false,
    verbose: 0,
    speed: 42.0,
    output: "foo.txt",
    nb_cars: None,
    level: [],
    files: [],
}
$ ./basic -o foo.txt -dvvvs 1337 -l alice -l bob --nb-cars 4 bar.txt baz.txt
Opt {
    debug: true,
    verbose: 3,
    speed: 1337.0,
    output: "foo.txt",
    nb_cars: Some(
        4,
    ),
    level: [
        "alice",
        "bob",
    ],
    files: [
        "bar.txt",
        "baz.txt",
    ],
}
```

## Attributes

You can control the way `#[derive(Parser)]` translates your struct into an
actual [`clap::App`][] invocation via `#[clap(...)]` attributes.

The attributes fall into two categories:

- `clap::Parser`'s own [magical methods](#magical-methods).

   They are used by `clap::Parser` itself. They come mostly in
   `attr = ["whatever"]` form, but some `attr(args...)` also exist.

- [`raw` attributes](#raw-methods).

    They represent explicit `clap::Arg/App` method calls.
    They are what used to be explicit `#[clap(raw(...))]` attrs

Every `clap attribute` looks like comma-separated sequence of methods:

```rust
# #[derive(clap::Parser)] struct S {
#
#[clap(
    short, // method with no arguments - always magical
    long = "--long-option", // method with one argument
    required_if("out", "file"), // method with one and more args
    parse(from_os_str = path::to::parser) // some magical methods have their own syntax
)]
#
# s: () } mod path { pub(crate) mod to { pub(crate) fn parser(_: &std::ffi::OsStr) {} }}
```

`#[clap(...)]` attributes can be placed on top of `struct`, `enum`,
`struct` field or `enum` variant. Attributes on top of `struct` or `enum`
represent `clap::App` method calls, field or variant attributes correspond
to `clap::Arg` method calls.

In other words, the `Opt` struct from the example above
will be turned into this (*details omitted*):

```rust
# use clap::{App, Arg, crate_version};
App::new("basic")
    .version(crate_version!()) // Parsed from the Cargo.toml.
    .about("A basic example")
.arg(Arg::new("debug")
    .about("Activate debug mode")
    .short('d')
    .long("debug"))
.arg(Arg::new("speed")
    .about("Set speed")
    .short('s')
    .long("speed")
    .default_value("42"))
// and so on
# ;
```

## Raw methods

They are the reason why `clap::Parser` is so flexible. **Every and each method
from `clap::App/Arg` can be used this way!** See the [`clap::App`
methods](https://docs.rs/clap/2/clap/struct.App.html) and [`clap::Arg`
methods](https://docs.rs/clap/2/clap/struct.Arg.html).

```rust
# #[derive(clap::Parser)] struct S {
#
#[clap(
    global = true, // name = arg form, neat for one-arg methods
    required_if("out", "file") // name(arg1, arg2, ...) form.
)]
#
# s: String }
```

The first form can only be used for methods which take only one argument. The
second form must be used with multi-arg methods, but can also be used with
single-arg methods. These forms are identical otherwise.

As long as `method_name` is not one of the magical methods - it will be
translated into a mere method call.

**Note:**

"Raw methods" are direct replacement for `#[clap(raw(...))]` attributes, any
time you would have used a `raw()` attribute you should use a raw method.

## Magical methods

They are the reason why `clap::Parser` is so easy to use and convenient in most
cases. Many of them have defaults.

Methods may be used on "top level" (on top of a `struct`, `enum` or `enum` variant)
and/or on "field-level" (on top of a `struct` field or *inside* of an enum variant).
Top level (non-magical) methods correspond to `App::method` calls, field-level methods
are `Arg::method` calls.

```ignore
#[clap(top_level)]
struct Foo {
    #[clap(field_level)]
    field: u32
}

#[clap(top_level)]
enum Bar {
    #[clap(top_level)]
    Pineapple {
        #[clap(field_level)]
        chocolate: String
    },

    #[clap(top_level)]
    Orange,
}
```

- `name`: `[name = expr]`
  - Top-level: `App::new(expr)`.

    The binary name displayed in help messages. Set by default to the crate name provided by Cargo.

  - Field-level: `Arg::new(expr)`.

    The name for the argument the field stands for, this name appears in help messages.
    Defaults to a name, deduced from a field, see also
    [`rename_all`](#specifying-argument-types).

- `version`: `version [= "version"]`

    Top-level only: `App::version("version" or env!(CARGO_PKG_VERSION))`.

    The version displayed in help messages.
    A version is no longer set by default, to use the crate version provided by Cargo,
    use `#[clap(version)]` without a value.

- `author`: `author [= "author"]`

    Top-level only: `App::author("author" or env!(CARGO_PKG_AUTHORS))`.

    Author/maintainer of the binary, this name appears in help messages.
    A version is not set by default, to use the crate author given by Cargo,
    use `#[clap(author)]` without a value.

- `about`: `about [= "about"]`

    Top-level only: `App::about("about" or env!(CARGO_PKG_DESCRIPTION))`.

    Short description of the binary, appears in help messages.
    Defaults to the docstring of the top-level struct (if set).
    To use the crate author given by Cargo, use `#[clap(about)]` without a value.

- [`short`](#specifying-argument-types): `short [= "short-opt-name"]`

    Field-level only.

- [`long`](#specifying-argument-types): `long [= "long-opt-name"]`

    Field-level only.

- [`default_value`](#default-values): `default_value = "default value"`

    Field-level only.

- [`default_value_t`](#default-values): `default_value_t`

    Field-level only.

- [`rename_all`](#specifying-argument-types):
    [`rename_all = "kebab"/"snake"/"screaming-snake"/"camel"/"pascal"/"verbatim"/"lower"/"upper"]`

    Top-level or field-level.

- [`parse`](#custom-string-parsers): `parse(type [= path::to::parser::fn])`

    Field-level only.

- [`skip`](#skipping-fields): `skip [= expr]`

    Field-level only.

- [`flatten`](#flattening): `flatten`

    Field-level or on single-typed tuple variants.

- [`subcommand`](#subcommands): `subcommand`

    Field-level only.

- [`external_subcommand`](#external-subcommands)

    Usable only on enum variants.

- [`env`](#environment-variable-fallback): `env [= str_literal]`

    Field-level only.

- [`rename_all_env`](#auto-deriving-environment-variables):
    [`rename_all_env = "kebab"/"snake"/"screaming-snake"/"camel"/"pascal"/"verbatim"/"lower"/"upper"]`

    Top-level or field-level.

- [`verbatim_doc_comment`](#doc-comment-preprocessing-and-clapverbatim_doc_comment):
    `verbatim_doc_comment`

    Top-level or field-level.

## Type magic

One of major things that makes `clap::Parser` so awesome is its type magic.
Do you want optional positional argument? Use `Option<T>`! Or perhaps optional argument
that optionally takes value (`[--opt=[val]]`)? Use `Option<Option<T>>`!

Here is the table of types and `clap` methods they correspond to:

| Type                         | Effect                                            | Added method call to `clap::Arg`                                 |
|------------------------------|---------------------------------------------------|------------------------------------------------------------------|
| `bool`                       | `true` if the flag is present                     | `.takes_value(false).multiple(false)`                            |
| `Option<T: FromStr>`         | optional positional argument or option            | `.takes_value(true).multiple(false)`                             |
| `Option<Option<T: FromStr>>` | optional option with optional value               | `.takes_value(true).multiple(false).min_values(0).max_values(1)` |
| `Vec<T: FromStr>`            | list of options or the other positional arguments | `.takes_value(true).multiple(true)`                              |
| `Option<Vec<T: FromStr>`     | optional list of options                          | `.takes_values(true).multiple(true).min_values(0)`               |
| `T: FromStr`                 | required option or positional argument            | `.takes_value(true).multiple(false).required(!has_default)`      |

The `FromStr` trait is used to convert the argument to the given type, and the
`Arg::validator` method is set to a method using `to_string()` (`FromStr::Err`
must implement `std::fmt::Display`). If you would like to use a custom string
parser other than `FromStr`, see the [same titled
section](#custom-string-parsers) below.

**Important:**

Note that *only literal occurrences* of this type are special, for example
`Option<T>` is special while `::std::option::Option<T>` is not.

If you need to avoid special casing you can make a `type` alias and use it in
place of the said type.

**Note:**

`bool` cannot be used as positional argument unless you provide an explicit
parser. If you need a positional bool, for example to parse `true` or `false`,
you must annotate the field with explicit
[`#[clap(parse(...))]`](#custom-string-parsers).

Thus, the `speed` argument is generated as:

```rust
# use std::str::FromStr;
clap::Arg::new("speed")
    .takes_value(true)
    .multiple_occurrences(false)
    .required(false)
    .validator(|s| FromStr::from_str(s).map(|_: f64| ()))
    .short('v')
    .long("velocity")
    .about("Set speed")
    .default_value("42");
```

## Specifying argument types

There are three types of arguments that can be supplied to each (sub-)command:

 - short (e.g. `-h`),
 - long (e.g. `--help`)
 - and positional.

`clap::Parser` defaults to creating positional arguments.

If you want to generate a long argument you can specify either `long = $NAME`,
or just `long` to get a long flag generated using the field name.  The
generated casing style can be modified using the `rename_all` attribute. See
the `rename_all` example for more.

For short arguments, `short` will use the first letter of the field name by
default, but just like the long option it's also possible to use a custom
letter through `short = $LETTER`.

If an argument is renamed using `name = $NAME` any following call to `short` or
`long` will use the new name.

**Attention**: If these arguments are used without an explicit name the
resulting flag is going to be renamed using `kebab-case` if the `rename_all`
attribute was not specified previously. The same is true for subcommands with
implicit naming through the related data structure.

```rust
use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
struct Opt {
    /// This option can be specified with something like `--foo-option
    /// value` or `--foo-option=value`
    #[clap(long)]
    foo_option: String,

    /// This option can be specified with something like `-b value` (but
    /// not `--bar-option value`).
    #[clap(short)]
    bar_option: String,

    /// This option can be specified either `--baz value` or `-z value`.
    #[clap(short = 'z', long = "baz")]
    baz_option: String,

    /// This option can be specified either by `--custom value` or
    /// `-c value`.
    #[clap(name = "custom", long, short)]
    custom_option: String,

    /// This option is positional, meaning it is the first unadorned string
    /// you provide (multiple others could follow).
    my_positional: String,

    /// This option is skipped and will be filled with the default value
    /// for its type (in this case 0).
    #[clap(skip)]
    skipped: u32,
}

# Opt::try_parse_from(
#    &["test", "--foo-option", "", "-b", "", "--baz", "", "--custom", "", "positional"]);
```

## Default values

In clap, default values for options can be specified via [`Arg::default_value`].

Of course, you can use as a raw method:

```rust
# use clap::Parser;
#[derive(Parser)]
struct Opt {
    #[clap(default_value = "", long)]
    prefix: String,
}
```

This is quite mundane and error-prone to type the `"..."` default by yourself,
especially when the Rust ecosystem uses the [`Default`][] trait for that. It
would be wonderful to have `clap::Parser` take the `Default_default` and fill
it for you. And yes, `clap::Parser` can do that.

To use the `Default::default` value for the provided type, use `default_value_t`.

```rust
# use clap::Parser;
#[derive(Parser)]
struct Opt {
    #[clap(default_value_t, long)]
    prefix: String,
}
```

## Help messages

In clap, help messages for the whole binary can be specified via [`App::about`]
and [`App::long_about`] while help messages for individual arguments can be
specified via [`Arg::about`] and [`Arg::long_about`]".

`long_*` variants are used when user calls the program with `--help` and
"short" variants are used with `-h` flag. In `clap::Parser`, you can use them
via [raw methods](#raw-methods), for example:

```rust
# use clap::Parser;

#[derive(Parser)]
#[clap(about = "I am a program and I work, just pass `-h`")]
struct Foo {
    #[clap(short, help = "Pass `-h` and you'll see me!")]
    bar: String,
}
```

For convenience, doc comments can be used instead of raw methods (this example
works exactly like the one above):

```rust
# use clap::Parser;

#[derive(Parser)]
/// I am a program and I work, just pass `-h`
struct Foo {
    /// Pass `-h` and you'll see me!
    bar: String,
}
```

Doc comments on [top-level](#magical-methods) will be turned into
`App::about/long_about` call (see below), doc comments on field-level are
`Arg::about/long_about` calls.

**Important:**

Raw methods have priority over doc comments!

### `long_about` and `--help`

A message passed to [`App::long_about`] will be displayed whenever your program
is called with `--help` instead of `-h`. Of course, you can use them via raw
methods as described [above](#help-messages).

The more convenient way is to use a so-called "long" doc comment:

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

A long doc comment consists of three parts:

- Short summary
- A blank line (whitespace only)
- Detailed description, all the rest

In other words, "long" doc comment consists of two or more paragraphs, with the
first being a summary and the rest being the detailed description.

**A long comment will result in two method calls**, `help(<summary>)` and
`long_help(<whole comment>)`, so clap will display the summary with `-h` and
the whole help message on `--help` (see below).

So, the example above will be turned into this (details omitted):

```rust
clap::App::new("<name>")
    .about("Hi there, I'm Robo!")
    .long_about("Hi there, I'm Robo!\n\n\
                 I like beeping, stumbling, eating your electricity,\
                 and making records of you singing in a shower.\
                 Pay up or I'll upload it to youtube!")
// args...
# ;
```

### `-h` vs `--help` (A.K.A `help()` vs `long_help()`)

The `-h` flag is not the same as `--help`.

`-h` corresponds to `App::about/Arg::about` and requests short "summary" messages
while `--help` corresponds to `Arg::long_help/App::long_about` and requests more
detailed, descriptive messages.

It is entirely up to `clap` what happens if you used only one of
[`Arg::about`]/[`Arg::long_about`], see `clap`'s documentation for these
methods.

As of clap v3.0.0, if only a short message ([`Arg::about`]) or only a long
([`Arg::long_about`]) message is provided, clap will use it for both -h and
--help.

There is also `<name> help` or `<name> help <subcommand>`, which behaves like
`-h` (showing the short help) unless
[`AppSettings::UseLongFormatForHelpSubcommand`][] is set, in which case it
behaves like `--help`.


### Doc comment preprocessing and `#[clap(verbatim_doc_comment)]`

`clap::Parser` applies some preprocessing to doc comments to ease the most common uses:

- Strip leading and trailing whitespace from every line, if present.
- Strip leading and trailing blank lines, if present.
- Interpret each group of non-empty lines as a word-wrapped paragraph.
  We replace newlines within paragraphs with spaces to allow the output
  to be re-wrapped to the terminal width.
- Strip any excess blank lines so that there is exactly one per paragraph break.
- If the first paragraph ends in exactly one period,
  remove the trailing period (i.e. strip trailing periods but not trailing ellipses).

Sometimes you don't want this preprocessing to apply, for example the comment
contains some ASCII art or markdown tables, you would need to preserve LFs
along with blank lines and the leading/trailing whitespace. You can ask
`clap::Parser` to preserve them via `#[clap(verbatim_doc_comment)]` attribute.

**This attribute must be applied to each field separately**, there's no global
switch.

**Important:**

Keep in mind that `clap::Parser` will *still* remove one leading space from
each line, even if this attribute is present, to allow for a space between
`///` and the content.

Also, `clap::Parser` will *still* remove leading and trailing blank lines so
these formats are equivalent:

```rust
/** This is a doc comment

Hello! */

/**
This is a doc comment

Hello!
*/

/// This is a doc comment
///
/// Hello!
#
# mod m {}
```

[`App::about`]:      https://docs.rs/clap/2/clap/struct.App.html#method.about
[`App::long_about`]: https://docs.rs/clap/2/clap/struct.App.html#method.long_about
[`Arg::help`]:       https://docs.rs/clap/2/clap/struct.Arg.html#method.help
[`Arg::long_help`]:  https://docs.rs/clap/2/clap/struct.Arg.html#method.long_help

## Environment variable fallback

It is possible to specify an environment variable fallback option for an
arguments so that its value is taken from the specified environment variable if
not given through the command-line:

```rust
# use clap::Parser;

#[derive(Parser)]
struct Foo {
    #[clap(short, long, env = "PARAMETER_VALUE")]
    parameter_value: String,
}
```

By default, values from the environment are shown in the help output (i.e. when
invoking `--help`):

```console
$ cargo run -- --help
...
OPTIONS:
  -p, --parameter-value <parameter-value>     [env: PARAMETER_VALUE=env_value]
```

In some cases this may be undesirable, for example when being used for passing
credentials or secret tokens. In those cases you can use `hide_env_values` to avoid
having `clap::Parser` emit the actual secret values:

```rust
# use clap::Parser;

#[derive(Parser)]
struct Foo {
    #[clap(long = "secret", env = "SECRET_VALUE", hide_env_values = true)]
    secret_value: String,
}
```

### Auto-deriving environment variables

Environment variables tend to be called after the corresponding `struct`'s
field, as in example above. The field is `secret_value` and the env var is
"SECRET_VALUE"; the name is the same, except casing is different.

It's pretty tedious and error-prone to type the same name twice, so you can ask
`clap::Parser` to do that for you.

```rust
# use clap::Parser;

#[derive(Parser)]
struct Foo {
    #[clap(long = "secret", env)]
    secret_value: String,
}
```

It works just like `#[clap(short/long)]`: if `env` is not set to some concrete
value the value will be derived from the field's name. This is controlled by
`#[clap(rename_all_env)]`.

`rename_all_env` works exactly as `rename_all` (including overriding) except
default casing is `SCREAMING_SNAKE_CASE` instead of `kebab-case`.

## Skipping fields

Sometimes you may want to add a field to your `Opt` struct that is not a
command line option and `clap` should know nothing about it. You can ask
`clap::Parser` to skip the field entirely via `#[clap(skip = value)]` (`value`
must implement `Into<FieldType>`) or `#[clap(skip)]` if you want assign the
field with `Default::default()` (obviously, the field's type must implement
`Default`).

```rust
# use clap::Parser;
#[derive(Parser)]
pub struct Opt {
    #[clap(long, short)]
    number: u32,

    // these fields are to be assigned with Default::default()

    #[clap(skip)]
    k: String,
    #[clap(skip)]
    v: Vec<u32>,

    // these fields get set explicitly

    #[clap(skip = vec![1, 2, 3])]
    k2: Vec<u32>,
    #[clap(skip = "cake")] // &str implements Into<String>
    v2: String,
}
```

## Subcommands

Some applications, especially large ones, split their functionality through the
use of "subcommands". Each of these act somewhat like a separate command, but
is part of the larger group. One example is `git`, which has subcommands such
as `add`, `commit`, and `clone`, to mention just a few.

`clap` has this functionality, and `clap::Parser` supports it through enums:

```rust
# use clap::Parser;

# use std::path::PathBuf;
#[derive(Parser)]
#[clap(about = "the stupid content tracker")]
enum Git {
    Add {
        #[clap(short)]
        interactive: bool,
        #[clap(short)]
        patch: bool,
        #[clap(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    Fetch {
        #[clap(long)]
        dry_run: bool,
        #[clap(long)]
        all: bool,
        repository: Option<String>,
    },
    Commit {
        #[clap(short)]
        message: Option<String>,
        #[clap(short)]
        all: bool,
    },
}
```

Using `derive(Parser)` on an enum instead of a struct will produce a
`clap::App` that only takes subcommands. So `git add`, `git fetch`, and `git
commit` would be commands allowed for the above example.

`clap::Parser` also provides support for applications where certain flags need
to apply to all subcommands, as well as nested subcommands:

```rust
# use clap::Parser;
#[derive(Parser)]
struct MakeCookie {
    #[clap(name = "supervisor", default_value = "Puck", long = "supervisor")]
    supervising_faerie: String,
    /// The faerie tree this cookie is being made in.
    tree: Option<String>,
    #[clap(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    /// Pound acorns into flour for cookie dough.
    Pound {
        acorns: u32,
    },
    /// Add magical sparkles -- the secret ingredient!
    Sparkle {
        #[clap(short, parse(from_occurrences))]
        magicality: u64,
        #[clap(short)]
        color: String,
    },
    Finish(Finish),
}

// Subcommand can also be externalized by using a 1-uple enum variant
#[derive(Parser)]
struct Finish {
    #[clap(short)]
    time: u32,
    #[clap(subcommand)] // Note that we mark a field as a subcommand
    finish_type: FinishType,
}

// subsubcommand!
#[derive(Parser)]
enum FinishType {
    Glaze {
        applications: u32,
    },
    Powder {
        flavor: String,
        dips: u32,
    }
}
```

Marking a field with `#[clap(subcommand)]` will add the subcommands of the
designated enum to the current `clap::App`. The designated enum *must* also
be derived `clap::Parser`. So the above example would take the following
commands:

- `make-cookie pound 50`
- `make-cookie sparkle -mmm --color "green"`
- `make-cookie finish 130 glaze 3`

### Optional subcommands

Subcommands may be optional:

```rust
# use clap::Parser;
#[derive(Parser)]
struct Foo {
    file: String,
    #[clap(subcommand)]
    cmd: Option<Command>,
}

#[derive(Parser)]
enum Command {
    Bar,
    Baz,
    Quux,
}
```

### External subcommands

Sometimes you want to support not only the set of well-known subcommands
but you also want to allow other, user-driven subcommands. `clap` supports
this via [`AppSettings::AllowExternalSubcommands`].

`clap::Parser` provides it's own dedicated syntax for that:

```rust
# use clap::Parser;
#[derive(Debug, PartialEq, Parser)]
struct Opt {
    #[clap(subcommand)]
    sub: Subcommands,
}

#[derive(Debug, PartialEq, Parser)]
enum Subcommands {
    // normal subcommand
    Add,

    // `external_subcommand` tells clap::Parser to put
    // all the extra arguments into this Vec
    #[clap(external_subcommand)]
    Other(Vec<String>),
}

// normal subcommand
assert_eq!(
    Opt::parse_from(&["test", "add"]),
    Opt {
        sub: Subcommands::Add
    }
);

assert_eq!(
    Opt::parse_from(&["test", "git", "status"]),
    Opt {
        sub: Subcommands::Other(vec!["git".into(), "status".into()])
    }
);

// Please note that if you'd wanted to allow "no subcommands at all" case
// you should have used `sub: Option<Subcommands>` above
assert!(Opt::try_parse_from(&["test"]).is_err());
```

In other words, you just add an extra tuple variant marked with
`#[clap(subcommand)]`, and its type must be either `Vec<String>` or
`Vec<OsString>`. `clap::Parser` will detect `String` in this context and use
appropriate `clap` API.

### Flattening subcommands

It is also possible to combine multiple enums of subcommands into one. All the
subcommands will be on the same level.

```rust
# use clap::Parser;
#[derive(Parser)]
enum BaseCli {
    Ghost10 {
        arg1: i32,
    }
}

#[derive(Parser)]
enum Opt {
    #[clap(flatten)]
    BaseCli(BaseCli),
    Dex {
        arg2: i32,
    },
}
```

```shell
cli ghost10 42
cli dex 42
```

## Flattening

It can sometimes be useful to group related arguments in a substruct, while
keeping the command-line interface flat. In these cases you can mark a field as
`flatten` and give it another type that derives `clap::Parser`:

```rust
# use clap::Parser;
#[derive(Parser)]
struct Cmdline {
    /// switch on verbosity
    #[clap(short)]
    verbose: bool,
    #[clap(flatten)]
    daemon_opts: DaemonOpts,
}

#[derive(Parser)]
struct DaemonOpts {
    /// daemon user
    #[clap(short)]
    user: String,
    /// daemon group
    #[clap(short)]
    group: String,
}
```

In this example, the derived `Cmdline` parser will support the options `-v`,
`-u` and `-g`.

This feature also makes it possible to define a `clap::Parser` struct in a
library, parse the corresponding arguments in the main argument parser, and
pass off this struct to a handler provided by that library.

## Custom string parsers

If the field type does not have a `FromStr` implementation, or you would like
to provide a custom parsing scheme other than `FromStr`, you may provide a
custom string parser using `parse(...)` like this:

```rust
# use clap::Parser;
use std::num::ParseIntError;
use std::path::PathBuf;

fn parse_hex(src: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(src, 16)
}

#[derive(Parser)]
struct HexReader {
    #[clap(short, parse(try_from_str = parse_hex))]
    number: u32,
    #[clap(short, parse(from_os_str))]
    output: PathBuf,
}
```

There are five kinds of custom parsers:

| Kind               | Signature                           | Default                         |
|--------------------|-------------------------------------|---------------------------------|
| `from_str`         | `fn(&str) -> T`                     | `::std::convert::From::from`    |
| `try_from_str`     | `fn(&str) -> Result<T, E>`          | `::std::str::FromStr::from_str` |
| `from_os_str`      | `fn(&OsStr) -> T`                   | `::std::convert::From::from`    |
| `try_from_os_str`  | `fn(&OsStr) -> Result<T, OsString>` | (no default function)           |
| `from_occurrences` | `fn(u64) -> T`                      | `value as T`                    |
| `from_flag`        | `fn(bool) -> T`                     | `::std::convert::From::from`    |

The `from_occurrences` parser is special. Using `parse(from_occurrences)`
results in the _number of flags occurrences_ being stored in the relevant field
or being passed to the supplied function. In other words, it converts something
like `-vvv` to `3`. This is equivalent to `.takes_value(false).multiple(true)`.
Note that the default parser can only be used with fields of integer types
(`u8`, `usize`, `i64`, etc.).

The `from_flag` parser is also special. Using `parse(from_flag)` or
`parse(from_flag = some_func)` will result in the field being treated as a flag
even if it does not have type `bool`.

When supplying a custom string parser, `bool` will not be treated specially:

| Type        | Effect            | Added method call to `clap::Arg`                            |
|-------------|-------------------|-------------------------------------------------------------|
| `Option<T>` | optional argument | `.takes_value(true).multiple(false)`                        |
| `Vec<T>`    | list of arguments | `.takes_value(true).multiple(true)`                         |
| `T`         | required argument | `.takes_value(true).multiple(false).required(!has_default)` |

In the `try_from_*` variants, the function will run twice on valid input: once
to validate, and once to parse. Hence, make sure the function is
side-effect-free.

## Generics

Generic structs and enums can be used. They require explicit trait bounds on
any generic types that will be used by the `clap::Parser` derive macro. In some
cases, associated types will require additional bounds. See the usage of
`FromStr` below for an example of this.

```rust
# use clap::Parser;
use std::{fmt, str::FromStr, error::Error};

// a struct with single custom argument
#[derive(Parser)]
struct GenericArgs<T: FromStr> where <T as FromStr>::Err: fmt::Display + fmt::Debug + Send + Sync + 'static + Error {
    generic_arg_1: String,
    generic_arg_2: String,
    custom_arg_1: T,
}
```

or

```rust
# use clap::{Args, Parser};
// a struct with multiple custom arguments in a substructure
#[derive(Parser)]
struct GenericArgs<T: Parser + Args> {
    generic_arg_1: String,
    generic_arg_2: String,
    #[clap(flatten)]
    custom_args: T,
}
```

## clap_derive rustc version policy

- Minimum rustc version modification must be specified in the
  [changelog](https://github.com/clap-rs/clap_derive/blob/master/CHANGELOG.md)
  and in the [travis
  configuration](https://github.com/clap-rs/clap_derive/blob/master/.travis.yaml).
- Contributors can increment minimum rustc version without any justification if
  the new version is required by the latest version of one of clap_derive's
  dependencies (`cargo update` will not fail on clap_derive).
- Contributors can increment minimum rustc version if the library user
  experience is improved.

## Why

I've (@TeXitoi) used [docopt](https://crates.io/crates/docopt) for a long time
(pre rust 1.0). I really like the fact that you have a structure with the
parsed argument: no need to convert `String` to `f64`, no useless `unwrap`. But
on the other hand, I don't like to write by hand the usage string. That's like
going back to the golden age of WYSIWYG editors.  Field naming is also a bit
artificial.

Today, the new standard to read command line arguments in Rust is
[clap](https://crates.io/crates/clap).  This library is so feature full! But I
think there is one downside: even if you can validate argument and expressing
that an argument is required, you still need to transform something looking
like a hashmap of string vectors to something useful for your application.

Now, there is stable custom derive. Thus I can add to clap the automatic
conversion that I miss. Here is the result.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`AppSettings::AllowExternalSubcommands`]: https://docs.rs/clap/2.32.0/clap/enum.AppSettings.html#variant.AllowExternalSubcommands
[`clap::App`]: https://docs.rs/clap/latest/clap/struct.App.html
[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[`Arg::default_value`]: https://docs.rs/clap/2.33.0/clap/struct.Arg.html#method.default_value
[`AppSettings::UseLongFormatForHelpSubcommand`]: https://docs.rs/clap/latest/clap/enum.AppSettings.html#variant.UseLongFormatForHelpSubcommand
