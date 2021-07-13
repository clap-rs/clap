// Std
use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    fmt::{Debug, Display},
    iter::{Cloned, Flatten, Map},
    slice::Iter,
    str::FromStr,
};

// Third Party
use indexmap::IndexMap;

// Internal
use crate::{
    parse::MatchedArg,
    util::{termcolor::ColorChoice, Id, Key},
    {Error, INVALID_UTF8},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SubCommand {
    pub(crate) id: Id,
    pub(crate) name: String,
    pub(crate) matches: ArgMatches,
}

/// Used to get information about the arguments that were supplied to the program at runtime by
/// the user. New instances of this struct are obtained by using the [`App::get_matches`] family of
/// methods.
///
/// # Examples
///
/// ```no_run
/// # use clap::{App, Arg};
/// let matches = App::new("MyApp")
///     .arg(Arg::new("out")
///         .long("output")
///         .required(true)
///         .takes_value(true))
///     .arg(Arg::new("debug")
///         .short('d')
///         .multiple_occurrences(true))
///     .arg(Arg::new("cfg")
///         .short('c')
///         .takes_value(true))
///     .get_matches(); // builds the instance of ArgMatches
///
/// // to get information about the "cfg" argument we created, such as the value supplied we use
/// // various ArgMatches methods, such as ArgMatches::value_of
/// if let Some(c) = matches.value_of("cfg") {
///     println!("Value for -c: {}", c);
/// }
///
/// // The ArgMatches::value_of method returns an Option because the user may not have supplied
/// // that argument at runtime. But if we specified that the argument was "required" as we did
/// // with the "out" argument, we can safely unwrap because `clap` verifies that was actually
/// // used at runtime.
/// println!("Value for --output: {}", matches.value_of("out").unwrap());
///
/// // You can check the presence of an argument
/// if matches.is_present("out") {
///     // Another way to check if an argument was present, or if it occurred multiple times is to
///     // use occurrences_of() which returns 0 if an argument isn't found at runtime, or the
///     // number of times that it occurred, if it was. To allow an argument to appear more than
///     // once, you must use the .multiple_occurrences(true) method, otherwise it will only return 1 or 0.
///     if matches.occurrences_of("debug") > 2 {
///         println!("Debug mode is REALLY on, don't be crazy");
///     } else {
///         println!("Debug mode kind of on");
///     }
/// }
/// ```
/// [`App::get_matches`]: crate::App::get_matches()
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgMatches {
    pub(crate) args: IndexMap<Id, MatchedArg>,
    pub(crate) subcommand: Option<Box<SubCommand>>,
}

impl Default for ArgMatches {
    fn default() -> Self {
        ArgMatches {
            args: IndexMap::new(),
            subcommand: None,
        }
    }
}

impl ArgMatches {
    /// Gets the value of a specific [option] or [positional] argument (i.e. an argument that takes
    /// an additional value at runtime). If the option wasn't present at runtime
    /// it returns `None`.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer [`ArgMatches::values_of`] as `ArgMatches::value_of` will only return the *first*
    /// value.
    ///
    /// *NOTE:* This will always return `Some(value)` if [`default_value`] has been set.
    /// [`occurrences_of`] can be used to check if a value is present at runtime.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if the value contains invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::new("output")
    ///         .takes_value(true))
    ///     .get_matches_from(vec!["myapp", "something"]);
    ///
    /// assert_eq!(m.value_of("output"), Some("something"));
    /// ```
    /// [option]: crate::Arg::takes_value()
    /// [positional]: crate::Arg::index()
    /// [`ArgMatches::values_of`]: ArgMatches::values_of()
    /// [`default_value`]: crate::Arg::default_value()
    /// [`occurrences_of`]: crate::ArgMatches::occurrences_of()
    pub fn value_of<T: Key>(&self, id: T) -> Option<&str> {
        if let Some(arg) = self.args.get(&Id::from(id)) {
            if let Some(v) = arg.get_val(0) {
                return Some(v.to_str().expect(INVALID_UTF8));
            }
        }
        None
    }

    /// Gets the lossy value of a specific argument. If the argument wasn't present at runtime
    /// it returns `None`. A lossy value is one which contains invalid UTF-8 code points, those
    /// invalid points will be replaced with `\u{FFFD}`
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer [`Arg::values_of_lossy`] as `value_of_lossy()` will only return the *first* value.
    ///
    /// *NOTE:* This will always return `Some(value)` if [`default_value`] has been set.
    /// [`occurrences_of`] can be used to check if a value is present at runtime.
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::{OsStrExt,OsStringExt};
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi {0xe9}!"
    ///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
    /// assert_eq!(&*m.value_of_lossy("arg").unwrap(), "Hi \u{FFFD}!");
    /// ```
    /// [`default_value`]: crate::Arg::default_value()
    /// [`occurrences_of`]: ArgMatches::occurrences_of()
    /// [`Arg::values_of_lossy`]: ArgMatches::values_of_lossy()
    pub fn value_of_lossy<T: Key>(&self, id: T) -> Option<Cow<'_, str>> {
        if let Some(arg) = self.args.get(&Id::from(id)) {
            if let Some(v) = arg.get_val(0) {
                return Some(v.to_string_lossy());
            }
        }
        None
    }

    /// Gets the OS version of a string value of a specific argument. If the option wasn't present
    /// at runtime it returns `None`. An OS value on Unix-like systems is any series of bytes,
    /// regardless of whether or not they contain valid UTF-8 code points. Since [`String`]s in
    /// Rust are guaranteed to be valid UTF-8, a valid filename on a Unix system as an argument
    /// value may contain invalid UTF-8 code points.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer [`ArgMatches::values_of_os`] as `Arg::value_of_os` will only return the *first*
    /// value.
    ///
    /// *NOTE:* This will always return `Some(value)` if [`default_value`] has been set.
    /// [`occurrences_of`] can be used to check if a value is present at runtime.
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::{OsStrExt,OsStringExt};
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi {0xe9}!"
    ///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
    /// assert_eq!(&*m.value_of_os("arg").unwrap().as_bytes(), [b'H', b'i', b' ', 0xe9, b'!']);
    /// ```
    /// [`default_value`]: crate::Arg::default_value()
    /// [`occurrences_of`]: ArgMatches::occurrences_of()
    /// [`ArgMatches::values_of_os`]: ArgMatches::values_of_os()
    pub fn value_of_os<T: Key>(&self, id: T) -> Option<&OsStr> {
        self.args
            .get(&Id::from(id))
            .and_then(|arg| arg.get_val(0).map(OsString::as_os_str))
    }

    /// Gets a [`Values`] struct which implements [`Iterator`] for values of a specific argument
    /// (i.e. an argument that takes multiple values at runtime). If the option wasn't present at
    /// runtime it returns `None`
    ///
    /// # Panics
    ///
    /// This method will panic if any of the values contain invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myprog")
    ///     .arg(Arg::new("output")
    ///         .multiple_values(true)
    ///         .short('o')
    ///         .takes_value(true))
    ///     .get_matches_from(vec![
    ///         "myprog", "-o", "val1", "val2", "val3"
    ///     ]);
    /// let vals: Vec<&str> = m.values_of("output").unwrap().collect();
    /// assert_eq!(vals, ["val1", "val2", "val3"]);
    /// ```
    /// [`Iterator`]: std::iter::Iterator
    pub fn values_of<T: Key>(&self, id: T) -> Option<Values> {
        self.args.get(&Id::from(id)).map(|arg| {
            fn to_str_slice(o: &OsString) -> &str {
                o.to_str().expect(INVALID_UTF8)
            }

            Values {
                iter: arg.vals_flatten().map(to_str_slice),
            }
        })
    }

    /// Placeholder documentation.
    pub fn grouped_values_of<T: Key>(&self, id: T) -> Option<GroupedValues> {
        #[allow(clippy::type_complexity)]
        let arg_values: for<'a> fn(
            &'a MatchedArg,
        ) -> Map<
            Iter<'a, Vec<OsString>>,
            fn(&Vec<OsString>) -> Vec<&str>,
        > = |arg| {
            arg.vals()
                .map(|g| g.iter().map(|x| x.to_str().expect(INVALID_UTF8)).collect())
        };
        self.args
            .get(&Id::from(id))
            .map(arg_values)
            .map(|iter| GroupedValues { iter })
    }

    /// Gets the lossy values of a specific argument. If the option wasn't present at runtime
    /// it returns `None`. A lossy value is one where if it contains invalid UTF-8 code points,
    /// those invalid points will be replaced with `\u{FFFD}`
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStringExt;
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from("<arg>... 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi"
    ///                             OsString::from_vec(vec![b'H', b'i']),
    ///                             // "{0xe9}!"
    ///                             OsString::from_vec(vec![0xe9, b'!'])]);
    /// let mut itr = m.values_of_lossy("arg").unwrap().into_iter();
    /// assert_eq!(&itr.next().unwrap()[..], "Hi");
    /// assert_eq!(&itr.next().unwrap()[..], "\u{FFFD}!");
    /// assert_eq!(itr.next(), None);
    /// ```
    pub fn values_of_lossy<T: Key>(&self, id: T) -> Option<Vec<String>> {
        self.args.get(&Id::from(id)).map(|arg| {
            arg.vals_flatten()
                .map(|v| v.to_string_lossy().into_owned())
                .collect()
        })
    }

    /// Gets a [`OsValues`] struct which is implements [`Iterator`] for [`OsString`] values of a
    /// specific argument. If the option wasn't present at runtime it returns `None`. An OS value
    /// on Unix-like systems is any series of bytes, regardless of whether or not they contain
    /// valid UTF-8 code points. Since [`String`]s in Rust are guaranteed to be valid UTF-8, a valid
    /// filename as an argument value on Linux (for example) may contain invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{App, Arg};
    /// use std::ffi::{OsStr,OsString};
    /// use std::os::unix::ffi::{OsStrExt,OsStringExt};
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from("<arg>... 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                                 // "Hi"
    ///                                 OsString::from_vec(vec![b'H', b'i']),
    ///                                 // "{0xe9}!"
    ///                                 OsString::from_vec(vec![0xe9, b'!'])]);
    ///
    /// let mut itr = m.values_of_os("arg").unwrap().into_iter();
    /// assert_eq!(itr.next(), Some(OsStr::new("Hi")));
    /// assert_eq!(itr.next(), Some(OsStr::from_bytes(&[0xe9, b'!'])));
    /// assert_eq!(itr.next(), None);
    /// ```
    /// [`Iterator`]: std::iter::Iterator
    /// [`OsString`]: std::ffi::OsString
    /// [`String`]: std::string::String
    pub fn values_of_os<T: Key>(&self, id: T) -> Option<OsValues> {
        fn to_str_slice(o: &OsString) -> &OsStr {
            o
        }

        self.args.get(&Id::from(id)).map(|arg| OsValues {
            iter: arg.vals_flatten().map(to_str_slice),
        })
    }

    /// Gets the value of a specific argument (i.e. an argument that takes an additional
    /// value at runtime) and then converts it into the result type using [`std::str::FromStr`].
    ///
    /// There are two types of errors, parse failures and those where the argument wasn't present
    /// (such as a non-required argument). Check [`ErrorKind`] to distinguish them.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer [`ArgMatches::values_of_t`] as this method will only return the *first*
    /// value.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if the value contains invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate clap;
    /// # use clap::App;
    /// let matches = App::new("myapp")
    ///               .arg("[length] 'Set the length to use as a pos whole num, i.e. 20'")
    ///               .get_matches_from(&["test", "12"]);
    ///
    /// // Specify the type explicitly (or use turbofish)
    /// let len: u32 = matches.value_of_t("length").unwrap_or_else(|e| e.exit());
    /// assert_eq!(len, 12);
    ///
    /// // You can often leave the type for rustc to figure out
    /// let also_len = matches.value_of_t("length").unwrap_or_else(|e| e.exit());
    /// // Something that expects u32
    /// let _: u32 = also_len;
    /// ```
    ///
    /// [`ArgMatches::values_of_t`]: ArgMatches::values_of_t()
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    /// [`ErrorKind`]: crate::ErrorKind
    pub fn value_of_t<R>(&self, name: &str) -> Result<R, Error>
    where
        R: FromStr,
        <R as FromStr>::Err: Display,
    {
        self.parse_t(name, R::from_str)
    }

    /// Gets the value of a specific argument (i.e. an argument that takes an additional
    /// value at runtime) and then converts it into the result type using `parse`.
    ///
    /// There are two types of errors, parse failures and those where the argument wasn't present
    /// (such as a non-required argument). Check [`ErrorKind`] to distinguish them.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer [`ArgMatches::parse_vec_t`] as this method will only return the *first*
    /// value.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if the value contains invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate clap;
    /// # use clap::App;
    /// # use std::str::FromStr;
    /// let matches = App::new("myapp")
    ///               .arg("[length] 'Set the length to use as a pos whole num, i.e. 20'")
    ///               .get_matches_from(&["test", "12"]);
    ///
    /// // Specify the type explicitly (or use turbofish)
    /// let len: u32 = matches.parse_t("length", FromStr::from_str).unwrap_or_else(|e| e.exit());
    /// assert_eq!(len, 12);
    ///
    /// // You can often leave the type for rustc to figure out
    /// let also_len = matches
    ///     .parse_t("length", FromStr::from_str)
    ///     .unwrap_or_else(|e| e.exit());
    /// // Something that expects u32
    /// let _: u32 = also_len;
    /// ```
    ///
    /// [`ErrorKind`]: enum.ErrorKind.html
    /// [`ArgMatches::parse_vec_t`]: ./struct.ArgMatches.html#method.parse_vec_t
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    pub fn parse_t<R, E>(
        &self,
        name: &str,
        parse: impl Fn(&str) -> Result<R, E>,
    ) -> Result<R, Error>
    where
        E: Display,
    {
        self.parse_optional_t(name, parse)?
            .ok_or_else(|| Error::argument_not_found_auto(name.to_string()))
    }

    /// Gets the optional value of a specific argument (i.e. an argument that takes an additional
    /// value at runtime) and then converts it into the result type using `parse`.
    ///
    /// In the case where the argument wasn't present, `Ok(None)` is returned. In the
    /// case where it was present and parsing succeeded, `Ok(Some(value))` is returned.
    /// If parsing (of any value) has failed, returns Err.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer [`ArgMatches::parse_optional_vec_t`] as this method will only return the *first*
    /// value.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if the value contains invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate clap;
    /// # use clap::App;
    /// # use std::str::FromStr;
    /// let matches = App::new("myapp")
    ///               .arg("[length] @20 'Set the length to use as a pos whole num, i.e. 20'")
    ///               .get_matches_from(&["test", "12"]);
    ///
    /// // Specify the type explicitly (or use turbofish)
    /// let len: Option<u32> = matches
    ///     .parse_optional_t("length", FromStr::from_str)
    ///     .unwrap_or_else(|e| e.exit());
    /// assert_eq!(len, Some(12));
    ///
    /// // You can often leave the type for rustc to figure out
    /// let also_len = matches
    ///     .parse_optional_t("length", FromStr::from_str)
    ///     .unwrap_or_else(|e| e.exit());
    /// // Something that expects Option<u32>
    /// let _: Option<u32> = also_len;
    /// ```
    ///
    /// [`ErrorKind`]: enum.ErrorKind.html
    /// [`ArgMatches::parse_optional_vec_t`]: ./struct.ArgMatches.html#method.parse_optional_vec_t
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    pub fn parse_optional_t<R, E>(
        &self,
        name: &str,
        parse: impl Fn(&str) -> Result<R, E>,
    ) -> Result<Option<R>, Error>
    where
        E: Display,
    {
        Ok(match self.value_of(name) {
            Some(v) => Some(parse(v).map_err(|e| {
                let message = format!(
                    "The argument '{}' isn't a valid value for '{}': {}",
                    v, name, e
                );

                Error::value_validation(
                    name.to_string(),
                    v.to_string(),
                    message.into(),
                    ColorChoice::Auto,
                )
            })?),
            None => None,
        })
    }

    /// Gets the value of a specific argument (i.e. an argument that takes an additional
    /// value at runtime) and then converts it into the result type using `parse`, which takes
    /// the OS version of the string value of the argument.
    ///
    /// There are two types of errors, parse failures and those where the argument wasn't present
    /// (such as a non-required argument). Check [`ErrorKind`] to distinguish them.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer [`ArgMatches::parse_vec_t_os`] as `Arg::parse_t_os` will only return the *first*
    /// value.
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::{OsStrExt,OsStringExt};
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi {0xe9}!"
    ///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
    /// assert_eq!(&*m.value_of_os("arg").unwrap().as_bytes(), [b'H', b'i', b' ', 0xe9, b'!']);
    /// ```
    /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
    /// [`ArgMatches::parse_vec_t_os`]: ./struct.ArgMatches.html#method.parse_vec_t_os
    pub fn parse_t_os<R>(
        &self,
        name: &str,
        parse: impl Fn(&OsStr) -> Result<R, OsString>,
    ) -> Result<R, Error> {
        self.parse_optional_t_os(name, parse)?
            .ok_or_else(|| Error::argument_not_found_auto(name.to_string()))
    }

    /// Gets optional the value of a specific argument (i.e. an argument that takes an additional
    /// value at runtime) and then converts it into the result type using `parse`, which takes
    /// the OS version of the string value of the argument.
    ///
    /// In the case where the argument wasn't present, `Ok(None)` is returned. In the
    /// case where it was present and parsing succeeded, `Ok(Some(value))` is returned.
    /// If parsing (of any value) has failed, returns Err.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer [`ArgMatches::parse_vec_t_os`] as `Arg::parse_t_os` will only return the *first*
    /// value.
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::{OsStrExt,OsStringExt};
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi {0xe9}!"
    ///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
    /// assert_eq!(&*m.value_of_os("arg").unwrap().as_bytes(), [b'H', b'i', b' ', 0xe9, b'!']);
    /// ```
    /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
    /// [`ArgMatches::parse_vec_t_os`]: ./struct.ArgMatches.html#method.parse_vec_t_os
    pub fn parse_optional_t_os<R>(
        &self,
        name: &str,
        parse: impl Fn(&OsStr) -> Result<R, OsString>,
    ) -> Result<Option<R>, Error> {
        Ok(match self.value_of_os(name) {
            Some(v) => Some(parse(v).map_err(|e| {
                let message = format!(
                    "The argument '{}' isn't a valid value for '{}': {}",
                    v.to_string_lossy(),
                    name,
                    e.to_string_lossy()
                );

                Error::value_validation(
                    name.to_string(),
                    v.to_string_lossy().to_string(),
                    message.into(),
                    ColorChoice::Auto,
                )
            })?),
            None => None,
        })
    }

    /// Gets the value of a specific argument (i.e. an argument that takes an additional
    /// value at runtime) and then converts it into the result type using [`std::str::FromStr`].
    ///
    /// If either the value is not present or parsing failed, exits the program.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if the value contains invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate clap;
    /// # use clap::App;
    /// let matches = App::new("myapp")
    ///               .arg("[length] 'Set the length to use as a pos whole num, i.e. 20'")
    ///               .get_matches_from(&["test", "12"]);
    ///
    /// // Specify the type explicitly (or use turbofish)
    /// let len: u32 = matches.value_of_t_or_exit("length");
    /// assert_eq!(len, 12);
    ///
    /// // You can often leave the type for rustc to figure out
    /// let also_len = matches.value_of_t_or_exit("length");
    /// // Something that expects u32
    /// let _: u32 = also_len;
    /// ```
    ///
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    pub fn value_of_t_or_exit<R>(&self, name: &str) -> R
    where
        R: FromStr,
        <R as FromStr>::Err: Display,
    {
        self.value_of_t(name).unwrap_or_else(|e| e.exit())
    }

    /// Gets the typed values of a specific argument (i.e. an argument that takes multiple
    /// values at runtime) and then converts them into the result type using [`std::str::FromStr`].
    ///
    /// If parsing (of any value) has failed, returns Err.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if any of the values contains invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate clap;
    /// # use clap::App;
    /// let matches = App::new("myapp")
    ///               .arg("[length]... 'A sequence of integers because integers are neat!'")
    ///               .get_matches_from(&["test", "12", "77", "40"]);
    ///
    /// // Specify the type explicitly (or use turbofish)
    /// let len: Vec<u32> = matches.values_of_t("length").unwrap_or_else(|e| e.exit());
    /// assert_eq!(len, vec![12, 77, 40]);
    ///
    /// // You can often leave the type for rustc to figure out
    /// let also_len = matches.values_of_t("length").unwrap_or_else(|e| e.exit());
    /// // Something that expects Vec<u32>
    /// let _: Vec<u32> = also_len;
    /// ```
    ///
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    pub fn values_of_t<R>(&self, name: &str) -> Result<Vec<R>, Error>
    where
        R: FromStr,
        <R as FromStr>::Err: Display,
    {
        self.parse_vec_t(name, R::from_str)
    }

    /// Gets the typed values of a specific argument (i.e. an argument that takes multiple
    /// values at runtime) and then converts them into the result type using `parse`.
    ///
    /// If parsing (of any value) has failed, returns Err.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if any of the values contains invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate clap;
    /// # use clap::App;
    /// # use std::str::FromStr;
    /// let matches = App::new("myapp")
    ///               .arg("[length]... 'A sequence of integers because integers are neat!'")
    ///               .get_matches_from(&["test", "12", "77", "40"]);
    ///
    /// // Specify the type explicitly (or use turbofish)
    /// let len: Vec<u32> = matches
    ///     .parse_vec_t("length", FromStr::from_str)
    ///     .unwrap_or_else(|e| e.exit());
    /// assert_eq!(len, vec![12, 77, 40]);
    ///
    /// // You can often leave the type for rustc to figure out
    /// let also_len = matches
    ///     .parse_vec_t("length", FromStr::from_str)
    ///     .unwrap_or_else(|e| e.exit());
    /// // Something that expects Vec<u32>
    /// let _: Vec<u32> = also_len;
    /// ```
    ///
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    pub fn parse_vec_t<R, E>(
        &self,
        name: &str,
        parse: impl Fn(&str) -> Result<R, E>,
    ) -> Result<Vec<R>, Error>
    where
        E: Display,
    {
        self.parse_optional_vec_t(name, parse)?
            .ok_or_else(|| Error::argument_not_found_auto(name.to_string()))
    }

    /// Gets the optional typed values of a specific argument (i.e. an argument that takes multiple
    /// values at runtime) and then converts them into the result type using `parse`.
    ///
    /// In the case where the argument wasn't present, `Ok(None)` is returned. In the
    /// case where it was present and parsing succeeded, `Ok(Some(vec))` is returned.
    /// If parsing (of any value) has failed, returns Err.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if any of the values contains invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate clap;
    /// # use clap::App;
    /// # use std::str::FromStr;
    /// let matches = App::new("myapp")
    ///               .arg("[length]... 'A sequence of integers because integers are neat!'")
    ///               .get_matches_from(&["test", "12", "77", "40"]);
    ///
    /// // Specify the type explicitly (or use turbofish)
    /// let len: Option<Vec<u32>> = matches
    ///     .parse_optional_vec_t("length", FromStr::from_str)
    ///     .unwrap_or_else(|e| e.exit());
    /// assert_eq!(len, Some(vec![12, 77, 40]));
    ///
    /// // You can often leave the type for rustc to figure out
    /// let also_len = matches
    ///     .parse_optional_vec_t("length", FromStr::from_str)
    ///     .unwrap_or_else(|e| e.exit());
    /// // Something that expects Option<Vec<u32>>
    /// let _: Option<Vec<u32>> = also_len;
    /// ```
    ///
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    pub fn parse_optional_vec_t<R, E>(
        &self,
        name: &str,
        parse: impl Fn(&str) -> Result<R, E>,
    ) -> Result<Option<Vec<R>>, Error>
    where
        E: Display,
    {
        Ok(match self.values_of(name) {
            Some(vals) => Some(
                vals.map(|v| {
                    parse(v).map_err(|e| {
                        let message = format!("The argument '{}' isn't a valid value: {}", v, e);

                        Error::value_validation(
                            name.to_string(),
                            v.to_string(),
                            message.into(),
                            ColorChoice::Auto,
                        )
                    })
                })
                .collect::<Result<_, _>>()?,
            ),
            None => None,
        })
    }

    /// Gets the typed values of a specific argument (i.e. an argument that takes multiple
    /// values at runtime) and then converts them into the result type using `parse`, which takes
    /// the OS version of the string value of the argument.
    ///
    /// If parsing (of any value) has failed, returns Err.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if any of the values contains invalid UTF-8 code points.
    ///
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    pub fn parse_vec_t_os<R>(
        &self,
        name: &str,
        parse: impl Fn(&OsStr) -> Result<R, OsString>,
    ) -> Result<Vec<R>, Error> {
        self.parse_optional_vec_t_os(name, parse)?
            .ok_or_else(|| Error::argument_not_found_auto(name.to_string()))
    }

    /// Gets the typed values of a specific argument (i.e. an argument that takes multiple
    /// values at runtime) and then converts them into the result type using `parse`, which takes
    /// the OS version of the string value of the argument.
    ///
    /// In the case where the argument wasn't present, `Ok(None)` is returned. In the
    /// case where it was present and parsing succeeded, `Ok(Some(vec))` is returned.
    /// If parsing (of any value) has failed, returns Err.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if any of the values contains invalid UTF-8 code points.
    ///
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    pub fn parse_optional_vec_t_os<R>(
        &self,
        name: &str,
        parse: impl Fn(&OsStr) -> Result<R, OsString>,
    ) -> Result<Option<Vec<R>>, Error> {
        Ok(match self.values_of_os(name) {
            Some(vals) => Some(
                vals.map(|v| {
                    parse(v).map_err(|e| {
                        let message = format!(
                            "The argument '{}' isn't a valid value: {}",
                            v.to_string_lossy(),
                            e.to_string_lossy()
                        );

                        Error::value_validation(
                            name.to_string(),
                            v.to_string_lossy().to_string(),
                            message.into(),
                            ColorChoice::Auto,
                        )
                    })
                })
                .collect::<Result<_, _>>()?,
            ),
            None => None,
        })
    }

    /// Gets the typed values of a specific argument (i.e. an argument that takes multiple
    /// values at runtime) and then converts them into the result type using [`std::str::FromStr`].
    ///
    /// If parsing (of any value) has failed, exits the program.
    ///
    /// # Panics
    ///
    /// This method will [`panic!`] if any of the values contains invalid UTF-8 code points.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate clap;
    /// # use clap::App;
    /// let matches = App::new("myapp")
    ///               .arg("[length]... 'A sequence of integers because integers are neat!'")
    ///               .get_matches_from(&["test", "12", "77", "40"]);
    ///
    /// // Specify the type explicitly (or use turbofish)
    /// let len: Vec<u32> = matches.values_of_t_or_exit("length");
    /// assert_eq!(len, vec![12, 77, 40]);
    ///
    /// // You can often leave the type for rustc to figure out
    /// let also_len = matches.values_of_t_or_exit("length");
    /// // Something that expects Vec<u32>
    /// let _: Vec<u32> = also_len;
    /// ```
    ///
    /// [`panic!`]: https://doc.rust-lang.org/std/macro.panic!.html
    pub fn values_of_t_or_exit<R>(&self, name: &str) -> Vec<R>
    where
        R: FromStr,
        <R as FromStr>::Err: Display,
    {
        self.values_of_t(name).unwrap_or_else(|e| e.exit())
    }

    /// Returns `true` if an argument was present at runtime, otherwise `false`.
    ///
    /// *NOTE:* This will always return `true` if [`default_value`] has been set.
    /// [`occurrences_of`] can be used to check if a value is present at runtime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myprog")
    ///     .arg(Arg::new("debug")
    ///         .short('d'))
    ///     .get_matches_from(vec![
    ///         "myprog", "-d"
    ///     ]);
    ///
    /// assert!(m.is_present("debug"));
    /// ```
    ///
    /// [`default_value`]: crate::Arg::default_value()
    /// [`occurrences_of`]: ArgMatches::occurrences_of()
    pub fn is_present<T: Key>(&self, id: T) -> bool {
        let id = Id::from(id);
        self.args.contains_key(&id)
    }

    /// Returns the number of times an argument was used at runtime. If an argument isn't present
    /// it will return `0`.
    ///
    /// **NOTE:** This returns the number of times the argument was used, *not* the number of
    /// values. For example, `-o val1 val2 val3 -o val4` would return `2` (2 occurrences, but 4
    /// values).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("myprog")
    ///     .arg(Arg::new("debug")
    ///         .short('d')
    ///         .setting(ArgSettings::MultipleOccurrences))
    ///     .get_matches_from(vec![
    ///         "myprog", "-d", "-d", "-d"
    ///     ]);
    ///
    /// assert_eq!(m.occurrences_of("debug"), 3);
    /// ```
    ///
    /// This next example shows that counts actual uses of the argument, not just `-`'s
    ///
    /// ```rust
    /// # use clap::{App, Arg, ArgSettings};
    /// let m = App::new("myprog")
    ///     .arg(Arg::new("debug")
    ///         .short('d')
    ///         .setting(ArgSettings::MultipleOccurrences))
    ///     .arg(Arg::new("flag")
    ///         .short('f'))
    ///     .get_matches_from(vec![
    ///         "myprog", "-ddfd"
    ///     ]);
    ///
    /// assert_eq!(m.occurrences_of("debug"), 3);
    /// assert_eq!(m.occurrences_of("flag"), 1);
    /// ```
    pub fn occurrences_of<T: Key>(&self, id: T) -> u64 {
        self.args.get(&Id::from(id)).map_or(0, |a| a.occurs)
    }

    /// Gets the starting index of the argument in respect to all other arguments. Indices are
    /// similar to argv indices, but are not exactly 1:1.
    ///
    /// For flags (i.e. those arguments which don't have an associated value), indices refer
    /// to occurrence of the switch, such as `-f`, or `--flag`. However, for options the indices
    /// refer to the *values* `-o val` would therefore not represent two distinct indices, only the
    /// index for `val` would be recorded. This is by design.
    ///
    /// Besides the flag/option descrepancy, the primary difference between an argv index and clap
    /// index, is that clap continues counting once all arguments have properly separated, whereas
    /// an argv index does not.
    ///
    /// The examples should clear this up.
    ///
    /// *NOTE:* If an argument is allowed multiple times, this method will only give the *first*
    /// index.
    ///
    /// # Examples
    ///
    /// The argv indices are listed in the comments below. See how they correspond to the clap
    /// indices. Note that if it's not listed in a clap index, this is because it's not saved in
    /// in an `ArgMatches` struct for querying.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::new("flag")
    ///         .short('f'))
    ///     .arg(Arg::new("option")
    ///         .short('o')
    ///         .takes_value(true))
    ///     .get_matches_from(vec!["myapp", "-f", "-o", "val"]);
    ///             // ARGV idices: ^0       ^1    ^2    ^3
    ///             // clap idices:          ^1          ^3
    ///
    /// assert_eq!(m.index_of("flag"), Some(1));
    /// assert_eq!(m.index_of("option"), Some(3));
    /// ```
    ///
    /// Now notice, if we use one of the other styles of options:
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::new("flag")
    ///         .short('f'))
    ///     .arg(Arg::new("option")
    ///         .short('o')
    ///         .takes_value(true))
    ///     .get_matches_from(vec!["myapp", "-f", "-o=val"]);
    ///             // ARGV idices: ^0       ^1    ^2
    ///             // clap idices:          ^1       ^3
    ///
    /// assert_eq!(m.index_of("flag"), Some(1));
    /// assert_eq!(m.index_of("option"), Some(3));
    /// ```
    ///
    /// Things become much more complicated, or clear if we look at a more complex combination of
    /// flags. Let's also throw in the final option style for good measure.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::new("flag")
    ///         .short('f'))
    ///     .arg(Arg::new("flag2")
    ///         .short('F'))
    ///     .arg(Arg::new("flag3")
    ///         .short('z'))
    ///     .arg(Arg::new("option")
    ///         .short('o')
    ///         .takes_value(true))
    ///     .get_matches_from(vec!["myapp", "-fzF", "-oval"]);
    ///             // ARGV idices: ^0      ^1       ^2
    ///             // clap idices:         ^1,2,3    ^5
    ///             //
    ///             // clap sees the above as 'myapp -f -z -F -o val'
    ///             //                         ^0    ^1 ^2 ^3 ^4 ^5
    /// assert_eq!(m.index_of("flag"), Some(1));
    /// assert_eq!(m.index_of("flag2"), Some(3));
    /// assert_eq!(m.index_of("flag3"), Some(2));
    /// assert_eq!(m.index_of("option"), Some(5));
    /// ```
    ///
    /// One final combination of flags/options to see how they combine:
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::new("flag")
    ///         .short('f'))
    ///     .arg(Arg::new("flag2")
    ///         .short('F'))
    ///     .arg(Arg::new("flag3")
    ///         .short('z'))
    ///     .arg(Arg::new("option")
    ///         .short('o')
    ///         .takes_value(true))
    ///     .get_matches_from(vec!["myapp", "-fzFoval"]);
    ///             // ARGV idices: ^0       ^1
    ///             // clap idices:          ^1,2,3^5
    ///             //
    ///             // clap sees the above as 'myapp -f -z -F -o val'
    ///             //                         ^0    ^1 ^2 ^3 ^4 ^5
    /// assert_eq!(m.index_of("flag"), Some(1));
    /// assert_eq!(m.index_of("flag2"), Some(3));
    /// assert_eq!(m.index_of("flag3"), Some(2));
    /// assert_eq!(m.index_of("option"), Some(5));
    /// ```
    ///
    /// The last part to mention is when values are sent in multiple groups with a [delimiter].
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::new("option")
    ///         .short('o')
    ///         .use_delimiter(true)
    ///         .multiple_values(true))
    ///     .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);
    ///             // ARGV idices: ^0       ^1
    ///             // clap idices:             ^2   ^3   ^4
    ///             //
    ///             // clap sees the above as 'myapp -o val1 val2 val3'
    ///             //                         ^0    ^1 ^2   ^3   ^4
    /// assert_eq!(m.index_of("option"), Some(2));
    /// assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 3, 4]);
    /// ```
    /// [delimiter]: crate::Arg::value_delimiter()
    pub fn index_of<T: Key>(&self, name: T) -> Option<usize> {
        if let Some(arg) = self.args.get(&Id::from(name)) {
            if let Some(i) = arg.get_index(0) {
                return Some(i);
            }
        }
        None
    }

    /// Gets all indices of the argument in respect to all other arguments. Indices are
    /// similar to argv indices, but are not exactly 1:1.
    ///
    /// For flags (i.e. those arguments which don't have an associated value), indices refer
    /// to occurrence of the switch, such as `-f`, or `--flag`. However, for options the indices
    /// refer to the *values* `-o val` would therefore not represent two distinct indices, only the
    /// index for `val` would be recorded. This is by design.
    ///
    /// *NOTE:* For more information about how clap indices compare to argv indices, see
    /// [`ArgMatches::index_of`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::new("option")
    ///         .short('o')
    ///         .use_delimiter(true)
    ///         .multiple_values(true))
    ///     .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);
    ///             // ARGV idices: ^0       ^1
    ///             // clap idices:             ^2   ^3   ^4
    ///             //
    ///             // clap sees the above as 'myapp -o val1 val2 val3'
    ///             //                         ^0    ^1 ^2   ^3   ^4
    /// assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 3, 4]);
    /// ```
    ///
    /// Another quick example is when flags and options are used together
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::new("option")
    ///         .short('o')
    ///         .takes_value(true)
    ///         .multiple_occurrences(true))
    ///     .arg(Arg::new("flag")
    ///         .short('f')
    ///         .multiple_occurrences(true))
    ///     .get_matches_from(vec!["myapp", "-o", "val1", "-f", "-o", "val2", "-f"]);
    ///             // ARGV idices: ^0       ^1    ^2      ^3    ^4    ^5      ^6
    ///             // clap idices:                ^2      ^3          ^5      ^6
    ///
    /// assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2, 5]);
    /// assert_eq!(m.indices_of("flag").unwrap().collect::<Vec<_>>(), &[3, 6]);
    /// ```
    ///
    /// One final example, which is an odd case; if we *don't* use  value delimiter as we did with
    /// the first example above instead of `val1`, `val2` and `val3` all being distinc values, they
    /// would all be a single value of `val1,val2,val3`, in which case they'd only receive a single
    /// index.
    ///
    /// ```rust
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::new("option")
    ///         .short('o')
    ///         .takes_value(true)
    ///         .multiple_values(true))
    ///     .get_matches_from(vec!["myapp", "-o=val1,val2,val3"]);
    ///             // ARGV idices: ^0       ^1
    ///             // clap idices:             ^2
    ///             //
    ///             // clap sees the above as 'myapp -o "val1,val2,val3"'
    ///             //                         ^0    ^1  ^2
    /// assert_eq!(m.indices_of("option").unwrap().collect::<Vec<_>>(), &[2]);
    /// ```
    /// [`ArgMatches::index_of`]: ArgMatches::index_of()
    /// [delimiter]: Arg::value_delimiter()
    pub fn indices_of<T: Key>(&self, id: T) -> Option<Indices<'_>> {
        self.args.get(&Id::from(id)).map(|arg| Indices {
            iter: arg.indices(),
        })
    }

    /// Because [`Subcommand`]s are essentially "sub-[`App`]s" they have their own [`ArgMatches`]
    /// as well. This method returns the [`ArgMatches`] for a particular subcommand or `None` if
    /// the subcommand wasn't present at runtime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, };
    /// let app_m = App::new("myprog")
    ///     .arg(Arg::new("debug")
    ///         .short('d'))
    ///     .subcommand(App::new("test")
    ///         .arg(Arg::new("opt")
    ///             .long("option")
    ///             .takes_value(true)))
    ///     .get_matches_from(vec![
    ///         "myprog", "-d", "test", "--option", "val"
    ///     ]);
    ///
    /// // Both parent commands, and child subcommands can have arguments present at the same times
    /// assert!(app_m.is_present("debug"));
    ///
    /// // Get the subcommand's ArgMatches instance
    /// if let Some(sub_m) = app_m.subcommand_matches("test") {
    ///     // Use the struct like normal
    ///     assert_eq!(sub_m.value_of("opt"), Some("val"));
    /// }
    /// ```
    ///
    /// [`Subcommand`]: crate::Subcommand
    /// [`App`]: crate::App
    pub fn subcommand_matches<T: Key>(&self, id: T) -> Option<&ArgMatches> {
        if let Some(ref s) = self.subcommand {
            if s.id == id.into() {
                return Some(&s.matches);
            }
        }
        None
    }

    /// Because [`Subcommand`]s are essentially "sub-[`App`]s" they have their own [`ArgMatches`]
    /// as well.But simply getting the sub-[`ArgMatches`] doesn't help much if we don't also know
    /// which subcommand was actually used. This method returns the name of the subcommand that was
    /// used at runtime, or `None` if one wasn't.
    ///
    /// *NOTE*: Subcommands form a hierarchy, where multiple subcommands can be used at runtime,
    /// but only a single subcommand from any group of sibling commands may used at once.
    ///
    /// An ASCII art depiction may help explain this better...Using a fictional version of `git` as
    /// the demo subject. Imagine the following are all subcommands of `git` (note, the author is
    /// aware these aren't actually all subcommands in the real `git` interface, but it makes
    /// explanation easier)
    ///
    /// ```notrust
    ///              Top Level App (git)                         TOP
    ///                              |
    ///       -----------------------------------------
    ///      /             |                \          \
    ///   clone          push              add       commit      LEVEL 1
    ///     |           /    \            /    \       |
    ///    url      origin   remote    ref    name   message     LEVEL 2
    ///             /                  /\
    ///          path            remote  local                   LEVEL 3
    /// ```
    ///
    /// Given the above fictional subcommand hierarchy, valid runtime uses would be (not an all
    /// inclusive list, and not including argument options per command for brevity and clarity):
    ///
    /// ```sh
    /// $ git clone url
    /// $ git push origin path
    /// $ git add ref local
    /// $ git commit message
    /// ```
    ///
    /// Notice only one command per "level" may be used. You could not, for example, do `$ git
    /// clone url push origin path`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    ///  let app_m = App::new("git")
    ///      .subcommand(App::new("clone"))
    ///      .subcommand(App::new("push"))
    ///      .subcommand(App::new("commit"))
    ///      .get_matches();
    ///
    /// match app_m.subcommand_name() {
    ///     Some("clone")  => {}, // clone was used
    ///     Some("push")   => {}, // push was used
    ///     Some("commit") => {}, // commit was used
    ///     _              => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    /// [`Subcommand`]: crate::Subcommand
    /// [`App`]: crate::App
    #[inline]
    pub fn subcommand_name(&self) -> Option<&str> {
        self.subcommand.as_ref().map(|sc| &*sc.name)
    }

    /// This brings together [`ArgMatches::subcommand_matches`] and [`ArgMatches::subcommand_name`]
    /// by returning a tuple with both pieces of information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, };
    ///  let app_m = App::new("git")
    ///      .subcommand(App::new("clone"))
    ///      .subcommand(App::new("push"))
    ///      .subcommand(App::new("commit"))
    ///      .get_matches();
    ///
    /// match app_m.subcommand() {
    ///     Some(("clone",  sub_m)) => {}, // clone was used
    ///     Some(("push",   sub_m)) => {}, // push was used
    ///     Some(("commit", sub_m)) => {}, // commit was used
    ///     _                       => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    ///
    /// Another useful scenario is when you want to support third party, or external, subcommands.
    /// In these cases you can't know the subcommand name ahead of time, so use a variable instead
    /// with pattern matching!
    ///
    /// ```rust
    /// # use clap::{App, AppSettings};
    /// // Assume there is an external subcommand named "subcmd"
    /// let app_m = App::new("myprog")
    ///     .setting(AppSettings::AllowExternalSubcommands)
    ///     .get_matches_from(vec![
    ///         "myprog", "subcmd", "--option", "value", "-fff", "--flag"
    ///     ]);
    ///
    /// // All trailing arguments will be stored under the subcommand's sub-matches using an empty
    /// // string argument name
    /// match app_m.subcommand() {
    ///     Some((external, sub_m)) => {
    ///          let ext_args: Vec<&str> = sub_m.values_of("").unwrap().collect();
    ///          assert_eq!(external, "subcmd");
    ///          assert_eq!(ext_args, ["--option", "value", "-fff", "--flag"]);
    ///     },
    ///     _ => {},
    /// }
    /// ```
    /// [`ArgMatches::subcommand_matches`]: ArgMatches::subcommand_matches()
    /// [`ArgMatches::subcommand_name`]: ArgMatches::subcommand_name()
    #[inline]
    pub fn subcommand(&self) -> Option<(&str, &ArgMatches)> {
        self.subcommand.as_ref().map(|sc| (&*sc.name, &sc.matches))
    }
}

