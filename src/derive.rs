//! This module contains traits that are usable with the `#[derive(...)].`
//! macros in [`clap_derive`].

use crate::{App, ArgMatches, ArgValue, Error};

use std::ffi::OsString;

/// Parse command-line arguments into `Self`.
///
/// The primary one-stop-shop trait used to create an instance of a `clap`
/// [`App`], conduct the parsing, and turn the resulting [`ArgMatches`] back
/// into concrete instance of the user struct.
///
/// This trait is primarily a convenience on top of [`FromArgMatches`] +
/// [`IntoApp`] which uses those two underlying traits to build the two
/// fundamental functions `parse` which uses the `std::env::args_os` iterator,
/// and `parse_from` which allows the consumer to supply the iterator (along
/// with fallible options for each).
///
/// See also [`Subcommand`] and [`Args`].
///
/// # Examples
///
/// The following example creates a `Context` struct that would be used
/// throughout the application representing the normalized values coming from
/// the CLI.
///
/// ```rust
/// # use clap::{Clap};
/// /// My super CLI
/// #[derive(Clap)]
/// #[clap(name = "demo")]
/// struct Context {
///     /// More verbose output
///     #[clap(long)]
///     verbose: bool,
///     /// An optional name
///     #[clap(short, long)]
///     name: Option<String>,
/// }
/// ```
///
/// The equivalent [`App`] struct + `From` implementation:
///
/// ```rust
/// # use clap::{App, Arg, ArgMatches};
/// App::new("demo")
///     .about("My super CLI")
///     .arg(Arg::new("verbose")
///         .long("verbose")
///         .about("More verbose output"))
///     .arg(Arg::new("name")
///         .long("name")
///         .short('n')
///         .about("An optional name")
///         .takes_value(true));
///
/// struct Context {
///     verbose: bool,
///     name: Option<String>,
/// }
///
/// impl From<ArgMatches> for Context {
///     fn from(m: ArgMatches) -> Self {
///         Context {
///             verbose: m.is_present("verbose"),
///             name: m.value_of("name").map(|n| n.to_owned()),
///         }
///     }
/// }
/// ```
///
pub trait Clap: FromArgMatches + IntoApp + Sized {
    /// Parse from `std::env::args_os()`, exit on error
    fn parse() -> Self {
        let matches = <Self as IntoApp>::into_app().get_matches();
        <Self as FromArgMatches>::from_arg_matches(&matches).expect("IntoApp validated everything")
    }

    /// Parse from `std::env::args_os()`, return Err on error.
    fn try_parse() -> Result<Self, Error> {
        let matches = <Self as IntoApp>::into_app().try_get_matches()?;
        Ok(<Self as FromArgMatches>::from_arg_matches(&matches)
            .expect("IntoApp validated everything"))
    }

    /// Parse from iterator, exit on error
    fn parse_from<I, T>(itr: I) -> Self
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app().get_matches_from(itr);
        <Self as FromArgMatches>::from_arg_matches(&matches).expect("IntoApp validated everything")
    }

    /// Parse from iterator, return Err on error.
    fn try_parse_from<I, T>(itr: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app().try_get_matches_from(itr)?;
        Ok(<Self as FromArgMatches>::from_arg_matches(&matches)
            .expect("IntoApp validated everything"))
    }

    /// Update from iterator, exit on error
    fn update_from<I, T>(&mut self, itr: I)
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        // TODO find a way to get partial matches
        let matches = <Self as IntoApp>::into_app_for_update().get_matches_from(itr);
        <Self as FromArgMatches>::update_from_arg_matches(self, &matches);
    }

    /// Update from iterator, return Err on error.
    fn try_update_from<I, T>(&mut self, itr: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app_for_update().try_get_matches_from(itr)?;
        <Self as FromArgMatches>::update_from_arg_matches(self, &matches);
        Ok(())
    }
}

/// Build an [`App`] relevant for a user-defined container.
pub trait IntoApp: Sized {
    /// Build an [`App`] that can instantiate `Self`.
    ///
    /// See [`FromArgMatches::from_arg_matches`] for instantiating `Self`.
    fn into_app<'help>() -> App<'help>;
    /// Build an [`App`] that can update `self`.
    ///
    /// See [`FromArgMatches::update_from_arg_matches`] for updating `self`.
    fn into_app_for_update<'help>() -> App<'help>;
}

/// Converts an instance of [`ArgMatches`] to a user-defined container.
pub trait FromArgMatches: Sized {
    /// Instantiate `Self` from [`ArgMatches`], parsing the arguments as needed.
    ///
    /// Motivation: If our application had two CLI options, `--name
    /// <STRING>` and the flag `--debug`, we may create a struct as follows:
    ///
    /// ```no_run
    /// struct Context {
    ///     name: String,
    ///     debug: bool
    /// }
    /// ```
    ///
    /// We then need to convert the `ArgMatches` that `clap` generated into our struct.
    /// `from_arg_matches` serves as the equivalent of:
    ///
    /// ```no_run
    /// # use clap::ArgMatches;
    /// # struct Context {
    /// #   name: String,
    /// #   debug: bool
    /// # }
    /// impl From<ArgMatches> for Context {
    ///    fn from(m: ArgMatches) -> Self {
    ///        Context {
    ///            name: m.value_of("name").unwrap().to_string(),
    ///            debug: m.is_present("debug"),
    ///        }
    ///    }
    /// }
    /// ```
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self>;

    /// Assign values from `ArgMatches` to `self`.
    fn update_from_arg_matches(&mut self, matches: &ArgMatches);
}

