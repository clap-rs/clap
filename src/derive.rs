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
///         .long("n")
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
/// [`App`]: ./struct.App.html
/// [`ArgMatches`]: ./struct.ArgMatches.html
/// [`FromArgMatches`]: ./trait.FromArgMatches.html
/// [`IntoApp`]: ./trait.IntoApp.html
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
        let matches = <Self as IntoApp>::into_update_app().get_matches_from(itr);
        <Self as FromArgMatches>::update_from_arg_matches(self, &matches);
    }

    /// Update from iterator, return Err on error.
    fn try_update_from<I, T>(&mut self, itr: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_update_app().try_get_matches_from(itr)?;
        <Self as FromArgMatches>::update_from_arg_matches(self, &matches);
        Ok(())
    }
}

/// Build an App according to the struct
///
/// Also serves for flattening
pub trait IntoApp: Sized {
    /// @TODO @release @docs
    fn into_app<'help>() -> App<'help>;
    /// @TODO @release @docs
    fn augment_clap(app: App<'_>) -> App<'_>;
    /// @TODO @release @docs
    fn into_update_app<'help>() -> App<'help>;
    /// @TODO @release @docs
    fn augment_update_clap(app: App<'_>) -> App<'_>;
}

/// Converts an instance of [`ArgMatches`] to a consumer defined struct.
///
/// [`ArgMatches`]: ./struct.ArgMatches.html
pub trait FromArgMatches: Sized {
    /// It's common to have an "application context" struct (sometimes called
    /// config) that represents all the normalized values after being processed by
    /// the CLI.
    ///
    /// For instance, if an application we made had two CLI options, `--name
    /// <STRING>` and a flag `--debug` to distinguish "debugging mode" for our made
    /// up CLI, we may create a context struct as follows:
    ///
    /// ```no_run
    /// struct Context {
    ///     name: String,
    ///     debug: bool
    /// }
    /// ```
    ///
    /// And after letting `clap` parse the CLI, we get back and instance of
    /// `ArgMatches`, we may create a `From` implementation like so:
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

    /// @TODO@ @release @docs
    fn update_from_arg_matches(&mut self, matches: &ArgMatches);
}

/// @TODO @release @docs
pub trait Subcommand: Sized {
    /// @TODO @release @docs
    fn from_subcommand(subcommand: Option<(&str, &ArgMatches)>) -> Option<Self>;
    /// @TODO @release @docs
    fn update_from_subcommand(&mut self, subcommand: Option<(&str, &ArgMatches)>);
    /// @TODO @release @docs
    fn augment_subcommands(app: App<'_>) -> App<'_>;
    /// @TODO @release @docs
    fn augment_update_subcommands(app: App<'_>) -> App<'_>;
}

/// @TODO @release @docs
pub trait ArgEnum: Sized {
    /// @TODO @release @docs
    const VARIANTS: &'static [&'static str];

    /// @TODO @release @docs
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
    fn into_update_app<'help>() -> App<'help> {
        <T as IntoApp>::into_update_app()
    }
    fn augment_update_clap(app: App<'_>) -> App<'_> {
        <T as IntoApp>::augment_update_clap(app)
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
    fn augment_update_subcommands(app: App<'_>) -> App<'_> {
        <T as Subcommand>::augment_update_subcommands(app)
    }
}
