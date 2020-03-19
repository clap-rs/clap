/// A convenience macro for loading the YAML file at compile time (relative to the current file,
/// like modules work). That YAML object can then be passed to this function.
///
/// # Panics
///
/// The YAML file must be properly formatted or this function will panic!(). A good way to
/// ensure this doesn't happen is to run your program with the `--help` switch. If this passes
/// without error, you needn't worry because the YAML is properly formatted.
///
/// # Examples
///
/// The following example shows how to load a properly formatted YAML file to build an instance
/// of an `App` struct.
///
/// ```ignore
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let yml = load_yaml!("app.yml");
/// let app = App::from_yaml(yml);
///
/// // continued logic goes here, such as `app.get_matches()` etc.
/// # }
/// ```
#[cfg(feature = "yaml")]
#[macro_export]
macro_rules! load_yaml {
    ($yml:expr) => {
        &$crate::YamlLoader::load_from_str(include_str!($yml)).expect("failed to load YAML file")[0]
    };
}

/// Convenience macro getting a typed value `T` where `T` implements [`std::str::FromStr`] from an
/// argument value. This macro returns a `Result<T,String>` which allows you as the developer to
/// decide what you'd like to do on a failed parse. There are two types of errors, parse failures
/// and those where the argument wasn't present (such as a non-required argument). You can use
/// it to get a single value, or a iterator as with the [`ArgMatches::values_of`]
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let matches = App::new("myapp")
///               .arg("[length] 'Set the length to use as a pos whole num, i.e. 20'")
///               .get_matches();
///
/// let len      = value_t!(matches.value_of("length"), u32).unwrap_or_else(|e| e.exit());
/// let also_len = value_t!(matches, "length", u32).unwrap_or_else(|e| e.exit());
///
/// println!("{} + 2: {}", len, len + 2);
/// # }
/// ```
/// [`std::str::FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`ArgMatches::values_of`]: ./struct.ArgMatches.html#method.values_of
/// [`Result<T,String>`]: https://doc.rust-lang.org/std/result/enum.Result.html
#[macro_export]
macro_rules! value_t {
    ($m:ident, $v:expr, $t:ty) => {
        $crate::value_t!($m.value_of($v), $t)
    };
    ($m:ident.value_of($v:expr), $t:ty) => {
        if let Some(v) = $m.value_of($v) {
            match v.parse::<$t>() {
                Ok(val) => Ok(val),
                Err(_) => Err($crate::Error::value_validation_auto(&format!(
                    "The argument '{}' isn't a valid value",
                    v
                ))),
            }
        } else {
            Err($crate::Error::argument_not_found_auto($v))
        }
    };
}

/// Convenience macro getting a typed value `T` where `T` implements [`std::str::FromStr`] or
/// exiting upon error, instead of returning a [`Result`] type.
///
/// **NOTE:** This macro is for backwards compatibility sake. Prefer
/// [`value_t!(/* ... */).unwrap_or_else(|e| e.exit())`]
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let matches = App::new("myapp")
///               .arg("[length] 'Set the length to use as a pos whole num, i.e. 20'")
///               .get_matches();
///
/// let len      = value_t_or_exit!(matches.value_of("length"), u32);
/// let also_len = value_t_or_exit!(matches, "length", u32);
///
/// println!("{} + 2: {}", len, len + 2);
/// # }
/// ```
/// [`std::str::FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`value_t!(/* ... */).unwrap_or_else(|e| e.exit())`]: ./macro.value_t!.html
#[macro_export]
macro_rules! value_t_or_exit {
    ($m:ident, $v:expr, $t:ty) => {
        $crate::value_t_or_exit!($m.value_of($v), $t)
    };
    ($m:ident.value_of($v:expr), $t:ty) => {
        if let Some(v) = $m.value_of($v) {
            match v.parse::<$t>() {
                Ok(val) => val,
                Err(_) => $crate::Error::value_validation_auto(&format!(
                    "The argument '{}' isn't a valid value",
                    v
                ))
                .exit(),
            }
        } else {
            $crate::Error::argument_not_found_auto($v).exit()
        }
    };
}

