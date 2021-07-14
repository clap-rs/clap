//! This module contains traits that are usable with the `#[derive(...)].`
//! macros in [`clap_derive`].

use crate::{App, ArgMatches, Error};

use std::ffi::OsString;

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
        <Self as FromArgMatches>::from_arg_matches(&matches)
    }

    /// Parse from `std::env::args_os()`, return Err on error.
    fn try_parse() -> Result<Self, Error> {
        let matches = <Self as IntoApp>::into_app().try_get_matches()?;
        Ok(<Self as FromArgMatches>::from_arg_matches(&matches))
    }

    /// Parse from iterator, exit on error
    fn parse_from<I, T>(itr: I) -> Self
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app().get_matches_from(itr);
        <Self as FromArgMatches>::from_arg_matches(&matches)
    }

    /// Parse from iterator, return Err on error.
    fn try_parse_from<I, T>(itr: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app().try_get_matches_from(itr)?;
        Ok(<Self as FromArgMatches>::from_arg_matches(&matches))
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

/// Build an [`App`] according to the struct
pub trait IntoApp: Sized {
    /// Build an [`App`] that can instantiate `Self`.
    ///
    /// See [`FromArgMatches::from_arg_matches`] for instantiating `Self`.
    fn into_app<'help>() -> App<'help>;
    /// Build an [`App`] that can update `self`.
    ///
    /// See [`FromArgMatches::update_from_arg_matches`] for updating `self`.
    fn into_app_for_update<'help>() -> App<'help>;
    /// Append to [`App`] so it can instantiate `Self`.
    ///
    /// This is used to implement `#[clap(flatten)]`
    fn augment_clap(app: App<'_>) -> App<'_>;
    /// Append to [`App`] so it can update `self`.
    ///
    /// This is used to implement `#[clap(flatten)]`
    fn augment_clap_for_update(app: App<'_>) -> App<'_>;
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
    fn from_arg_matches(matches: &ArgMatches) -> Self;

    /// Assign values from `ArgMatches` to `self`.
    fn update_from_arg_matches(&mut self, matches: &ArgMatches);
}

/// @TODO @release @docs
pub trait Subcommand: Sized {
    /// Instantiate `Self` from subcommand name and [`ArgMatches`].
    ///
    /// Returns `None` if subcommand does not exist
    fn from_subcommand(subcommand: Option<(&str, &ArgMatches)>) -> Option<Self>;
    /// Assign values from `ArgMatches` to `self`.
    fn update_from_subcommand(&mut self, subcommand: Option<(&str, &ArgMatches)>);
    /// Append to [`App`] so it can instantiate `Self`.
    ///
    /// This is used to implement `#[clap(flatten)]`
    ///
    /// See also [`IntoApp`].
    fn augment_subcommands(app: App<'_>) -> App<'_>;
    /// Append to [`App`] so it can update `self`.
    ///
    /// This is used to implement `#[clap(flatten)]`
    ///
    /// See also [`IntoApp`].
    fn augment_subcommands_for_update(app: App<'_>) -> App<'_>;
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
    const VARIANTS: &'static [&'static str];

    /// Parse an argument into `Self`.
    fn from_str(input: &str, case_insensitive: bool) -> Result<Self, String>;
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
    fn augment_clap(app: App<'_>) -> App<'_> {
        <T as IntoApp>::augment_clap(app)
    }
    fn into_app_for_update<'help>() -> App<'help> {
        <T as IntoApp>::into_app_for_update()
    }
    fn augment_clap_for_update(app: App<'_>) -> App<'_> {
        <T as IntoApp>::augment_clap_for_update(app)
    }
}

impl<T: FromArgMatches> FromArgMatches for Box<T> {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Box::new(<T as FromArgMatches>::from_arg_matches(matches))
    }
    fn update_from_arg_matches(&mut self, matches: &ArgMatches) {
        <T as FromArgMatches>::update_from_arg_matches(self, matches);
    }
}

impl<T: Subcommand> Subcommand for Box<T> {
    fn from_subcommand(subcommand: Option<(&str, &ArgMatches)>) -> Option<Self> {
        <T as Subcommand>::from_subcommand(subcommand).map(Box::new)
    }
    fn update_from_subcommand(&mut self, subcommand: Option<(&str, &ArgMatches)>) {
        <T as Subcommand>::update_from_subcommand(self, subcommand);
    }
    fn augment_subcommands(app: App<'_>) -> App<'_> {
        <T as Subcommand>::augment_subcommands(app)
    }
    fn augment_subcommands_for_update(app: App<'_>) -> App<'_> {
        <T as Subcommand>::augment_subcommands_for_update(app)
    }
}
