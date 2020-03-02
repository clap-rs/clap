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

use super::{dummies, from_argmatches, into_app, subcommand};
use proc_macro2::Ident;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{self, punctuated, token, Attribute, DataEnum};

pub fn derive_clap(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    use syn::Data::*;

    let ident = &input.ident;

    match input.data {
        Struct(syn::DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => {
            dummies::clap_struct(ident);
            gen_for_struct(ident, &fields.named, &input.attrs)
        }
        Struct(syn::DataStruct {
            fields: syn::Fields::Unit,
            ..
        }) => {
            dummies::clap_struct(ident);
            gen_for_struct(
                ident,
                &punctuated::Punctuated::<syn::Field, token::Comma>::new(),
                &input.attrs,
            )
        }
        Enum(ref e) => {
            dummies::clap_enum(ident);
            gen_for_enum(ident, &input.attrs, e)
        }
        _ => abort_call_site!("`#[derive(Clap)]` only supports non-tuple structs and enums"),
    }
}

fn gen_for_struct(
    name: &syn::Ident,
    fields: &punctuated::Punctuated<syn::Field, token::Comma>,
    attrs: &[syn::Attribute],
) -> proc_macro2::TokenStream {
    let (into_app, attrs) = into_app::gen_for_struct(name, fields, attrs);
    let from_arg_matches = from_argmatches::gen_for_struct(name, fields, &attrs);

    quote! {
        impl ::clap::Clap for #name {}

        #into_app
        #from_arg_matches
    }
}

fn gen_for_enum(name: &Ident, attrs: &[Attribute], e: &DataEnum) -> proc_macro2::TokenStream {
    let into_app = into_app::gen_for_enum(name);
    let from_arg_matches = from_argmatches::gen_for_enum(name);
    let subcommand = subcommand::gen_for_enum(name, attrs, e);
    quote! {
        impl ::clap::Clap for #name { }

        #into_app
        #from_arg_matches
        #subcommand
    }
}
