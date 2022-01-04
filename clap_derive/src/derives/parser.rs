// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
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
    derives::{args, into_app, subcommand},
    dummies,
};

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{
    self, punctuated::Punctuated, token::Comma, Attribute, Data, DataEnum, DataStruct, DeriveInput,
    Field, Fields, Generics, Ident,
};

pub fn derive_parser(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => {
            dummies::parser_struct(ident);
            gen_for_struct(ident, &input.generics, &fields.named, &input.attrs)
        }
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            dummies::parser_struct(ident);
            gen_for_struct(
                ident,
                &input.generics,
                &Punctuated::<Field, Comma>::new(),
                &input.attrs,
            )
        }
        Data::Enum(ref e) => {
            dummies::parser_enum(ident);
            gen_for_enum(ident, &input.generics, &input.attrs, e)
        }
        _ => abort_call_site!("`#[derive(Parser)]` only supports non-tuple structs and enums"),
    }
}

fn gen_for_struct(
    name: &Ident,
    generics: &Generics,
    fields: &Punctuated<Field, Comma>,
    attrs: &[Attribute],
) -> TokenStream {
    let into_app = into_app::gen_for_struct(name, generics, attrs);
    let args = args::gen_for_struct(name, generics, fields, attrs);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics clap::Parser for #name #ty_generics #where_clause {}

        #into_app
        #args
    }
}

fn gen_for_enum(
    name: &Ident,
    generics: &Generics,
    attrs: &[Attribute],
    e: &DataEnum,
) -> TokenStream {
    let into_app = into_app::gen_for_enum(name, generics, attrs);
    let subcommand = subcommand::gen_for_enum(name, generics, attrs, e);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics clap::Parser for #name #ty_generics #where_clause {}

        #into_app
        #subcommand
    }
}
