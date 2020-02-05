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

use proc_macro2;
use syn;
use syn::punctuated;
use syn::spanned::Spanned as _;
use syn::token;

use super::{
    spanned::Sp, sub_type, Attrs, Kind, Name, ParserKind, Ty, DEFAULT_CASING, DEFAULT_ENV_CASING,
};

pub fn derive_from_argmatches(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    use syn::Data::*;

    let struct_name = &input.ident;
    let inner_impl = match input.data {
        Struct(syn::DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => {
            let name = env::var("CARGO_PKG_NAME")
                .ok()
                .unwrap_or_else(String::default);

            let attrs = Attrs::from_struct(
                proc_macro2::Span::call_site(),
                &input.attrs,
                Name::Assigned(quote!(#name)),
                Sp::call_site(DEFAULT_CASING),
                Sp::call_site(DEFAULT_ENV_CASING),
            );

            gen_from_argmatches_impl_for_struct(struct_name, &fields.named, &attrs)
        }
        // Enum(ref e) => clap_for_enum_impl(struct_name, &e.variants, &input.attrs),
        _ => panic!("clap_derive only supports non-tuple structs"), // and enums"),
    };

    quote!(#inner_impl)
}

pub fn gen_from_argmatches_impl_for_struct(
    name: &syn::Ident,
    fields: &punctuated::Punctuated<syn::Field, token::Comma>,
    parent_attribute: &Attrs,
) -> proc_macro2::TokenStream {
    let from_argmatches_fn = gen_from_argmatches_fn_for_struct(name, fields, parent_attribute);

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
    parent_attribute: &Attrs,
) -> proc_macro2::TokenStream {
    let field_block = gen_constructor(fields, parent_attribute);

    quote! {
        fn from_argmatches(matches: &::clap::ArgMatches) -> Self {
            #struct_name #field_block
        }
    }
}

pub fn gen_constructor(
    fields: &punctuated::Punctuated<syn::Field, token::Comma>,
    parent_attribute: &Attrs,
) -> proc_macro2::TokenStream {
    let fields = fields.iter().map(|field| {
        let attrs = Attrs::from_field(
            field,
            parent_attribute.casing(),
            parent_attribute.env_casing(),
        );
        let field_name = field.ident.as_ref().unwrap();
        let kind = attrs.kind();
        match &*attrs.kind() {
            Kind::Subcommand(ty) => {
                let subcmd_type = match (**ty, sub_type(&field.ty)) {
                    (Ty::Option, Some(sub_type)) => sub_type,
                    _ => &field.ty,
                };
                let unwrapper = match **ty {
                    Ty::Option => quote!(),
                    _ => quote_spanned!( ty.span()=> .unwrap() ),
                };
                quote_spanned! { kind.span()=>
                    #field_name: <#subcmd_type>::from_subcommand(matches.subcommand())#unwrapper
                }
            }

            Kind::Flatten => quote_spanned! { kind.span()=>
                #field_name: ::clap::FromArgMatches::from_argmatches(matches)
            },

            Kind::Skip(val) => match val {
                None => quote_spanned!(kind.span()=> #field_name: Default::default()),
                Some(val) => quote_spanned!(kind.span()=> #field_name: (#val).into()),
            },

            Kind::Arg(ty) => {
                use self::ParserKind::*;

                let parser = attrs.parser();
                let func = &parser.func;
                let span = parser.kind.span();
                let (value_of, values_of, parse) = match *parser.kind {
                    FromStr => (
                        quote_spanned!(span=> value_of),
                        quote_spanned!(span=> values_of),
                        func.clone(),
                    ),
                    TryFromStr => (
                        quote_spanned!(span=> value_of),
                        quote_spanned!(span=> values_of),
                        quote_spanned!(func.span()=> |s| #func(s).unwrap()),
                    ),
                    FromOsStr => (
                        quote_spanned!(span=> value_of_os),
                        quote_spanned!(span=> values_of_os),
                        func.clone(),
                    ),
                    TryFromOsStr => (
                        quote_spanned!(span=> value_of_os),
                        quote_spanned!(span=> values_of_os),
                        quote_spanned!(func.span()=> |s| #func(s).unwrap()),
                    ),
                    FromOccurrences => (
                        quote_spanned!(span=> occurrences_of),
                        quote!(),
                        func.clone(),
                    ),
                    FromFlag => (quote!(), quote!(), func.clone()),
                };

                let flag = *attrs.parser().kind == ParserKind::FromFlag;
                let occurrences = *attrs.parser().kind == ParserKind::FromOccurrences;
                let name = attrs.cased_name();
                let field_value = match **ty {
                    Ty::Bool => quote_spanned! { ty.span()=>
                        matches.is_present(#name)
                    },

                    Ty::Option => quote_spanned! { ty.span()=>
                        matches.#value_of(#name)
                            .map(#parse)
                    },

                    Ty::OptionOption => quote_spanned! { ty.span()=>
                        if matches.is_present(#name) {
                            Some(matches.#value_of(#name).map(#parse))
                        } else {
                            None
                        }
                    },

                    Ty::OptionVec => quote_spanned! { ty.span()=>
                        if matches.is_present(#name) {
                            Some(matches.#values_of(#name)
                                 .map(|v| v.map(#parse).collect())
                                 .unwrap_or_else(Vec::new))
                        } else {
                            None
                        }
                    },

                    Ty::Vec => quote_spanned! { ty.span()=>
                        matches.#values_of(#name)
                            .map(|v| v.map(#parse).collect())
                            .unwrap_or_else(Vec::new)
                    },

                    Ty::Other if occurrences => quote_spanned! { ty.span()=>
                        #parse(matches.#value_of(#name))
                    },

                    Ty::Other if flag => quote_spanned! { ty.span()=>
                        #parse(matches.is_present(#name))
                    },

                    Ty::Other => quote_spanned! { ty.span()=>
                        matches.#value_of(#name)
                            .map(#parse)
                            .unwrap()
                    },
                };

                quote_spanned!(field.span()=> #field_name: #field_value )
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
