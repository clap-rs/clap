// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This work was derived from Structopt (https://github.com/TeXitoi/structopt)
// commit#ea76fa1b1b273e65e3b0b1046643715b49bec51f which is licensed under the
// MIT/Apache 2.0 license.

//! This crate is custom derive for clap. It should not be used
//! directly. See [clap documentation](https://docs.rs/clap)
//! for the usage of `#[derive(Clap)]`.

extern crate proc_macro;

use proc_macro_error::proc_macro_error;

mod derives;

// /// It is required to have this seperate and specificly defined.
// #[proc_macro_derive(ArgEnum, attributes(case_sensitive))]
// pub fn arg_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input: syn::DeriveInput = syn::parse(input).unwrap();
//     derives::derive_arg_enum(&input).into()
// }

/// Generates the `Clap` impl.
#[proc_macro_derive(Clap, attributes(clap))]
#[proc_macro_error]
pub fn clap(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    derives::derive_clap(&input).into()
}

/// Generates the `IntoApp` impl.
#[proc_macro_derive(IntoApp, attributes(clap))]
#[proc_macro_error]
pub fn into_app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    derives::derive_into_app(&input).into()
}

/// Generates the `FromArgMatches` impl.
#[proc_macro_derive(FromArgMatches, attributes(clap))]
#[proc_macro_error]
pub fn from_arg_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    derives::derive_from_argmatches(&input).into()
}

/// Generates the `Subcommand` impl.
#[proc_macro_derive(Subcommand, attributes(clap))]
#[proc_macro_error]
pub fn subcommand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    derives::derive_subcommand(&input).into()
}