/// Convenience macro getting a typed value [`Vec<T>`] where `T` implements [`std::str::FromStr`]
/// This macro returns a [`clap::Result<Vec<T>>`] which allows you as the developer to decide
/// what you'd like to do on a failed parse.
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let matches = App::new("myapp")
///               .arg("[seq]... 'A sequence of pos whole nums, i.e. 20 45'")
///               .get_matches();
///
/// let vals = values_t!(matches.values_of("seq"), u32).unwrap_or_else(|e| e.exit());
/// for v in &vals {
///     println!("{} + 2: {}", v, v + 2);
/// }
///
/// let vals = values_t!(matches, "seq", u32).unwrap_or_else(|e| e.exit());
/// for v in &vals {
///     println!("{} + 2: {}", v, v + 2);
/// }
/// # }
/// ```
/// [`std::str::FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`Vec<T>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`clap::Result<Vec<T>>`]: ./type.Result.html
#[macro_export]
macro_rules! values_t {
    ($m:ident, $v:expr, $t:ty) => {
        $crate::values_t!($m.values_of($v), $t)
    };
    ($m:ident.values_of($v:expr), $t:ty) => {
        if let Some(vals) = $m.values_of($v) {
            let mut tmp = vec![];
            let mut err = None;
            for pv in vals {
                match pv.parse::<$t>() {
                    Ok(rv) => tmp.push(rv),
                    Err(..) => {
                        err = Some($crate::Error::value_validation_auto(&format!(
                            "The argument '{}' isn't a valid value",
                            pv
                        )));
                        break;
                    }
                }
            }
            match err {
                Some(e) => Err(e),
                None => Ok(tmp),
            }
        } else {
            Err($crate::Error::argument_not_found_auto($v))
        }
    };
}

/// Convenience macro getting a typed value [`Vec<T>`] where `T` implements [`std::str::FromStr`]
/// or exiting upon error.
///
/// **NOTE:** This macro is for backwards compatibility sake. Prefer
/// [`values_t!(/* ... */).unwrap_or_else(|e| e.exit())`]
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let matches = App::new("myapp")
///               .arg("[seq]... 'A sequence of pos whole nums, i.e. 20 45'")
///               .get_matches();
///
/// let vals = values_t_or_exit!(matches.values_of("seq"), u32);
/// for v in &vals {
///     println!("{} + 2: {}", v, v + 2);
/// }
///
/// // type for example only
/// let vals: Vec<u32> = values_t_or_exit!(matches, "seq", u32);
/// for v in &vals {
///     println!("{} + 2: {}", v, v + 2);
/// }
/// # }
/// ```
/// [`values_t!(/* ... */).unwrap_or_else(|e| e.exit())`]: ./macro.values_t!.html
/// [`std::str::FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`Vec<T>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[macro_export]
macro_rules! values_t_or_exit {
    ($m:ident, $v:expr, $t:ty) => {
        $crate::values_t_or_exit!($m.values_of($v), $t)
    };
    ($m:ident.values_of($v:expr), $t:ty) => {
        if let Some(vals) = $m.values_of($v) {
            vals.map(|v| {
                v.parse::<$t>().unwrap_or_else(|_| {
                    $crate::Error::value_validation_auto(&format!(
                        "One or more arguments aren't valid values"
                    ))
                    .exit()
                })
            })
            .collect::<Vec<$t>>()
        } else {
            $crate::Error::argument_not_found_auto($v).exit()
        }
    };
}

// _clap_count_exprs! is derived from https://github.com/DanielKeep/rust-grabbag
// commit: 82a35ca5d9a04c3b920622d542104e3310ee5b07
// License: MIT
// Copyright ⓒ 2015 grabbag contributors.
// Licensed under the MIT license (see LICENSE or <http://opensource.org
// /licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
// <http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
// files in the project carrying such notice may not be copied, modified,
// or distributed except according to those terms.
//
/// Counts the number of comma-delimited expressions passed to it.  The result is a compile-time
/// evaluable expression, suitable for use as a static array size, or the value of a `const`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate clap;
/// # fn main() {
/// const COUNT: usize = _clap_count_exprs!(a, 5+1, "hi there!".into_string());
/// assert_eq!(COUNT, 3);
/// # }
/// ```
#[macro_export]
macro_rules! _clap_count_exprs {
    () => { 0 };
    ($e:expr) => { 1 };
    ($e:expr, $($es:expr),+) => { 1 + $crate::_clap_count_exprs!($($es),*) };
}

