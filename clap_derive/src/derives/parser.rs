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

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::Ident;
use syn::{
    self, punctuated::Punctuated, token::Comma, Data, DataEnum, DataStruct, DeriveInput, Field,
    Fields, Generics,
};

use crate::derives::{args, into_app, subcommand};
use crate::dummies;
use crate::item::Item;
use crate::item::Name;

pub fn derive_parser(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => {
            dummies::parser_struct(ident);

            let name = Name::Assigned(quote!(std::env::var("CARGO_PKG_NAME")
                .ok()
                .unwrap_or_default()));
            let item = Item::from_args_struct(input, name);
            gen_for_struct(&item, ident, &input.generics, &fields.named)
        }
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            dummies::parser_struct(ident);

            let name = Name::Assigned(quote!(std::env::var("CARGO_PKG_NAME")
                .ok()
                .unwrap_or_default()));
            let item = Item::from_args_struct(input, name);
            gen_for_struct(
                &item,
                ident,
                &input.generics,
                &Punctuated::<Field, Comma>::new(),
            )
        }
        Data::Enum(ref e) => {
            dummies::parser_enum(ident);

            let name = Name::Assigned(quote!(std::env::var("CARGO_PKG_NAME")
                .ok()
                .unwrap_or_default()));
            let item = Item::from_subcommand_enum(input, name);
            gen_for_enum(&item, ident, &input.generics, e)
        }
        _ => abort_call_site!("`#[derive(Parser)]` only supports non-tuple structs and enums"),
    }
}

fn gen_for_struct(
    item: &Item,
    item_name: &Ident,
    generics: &Generics,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let into_app = into_app::gen_for_struct(item, item_name, generics);
    let args = args::gen_for_struct(item, item_name, generics, fields);

    quote! {
        impl #impl_generics clap::Parser for #item_name #ty_generics #where_clause {}

        #into_app
        #args
    }
}

fn gen_for_enum(item: &Item, item_name: &Ident, generics: &Generics, e: &DataEnum) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let into_app = into_app::gen_for_enum(item, item_name, generics);
    let subcommand = subcommand::gen_for_enum(item, item_name, generics, e);

    quote! {
        impl #impl_generics clap::Parser for #item_name #ty_generics #where_clause {}

        #into_app
        #subcommand
    }
}
