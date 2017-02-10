// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

#![deny(missing_docs)]

//! `StructOpt` trait definition
//!
//! This crate defines the `StructOpt` trait.  Alone, this crate is of
//! little interest.  See the `structopt-derive` crate to
//! automatically generate implementation of this trait.

extern crate clap;

/// A struct that is converted from command line arguments.
pub trait StructOpt {
    /// Returns the corresponding `clap::App`.
    fn clap<'a, 'b>() -> clap::App<'a, 'b>;

    /// Creates the struct from `clap::ArgMatches`.
    fn from_clap(clap::ArgMatches) -> Self;

    /// Gets the struct from the command line arguments.  Print the
    /// error message and quit the program in case of failure.
    fn from_args() -> Self where Self: Sized {
        Self::from_clap(Self::clap().get_matches())
    }
}