/// Convenience macro to generate more complete enums with variants to be used as a type when
/// parsing arguments. This enum also provides a `variants()` function which can be used to
/// retrieve a `Vec<&'static str>` of the variant names, as well as implementing [`FromStr`] and
/// [`Display`] automatically.
///
/// **NOTE:** Case insensitivity is supported for ASCII characters only. It's highly recommended to
/// use [`Arg::case_insensitive(true)`] for args that will be used with these enums
///
/// **NOTE:** This macro automatically implements [`std::str::FromStr`] and [`std::fmt::Display`]
///
/// **NOTE:** These enums support pub (or not) and uses of the `#[derive()]` traits
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::{App, Arg};
/// arg_enum!{
///     #[derive(PartialEq, Debug)]
///     pub enum Foo {
///         Bar,
///         Baz,
///         Qux
///     }
/// }
/// // Foo enum can now be used via Foo::Bar, or Foo::Baz, etc
/// // and implements std::str::FromStr to use with the value_t! macros
/// fn main() {
///     let m = App::new("app")
///                 .arg(Arg::from("<foo> 'the foo'")
///                     .possible_values(&Foo::variants())
///                     .case_insensitive(true))
///                 .get_matches_from(vec![
///                     "app", "baz"
///                 ]);
///     let f = value_t!(m, "foo", Foo).unwrap_or_else(|e| e.exit());
///
///     assert_eq!(f, Foo::Baz);
/// }
/// ```
/// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`std::str::FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
/// [`std::fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
/// [`Arg::case_insensitive(true)`]: ./struct.Arg.html#method.case_insensitive
#[macro_export]
macro_rules! arg_enum {
    (@as_item $($i:item)*) => ($($i)*);
    (@impls ( $($tts:tt)* ) -> ($e:ident, $($v:ident),+)) => {
        $crate::arg_enum!(@as_item
        $($tts)*

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> ::std::result::Result<Self,Self::Err> {
                match s {
                    $(stringify!($v) |
                    _ if s.eq_ignore_ascii_case(stringify!($v)) => Ok($e::$v)),+,
                    _ => Err({
                        let v = vec![
                            $(stringify!($v),)+
                        ];
                        format!("valid values: {}",
                            v.join(" ,"))
                    }),
                }
            }
        }
        impl ::std::fmt::Display for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($e::$v => write!(f, stringify!($v)),)+
                }
            }
        }
        impl $e {
            #[allow(dead_code)]
            #[allow(missing_docs)]
            pub fn variants() -> [&'static str; $crate::_clap_count_exprs!($(stringify!($v)),+)] {
                [
                    $(stringify!($v),)+
                ]
            }
        });
    };
    ($(#[$($m:meta),+])+ pub enum $e:ident { $($v:ident $(=$val:expr)*,)+ } ) => {
        $crate::arg_enum!(@impls
            ($(#[$($m),+])+
            pub enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };
    ($(#[$($m:meta),+])+ pub enum $e:ident { $($v:ident $(=$val:expr)*),+ } ) => {
        $crate::arg_enum!(@impls
            ($(#[$($m),+])+
            pub enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };
    ($(#[$($m:meta),+])+ enum $e:ident { $($v:ident $(=$val:expr)*,)+ } ) => {
        $crate::arg_enum!($(#[$($m:meta),+])+
            enum $e:ident {
                $($v:ident $(=$val:expr)*),+
            }
        );
    };
    ($(#[$($m:meta),+])+ enum $e:ident { $($v:ident $(=$val:expr)*),+ } ) => {
        $crate::arg_enum!(@impls
            ($(#[$($m),+])+
            enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };
    (pub enum $e:ident { $($v:ident $(=$val:expr)*,)+ } ) => {
        $crate::arg_enum!(pub enum $e:ident {
            $($v:ident $(=$val:expr)*),+
        });
    };
    (pub enum $e:ident { $($v:ident $(=$val:expr)*),+ } ) => {
        $crate::arg_enum!(@impls
            (pub enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };
    (enum $e:ident { $($v:ident $(=$val:expr)*,)+ } ) => {
        $crate::arg_enum!(enum $e:ident {
            $($v:ident $(=$val:expr)*),+
        });
    };
    (enum $e:ident { $($v:ident $(=$val:expr)*),+ } ) => {
        $crate::arg_enum!(@impls
            (enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };
}

/// Allows you to pull the version from your Cargo.toml at compile time as
/// `MAJOR.MINOR.PATCH_PKGVERSION_PRE`
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let m = App::new("app")
///             .version(crate_version!())
///             .get_matches();
/// # }
/// ```
#[cfg(feature = "cargo")]
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Allows you to pull the authors for the app from your Cargo.toml at
/// compile time in the form:
/// `"author1 lastname <author1@example.com>:author2 lastname <author2@example.com>"`
///
/// You can replace the colons with a custom separator by supplying a
/// replacement string, so, for example,
/// `crate_authors!(",\n")` would become
/// `"author1 lastname <author1@example.com>,\nauthor2 lastname <author2@example.com>,\nauthor3 lastname <author3@example.com>"`
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let m = App::new("app")
///             .author(crate_authors!("\n"))
///             .get_matches();
/// # }
/// ```
#[cfg(feature = "cargo")]
#[macro_export]
macro_rules! crate_authors {
    ($sep:expr) => {{
        use std::ops::Deref;
        use std::boxed::Box;
        use std::cell::Cell;

        #[allow(missing_copy_implementations)]
        #[allow(dead_code)]
        struct CargoAuthors {
            authors: Cell<Option<&'static str>>,
            __private_field: (),
        };

        impl Deref for CargoAuthors {
            type Target = str;

            fn deref(&self) -> &'static str {
                let authors = self.authors.take();
                if authors.is_some() {
                    let unwrapped_authors = authors.unwrap();
                    self.authors.replace(Some(unwrapped_authors));
                    unwrapped_authors
                } else {
                    // This caches the result for subsequent invocations of the same instance of the macro
                    // to avoid performing one memory allocation per call.
                    // If performance ever becomes a problem for this code, it should be moved to build.rs
                    let s: Box<String> = Box::new(env!("CARGO_PKG_AUTHORS").replace(':', $sep));
                    let static_string = Box::leak(s);
                    self.authors.replace(Some(&*static_string));
                    &*static_string // weird but compiler-suggested way to turn a String into &str
                }
            }
        }

        &*CargoAuthors {
            authors: std::cell::Cell::new(Option::None),
            __private_field: (),
        }
    }};
    () => {
        env!("CARGO_PKG_AUTHORS")
    };
}

/// Allows you to pull the description from your Cargo.toml at compile time.
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let m = App::new("app")
///             .about(crate_description!())
///             .get_matches();
/// # }
/// ```
#[cfg(feature = "cargo")]
#[macro_export]
macro_rules! crate_description {
    () => {
        env!("CARGO_PKG_DESCRIPTION")
    };
}

/// Allows you to pull the name from your Cargo.toml at compile time.
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let m = App::new(crate_name!())
///             .get_matches();
/// # }
/// ```
#[cfg(feature = "cargo")]
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Allows you to build the `App` instance from your Cargo.toml at compile time.
///
/// Equivalent to using the `crate_*!` macros with their respective fields.
///
/// Provided separator is for the [`crate_authors!`](macro.crate_authors.html) macro,
/// refer to the documentation therefor.
///
/// **NOTE:** Changing the values in your `Cargo.toml` does not trigger a re-build automatically,
/// and therefore won't change the generated output until you recompile.
///
/// **Pro Tip:** In some cases you can "trick" the compiler into triggering a rebuild when your
/// `Cargo.toml` is changed by including this in your `src/main.rs` file
/// `include_str!("../Cargo.toml");`
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # fn main() {
/// let m = app_from_crate!().get_matches();
/// # }
/// ```
#[cfg(feature = "cargo")]
#[macro_export]
macro_rules! app_from_crate {
    () => {
        $crate::App::new($crate::crate_name!())
            .version($crate::crate_version!())
            .author($crate::crate_authors!())
            .about($crate::crate_description!())
    };
    ($sep:expr) => {
        $crate::App::new($crate::crate_name!())
            .version($crate::crate_version!())
            .author($crate::crate_authors!($sep))
            .about($crate::crate_description!())
    };
}

/// Build `App`, `Arg`s, ``s and `Group`s with Usage-string like input
/// but without the associated parsing runtime cost.
///
/// `clap_app!` also supports several shorthand syntaxes.
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # fn main() {
/// let matches = clap_app!(myapp =>
///     (version: "1.0")
///     (author: "Kevin K. <kbknapp@gmail.com>")
///     (about: "Does awesome things")
///     (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
///     (@arg INPUT: +required "Sets the input file to use")
///     (@arg debug: -d ... "Sets the level of debugging information")
///     (@group difficulty =>
///         (@arg hard: -h --hard "Sets hard mode")
///         (@arg normal: -n --normal "Sets normal mode")
///         (@arg easy: -e --easy "Sets easy mode")
///     )
///     (@subcommand test =>
///         (about: "controls testing features")
///         (version: "1.3")
///         (author: "Someone E. <someone_else@other.com>")
///         (@arg verbose: -v --verbose "Print test information verbosely")
///     )
/// );
/// # }
/// ```
/// # Shorthand Syntax for Args
///
/// * A single hyphen followed by a character (such as `-c`) sets the [`Arg::short`]
/// * A double hyphen followed by a character or word (such as `--config`) sets [`Arg::long`]
///   * If one wishes to use a [`Arg::long`] with a hyphen inside (i.e. `--config-file`), you
///     must use `--("config-file")` due to limitations of the Rust macro system.
/// * Three dots (`...`) sets [`Arg::multiple(true)`]
/// * Angled brackets after either a short or long will set [`Arg::value_name`] and
/// `Arg::required(true)` such as `--config <FILE>` = `Arg::value_name("FILE")` and
/// `Arg::required(true)`
/// * Square brackets after either a short or long will set [`Arg::value_name`] and
/// `Arg::required(false)` such as `--config [FILE]` = `Arg::value_name("FILE")` and
/// `Arg::required(false)`
/// * There are short hand syntaxes for Arg methods that accept booleans
///   * A plus sign will set that method to `true` such as `+required` = `Arg::required(true)`
///   * An exclamation will set that method to `false` such as `!required` = `Arg::required(false)`
/// * A `#{min, max}` will set [`Arg::min_values(min)`] and [`Arg::max_values(max)`]
/// * An asterisk (`*`) will set `Arg::required(true)`
/// * Curly brackets around a `fn` will set [`Arg::validator`] as in `{fn}` = `Arg::validator(fn)`
/// * An Arg method that accepts a string followed by square brackets will set that method such as
/// `conflicts_with[FOO]` will set `Arg::conflicts_with("FOO")` (note the lack of quotes around
/// `FOO` in the macro)
/// * An Arg method that takes a string and can be set multiple times (such as
/// [`Arg::conflicts_with`]) followed by square brackets and a list of values separated by spaces
/// will set that method such as `conflicts_with[FOO BAR BAZ]` will set
/// `Arg::conflicts_with("FOO")`, `Arg::conflicts_with("BAR")`, and `Arg::conflicts_with("BAZ")`
/// (note the lack of quotes around the values in the macro)
///
/// # Shorthand Syntax for Groups
///
/// * There are short hand syntaxes for `ArgGroup` methods that accept booleans
///   * A plus sign will set that method to `true` such as `+required` = `ArgGroup::required(true)`
///   * An exclamation will set that method to `false` such as `!required` = `ArgGroup::required(false)`
///
/// # Alternative form for non-ident values
///
/// Certain places that normally accept an `ident`, will optionally accept an alternative of `("expr enclosed by parens")`
/// * `(@arg something: --something)` could also be `(@arg ("something-else"): --("something-else"))`
/// * `(@subcommand something => ...)` could also be `(@subcommand ("something-else") => ...)`
///
/// [`Arg::short`]: ./struct.Arg.html#method.short
/// [`Arg::long`]: ./struct.Arg.html#method.long
/// [`Arg::multiple(true)`]: ./struct.Arg.html#method.multiple
/// [`Arg::value_name`]: ./struct.Arg.html#method.value_name
/// [`Arg::min_values(min)`]: ./struct.Arg.html#method.min_values
/// [`Arg::max_values(max)`]: ./struct.Arg.html#method.max_values
/// [`Arg::validator`]: ./struct.Arg.html#method.validator
/// [`Arg::conflicts_with`]: ./struct.Arg.html#method.conflicts_with
#[macro_export]
macro_rules! clap_app {
    (@app ($builder:expr)) => { $builder };
    (@app ($builder:expr) (@arg ($name:expr): $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.arg(
                $crate::clap_app!{ @arg ($crate::Arg::with_name($name)) (-) $($tail)* }))
            $($tt)*
        }
    };
    (@app ($builder:expr) (@arg $name:ident: $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.arg(
                $crate::clap_app!{ @arg ($crate::Arg::with_name(stringify!($name))) (-) $($tail)* }))
            $($tt)*
        }
    };
    (@app ($builder:expr) (@setting $setting:ident) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.setting($crate::AppSettings::$setting))
            $($tt)*
        }
    };
// Treat the application builder as an argument to set its attributes
    (@app ($builder:expr) (@attributes $($attr:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app ($crate::clap_app!{ @arg ($builder) $($attr)* }) $($tt)* }
    };
    (@app ($builder:expr) (@group $name:ident => $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($crate::clap_app!{ @group ($builder, $crate::ArgGroup::with_name(stringify!($name))) $($tail)* })
            $($tt)*
        }
    };
    (@app ($builder:expr) (@group $name:ident !$ident:ident => $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($crate::clap_app!{ @group ($builder, $crate::ArgGroup::with_name(stringify!($name)).$ident(false)) $($tail)* })
            $($tt)*
        }
    };
    (@app ($builder:expr) (@group $name:ident +$ident:ident => $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($crate::clap_app!{ @group ($builder, $crate::ArgGroup::with_name(stringify!($name)).$ident(true)) $($tail)* })
            $($tt)*
        }
    };
// Handle subcommand creation
    (@app ($builder:expr) (@subcommand $name:ident => $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.subcommand(
                $crate::clap_app!{ @app ($crate::App::new(stringify!($name))) $($tail)* }
            ))
            $($tt)*
        }
    };
    (@app ($builder:expr) (@subcommand ($name:expr) => $($tail:tt)*) $($tt:tt)*) => {
        clap_app!{ @app
            ($builder.subcommand(
                $crate::clap_app!{ @app ($crate::App::new($name)) $($tail)* }
            ))
            $($tt)*
        }
    };
// Yaml like function calls - used for setting various meta directly against the app
    (@app ($builder:expr) ($ident:ident: $($v:expr),*) $($tt:tt)*) => {
// clap_app!{ @app ($builder.$ident($($v),*)) $($tt)* }
        $crate::clap_app!{ @app
            ($builder.$ident($($v),*))
            $($tt)*
        }
    };

// Add members to group and continue argument handling with the parent builder
    (@group ($builder:expr, $group:expr)) => { $builder.group($group) };
    // Treat the group builder as an argument to set its attributes
    (@group ($builder:expr, $group:expr) (@attributes $($attr:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @group ($builder, $crate::clap_app!{ @arg ($group) (-) $($attr)* }) $($tt)* }
    };
    (@group ($builder:expr, $group:expr) (@arg $name:ident: $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @group
            ($crate::clap_app!{ @app ($builder) (@arg $name: $($tail)*) },
             $group.arg(stringify!($name)))
            $($tt)*
        }
    };

// No more tokens to munch
    (@arg ($arg:expr) $modes:tt) => { $arg };
// Shorthand tokens influenced by the usage_string
    (@arg ($arg:expr) $modes:tt --($long:expr) $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.long($long)) $modes $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt --$long:ident $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.long(stringify!($long))) $modes $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt -$short:ident $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.short(stringify!($short).chars().next().unwrap())) $modes $($tail)* }
    };
    (@arg ($arg:expr) (-) <$var:ident> $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.value_name(stringify!($var))) (+) +takes_value +required $($tail)* }
    };
    (@arg ($arg:expr) (+) <$var:ident> $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.value_name(stringify!($var))) (+) $($tail)* }
    };
    (@arg ($arg:expr) (-) [$var:ident] $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.value_name(stringify!($var))) (+) +takes_value $($tail)* }
    };
    (@arg ($arg:expr) (+) [$var:ident] $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.value_name(stringify!($var))) (+) $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt ... $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg) $modes +multiple $($tail)* }
    };
// Shorthand magic
    (@arg ($arg:expr) $modes:tt #{$n:expr, $m:expr} $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg) $modes min_values($n) max_values($m) $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt * $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg) $modes +required $($tail)* }
    };
// !foo -> .foo(false)
    (@arg ($arg:expr) $modes:tt !$ident:ident $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.$ident(false)) $modes $($tail)* }
    };
