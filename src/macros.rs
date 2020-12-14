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
/// let yaml = load_yaml!("app.yaml");
/// let app = App::from(yaml);
///
/// // continued logic goes here, such as `app.get_matches()` etc.
/// # }
/// ```
#[cfg(feature = "yaml")]
#[macro_export]
macro_rules! load_yaml {
    ($yaml:expr) => {
        &$crate::YamlLoader::load_from_str(include_str!($yaml)).expect("failed to load YAML file")
            [0]
    };
}

/// Allows you to pull the licence from your Cargo.toml at compile time. If the `license` field is
/// empty, then the `licence-field` is read. If both fields are empty, then an empty string is
/// returned.
///
/// # Examples
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let m = App::new("app")
///             .version(crate_license!())
///             .get_matches();
/// # }
/// ```
#[cfg(feature = "cargo")]
#[macro_export]
macro_rules! crate_license {
    () => {{
        let mut license = env!("CARGO_PKG_LICENSE");
        if license.is_empty() {
            license = env!("CARGO_PKG_LICENSE_FILE");
        }
        if license.is_empty() {
            license = "";
        }
        license
    }};
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
        clap::lazy_static::lazy_static! {
            static ref CACHED: String = env!("CARGO_PKG_AUTHORS").replace(':', $sep);
        }

        let s: &'static str = &*CACHED;
        s
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
            .license($crate::crate_license!())
    };
    ($sep:expr) => {
        $crate::App::new($crate::crate_name!())
            .version($crate::crate_version!())
            .author($crate::crate_authors!($sep))
            .about($crate::crate_description!())
            .license($crate::crate_license!())
    };
}

