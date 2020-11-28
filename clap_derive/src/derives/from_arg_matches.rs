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
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, quote_spanned};
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, Field, Ident, Type};

use crate::{
    attrs::{Attrs, Kind, ParserKind},
    utils::{sub_type, subty_if_name, Sp, Ty},
};

pub fn gen_for_struct(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
    parent_attribute: &Attrs,
) -> TokenStream {
    let constructor = gen_constructor(fields, parent_attribute);
    let updater = gen_updater(fields, parent_attribute, true);

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
        impl ::clap::FromArgMatches for #struct_name {
            fn from_arg_matches(arg_matches: &::clap::ArgMatches) -> Self {
                #struct_name #constructor
            }

            fn update_from_arg_matches(&mut self, arg_matches: &::clap::ArgMatches) {
                #updater
            }
        }
    }
}

pub fn gen_for_enum(name: &Ident) -> TokenStream {
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
        impl ::clap::FromArgMatches for #name {
            fn from_arg_matches(arg_matches: &::clap::ArgMatches) -> Self {
                <#name as ::clap::Subcommand>::from_subcommand(arg_matches.subcommand()).unwrap()
            }
            fn update_from_arg_matches(&mut self, arg_matches: &::clap::ArgMatches) {
                <#name as ::clap::Subcommand>::update_from_subcommand(self, arg_matches.subcommand());
            }
        }
    }
}

fn gen_arg_enum_parse(ty: &Type, attrs: &Attrs) -> TokenStream {
    let ci = attrs.case_insensitive();

    quote_spanned! { ty.span()=>
        |s| <#ty as ::clap::ArgEnum>::from_str(s, #ci).unwrap()
    }
}

fn gen_parsers(
    attrs: &Attrs,
    ty: &Sp<Ty>,
    field_name: &Ident,
    field: &Field,
    update: Option<&TokenStream>,
) -> TokenStream {
    use self::ParserKind::*;

    let parser = attrs.parser();
    let func = &parser.func;
    let span = parser.kind.span();
    let (value_of, values_of, mut parse) = match *parser.kind {
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
        Ty::Bool => {
            if update.is_some() {
                quote_spanned! { ty.span()=>
                    *#field_name || arg_matches.is_present(#name)
                }
            } else {
                quote_spanned! { ty.span()=>
                    arg_matches.is_present(#name)
                }
            }
        }

        Ty::Option => {
            if attrs.is_enum() {
                if let Some(subty) = subty_if_name(&field.ty, "Option") {
                    parse = gen_arg_enum_parse(subty, &attrs);
                }
            }

            quote_spanned! { ty.span()=>
                arg_matches.#value_of(#name)
                    .map(#parse)
            }
        }

        Ty::OptionOption => quote_spanned! { ty.span()=>
            if arg_matches.is_present(#name) {
                Some(arg_matches.#value_of(#name).map(#parse))
            } else {
                None
            }
        },

        Ty::OptionVec => quote_spanned! { ty.span()=>
            if arg_matches.is_present(#name) {
                Some(arg_matches.#values_of(#name)
                     .map(|v| v.map(#parse).collect())
                     .unwrap_or_else(Vec::new))
            } else {
                None
            }
        },

        Ty::Vec => {
            if attrs.is_enum() {
                if let Some(subty) = subty_if_name(&field.ty, "Vec") {
                    parse = gen_arg_enum_parse(subty, &attrs);
                }
            }

            quote_spanned! { ty.span()=>
                arg_matches.#values_of(#name)
                    .map(|v| v.map(#parse).collect())
                    .unwrap_or_else(Vec::new)
            }
        }

        Ty::Other if occurrences => quote_spanned! { ty.span()=>
            #parse(arg_matches.#value_of(#name))
        },

        Ty::Other if flag => quote_spanned! { ty.span()=>
            #parse(arg_matches.is_present(#name))
        },

        Ty::Other => {
            if attrs.is_enum() {
                parse = gen_arg_enum_parse(&field.ty, &attrs);
            }

            quote_spanned! { ty.span()=>
                arg_matches.#value_of(#name)
                    .map(#parse)
                    .unwrap()
            }
        }
    };

    if let Some(access) = update {
        quote_spanned! { field.span()=>
            if arg_matches.is_present(#name) {
                #access
                *#field_name = #field_value
            }
        }
    } else {
        quote_spanned!(field.span()=> #field_name: #field_value )
    }
}

pub fn gen_constructor(fields: &Punctuated<Field, Comma>, parent_attribute: &Attrs) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let attrs = Attrs::from_field(
            field,
            parent_attribute.casing(),
            parent_attribute.env_casing(),
        );
        let field_name = field.ident.as_ref().unwrap();
        let kind = attrs.kind();
        match &*kind {
            Kind::ExternalSubcommand => {
                abort! { kind.span(),
                    "`external_subcommand` can be used only on enum variants"
                }
            }
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
                    #field_name: {
                        <#subcmd_type as ::clap::Subcommand>::from_subcommand(arg_matches.subcommand())
                        #unwrapper
                    }
                }
            }

            Kind::Flatten => quote_spanned! { kind.span()=>
                #field_name: ::clap::FromArgMatches::from_arg_matches(arg_matches)
            },

            Kind::Skip(val) => match val {
                None => quote_spanned!(kind.span()=> #field_name: Default::default()),
                Some(val) => quote_spanned!(kind.span()=> #field_name: (#val).into()),
            },

            Kind::Arg(ty) => gen_parsers(&attrs, ty, field_name, field, None),
        }
    });

    quote! {{
        #( #fields ),*
    }}
}

pub fn gen_updater(
    fields: &Punctuated<Field, Comma>,
    parent_attribute: &Attrs,
    use_self: bool,
) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let attrs = Attrs::from_field(
            field,
            parent_attribute.casing(),
            parent_attribute.env_casing(),
        );
        let field_name = field.ident.as_ref().unwrap();
        let kind = attrs.kind();

        let access = if use_self {
            quote! {
                #[allow(non_snake_case)]
                let #field_name = &mut self.#field_name;
            }
        } else {
            quote!()
        };

        match &*kind {
            Kind::ExternalSubcommand => {
                abort! { kind.span(),
                    "`external_subcommand` can be used only on enum variants"
                }
            }
            Kind::Subcommand(ty) => {
                let subcmd_type = match (**ty, sub_type(&field.ty)) {
                    (Ty::Option, Some(sub_type)) => sub_type,
                    _ => &field.ty,
                };

                let updater = quote_spanned!{ ty.span()=>
                    <#subcmd_type as ::clap::Subcommand>::update_from_subcommand(#field_name, subcmd);
                };

                let updater = match **ty {
                    Ty::Option => quote_spanned! { kind.span()=>
                        if let Some(#field_name) = #field_name.as_mut() {
                            #updater
                        } else {
                            *#field_name = <#subcmd_type as ::clap::Subcommand>::from_subcommand(
                                subcmd
                            )
                        }
                    },
                    _ => quote_spanned!{ kind.span()=>
                        #updater
                    }
                };

                quote_spanned! { kind.span()=>
                    {
                        let subcmd = arg_matches.subcommand();
                        #access
                        #updater
                    }
                }
            }

            Kind::Flatten => quote_spanned! { kind.span()=> {
                    #access
                    ::clap::FromArgMatches::update_from_arg_matches(#field_name, arg_matches);
                }
            },

            Kind::Skip(_) => quote!(),

            Kind::Arg(ty) => {
                gen_parsers(&attrs, ty, field_name, field, Some(&access))
            }
        }
    });

    quote! {
        #( #fields )*
    }
}
