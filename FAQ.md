1. [Comparisons](#comparisons)
   1. [How does `clap` compare to structopt?](#how-does-clap-compare-to-structopt)
   2. [How does `clap` compare to getopts?](#how-does-clap-compare-to-getopts)
   3. [How does `clap` compare to docopt.rs?](#how-does-clap-compare-to-docoptrs)
   4. [What are some reasons to use `clap`? (The Pitch)](#what-are-some-reasons-to-use-clap-the-pitch)
   5. [What are some reasons *not* to use `clap`? (The Anti Pitch)](#what-are-some-reasons-not-to-use-clap-the-anti-pitch)
   6. [Reasons to use `clap`](#reasons-to-use-clap)
   7. [Reasons to `docopt`](#reasons-to-docopt)
   8. [Reasons to use `getopts`](#reasons-to-use-getopts)
2. [How many methods are there to create an App/Arg?](#how-many-methods-are-there-to-create-an-apparg)
3. [Why is there a default subcommand of help?](#why-is-there-a-default-subcommand-of-help)

### Comparisons

First, let me say that these comparisons are highly subjective, and not meant in a critical or harsh manner. All the argument parsing libraries out there (to include `clap`) have their own strengths and weaknesses. Sometimes it just comes down to personal taste when all other factors are equal. When in doubt, try them all and pick one that you enjoy :) There's plenty of room in the Rust community for multiple implementations!

#### How does `clap` compare to [structopt](https://github.com/TeXitoi/structopt)?

Simple! `clap` *is* `structopt`. With the 3.0 release, `clap` imported the `structopt` code into it's own codebase as the [`clap_derive`](https://github.com/clap-rs/clap/tree/master/clap_derive) crate. Since `structopt` already used `clap` under the hood, the transition was nearly painless, and is 100% feature compatible.

If you were using `structopt` before, you have to change the attributes from `#[structopt(...)]` to `#[clap(...)]`.

Also the derive statements changed from `#[derive(Structopt)]` to `#[derive(Clap)]`. There is also some additional functionality and breaking changes that's been added to the `clap_derive` crate. See the documentation for that crate, for more details.

#### How does `clap` compare to [getopts](https://github.com/rust-lang-nursery/getopts)?

`getopts` is a very basic, fairly minimalist argument parsing library. This isn't a bad thing, sometimes you don't need tons of features, you just want to parse some simple arguments, and have some help text generated for you based on valid arguments you specify. The downside to this approach is that you must manually implement most of the common features (such as checking to display help messages, usage strings, etc.). If you want a highly custom argument parser, and don't mind writing the majority of the functionality yourself, `getopts` is an excellent base.

`getopts` also doesn't allocate much, or at all. This gives it a very small performance boost. Although, as you start implementing additional features, that boost quickly disappears.

Personally, I find many, many uses of `getopts` are manually implementing features that `clap` provides by default. Using `clap` simplifies your codebase allowing you to focus on your application, and not argument parsing.

#### How does `clap` compare to [docopt.rs](https://github.com/docopt/docopt.rs)?

I first want to say I'm a big a fan of BurntSushi's work, the creator of `Docopt.rs`. I aspire to produce the quality of libraries that this man does! When it comes to comparing these two libraries they are very different. `docopt` tasks you with writing a help message, and then it parses that message for you to determine all valid arguments and their use. Some people LOVE this approach, others do not. If you're willing to write a detailed help message, it's nice that you can stick that in your program and have `docopt` do the rest. On the downside, it's far less flexible.

`docopt` is also excellent at translating arguments into Rust types automatically. There is even a syntax extension which will do all this for you, if you're willing to use a nightly compiler (use of a stable compiler requires you to somewhat manually translate from arguments to Rust types). To use BurntSushi's words, `docopt` is also a sort of black box. You get what you get, and it's hard to tweak implementation or customize the experience for your use case.

Because `docopt` is doing a ton of work to parse your help messages and determine what you were trying to communicate as valid arguments, it's also one of the more heavy weight parsers performance-wise. For most applications this isn't a concern and this isn't to say `docopt` is slow, in fact far from it. This is just something to keep in mind while comparing.

#### What are some reasons to use `clap`? (The Pitch)

`clap` is as fast, and as lightweight as possible while still giving all the features you'd expect from a modern argument parser. In fact, for the amount and type of features `clap` offers it remains about as fast as `getopts`. If you use `clap` when just need some simple arguments parsed, you'll find it's a walk in the park. `clap` also makes it possible to represent extremely complex, and advanced requirements, without too much thought. `clap` aims to be intuitive, easy to use, and fully capable for wide variety use cases and needs.

#### What are some reasons *not* to use `clap`? (The Anti Pitch)

Depending on the style in which you choose to define the valid arguments, `clap` can be very verbose. `clap` also offers so many finetuning knobs and dials, that learning everything can seem overwhelming. I strive to keep the simple cases simple, but when turning all those custom dials it can get complex. `clap` is also opinionated about parsing. Even though so much can be tweaked and tuned with `clap` (and I'm adding more all the time), there are still certain features which `clap` implements in specific ways which may be contrary to some users use-cases.

#### Reasons to use `clap`

 * You want all the nice CLI features your users may expect, yet you don't want to implement them all yourself. You'd like to focus your application, not argument parsing.
 * In addition to the point above; you don't want to sacrifice performance to get all those nice features
 * You have complex requirements/conflicts between your various valid args.
 * You want to use subcommands (although other libraries also support subcommands, they are not nearly as feature rich as those provided by `clap`)
 * You want some sort of custom validation built into the argument parsing process, instead of as part of your application (which allows for earlier failures, better error messages, more cohesive experience, etc.)
 * You need more performance than `docopt` provides

#### Reasons to `docopt`

 * You want *automatic* serialization of your arguments into Rust types (Although `clap` can do this, docopt is better at it)
 * You are on nightly Rust and want the library to automatically generate an "arguments struct" from the matched args
 * You are porting an application which uses docopt and already have the usage string already defined

#### Reasons to use `getopts`

 * You need absolutely as few allocations as possible and don't mind implementing nearly everything yourself
 * You want a portion of the arg parsing process to be very custom (again, implementing the details yourself)


### How many methods are there to create an App/Arg?

To build an `App` there are three:

* Derive Macros
* Builder Pattern
* Yaml

To build an `Arg` there are four:

* Derive Macros
* Builder Pattern
* Usage Strings
* Yaml

### Why is there a default subcommand of help?

There is only a default subcommand of `help` when other subcommands have been defined manually. So it's opt-in(ish), being that you only get a `help` subcommand if you're actually using subcommands.

Also, if the user defined a `help` subcommand themselves, the auto-generated one wouldn't be added (meaning it's only generated if the user hasn't defined one themselves).