/// Build `App`, `Arg` and `Group` with Usage-string like input
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
///     (@group difficulty: +required !multiple
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
/// )
/// .get_matches();
/// # }
/// ```
///
/// # Shorthand Syntax for Args
///
/// * A single hyphen followed by a character (such as `-c`) sets the [`Arg::short`]
/// * A double hyphen followed by a character or word (such as `--config`) sets [`Arg::long`]
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
/// * Some shorthand syntaxes for `Arg` are also available for `ArgGroup`:
///   * For methods accepting a boolean: `+required` and `!multiple`, etc.
///   * For methods accepting strings: `conflicts_with[FOO]` and `conflicts_with[FOO BAR BAZ]`, etc.
///   * `*` for `ArgGroup::required`
///   * `...` for `ArgGroup::multiple`
///
/// # Alternative form for non-ident values
///
/// Certain places that normally accept an `ident` also optionally accept an alternative of `("expr enclosed by parens")`
/// * `(@arg something: --something)` could also be `(@arg ("something-else"): --("something-else"))`
/// * `(@subcommand something => ...)` could also be `(@subcommand ("something-else") => ...)`
///
/// Or it can be even simpler by using the literal directly
/// * `(@arg "something-else": --"something-else")`
/// * `(@subcommand "something-else" => ...)`
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
                $crate::clap_app!{ @arg ($crate::Arg::new($name)) (-) $($tail)* }))
            $($tt)*
        }
    };
    (@app ($builder:expr) (@arg $name:literal: $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.arg(
                $crate::clap_app!{ @arg ($crate::Arg::new(stringify!($name).trim_matches('"'))) (-) $($tail)* }))
            $($tt)*
        }
    };
    (@app ($builder:expr) (@arg $name:ident: $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.arg(
                $crate::clap_app!{ @arg ($crate::Arg::new(stringify!($name))) (-) $($tail)* }))
            $($tt)*
        }
    };
    // Global Settings
    (@app ($builder:expr) (@global_setting $setting:ident) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.global_setting($crate::AppSettings::$setting))
            $($tt)*
        }
    };
    // Settings
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
    // ArgGroup
    (@app ($builder:expr) (@group $name:ident: $($attrs:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($crate::clap_app!{ @group ($builder, $crate::ArgGroup::new(stringify!($name))) $($attrs)* })
            $($tt)*
        }
    };
    // Handle subcommand creation
    (@app ($builder:expr) (@subcommand ($name:expr) => $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.subcommand(
                $crate::clap_app!{ @app ($crate::App::new($name)) $($tail)* }
            ))
            $($tt)*
        }
    };
    (@app ($builder:expr) (@subcommand $name:literal => $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.subcommand(
                $crate::clap_app!{ @app ($crate::App::new(stringify!($name).trim_matches('"'))) $($tail)* }
            ))
            $($tt)*
        }
    };
    (@app ($builder:expr) (@subcommand $name:ident => $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.subcommand(
                $crate::clap_app!{ @app ($crate::App::new(stringify!($name))) $($tail)* }
            ))
            $($tt)*
        }
    };
    // Yaml like function calls - used for setting various meta directly against the app
    (@app ($builder:expr) ($ident:ident: $($v:expr),*) $($tt:tt)*) => {
        $crate::clap_app!{ @app
            ($builder.$ident($($v),*))
            $($tt)*
        }
    };
    // Add members to group and continue argument handling with the parent builder
    (@group ($builder:expr, $group:expr)) => { $builder.group($group) };
    // Treat the group builder as an argument to set its attributes
    (@group ($builder:expr, $group:expr) (@attributes $($attr:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @group ($builder, $group) $($attr)* $($tt)* }
    };
    (@group ($builder:expr, $group:expr) (@arg ($name:expr): $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @group
            ($crate::clap_app!{ @app ($builder) (@arg ($name): $($tail)*) },
             $group.arg($name))
            $($tt)*
        }
    };
    (@group ($builder:expr, $group:expr) (@arg $name:literal: $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @group
            ($crate::clap_app!{ @app ($builder) (@arg $name: $($tail)*) },
             $group.arg(stringify!($name).trim_matches('"')))
            $($tt)*
        }
    };
    (@group ($builder:expr, $group:expr) (@arg $name:ident: $($tail:tt)*) $($tt:tt)*) => {
        $crate::clap_app!{ @group
            ($crate::clap_app!{ @app ($builder) (@arg $name: $($tail)*) },
             $group.arg(stringify!($name)))
            $($tt)*
        }
    };
    // Handle group attributes
    (@group ($builder:expr, $group:expr) !$ident:ident $($tail:tt)*) => {
        $crate::clap_app!{ @group ($builder, $group.$ident(false)) $($tail)* }
    };
    (@group ($builder:expr, $group:expr) +$ident:ident $($tail:tt)*) => {
        $crate::clap_app!{ @group ($builder, $group.$ident(true)) $($tail)* }
    };
    (@group ($builder:expr, $group:expr) * $($tail:tt)*) => {
        $crate::clap_app!{ @group ($builder, $group) +required $($tail)* }
    };
    (@group ($builder:expr, $group:expr) ... $($tail:tt)*) => {
        $crate::clap_app!{ @group ($builder, $group) +multiple $($tail)* }
    };
    (@group ($builder:expr, $group:expr) $ident:ident[$($target:literal)*] $($tail:tt)*) => {
        $crate::clap_app!{ @group ($builder, $group $( .$ident(stringify!($target).trim_matches('"')) )*) $($tail)* }
    };
    (@group ($builder:expr, $group:expr) $ident:ident[$($target:ident)*] $($tail:tt)*) => {
        $crate::clap_app!{ @group ($builder, $group $( .$ident(stringify!($target)) )*) $($tail)* }
    };
    (@group ($builder:expr, $group:expr) $ident:ident($($expr:expr),*) $($tail:tt)*) => {
        $crate::clap_app!{ @group ($builder, $group.$ident($($expr),*)) $($tail)* }
    };
    (@group ($builder:expr, $group:expr) $ident:ident($($expr:expr,)*) $($tail:tt)*) => {
        $crate::clap_app!{ @group ($builder, $group.$ident($($expr),*)) $($tail)* }
    };

    // No more tokens to munch
    (@arg ($arg:expr) $modes:tt) => { $arg };
    // Shorthand tokens influenced by the usage_string
    (@arg ($arg:expr) $modes:tt --($long:expr) $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.long($long)) $modes $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt --$long:literal $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.long(stringify!($long).trim_matches('"'))) $modes $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt --$long:ident $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.long(stringify!($long))) $modes $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt -($short:expr) $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.short($short)) $modes $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt -$short:literal $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.short($short.to_string().chars().next().expect(r#""" is not allowed here"#))) $modes $($tail)* }
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
    (@arg ($arg:expr) $modes:tt $desc:tt) => { $arg.about(clap_app!{ @as_expr $desc }) };
    // Handle functions that need to be called multiple times for each argument
    (@arg ($arg:expr) $modes:tt $ident:ident[$($target:literal)*] $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg $( .$ident(stringify!($target).trim_matches('"')) )*) $modes $($tail)* }
    };
    (@arg ($arg:expr) $modes:tt $ident:ident[$($target:ident)*] $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg $( .$ident(stringify!($target)) )*) $modes $($tail)* }
    };
    // Inherit builder's functions, e.g. `index(2)`, `requires_if("val", "arg")`
    (@arg ($arg:expr) $modes:tt $ident:ident($($expr:expr),*) $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.$ident($($expr),*)) $modes $($tail)* }
    };
    // Inherit builder's functions with trailing comma, e.g. `index(2,)`, `requires_if("val", "arg",)`
    (@arg ($arg:expr) $modes:tt $ident:ident($($expr:expr,)*) $($tail:tt)*) => {
        $crate::clap_app!{ @arg ($arg.$ident($($expr),*)) $modes $($tail)* }
    };
    // Build a subcommand outside of an app.
    (@subcommand ($name:expr) => $($tail:tt)*) => {
        $crate::clap_app!{ @app ($crate::App::new($name)) $($tail)* }
    };
    (@subcommand $name:literal => $($tail:tt)*) => {
        $crate::clap_app!{ @app ($crate::App::new(stringify!($name).trim_matches('"'))) $($tail)* }
    };
    (@subcommand $name:ident => $($tail:tt)*) => {
        $crate::clap_app!{ @app ($crate::App::new(stringify!($name))) $($tail)* }
    };
    // Start the magic
    (($name:expr) => $($tail:tt)*) => {{
        $crate::clap_app!{ @app ($crate::App::new($name)) $($tail)*}
    }};
    ($name:literal => $($tail:tt)*) => {{
        $crate::clap_app!{ @app ($crate::App::new(stringify!($name).trim_matches('"'))) $($tail)*}
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
macro_rules! wlnerr {
    ($($arg:tt)*) => ({
        use std::io::{Write, stderr};
        writeln!(&mut stderr(), $($arg)*).ok();
    })
}

#[cfg(feature = "debug")]
macro_rules! debug {
    ($($arg:tt)*) => {
        print!("[{:>w$}] \t", module_path!(), w = 28);
        println!($($arg)*)
    }
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($($arg:tt)*) => {};
}
