//! This module contains traits that are usable with `#[derive(...)].`

use crate::{App, ArgMatches, Error};
use std::ffi::OsString;

/// This trait is just a convenience on top of FromArgMatches + IntoApp
pub trait Clap: FromArgMatches + IntoApp + Sized {
    /// Parse from `std::env::args()`, exit on error
    fn parse() -> Self {
        let matches = <Self as IntoApp>::into_app().get_matches();
        <Self as FromArgMatches>::from_arg_matches(&matches)
    }

    /// Parse from `std::env::args()`, return Err on error.
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

    /// Parse from `std::env::args()`, return Err on error.
    fn try_parse_from<I, T>(itr: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = T>,
        // TODO (@CreepySkeleton): discover a way to avoid cloning here
        T: Into<OsString> + Clone,
    {
        let matches = <Self as IntoApp>::into_app().try_get_matches_from(itr)?;
        Ok(<Self as FromArgMatches>::from_arg_matches(&matches))
    }
}

/// Build an App according to the struct
///
/// Also serves for flattening
pub trait IntoApp: Sized {
    /// @TODO @release @docs
    fn into_app<'b>() -> App<'b>;
    /// @TODO @release @docs
    fn augment_clap(app: App<'_>) -> App<'_>;
}

/// Extract values from ArgMatches into the struct.
pub trait FromArgMatches: Sized {
    /// @TODO @release @docs
    fn from_arg_matches(matches: &ArgMatches) -> Self;
}

/// @TODO @release @docs
pub trait Subcommand: Sized {
    /// @TODO @release @docs
    fn from_subcommand(name: &str, matches: Option<&ArgMatches>) -> Option<Self>;
    /// @TODO @release @docs
    fn augment_subcommands(app: App<'_>) -> App<'_>;
}

/// @TODO @release @docs
pub trait ArgEnum {}
