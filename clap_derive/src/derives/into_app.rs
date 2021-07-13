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

use std::env;

use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{Attribute, Data, DataStruct, DeriveInput, Fields, Ident};

use crate::{
    attrs::{Attrs, Name, DEFAULT_CASING, DEFAULT_ENV_CASING},
    dummies,
    utils::Sp,
};

pub fn derive_into_app(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    dummies::into_app(ident);

    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(_),
            ..
        }) => gen_for_struct(ident, &input.attrs),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => gen_for_struct(ident, &input.attrs),
        Data::Enum(_) => gen_for_enum(ident, &input.attrs),
        _ => abort_call_site!("`#[derive(IntoApp)]` only supports non-tuple structs and enums"),
    }
}

pub fn gen_for_struct(struct_name: &Ident, attrs: &[Attribute]) -> TokenStream {
    let app_name = env::var("CARGO_PKG_NAME").ok().unwrap_or_default();

    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Assigned(quote!(#app_name)),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );
    let name = attrs.cased_name();

    let tokens = quote! {
        #[allow(dead_code, unreachable_code, unused_variables)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo
        )]
        #[deny(clippy::correctness)]
        impl clap::IntoApp for #struct_name {
            fn into_app<'b>() -> clap::App<'b> {
                let app = clap::App::new(#name);
                <#struct_name as clap::Args>::augment_args(app)
            }

            fn into_app_for_update<'b>() -> clap::App<'b> {
                let app = clap::App::new(#name);
                <#struct_name as clap::Args>::augment_args_for_update(app)
            }
        }
    };

    tokens
}

pub fn gen_for_enum(enum_name: &Ident, attrs: &[Attribute]) -> TokenStream {
    let app_name = env::var("CARGO_PKG_NAME").ok().unwrap_or_default();

    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Assigned(quote!(#app_name)),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );
    let name = attrs.cased_name();

    quote! {
        #[allow(dead_code, unreachable_code, unused_variables)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo
        )]
        #[deny(clippy::correctness)]
        impl clap::IntoApp for #enum_name {
            fn into_app<'b>() -> clap::App<'b> {
                let app = clap::App::new(#name)
                    .setting(clap::AppSettings::SubcommandRequiredElseHelp);
                <#enum_name as clap::Subcommand>::augment_subcommands(app)
            }

            fn into_app_for_update<'b>() -> clap::App<'b> {
                let app = clap::App::new(#name);
                <#enum_name as clap::Subcommand>::augment_subcommands_for_update(app)
            }
        }
    }
}