// The following were taken and adapted from vec_map source
// repo: https://github.com/contain-rs/vec-map
// commit: be5e1fa3c26e351761b33010ddbdaf5f05dbcc33
// license: MIT - Copyright (c) 2015 The Rust Project Developers

/// An iterator for getting multiple values out of an argument via the [`ArgMatches::values_of`]
/// method.
///
/// # Examples
///
/// ```rust
/// # use clap::{App, Arg};
/// let m = App::new("myapp")
///     .arg(Arg::new("output")
///         .short('o')
///         .multiple_values(true)
///         .takes_value(true))
///     .get_matches_from(vec!["myapp", "-o", "val1", "val2"]);
///
/// let mut values = m.values_of("output").unwrap();
///
/// assert_eq!(values.next(), Some("val1"));
/// assert_eq!(values.next(), Some("val2"));
/// assert_eq!(values.next(), None);
/// ```
/// [`ArgMatches::values_of`]: ArgMatches::values_of()
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct Values<'a> {
    #[allow(clippy::type_complexity)]
    iter: Map<Flatten<Iter<'a, Vec<OsString>>>, for<'r> fn(&'r OsString) -> &'r str>,
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Values<'a> {
    fn next_back(&mut self) -> Option<&'a str> {
        self.iter.next_back()
    }
}