/// Parse arguments into a user-defined container.
///
/// Implementing this trait lets a parent container delegate argument parsing behavior to `Self`.
/// with:
/// - `#[clap(flatten)] args: ChildArgs`: Attribute can only be used with struct fields that impl
///   `Args`.
/// - `Variant(ChildArgs)`: No attribute is used with enum variants that impl `Args`.
///
///
/// # Example
///
/// ```rust
/// #[derive(clap::Clap)]
/// struct Args {
///     #[clap(flatten)]
///     logging: LogArgs,
/// }
///
/// #[derive(clap::Args)]
/// struct LogArgs {
///     #[clap(long, short = 'v', parse(from_occurrences))]
///     verbose: i8,
/// }
/// ```
pub trait Args: FromArgMatches + Sized {
    /// Append to [`App`] so it can instantiate `Self`.
    ///
    /// See also [`IntoApp`].
    fn augment_args(app: App<'_>) -> App<'_>;
    /// Append to [`App`] so it can update `self`.
    ///
    /// This is used to implement `#[clap(flatten)]`
    ///
    /// See also [`IntoApp`].
    fn augment_args_for_update(app: App<'_>) -> App<'_>;
}

/// Parse a sub-command into a user-defined enum.
///
/// Implementing this trait lets a parent container delegate subcommand behavior to `Self`.
/// with:
/// - `#[clap(subcommand)] field: SubCmd`: Attribute can be used with either struct fields or enum
///   variants that impl `Subcommand`.
/// - `#[clap(flatten)] Variant(SubCmd)`: Attribute can only be used with enum variants that impl
///   `Subcommand`.
///
/// # Example
///
/// ```rust
/// #[derive(clap::Clap)]
/// struct Args {
///     #[clap(subcommand)]
///     action: Action,
/// }
///
/// #[derive(clap::Subcommand)]
/// enum Action {
///     Add,
///     Remove,
/// }
/// ```
pub trait Subcommand: FromArgMatches + Sized {
    /// Append to [`App`] so it can instantiate `Self`.
    ///
    /// See also [`IntoApp`].
    fn augment_subcommands(app: App<'_>) -> App<'_>;
    /// Append to [`App`] so it can update `self`.
    ///
    /// This is used to implement `#[clap(flatten)]`
    ///
    /// See also [`IntoApp`].
    fn augment_subcommands_for_update(app: App<'_>) -> App<'_>;
    /// Test whether `Self` can parse a specific subcommand
    fn has_subcommand(name: &str) -> bool;
}

/// Parse arguments into enums.
///
/// When deriving [`Clap`], a field whose type implements `ArgEnum` can have the attribute
/// `#[clap(arg_enum)]`.  In addition to parsing, help and error messages may report possible
/// variants.
///
/// # Example
///
/// ```rust
/// #[derive(clap::Clap)]
/// struct Args {
///     #[clap(arg_enum)]
///     level: Level,
/// }
///
/// #[derive(clap::ArgEnum)]
/// enum Level {
///     Debug,
///     Info,
///     Warning,
///     Error,
/// }
/// ```
pub trait ArgEnum: Sized {
    /// All possible argument choices, in display order.
    const VARIANTS: &'static [ArgValue<'static>];

    /// Parse an argument into `Self`.
    fn from_str(input: &str, case_insensitive: bool) -> Result<Self, String>;

    /// The canonical argument value.
    ///
    /// The value is `None` for skipped variants.
    fn as_arg(&self) -> Option<&'static str>;
}

impl<T: Clap> Clap for Box<T> {
    fn parse() -> Self {
        Box::new(<T as Clap>::parse())
    }

    fn try_parse() -> Result<Self, Error> {
        <T as Clap>::try_parse().map(Box::new)
    }

    fn parse_from<I, It>(itr: I) -> Self
    where
        I: IntoIterator<Item = It>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        It: Into<OsString> + Clone,
    {
        Box::new(<T as Clap>::parse_from(itr))
    }

    fn try_parse_from<I, It>(itr: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = It>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        It: Into<OsString> + Clone,
    {
        <T as Clap>::try_parse_from(itr).map(Box::new)
    }
}

impl<T: IntoApp> IntoApp for Box<T> {
    fn into_app<'help>() -> App<'help> {
        <T as IntoApp>::into_app()
    }
    fn into_app_for_update<'help>() -> App<'help> {
        <T as IntoApp>::into_app_for_update()
    }
}

impl<T: FromArgMatches> FromArgMatches for Box<T> {
    fn from_arg_matches(matches: &ArgMatches) -> Option<Self> {
        <T as FromArgMatches>::from_arg_matches(matches).map(Box::new)
    }
    fn update_from_arg_matches(&mut self, matches: &ArgMatches) {
        <T as FromArgMatches>::update_from_arg_matches(self, matches)
    }
}

impl<T: Args> Args for Box<T> {
    fn augment_args(app: App<'_>) -> App<'_> {
        <T as Args>::augment_args(app)
    }
    fn augment_args_for_update(app: App<'_>) -> App<'_> {
        <T as Args>::augment_args_for_update(app)
    }
}

impl<T: Subcommand> Subcommand for Box<T> {
    fn augment_subcommands(app: App<'_>) -> App<'_> {
        <T as Subcommand>::augment_subcommands(app)
    }
    fn augment_subcommands_for_update(app: App<'_>) -> App<'_> {
        <T as Subcommand>::augment_subcommands_for_update(app)
    }
    fn has_subcommand(name: &str) -> bool {
        <T as Subcommand>::has_subcommand(name)
    }
}