// +foo -> .foo(true)
    (@arg ($arg:expr) $modes:tt +$ident:ident $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.$ident(true)) $modes $($tail)* }
    };
// Validator
    (@arg ($arg:expr) $modes:tt {$fn_:expr} $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.validator($fn_)) $modes $($tail)* }
    };
    (@as_expr $expr:expr) => { $expr };
// Help
    (@arg ($arg:expr) $modes:tt $desc:tt) => { $arg.help(clap_app!{ @as_expr $desc }) };
// Handle functions that need to be called multiple times for each argument
    (@arg ($arg:expr) $modes:tt $ident:ident[$($target:ident)*] $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg $( .$ident(stringify!($target)) )*) $modes $($tail)* }
    };
// Inherit builder's functions
    (@arg ($arg:expr) $modes:tt $ident:ident($($expr:expr)*) $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.$ident($($expr)*)) $modes $($tail)* }
    };

// Build a subcommand outside of an app.
    (@subcommand $name:ident => $($tail:tt)*) => {
        $crate::clap_app!{ @app ($crate::App::new(stringify!($name))) $($tail)* }
    };
    (@subcommand ($name:expr) => $($tail:tt)*) => {
        $crate::clap_app!{ @app ($crate::App::new($name)) $($tail)* }
    };
// Start the magic
    (($name:expr) => $($tail:tt)*) => {{
        $crate::clap_app!{ @app ($crate::App::new($name)) $($tail)*}
    }};

    ($name:ident => $($tail:tt)*) => {{
        $crate::clap_app!{ @app ($crate::App::new(stringify!($name))) $($tail)*}
    }};
}

