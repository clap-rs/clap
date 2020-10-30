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

use crate::{
    derives::{arg_enum, from_arg_matches, into_app, subcommand},
    dummies,
};

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{
    self, punctuated::Punctuated, token::Comma, Attribute, Data, DataEnum, DataStruct, DeriveInput,
    Field, Fields, Ident,
};

pub fn derive_clap(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => {
            dummies::clap_struct(ident);
            gen_for_struct(ident, &fields.named, &input.attrs)
        }
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            dummies::clap_struct(ident);
            gen_for_struct(ident, &Punctuated::<Field, Comma>::new(), &input.attrs)
        }
        Data::Enum(ref e) => {
            dummies::clap_enum(ident);
            gen_for_enum(ident, &input.attrs, e)
        }
        _ => abort_call_site!("`#[derive(Clap)]` only supports non-tuple structs and enums"),
    }
}

fn gen_for_struct(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
    attrs: &[Attribute],
) -> TokenStream {
    let (into_app, attrs) = into_app::gen_for_struct(name, fields, attrs);
    let from_arg_matches = from_arg_matches::gen_for_struct(name, fields, &attrs);

    quote! {
        impl ::clap::Clap for #name {}

        #into_app
        #from_arg_matches
    }
}

fn gen_for_enum(name: &Ident, attrs: &[Attribute], e: &DataEnum) -> TokenStream {
    let into_app = into_app::gen_for_enum(name, attrs);
    let from_arg_matches = from_arg_matches::gen_for_enum(name);
    let subcommand = subcommand::gen_for_enum(name, attrs, e);
    let arg_enum = arg_enum::gen_for_enum(name, attrs, e);

    quote! {
        impl ::clap::Clap for #name {}

        #into_app
        #from_arg_matches
        #subcommand
        #arg_enum
    }
}