impl<'a> ExactSizeIterator for Values<'a> {}

/// Creates an empty iterator.
impl<'a> Default for Values<'a> {
    fn default() -> Self {
        static EMPTY: [Vec<OsString>; 0] = [];
        Values {
            iter: EMPTY[..].iter().flatten().map(|_| unreachable!()),
        }
    }
}

#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct GroupedValues<'a> {
    #[allow(clippy::type_complexity)]
    iter: Map<Iter<'a, Vec<OsString>>, fn(&Vec<OsString>) -> Vec<&str>>,
}

impl<'a> Iterator for GroupedValues<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for GroupedValues<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a> ExactSizeIterator for GroupedValues<'a> {}

/// Creates an empty iterator. Used for `unwrap_or_default()`.
impl<'a> Default for GroupedValues<'a> {
    fn default() -> Self {
        static EMPTY: [Vec<OsString>; 0] = [];
        GroupedValues {
            iter: EMPTY[..].iter().map(|_| unreachable!()),
        }
    }
}

/// An iterator for getting multiple values out of an argument via the [`ArgMatches::values_of_os`]
/// method. Usage of this iterator allows values which contain invalid UTF-8 code points unlike
/// [`Values`].
///
/// # Examples
///
#[cfg_attr(not(unix), doc = " ```ignore")]
#[cfg_attr(unix, doc = " ```")]
/// # use clap::{App, Arg};
/// use std::ffi::OsString;
/// use std::os::unix::ffi::{OsStrExt,OsStringExt};
///
/// let m = App::new("utf8")
///     .arg(Arg::from("<arg> 'some arg'"))
///     .get_matches_from(vec![OsString::from("myprog"),
///                             // "Hi {0xe9}!"
///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
/// assert_eq!(&*m.value_of_os("arg").unwrap().as_bytes(), [b'H', b'i', b' ', 0xe9, b'!']);
/// ```
/// [`ArgMatches::values_of_os`]: ArgMatches::values_of_os()
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct OsValues<'a> {
    #[allow(clippy::type_complexity)]
    iter: Map<Flatten<Iter<'a, Vec<OsString>>>, fn(&OsString) -> &OsStr>,
}

