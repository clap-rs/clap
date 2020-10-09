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
    attrs::{Attrs, Name, DEFAULT_CASING, DEFAULT_ENV_CASING},
    dummies,
    utils::Sp,
};

use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Data, DataEnum, DeriveInput,
    Fields, Ident, Variant,
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
    let variants = gen_variants(&lits);
    let from_str = gen_from_str(&lits);

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
        impl ::clap::ArgEnum for #name {
            #variants
            #from_str
        }
    }
}

fn lits(
    variants: &Punctuated<Variant, Comma>,
    parent_attribute: &Attrs,
) -> Vec<(TokenStream, Ident)> {
    variants
        .iter()
        .flat_map(|variant| {
            let attrs = Attrs::from_struct(
                variant.span(),
                &variant.attrs,
                Name::Derived(variant.ident.clone()),
                parent_attribute.casing(),
                parent_attribute.env_casing(),
            );

            let mut ret = vec![(attrs.cased_name(), variant.ident.clone())];

            attrs
                .enum_aliases()
                .into_iter()
                .for_each(|x| ret.push((x, variant.ident.clone())));

            ret
        })
        .collect::<Vec<_>>()
}

fn gen_variants(lits: &[(TokenStream, Ident)]) -> TokenStream {
    let lit = lits.iter().map(|l| &l.0).collect::<Vec<_>>();

    quote! {
        const VARIANTS: &'static [&'static str] = &[#(#lit),*];
    }
}

fn gen_from_str(lits: &[(TokenStream, Ident)]) -> TokenStream {
    let (lit, variant): (Vec<TokenStream>, Vec<Ident>) = lits.iter().cloned().unzip();

    quote! {
        fn from_str(input: &str, case_insensitive: bool) -> ::std::result::Result<Self, String> {
            let func = if case_insensitive {
                ::std::ascii::AsciiExt::eq_ignore_ascii_case
            } else {
                str::eq
            };

            match input {
                #(val if func(val, #lit) => Ok(Self::#variant),)*
                e => unreachable!("The impossible variant have been spotted: {}", e),
            }
        }
    }
}
