// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    attrs::{Attrs, Kind, Name, DEFAULT_CASING, DEFAULT_ENV_CASING},
    dummies,
    utils::Sp,
};

use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Data, DataEnum, DeriveInput, Fields, Ident,
    Variant,
};

pub fn derive_arg_enum(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    dummies::arg_enum(ident);

    match input.data {
        Data::Enum(ref e) => gen_for_enum(ident, &input.attrs, e),
        _ => abort_call_site!("`#[derive(ArgEnum)]` only supports enums"),
    }
}

pub fn gen_for_enum(name: &Ident, attrs: &[Attribute], e: &DataEnum) -> TokenStream {
    if !e.variants.iter().all(|v| matches!(v.fields, Fields::Unit)) {
        return quote!();
    };

    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Derived(name.clone()),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );

    let lits = lits(&e.variants, &attrs);
    let arg_values = gen_arg_values(&lits);
    let from_str = gen_from_str(&lits);
    let arg_value = gen_arg_value(&lits);

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
        impl clap::ArgEnum for #name {
            #arg_values
            #from_str
            #arg_value
        }
    }
}

fn lits(
    variants: &Punctuated<Variant, Comma>,
    parent_attribute: &Attrs,
) -> Vec<(TokenStream, Ident)> {
    variants
        .iter()
        .filter_map(|variant| {
            let attrs = Attrs::from_arg_enum_variant(
                variant,
                parent_attribute.casing(),
                parent_attribute.env_casing(),
            );
            if let Kind::Skip(_) = &*attrs.kind() {
                None
            } else {
                let fields = attrs.field_methods();
                let name = attrs.cased_name();
                Some((
                    quote! {
                        clap::ArgValue::new(#name)
                        #fields
                    },
                    variant.ident.clone(),
                ))
            }
        })
        .collect::<Vec<_>>()
}

fn gen_arg_values(lits: &[(TokenStream, Ident)]) -> TokenStream {
    let lit = lits.iter().map(|l| &l.0).collect::<Vec<_>>();

    quote! {
        fn arg_values() -> Vec<clap::ArgValue<'static>> {
            vec![#(#lit),*]
        }
    }
}

fn gen_from_str(lits: &[(TokenStream, Ident)]) -> TokenStream {
    let (lit, variant): (Vec<TokenStream>, Vec<Ident>) = lits.iter().cloned().unzip();

    quote! {
        fn from_str(input: &str, case_insensitive: bool) -> ::std::result::Result<Self, String> {
            match input {
                #(val if #lit.matches(val, case_insensitive) => Ok(Self::#variant),)*
                e => Err(format!("Invalid variant: {}", e)),
            }
        }
    }
}

fn gen_arg_value(lits: &[(TokenStream, Ident)]) -> TokenStream {
    let (lit, variant): (Vec<TokenStream>, Vec<Ident>) = lits.iter().cloned().unzip();

    quote! {
        fn arg_value(&self) -> Option<clap::ArgValue<'static>> {
            match self {
                #(Self::#variant => Some(#lit),)*
                _ => None
            }
        }
    }
}