impl<'a> Iterator for OsValues<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<&'a OsStr> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for OsValues<'a> {
    fn next_back(&mut self) -> Option<&'a OsStr> {
        self.iter.next_back()
    }
}

impl<'a> ExactSizeIterator for OsValues<'a> {}

/// Creates an empty iterator.
impl Default for OsValues<'_> {
    fn default() -> Self {
        static EMPTY: [Vec<OsString>; 0] = [];
        OsValues {
            iter: EMPTY[..].iter().flatten().map(|_| unreachable!()),
        }
    }
}

/// An iterator for getting multiple indices out of an argument via the [`ArgMatches::indices_of`]
/// method.
///
/// # Examples
///
/// ```rust
/// # use clap::{App, Arg};
/// let m = App::new("myapp")
///     .arg(Arg::new("output")
///         .short('o')
///         .multiple_values(true)
///         .takes_value(true))
///     .get_matches_from(vec!["myapp", "-o", "val1", "val2"]);
///
/// let mut indices = m.indices_of("output").unwrap();
///
/// assert_eq!(indices.next(), Some(2));
/// assert_eq!(indices.next(), Some(3));
/// assert_eq!(indices.next(), None);
/// ```
/// [`ArgMatches::indices_of`]: ArgMatches::indices_of()
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct Indices<'a> {
    iter: Cloned<Iter<'a, usize>>,
}

