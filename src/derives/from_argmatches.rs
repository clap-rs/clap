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

use proc_macro2;
use syn;
use syn::punctuated;
use syn::token;

use derives::{self, Attrs, Kind, Parser, Ty};

pub fn derive_from_argmatches(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    use syn::Data::*;

    let struct_name = &input.ident;
    let inner_impl = match input.data {
        Struct(syn::DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => gen_from_argmatches_impl_for_struct(struct_name, &fields.named),
        // Enum(ref e) => clap_for_enum_impl(struct_name, &e.variants, &input.attrs),
        _ => panic!("clap_derive only supports non-tuple structs"), // and enums"),
    };

    quote!(#inner_impl)
}

pub fn gen_from_argmatches_impl_for_struct(
    name: &syn::Ident,
    fields: &punctuated::Punctuated<syn::Field, token::Comma>,
) -> proc_macro2::TokenStream {
    let from_argmatches_fn = gen_from_argmatches_fn_for_struct(name, fields);

    quote! {
        impl ::clap::FromArgMatches for #name {
            #from_argmatches_fn
        }

        impl From<::clap::ArgMatches> for #name {
            fn from(m: ::clap::ArgMatches) -> Self {
                use ::clap::FromArgMatches;
                <Self as ::clap::FromArgMatches>::from_argmatches(&m)
            }
        }

        // @TODO impl TryFrom once stable
    }
}

pub fn gen_from_argmatches_fn_for_struct(
    struct_name: &syn::Ident,
    fields: &punctuated::Punctuated<syn::Field, token::Comma>,
) -> proc_macro2::TokenStream {
    let field_block = gen_constructor(fields);

    quote! {
        fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
            #struct_name #field_block
        }
    }
}

pub fn gen_constructor(
    fields: &punctuated::Punctuated<syn::Field, token::Comma>,
) -> proc_macro2::TokenStream {
    let fields = fields.iter().map(|field| {
        let attrs = Attrs::from_field(field);
        let field_name = field.ident.as_ref().unwrap();
        match attrs.kind() {
            Kind::Subcommand(ty) => {
                let subcmd_type = match (ty, derives::sub_type(&field.ty)) {
                    (Ty::Option, Some(sub_type)) => sub_type,
                    _ => &field.ty,
                };
                let unwrapper = match ty {
                    Ty::Option => quote!(),
                    _ => quote!( .unwrap() ),
                };
                quote!(#field_name: <#subcmd_type>::from_subcommand(matches.subcommand())#unwrapper)
            }
            Kind::FlattenStruct => {
                quote!(#field_name: ::clap::FromArgMatches::from_argmatches(matches))
            }
            Kind::Arg(ty) => {
                use self::Parser::*;
                let (value_of, values_of, parse) = match *attrs.parser() {
                    (FromStr, ref f) => (quote!(value_of), quote!(values_of), f.clone()),
                    (TryFromStr, ref f) => (
                        quote!(value_of),
                        quote!(values_of),
                        quote!(|s| #f(s).unwrap()),
                    ),
                    (FromOsStr, ref f) => (quote!(value_of_os), quote!(values_of_os), f.clone()),
                    (TryFromOsStr, ref f) => (
                        quote!(value_of_os),
                        quote!(values_of_os),
                        quote!(|s| #f(s).unwrap()),
                    ),
                    (FromOccurrences, ref f) => (quote!(occurrences_of), quote!(), f.clone()),
                };

                let occurences = attrs.parser().0 == Parser::FromOccurrences;
                let name = attrs.name();
                let field_value = match ty {
                    Ty::Bool => quote!(matches.is_present(#name)),
                    Ty::Option => quote! {
                        matches.#value_of(#name)
                            .as_ref()
                            .map(#parse)
                    },
                    Ty::Vec => quote! {
                        matches.#values_of(#name)
                            .map(|v| v.map(#parse).collect())
                            .unwrap_or_else(Vec::new)
                    },
                    Ty::Other if occurences => quote! {
                        #parse(matches.#value_of(#name))
                    },
                    Ty::Other => quote! {
                        matches.#value_of(#name)
                            .map(#parse)
                            .unwrap()
                    },
                };

                quote!( #field_name: #field_value )
            }
        }
    });

    quote! {{
        #( #fields ),*
    }}
}

pub fn gen_from_argmatches_impl_for_enum(name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl ::clap::FromArgMatches for #name {
            fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
                <#name>::from_subcommand(matches.subcommand())
                    .unwrap()
            }
        }

        impl From<::clap::ArgMatches> for #name {
            fn from(m: ::clap::ArgMatches) -> Self {
                use ::clap::FromArgMatches;
                <Self as ::clap::FromArgMatches>::from_argmatches(&m)
            }
        }

        // @TODO: impl TryFrom once stable
    }
}