macro_rules! impl_settings {
    ($settings:ident, $flags:ident,
        $( $setting:ident($str:expr) => $flag:path ),+
    ) => {
        impl $flags {
            pub(crate) fn set(&mut self, s: $settings) {
                match s {
                    $($settings::$setting => self.0.insert($flag)),*
                }
            }

            pub(crate) fn unset(&mut self, s: $settings) {
                match s {
                    $($settings::$setting => self.0.remove($flag)),*
                }
            }

            pub(crate) fn is_set(&self, s: $settings) -> bool {
                match s {
                    $($settings::$setting => self.0.contains($flag)),*
                }
            }
        }

        impl FromStr for $settings {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
                match &*s.to_ascii_lowercase() {
                    $( $str => Ok($settings::$setting), )*
                    _ => Err(format!("unknown AppSetting: `{}`", s)),
                }
            }
        }
    }
}

// Convenience for writing to stderr thanks to https://github.com/BurntSushi
macro_rules! wlnerr(
    ($($arg:tt)*) => ({
        use std::io::{Write, stderr};
        writeln!(&mut stderr(), $($arg)*).ok();
    })
);

#[cfg(feature = "debug")]
#[cfg_attr(feature = "debug", macro_use)]
#[cfg_attr(feature = "debug", allow(unused_macros))]
mod debug_macros {
    macro_rules! debugln {
        ($fmt:expr) => (println!(concat!("DEBUG:clap:", $fmt)));
        ($fmt:expr, $($arg:tt)*) => (println!(concat!("DEBUG:clap:",$fmt), $($arg)*));
    }
    macro_rules! sdebugln {
        ($fmt:expr) => (println!($fmt));
        ($fmt:expr, $($arg:tt)*) => (println!($fmt, $($arg)*));
    }
    macro_rules! debug {
        ($fmt:expr) => (print!(concat!("DEBUG:clap:", $fmt)));
        ($fmt:expr, $($arg:tt)*) => (print!(concat!("DEBUG:clap:",$fmt), $($arg)*));
    }
    macro_rules! sdebug {
        ($fmt:expr) => (print!($fmt));
        ($fmt:expr, $($arg:tt)*) => (print!($fmt, $($arg)*));
    }
}

