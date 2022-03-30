// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
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
use proc_macro_error::{abort, abort_call_site};
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
    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Derived(name.clone()),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );

    let lits = lits(&e.variants, &attrs);
    let value_variants = gen_value_variants(&lits);
    let to_possible_value = gen_to_possible_value(&lits);

    quote! {
        #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
        )]
        #[deny(clippy::correctness)]
        impl clap::ArgEnum for #name {
            #value_variants
            #to_possible_value
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
                if !matches!(variant.fields, Fields::Unit) {
                    abort!(variant.span(), "`#[derive(ArgEnum)]` only supports non-unit variants, unless they are skipped");
                }
                let fields = attrs.field_methods(false);
                let name = attrs.cased_name();
                Some((
                    quote! {
                        clap::PossibleValue::new(#name)
                        #fields
                    },
                    variant.ident.clone(),
                ))
            }
        })
        .collect::<Vec<_>>()
}

fn gen_value_variants(lits: &[(TokenStream, Ident)]) -> TokenStream {
    let lit = lits.iter().map(|l| &l.1).collect::<Vec<_>>();

    quote! {
        fn value_variants<'a>() -> &'a [Self]{
            &[#(Self::#lit),*]
        }
    }
}

fn gen_to_possible_value(lits: &[(TokenStream, Ident)]) -> TokenStream {
    let (lit, variant): (Vec<TokenStream>, Vec<Ident>) = lits.iter().cloned().unzip();

    quote! {
        fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::PossibleValue<'a>> {
            match self {
                #(Self::#variant => Some(#lit),)*
                _ => None
            }
        }
    }
}
