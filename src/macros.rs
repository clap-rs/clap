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
/// The following example shows how to load a properly formatted YAML file to build an instnace
/// of an `App` struct.
///
/// ```ignore
/// # use clap::App;
/// let yml = load_yaml!("app.yml");
/// let app = App::from_yaml(yml);
///
/// // continued logic goes here, such as `app.get_matches()` etc.
/// ```
#[cfg(feature = "yaml")]
#[macro_export]
macro_rules! load_yaml {
    ($yml:expr) => (
        &::clap::YamlLoader::load_from_str(include_str!($yml)).expect("failed to load YAML file")[0]
    );
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
///               .arg_from_usage("[length] 'Set the length to use as a pos whole num, i.e. 20'")
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
        value_t!($m.value_of($v), $t)
    };
    ($m:ident.value_of($v:expr), $t:ty) => {
        if let Some(v) = $m.value_of($v) {
            match v.parse::<$t>() {
                Ok(val) => Ok(val),
                Err(_)  =>
                    Err(::clap::Error::value_validation_auto(
                        format!("The argument '{}' isn't a valid value", v))),
            }
        } else {
            Err(::clap::Error::argument_not_found_auto($v))
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
///               .arg_from_usage("[length] 'Set the length to use as a pos whole num, i.e. 20'")
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
        value_t_or_exit!($m.value_of($v), $t)
    };
    ($m:ident.value_of($v:expr), $t:ty) => {
        if let Some(v) = $m.value_of($v) {
            match v.parse::<$t>() {
                Ok(val) => val,
                Err(_)  =>
                    ::clap::Error::value_validation_auto(
                        format!("The argument '{}' isn't a valid value", v)).exit(),
            }
        } else {
            ::clap::Error::argument_not_found_auto($v).exit()
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
///               .arg_from_usage("[seq]... 'A sequence of pos whole nums, i.e. 20 45'")
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
        values_t!($m.values_of($v), $t)
    };
    ($m:ident.values_of($v:expr), $t:ty) => {
        if let Some(vals) = $m.values_of($v) {
            let mut tmp = vec![];
            let mut err = None;
            for pv in vals {
                match pv.parse::<$t>() {
                    Ok(rv) => tmp.push(rv),
                    Err(..) => {
                        err = Some(::clap::Error::value_validation_auto(
                                format!("The argument '{}' isn't a valid value", pv)));
                        break
                    }
                }
            }
            match err {
                Some(e) => Err(e),
                None => Ok(tmp),
            }
        } else {
            Err(::clap::Error::argument_not_found_auto($v))
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
///               .arg_from_usage("[seq]... 'A sequence of pos whole nums, i.e. 20 45'")
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
        values_t_or_exit!($m.values_of($v), $t)
    };
    ($m:ident.values_of($v:expr), $t:ty) => {
        if let Some(vals) = $m.values_of($v) {
            vals.map(|v| v.parse::<$t>().unwrap_or_else(|_|{
                ::clap::Error::value_validation_auto(
                    format!("One or more arguments aren't valid values")).exit()
            })).collect::<Vec<$t>>()
        } else {
            ::clap::Error::argument_not_found_auto($v).exit()
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
    ($e:expr, $($es:expr),+) => { 1 + _clap_count_exprs!($($es),*) };
}

/// Convenience macro to generate more complete enums with variants to be used as a type when
/// parsing arguments. This enum also provides a `variants()` function which can be used to
/// retrieve a `Vec<&'static str>` of the variant names, as well as implementing [`FromStr`] and
/// [`Display`] automatically.
///
/// **NOTE:** Case insensitivity is supported for ASCII characters only
///
/// **NOTE:** This macro automatically implements [`std::str::FromStr`] and [`std::fmt::Display`]
///
/// **NOTE:** These enums support pub (or not) and uses of the #[derive()] traits
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::{App, Arg};
/// arg_enum!{
///     #[derive(Debug)]
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
///                 .arg_from_usage("<foo> 'the foo'")
///                 .get_matches();
///     let f = value_t!(m, "foo", Foo).unwrap_or_else(|e| e.exit());
///
///     // Use f like any other Foo variant...
/// }
/// ```
/// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`std::str::FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
/// [`std::fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
#[macro_export]
macro_rules! arg_enum {
    (@as_item $($i:item)*) => ($($i)*);
    (@impls ( $($tts:tt)* ) -> ($e:ident, $($v:ident),+)) => {
        arg_enum!(@as_item
        $($tts)*

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> ::std::result::Result<Self,Self::Err> {
                use ::std::ascii::AsciiExt;
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
            pub fn variants() -> [&'static str; _clap_count_exprs!($(stringify!($v)),+)] {
                [
                    $(stringify!($v),)+
                ]
            }
        });
    };
    ($(#[$($m:meta),+])+ pub enum $e:ident { $($v:ident $(=$val:expr)*),+ } ) => {
        arg_enum!(@impls
            ($(#[$($m),+])+
            pub enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };
    ($(#[$($m:meta),+])+ enum $e:ident { $($v:ident $(=$val:expr)*),+  } ) => {
        arg_enum!(@impls
            ($(#[$($m),+])+
            enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };
    (pub enum $e:ident { $($v:ident $(=$val:expr)*),+ } ) => {
        arg_enum!(@impls
            (pub enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };
    (enum $e:ident { $($v:ident $(=$val:expr)*),+ } ) => {
        arg_enum!(@impls
            (enum $e {
                $($v$(=$val)*),+
            }) -> ($e, $($v),+)
        );
    };
}

/// Allows you to pull the version from your Cargo.toml at compile time as
/// MAJOR.MINOR.PATCH_PKGVERSION_PRE
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
///     let m = App::new("app")
///                 .version(crate_version!())
///                 .get_matches();
/// # }
/// ```
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Allows you to pull the authors for the app from your Cargo.toml at
/// compile time as
/// "author1 lastname. <author1@example.com>",
///     "author2 lastname. <author2@example.com>"
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
///     let m = App::new("app")
///                 .author(crate_authors!())
///                 .get_matches();
/// # }
/// ```
#[macro_export]
macro_rules! crate_authors {
    () => {
        env!("CARGO_PKG_AUTHORS")
    };
}

/// App, Arg, SubCommand and Group builder macro (Usage-string like input) must be compiled with
/// the `unstable` feature in order to use.
#[cfg_attr(feature = "unstable", macro_export)]
macro_rules! clap_app {
    (@app ($builder:expr)) => { $builder };
    (@app ($builder:expr) (@arg $name:ident: $($tail:tt)*) $($tt:tt)*) => {
        clap_app!{ @app
            ($builder.arg(
                clap_app!{ @arg ($crate::Arg::with_name(stringify!($name))) (-) $($tail)* }))
            $($tt)*
        }
    };
    (@app ($builder:expr) (@setting $setting:ident) $($tt:tt)*) => {
        clap_app!{ @app
            ($builder.setting($crate::AppSettings::$setting))
            $($tt)*
        }
    };
// Treat the application builder as an argument to set it's attributes
    (@app ($builder:expr) (@attributes $($attr:tt)*) $($tt:tt)*) => {
        clap_app!{ @app (clap_app!{ @arg ($builder) $($attr)* }) $($tt)* }
    };
    (@app ($builder:expr) (@group $name:ident => $($tail:tt)*) $($tt:tt)*) => {
        clap_app!{ @app
            (clap_app!{ @group ($builder, $crate::ArgGroup::with_name(stringify!($name))) $($tail)* })
            $($tt)*
        }
    };
// Handle subcommand creation
    (@app ($builder:expr) (@subcommand $name:ident => $($tail:tt)*) $($tt:tt)*) => {
        clap_app!{ @app
            ($builder.subcommand(
                clap_app!{ @app ($crate::SubCommand::with_name(stringify!($name))) $($tail)* }
            ))
            $($tt)*
        }
    };
// Yaml like function calls - used for setting various meta directly against the app
    (@app ($builder:expr) ($ident:ident: $($v:expr),*) $($tt:tt)*) => {
// clap_app!{ @app ($builder.$ident($($v),*)) $($tt)* }
        clap_app!{ @app
            ($builder.$ident($($v),*))
            $($tt)*
        }
    };

// Add members to group and continue argument handling with the parent builder
    (@group ($builder:expr, $group:expr)) => { $builder.group($group) };
    (@group ($builder:expr, $group:expr) (@attributes $($attr:tt)*) $($tt:tt)*) => {
        clap_app!{ @group ($builder, clap_app!{ @arg ($group) (-) $($attr)* }) $($tt)* }
    };
    (@group ($builder:expr, $group:expr) (@arg $name:ident: $($tail:tt)*) $($tt:tt)*) => {
        clap_app!{ @group
            (clap_app!{ @app ($builder) (@arg $name: $($tail)*) },
             $group.arg(stringify!($name)))
            $($tt)*
        }
    };

// No more tokens to munch
    (@arg ($arg:expr) $modes:tt) => { $arg };
// Shorthand tokens influenced by the usage_string
    (@arg ($arg:expr) $modes:tt --$long:ident $($tail:tt)*) => {
        clap_app!{ @arg ($arg.long(stringify!($long))) $modes $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt -$short:ident $($tail:tt)*) => {
        clap_app!{ @arg ($arg.short(stringify!($short))) $modes $($tail)* }
    };
    (@arg ($arg:expr) (-) <$var:ident> $($tail:tt)*) => {
        clap_app!{ @arg ($arg.value_name(stringify!($var))) (+) +takes_value +required $($tail)* }
    };
    (@arg ($arg:expr) (+) <$var:ident> $($tail:tt)*) => {
        clap_app!{ @arg ($arg.value_name(stringify!($var))) (+) $($tail)* }
    };
    (@arg ($arg:expr) (-) [$var:ident] $($tail:tt)*) => {
        clap_app!{ @arg ($arg.value_name(stringify!($var))) (+) +takes_value $($tail)* }
    };
    (@arg ($arg:expr) (+) [$var:ident] $($tail:tt)*) => {
        clap_app!{ @arg ($arg.value_name(stringify!($var))) (+) $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt ... $($tail:tt)*) => {
        clap_app!{ @arg ($arg) $modes +multiple $($tail)* }
    };
// Shorthand magic
    (@arg ($arg:expr) $modes:tt #{$n:expr, $m:expr} $($tail:tt)*) => {
        clap_app!{ @arg ($arg) $modes min_values($n) max_values($m) $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt * $($tail:tt)*) => {
        clap_app!{ @arg ($arg) $modes +required $($tail)* }
    };
// !foo -> .foo(false)
    (@arg ($arg:expr) $modes:tt !$ident $($tail:tt)*) => {
        clap_app!{ @arg ($arg.$ident(false)) $modes $($tail)* }
    };
// foo -> .foo(true)
    (@arg ($arg:expr) $modes:tt +$ident:ident $($tail:tt)*) => {
        clap_app!{ @arg ($arg.$ident(true)) $modes $($tail)* }
    };
// Validator
    (@arg ($arg:expr) $modes:tt {$fn_:expr} $($tail:tt)*) => {
        clap_app!{ @arg ($arg.validator($fn_)) $modes $($tail)* }
    };
    (@as_expr $expr:expr) => { $expr };
// Help
    (@arg ($arg:expr) $modes:tt $desc:tt) => { $arg.help(clap_app!{ @as_expr $desc }) };
// Handle functions that need to be called multiple times for each argument
    (@arg ($arg:expr) $modes:tt $ident:ident[$($target:ident)*] $($tail:tt)*) => {
        clap_app!{ @arg ($arg $( .$ident(stringify!($target)) )*) $modes $($tail)* }
    };
// Inherit builder's functions
    (@arg ($arg:expr) $modes:tt $ident:ident($($expr:expr)*) $($tail:tt)*) => {
        clap_app!{ @arg ($arg.$ident($($expr)*)) $modes $($tail)* }
    };

// Build a subcommand outside of an app.
    (@subcommand $name:ident => $($tail:tt)*) => {
        clap_app!{ @app ($crate::SubCommand::with_name(stringify!($name))) $($tail)* }
    };
// Start the magic
    ($name:ident => $($tail:tt)*) => {{
        clap_app!{ @app ($crate::App::new(stringify!($name))) $($tail)*}
    }};
}

macro_rules! impl_settings {
    ($n:ident, $($v:ident => $c:ident),+) => {
        pub fn set(&mut self, s: $n) {
            match s {
                $($n::$v => self.0.insert($c)),+
            }
        }

        pub fn unset(&mut self, s: $n) {
            match s {
                $($n::$v => self.0.remove($c)),+
            }
        }

        pub fn is_set(&self, s: $n) -> bool {
            match s {
                $($n::$v => self.0.contains($c)),+
            }
        }
    };
}

// Convenience for writing to stderr thanks to https://github.com/BurntSushi
macro_rules! wlnerr(
    ($($arg:tt)*) => ({
        use std::io::{Write, stderr};
        writeln!(&mut stderr(), $($arg)*).ok();
    })
);
macro_rules! werr(
    ($($arg:tt)*) => ({
        use std::io::{Write, stderr};
        write!(&mut stderr(), $($arg)*).ok();
    })
);

#[cfg(feature = "debug")]
#[cfg_attr(feature = "debug", macro_use)]
mod debug_macros {
    macro_rules! debugln {
        ($fmt:expr) => (println!(concat!("*DEBUG:clap: ", $fmt)));
        ($fmt:expr, $($arg:tt)*) => (println!(concat!("*DEBUG:clap: ",$fmt), $($arg)*));
    }
    macro_rules! sdebugln {
        ($fmt:expr) => (println!($fmt));
        ($fmt:expr, $($arg:tt)*) => (println!($fmt, $($arg)*));
    }
    macro_rules! debug {
        ($fmt:expr) => (print!(concat!("*DEBUG:clap: ", $fmt)));
        ($fmt:expr, $($arg:tt)*) => (print!(concat!("*DEBUG:clap: ",$fmt), $($arg)*));
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
        ($fmt:expr) => ();
        ($fmt:expr, $($arg:tt)*) => ();
    }
    macro_rules! sdebugln {
        ($fmt:expr) => ();
        ($fmt:expr, $($arg:tt)*) => ();
    }
    macro_rules! sdebug {
        ($fmt:expr) => ();
        ($fmt:expr, $($arg:tt)*) => ();
    }
    macro_rules! debug {
        ($fmt:expr) => ();
        ($fmt:expr, $($arg:tt)*) => ();
    }
}

// Helper/deduplication macro for printing the correct number of spaces in help messages
// used in:
//    src/args/arg_builder/*.rs
//    src/app/mod.rs
macro_rules! write_spaces {
    ($num:expr, $w:ident) => ({
        debugln!("macro=write_spaces!;");
        for _ in 0..$num {
            try!(write!($w, " "));
        }
    })
}

// Helper/deduplication macro for printing the correct number of spaces in help messages
// used in:
//    src/args/arg_builder/*.rs
//    src/app/mod.rs
macro_rules! write_nspaces {
    ($dst:expr, $num:expr) => ({
        debugln!("macro=write_spaces!;");
        for _ in 0..$num {
            try!($dst.write(b" "));
        }
    })
}

// convenience macro for remove an item from a vec
macro_rules! vec_remove {
    ($vec:expr, $to_rem:expr) => {
        debugln!("macro=vec_remove!;");
        for i in (0 .. $vec.len()).rev() {
            let should_remove = &$vec[i] == $to_rem;
            if should_remove { $vec.swap_remove(i); }
        }
    };
}

// convenience macro for remove an item from a vec
macro_rules! vec_remove_all {
    ($vec:expr, $to_rem:expr) => {
        debugln!("macro=vec_remove!;");
        for i in (0 .. $vec.len()).rev() {
            let should_remove = $to_rem.contains(&$vec[i]);
            if should_remove { $vec.swap_remove(i); }
        }
    };
}