#[cfg(not(feature = "debug"))]
#[cfg_attr(not(feature = "debug"), macro_use)]
mod debug_macros {
    macro_rules! debugln {
        ($fmt:expr) => {};
        ($fmt:expr, $($arg:tt)*) => {};
    }
    macro_rules! sdebugln {
        ($fmt:expr) => {};
        ($fmt:expr, $($arg:tt)*) => {};
    }
    macro_rules! debug {
        ($fmt:expr) => {};
        ($fmt:expr, $($arg:tt)*) => {};
    }
}

// Helper/deduplication macro for printing the correct number of spaces in help messages
// used in:
//    src/args/arg_builder/*.rs
//    src/app/mod.rs
macro_rules! write_nspaces {
    ($dst:expr, $num:expr) => {{
        debugln!("write_spaces!: num={}", $num);
        for _ in 0..$num {
            $dst.write_all(b" ")?;
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! flags {
    ($app:expr, $how:ident) => {{
        $app.args
            .args
            .$how()
            .filter(|a| !a.is_set($crate::ArgSettings::TakesValue) && a.index.is_none())
            .filter(|a| !a.help_heading.is_some())
    }};
    ($app:expr) => {
        $crate::flags!($app, iter)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! opts {
    ($app:expr, $how:ident) => {{
        $app.args
            .args
            .$how()
            .filter(|a| a.is_set($crate::ArgSettings::TakesValue) && a.index.is_none())
            .filter(|a| !a.help_heading.is_some())
    }};
    ($app:expr) => {
        opts!($app, iter)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! positionals {
    ($app:expr, $how:ident) => {{
        $app.args
            .args
            .$how()
            .filter(|a| !(a.short.is_some() || a.long.is_some()))
    }};
    ($app:expr) => {
        positionals!($app, iter)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! subcommands {
    ($app:expr, $how:ident) => {
        $app.subcommands.$how()
    };
    ($app:expr) => {
        subcommands!($app, iter)
    };
}

macro_rules! groups_for_arg {
    ($app:expr, $grp:expr) => {{
        debugln!("Parser::groups_for_arg: name={}", $grp);
        $app.groups
            .iter()
            .filter(|grp| grp.args.iter().any(|&a| a == $grp))
            .map(|grp| grp.id)
    }};
}

macro_rules! find_subcmd_cloned {
    ($app:expr, $sc:expr) => {{
        subcommands!($app)
            .cloned()
            .find(|a| match_alias!(a, $sc, &*a.name))
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! find_subcmd {
    ($app:expr, $sc:expr, $how:ident) => {{
        subcommands!($app, $how).find(|a| match_alias!(a, $sc, &*a.name))
    }};
    ($app:expr, $sc:expr) => {
        find_subcmd!($app, $sc, iter)
    };
}

macro_rules! longs {
    ($app:expr, $how:ident) => {{
        use crate::mkeymap::KeyType;
        $app.args.keys.iter().map(|x| &x.key).filter_map(|a| {
            if let KeyType::Long(v) = a {
                Some(v)
            } else {
                None
            }
        })
    }};
    ($app:expr) => {
        longs!($app, iter)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! names {
    (@args $app:expr) => {{
        $app.args.args.iter().map(|a| &*a.name)
    }};
    (@sc $app:expr) => {{
        $app.subcommands.iter().map(|s| &*s.name).chain(
            $app.subcommands
                .iter()
                .filter(|s| s.aliases.is_some())
                .flat_map(|s| s.aliases.as_ref().unwrap().iter().map(|&(n, _)| n)),
        )
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! sc_names {
    ($app:expr) => {{
        names!(@sc $app)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! match_alias {
    ($a:expr, $to:expr, $what:expr) => {{
        $what == $to
            || ($a.aliases.is_some()
                && $a
                    .aliases
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|alias| alias.0 == $to))
    }};
}
