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
macro_rules! debugln {
    ($fmt:expr) => (println!(concat!("**DEBUG** ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("**DEBUG** ",$fmt), $($arg)*));
}

#[cfg(feature = "debug")]
macro_rules! debug {
    ($fmt:expr) => (print!(concat!("**DEBUG** ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("**DEBUG** ",$fmt), $($arg)*));
}

#[cfg(not(feature = "debug"))]
macro_rules! debugln {
    ($fmt:expr) => ();
    ($fmt:expr, $($arg:tt)*) => ();
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($fmt:expr) => ();
    ($fmt:expr, $($arg:tt)*) => ();
}

// De-duplication macro used in src/app.rs
macro_rules! print_opt_help {
    ($me:ident, $opt:ident, $spc:expr) => {
        if let Some(h) = $opt.help {
            if h.contains("{n}") {
                let mut hel = h.split("{n}");
                if let Some(part) = hel.next() {
                    print!("{}", part);
                }
                while let Some(part) = hel.next() {
                    print!("\n");
                    $me.print_spaces($spc);
                    print!("{}", part);
                    // print!("{}", hel.next().unwrap_or(""));
                }
            } else {
                print!("{}", h);
            }
            if let Some(ref pv) = $opt.possible_vals {
                print!(" [values:");
                for pv_s in pv.iter() {
                    print!(" {}", pv_s);
                }
                print!("]");
            }
        } 
    };
}

// De-duplication macro used in src/app.rs
macro_rules! parse_group_reqs {
    ($me:ident, $arg:ident) => {
        for ag in $me.groups.values() {
            let mut found = false;
            for name in ag.args.iter() {
                if name == &$arg.name {
                    $me.required.remove(ag.name);
                    if let Some(ref reqs) = ag.requires {
                        for r in reqs {
                            $me.required.insert(r);
                        }
                    }
                    if let Some(ref bl) = ag.conflicts {
                        for b in bl {
                            $me.blacklist.insert(b);
                        }
                    }
                    found = true;
                    break;
                }
            }
            if found {
                for name in ag.args.iter() {
                    if name == &$arg.name { continue }
                    $me.required.remove(name);
                    $me.blacklist.insert(name);
                }
            }
        }
    };
}

// De-duplication macro used in src/app.rs
macro_rules! validate_reqs {
    ($me:ident, $t:ident, $m:ident, $n:ident) => {
        if let Some(a) = $me.$t.get($n) {
            if let Some(ref bl) = a.blacklist {
                for n in bl.iter() {
                    if $m.args.contains_key(n) {
                        return false
                    } else if $me.groups.contains_key(n) {
                        let grp = $me.groups.get(n).unwrap();
                        for an in grp.args.iter() {
                            if $m.args.contains_key(an) {
                                return false
                            }
                        }
                    }
                }
            }
        }
    };
}

// Thanks to bluss and flan3002 in #rust IRC
//
// Helps with rightward drift when iterating over something and matching each item.
macro_rules! for_match {
    ($it:ident, $($p:pat => $($e:expr);+),*) => {
        for i in $it {
            match i {
            $(
                $p => { $($e)+ }
            )*
            }
        }
    };
}

/// Convenience macro getting a typed value `T` where `T` implements `std::str::FromStr`
/// This macro returns a `Result<T,String>` which allows you as the developer to decide
/// what you'd like to do on a failed parse. There are two types of errors, parse failures
/// and those where the argument wasn't present (such as a non-required argument).
///
/// You can use it to get a single value, or a `Vec<T>` with the `values_of()`
///
/// **NOTE:** Be cautious, as since this a macro invocation it's not exactly like
/// standard syntax.
///
///
/// # Example single value
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let matches = App::new("myapp")
///               .arg_from_usage("[length] 'Set the length to use as a pos whole num, i.e. 20'")
///               .get_matches();
/// let len = value_t!(matches.value_of("length"), u32)
///                 .unwrap_or_else(|e|{
///                     println!("{}",e);
///                     std::process::exit(1)
///                 });
///
/// println!("{} + 2: {}", len, len + 2);
/// # }
/// ```
///
///
/// # Example multiple values
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let matches = App::new("myapp")
///               .arg_from_usage("[seq]... 'A sequence of pos whole nums, i.e. 20 45'")
///               .get_matches();
/// for v in value_t!(matches.values_of("seq"), u32)
///             .unwrap_or_else(|e|{
///                 println!("{}",e);
///                 std::process::exit(1)
///             }) {
///     println!("{} + 2: {}", v, v + 2);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! value_t {
    ($m:ident.value_of($v:expr), $t:ty) => {
        match $m.value_of($v) {
            Some(v) => {
                match v.parse::<$t>() {
                    Ok(val) => Ok(val),
                    Err(_)  => Err(format!("'{}' isn't a valid value", ::clap::Format::Warning(v))),
                }
            },
            None => Err(format!("The argument '{}' not found", ::clap::Format::Warning($v)))
        }
    };
    ($m:ident.values_of($v:expr), $t:ty) => {
        match $m.values_of($v) {
            Some(ref v) => {
                let mut tmp = Vec::with_capacity(v.len());
                let mut err = None;
                for pv in v {
                    match pv.parse::<$t>() {
                        Ok(rv) => tmp.push(rv),
                        Err(e) => {
                            err = Some(format!("'{}' isn't a valid value\n\t{}", ::clap::Format::Warning(pv),e));
                            break
                        }
                    }
                }
                match err {
                    Some(e) => Err(e),
                    None => Ok(tmp)
                }
            },
            None => Err(format!("The argument '{}' was not found", ::clap::Format::Warning($v)))
        }
    };
}

/// Convenience macro getting a typed value `T` where `T` implements `std::str::FromStr`
/// This macro returns a `T` or `Vec<T>` or exits with a usage string upon failure. This
/// removes some of the boiler plate to handle failures from value_t! above.
///
/// You can use it to get a single value `T`, or a `Vec<T>` with the `values_of()`
///
/// **NOTE:** This should only be used on required arguments, as it can be confusing to the user
/// why they are getting error messages when it appears they're entering all required argumetns.
///
/// **NOTE:** Be cautious, as since this a macro invocation it's not exactly like
/// standard syntax.
///
///
/// # Example single value
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let matches = App::new("myapp")
///               .arg_from_usage("[length] 'Set the length to use as a pos whole num, i.e. 20'")
///               .get_matches();
/// let len = value_t_or_exit!(matches.value_of("length"), u32);
///
/// println!("{} + 2: {}", len, len + 2);
/// # }
/// ```
///
///
/// # Example multiple values
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
/// let matches = App::new("myapp")
///                   .arg_from_usage("[seq]... 'A sequence of pos whole nums, i.e. 20 45'")
///                   .get_matches();
/// for v in value_t_or_exit!(matches.values_of("seq"), u32) {
///     println!("{} + 2: {}", v, v + 2);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! value_t_or_exit {
    ($m:ident.value_of($v:expr), $t:ty) => {
        match $m.value_of($v) {
            Some(v) => {
                match v.parse::<$t>() {
                    Ok(val) => val,
                    Err(..)  => {
                        println!("{} '{}' isn't a valid value\n\n{}\n\nPlease re-run with {} for \
                            more information",
                            ::clap::Format::Error("error:"),
                            ::clap::Format::Warning(v.to_string()),
                            $m.usage(),
                            ::clap::Format::Good("--help"));
                        ::std::process::exit(1);
                    }
                }
            },
            None => {
                println!("{} The argument '{}' was not found or is not valid\n\n{}\n\nPlease re-run with \
                    {} for more information",
                    ::clap::Format::Error("error:"),
                    ::clap::Format::Warning($v.to_string()),
                    $m.usage(),
                    ::clap::Format::Good("--help"));
                ::std::process::exit(1);
            }
        }
    };
    ($m:ident.values_of($v:expr), $t:ty) => {
        match $m.values_of($v) {
            Some(ref v) => {
                let mut tmp = Vec::with_capacity(v.len());
                for pv in v {
                    match pv.parse::<$t>() {
                        Ok(rv) => tmp.push(rv),
                        Err(_)  => {
                            println!("{} '{}' isn't a valid value\n\n{}\n\nPlease re-run with {} for more \
                                information",
                                ::clap::Format::Error("error:"),
                                ::clap::Format::Warning(pv),
                                $m.usage(),
                                ::clap::Format::Good("--help"));
                            ::std::process::exit(1);
                        }
                    }
                }
                tmp
            },
            None => {
                println!("{} The argument '{}' not found or is not valid\n\n{}\n\nPlease re-run with \
                    {} for more information",
                    ::clap::Format::Error("error:"),
                    ::clap::Format::Warning($v.to_string()),
                    $m.usage(),
                    ::clap::Format::Good("--help"));
                ::std::process::exit(1);
            }
        }
    };
}

/// Convenience macro generated a simple enum with variants to be used as a type when parsing
/// arguments. This enum also provides a `variants()` function which can be used to retrieve a
/// `Vec<&'static str>` of the variant names.
///
/// **NOTE:** This macro automaically implements std::str::FromStr and std::fmt::Display
///
/// # Example
///
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::{App, Arg};
/// simple_enum!{Foo => Bar, Baz, Qux}
/// // Foo enum can now be used via Foo::Bar, or Foo::Baz, etc
/// // and implements std::str::FromStr to use with the value_t! macros
/// fn main() {
///     let enum_vals = ["Bar", "Baz", "Qux"];
///     let m = App::new("app")
///                 .arg(Arg::from_usage("<foo> 'the foo'")
///                     .possible_values(&enum_vals))
///                 .get_matches();
///     let f = value_t_or_exit!(m.value_of("foo"), Foo);
///
///     // Use f like any other Foo variant...
/// }
/// ```
#[macro_export]
macro_rules! simple_enum {
    ($e:ident => $($v:ident),+) => {
        enum $e {
            $($v),+
        }

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                match s {
                    $(stringify!($v) => Ok($e::$v),)+
                    _                => Err({
                                            let v = vec![
                                                $(stringify!($v),)+
                                            ];
                                            format!("valid values:{}",
                                                v.iter().fold(String::new(), |a, i| {
                                                    a + &format!(" {}", i)[..]
                                                }))
                                        })
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
            pub fn variants() -> Vec<&'static str> {
                vec![
                    $(stringify!($v),)+
                ]
            }
        }
    };
}

/// Convenience macro to generate more complete enums with variants to be used as a type when
/// parsing arguments. This enum also provides a `variants()` function which can be used to retrieve a
/// `Vec<&'static str>` of the variant names.
///
/// **NOTE:** Case insensitivity is supported for ASCII characters
///
/// **NOTE:** This macro automaically implements std::str::FromStr and std::fmt::Display
///
/// These enums support pub (or not) and use of the #[derive()] traits
///
///
/// # Example
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
///     let f = value_t_or_exit!(m.value_of("foo"), Foo);
///
///     // Use f like any other Foo variant...
/// }
/// ```
#[macro_export]
macro_rules! arg_enum {
    (enum $e:ident { $($v:ident),+ } ) => {
        enum $e {
            $($v),+
        }

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                use ::std::ascii::AsciiExt;
                match s {
                    $(stringify!($v) |
                    _ if s.eq_ignore_ascii_case(stringify!($v)) => Ok($e::$v),)+
                    _                => Err({
                                            let v = vec![
                                                $(stringify!($v),)+
                                            ];
                                            format!("valid values:{}",
                                                v.iter().fold(String::new(), |a, i| {
                                                    a + &format!(" {}", i)[..]
                                                }))
                                        })
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
            fn variants() -> Vec<&'static str> {
                vec![
                    $(stringify!($v),)+
                ]
            }
        }
    };
    (pub enum $e:ident { $($v:ident),+ } ) => {
        pub enum $e {
            $($v),+
        }

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                use ::std::ascii::AsciiExt;
                match s {
                    $(stringify!($v) |
                    _ if s.eq_ignore_ascii_case(stringify!($v)) => Ok($e::$v),)+
                    _                => Err({
                                            let v = vec![
                                                $(stringify!($v),)+
                                            ];
                                            format!("valid values:{}",
                                                v.iter().fold(String::new(), |a, i| {
                                                    a + &format!(" {}", i)[..]
                                                }))
                                        })
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
            pub fn variants() -> Vec<&'static str> {
                vec![
                    $(stringify!($v),)+
                ]
            }
        }
    };
    (#[derive($($d:ident),+)] enum $e:ident { $($v:ident),+ } ) => {
        #[derive($($d,)+)]
        enum $e {
            $($v),+
        }

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                use ::std::ascii::AsciiExt;
                match s {
                    $(stringify!($v) |
                    _ if s.eq_ignore_ascii_case(stringify!($v)) => Ok($e::$v),)+
                    _                => Err({
                                            let v = vec![
                                                $(stringify!($v),)+
                                            ];
                                            format!("valid values:{}",
                                                v.iter().fold(String::new(), |a, i| {
                                                    a + &format!(" {}", i)[..]
                                                }))
                                        })
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
            pub fn variants() -> Vec<&'static str> {
                vec![
                    $(stringify!($v),)+
                ]
            }
        }
    };
    (#[derive($($d:ident),+)] pub enum $e:ident { $($v:ident),+ } ) => {
        #[derive($($d,)+)]
        pub enum $e {
            $($v),+
        }

        impl ::std::str::FromStr for $e {
            type Err = String;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                use ::std::ascii::AsciiExt;
                match s {
                    $(stringify!($v) |
                    _ if s.eq_ignore_ascii_case(stringify!($v)) => Ok($e::$v),)+
                    _                => Err({
                                            let v = vec![
                                                $(stringify!($v),)+
                                            ];
                                            format!("valid values:{}",
                                                v.iter().fold(String::new(), |a, i| {
                                                    a + &format!(" {}", i)[..]
                                                }))
                                        })
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
            pub fn variants() -> Vec<&'static str> {
                vec![
                    $(stringify!($v),)+
                ]
            }
        }
    };
}

/// Allows you pull the version for an from your Cargo.toml as MAJOR.MINOR.PATCH_PKGVERSION_PRE
///
/// # Example
/// ```no_run
/// # #[macro_use]
/// # extern crate clap;
/// # use clap::App;
/// # fn main() {
///     let m = App::new("app")
///                 .version(&crate_version!()[..])
///                 .get_matches();
/// # }
/// ```
#[macro_export]
macro_rules! crate_version {
    () => {
        format!("{}.{}.{}{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
            option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
    }
}