impl<'a> Iterator for Indices<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Indices<'a> {
    fn next_back(&mut self) -> Option<usize> {
        self.iter.next_back()
    }
}

impl<'a> ExactSizeIterator for Indices<'a> {}

/// Creates an empty iterator.
impl<'a> Default for Indices<'a> {
    fn default() -> Self {
        static EMPTY: [usize; 0] = [];
        // This is never called because the iterator is empty:
        Indices {
            iter: EMPTY[..].iter().cloned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let mut values: Values = Values::default();
        assert_eq!(values.next(), None);
    }

    #[test]
    fn test_default_values_with_shorter_lifetime() {
        let matches = ArgMatches::default();
        let mut values = matches.values_of("").unwrap_or_default();
        assert_eq!(values.next(), None);
    }

    #[test]
    fn test_default_osvalues() {
        let mut values: OsValues = OsValues::default();
        assert_eq!(values.next(), None);
    }

    #[test]
    fn test_default_osvalues_with_shorter_lifetime() {
        let matches = ArgMatches::default();
        let mut values = matches.values_of_os("").unwrap_or_default();
        assert_eq!(values.next(), None);
    }

    #[test]
    fn test_default_indices() {
        let mut indices: Indices = Indices::default();
        assert_eq!(indices.next(), None);
    }

    #[test]
    fn test_default_indices_with_shorter_lifetime() {
        let matches = ArgMatches::default();
        let mut indices = matches.indices_of("").unwrap_or_default();
        assert_eq!(indices.next(), None);
    }
}
