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
use crate::{
    attrs::{Attrs, Kind, Name, DEFAULT_CASING, DEFAULT_ENV_CASING},
    derives::args,
    dummies,
    utils::{is_simple_ty, subty_if_name, Sp},
};

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::{quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataEnum, DeriveInput,
    FieldsUnnamed, Token, Variant,
};

pub fn derive_subcommand(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    dummies::subcommand(ident);

    match input.data {
        Data::Enum(ref e) => gen_for_enum(ident, &input.attrs, e),
        _ => abort_call_site!("`#[derive(Subcommand)]` only supports enums"),
    }
}

pub fn gen_for_enum(enum_name: &Ident, attrs: &[Attribute], e: &DataEnum) -> TokenStream {
    let from_arg_matches = gen_from_arg_matches_for_enum(enum_name, attrs, e);

    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Derived(enum_name.clone()),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );
    let augmentation = gen_augment(&e.variants, &attrs, false);
    let augmentation_update = gen_augment(&e.variants, &attrs, true);
    let has_subcommand = gen_has_subcommand(&e.variants, &attrs);

    quote! {
        #from_arg_matches

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
        impl clap::Subcommand for #enum_name {
            fn augment_subcommands <'b>(app: clap::App<'b>) -> clap::App<'b> {
                #augmentation
            }
            fn augment_subcommands_for_update <'b>(app: clap::App<'b>) -> clap::App<'b> {
                #augmentation_update
            }
            fn has_subcommand(name: &str) -> bool {
                #has_subcommand
            }
        }
    }
}

fn gen_from_arg_matches_for_enum(name: &Ident, attrs: &[Attribute], e: &DataEnum) -> TokenStream {
    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Derived(name.clone()),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );

    let from_arg_matches = gen_from_arg_matches(name, &e.variants, &attrs);
    let update_from_arg_matches = gen_update_from_arg_matches(name, &e.variants, &attrs);

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
        impl clap::FromArgMatches for #name {
            #from_arg_matches
            #update_from_arg_matches
        }
    }
}

fn gen_augment(
    variants: &Punctuated<Variant, Token![,]>,
    parent_attribute: &Attrs,
    override_required: bool,
) -> TokenStream {
    use syn::Fields::*;

    let subcommands: Vec<_> = variants
        .iter()
        .map(|variant| {
            let attrs = Attrs::from_variant(
                variant,
                parent_attribute.casing(),
                parent_attribute.env_casing(),
            );
            let kind = attrs.kind();

            match &*kind {
                Kind::ExternalSubcommand => {
                    quote_spanned! { kind.span()=>
                        let app = app.setting(clap::AppSettings::AllowExternalSubcommands);
                    }
                }

                Kind::Flatten => match variant.fields {
                    Unnamed(FieldsUnnamed { ref unnamed, .. }) if unnamed.len() == 1 => {
                        let ty = &unnamed[0];
                        if override_required {
                            quote! {
                                let app = <#ty as clap::Subcommand>::augment_subcommands_for_update(app);
                            }
                        } else {
                            quote! {
                                let app = <#ty as clap::Subcommand>::augment_subcommands(app);
                            }
                        }
                    }
                    _ => abort!(
                        variant,
                        "`flatten` is usable only with single-typed tuple variants"
                    ),
                },

                Kind::Subcommand(_) => {
                    let app_var = Ident::new("subcommand", Span::call_site());
                    let arg_block = match variant.fields {
                        Named(_) => {
                            abort!(variant, "non single-typed tuple enums are not supported")
                        }
                        Unit => quote!( #app_var ),
                        Unnamed(FieldsUnnamed { ref unnamed, .. }) if unnamed.len() == 1 => {
                            let ty = &unnamed[0];
                            if override_required {
                                quote_spanned! { ty.span()=>
                                    {
                                        <#ty as clap::Subcommand>::augment_subcommands_for_update(#app_var)
                                    }
                                }
                            } else {
                                quote_spanned! { ty.span()=>
                                    {
                                        <#ty as clap::Subcommand>::augment_subcommands(#app_var)
                                    }
                                }
                            }
                        }
                        Unnamed(..) => {
                            abort!(variant, "non single-typed tuple enums are not supported")
                        }
                    };

                    let name = attrs.cased_name();
                    let from_attrs = attrs.top_level_methods();
                    let version = attrs.version();
                    quote! {
                        let app = app.subcommand({
                            let #app_var = clap::App::new(#name);
                            let #app_var = #arg_block;
                            let #app_var = #app_var.setting(::clap::AppSettings::SubcommandRequiredElseHelp);
                            #app_var#from_attrs#version
                        });
                    }
                }

                _ => {
                    let app_var = Ident::new("subcommand", Span::call_site());
                    let arg_block = match variant.fields {
                        Named(ref fields) => {
                            args::gen_augment(&fields.named, &app_var, &attrs, override_required)
                        }
                        Unit => quote!( #app_var ),
                        Unnamed(FieldsUnnamed { ref unnamed, .. }) if unnamed.len() == 1 => {
                            let ty = &unnamed[0];
                            if override_required {
                                quote_spanned! { ty.span()=>
                                    {
                                        <#ty as clap::Args>::augment_args_for_update(#app_var)
                                    }
                                }
                            } else {
                                quote_spanned! { ty.span()=>
                                    {
                                        <#ty as clap::Args>::augment_args(#app_var)
                                    }
                                }
                            }
                        }
                        Unnamed(..) => {
                            abort!(variant, "non single-typed tuple enums are not supported")
                        }
                    };

                    let name = attrs.cased_name();
                    let from_attrs = attrs.top_level_methods();
                    let version = attrs.version();
                    quote! {
                        let app = app.subcommand({
                            let #app_var = clap::App::new(#name);
                            let #app_var = #arg_block;
                            #app_var#from_attrs#version
                        });
                    }
                }
            }
        })
        .collect();

    let app_methods = parent_attribute.top_level_methods();
    let version = parent_attribute.version();
    quote! {
            let app = app #app_methods;
            #( #subcommands )*;
            app #version
    }
}

fn gen_has_subcommand(
    variants: &Punctuated<Variant, Token![,]>,
    parent_attribute: &Attrs,
) -> TokenStream {
    use syn::Fields::*;

    let mut ext_subcmd = false;

    let (flatten_variants, variants): (Vec<_>, Vec<_>) = variants
        .iter()
        .filter_map(|variant| {
            let attrs = Attrs::from_variant(
                variant,
                parent_attribute.casing(),
                parent_attribute.env_casing(),
            );

            if let Kind::ExternalSubcommand = &*attrs.kind() {
                ext_subcmd = true;
                None
            } else {
                Some((variant, attrs))
            }
        })
        .partition(|(_, attrs)| {
            let kind = attrs.kind();
            matches!(&*kind, Kind::Flatten)
        });

    let match_arms = variants.iter().map(|(_variant, attrs)| {
        let sub_name = attrs.cased_name();
        quote! {
            #sub_name => true,
        }
    });
    let child_subcommands = flatten_variants
        .iter()
        .map(|(variant, _attrs)| match variant.fields {
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                quote! {
                    if <#ty as clap::Subcommand>::has_subcommand(name) {
                        return true;
                    }
                }
            }
            _ => abort!(
                variant,
                "`flatten` is usable only with single-typed tuple variants"
            ),
        });

    if ext_subcmd {
        quote! { true }
    } else {
        quote! {
            match name {
                #( #match_arms )*
                _ => {
                    #( #child_subcommands )else*

                    false
                }
            }
        }
    }
}

fn gen_from_arg_matches(
    name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
    parent_attribute: &Attrs,
) -> TokenStream {
    use syn::Fields::*;

    let mut ext_subcmd = None;

    let (flatten_variants, variants): (Vec<_>, Vec<_>) = variants
        .iter()
        .filter_map(|variant| {
            let attrs = Attrs::from_variant(
                variant,
                parent_attribute.casing(),
                parent_attribute.env_casing(),
            );

            if let Kind::ExternalSubcommand = &*attrs.kind() {
                if ext_subcmd.is_some() {
                    abort!(
                        attrs.kind().span(),
                        "Only one variant can be marked with `external_subcommand`, \
                         this is the second"
                    );
                }

                let ty = match variant.fields {
                    Unnamed(ref fields) if fields.unnamed.len() == 1 => &fields.unnamed[0].ty,

                    _ => abort!(
                        variant,
                        "The enum variant marked with `external_attribute` must be \
                         a single-typed tuple, and the type must be either `Vec<String>` \
                         or `Vec<OsString>`."
                    ),
                };

                let (span, str_ty, values_of) = match subty_if_name(ty, "Vec") {
                    Some(subty) => {
                        if is_simple_ty(subty, "String") {
                            (
                                subty.span(),
                                quote!(::std::string::String),
                                quote!(values_of),
                            )
                        } else if is_simple_ty(subty, "OsString") {
                            (
                                subty.span(),
                                quote!(::std::ffi::OsString),
                                quote!(values_of_os),
                            )
                        } else {
                            abort!(
                                ty.span(),
                                "The type must be either `Vec<String>` or `Vec<OsString>` \
                                 to be used with `external_subcommand`."
                            );
                        }
                    }

                    None => abort!(
                        ty.span(),
                        "The type must be either `Vec<String>` or `Vec<OsString>` \
                         to be used with `external_subcommand`."
                    ),
                };

                ext_subcmd = Some((span, &variant.ident, str_ty, values_of));
                None
            } else {
                Some((variant, attrs))
            }
        })
        .partition(|(_, attrs)| {
            let kind = attrs.kind();
            matches!(&*kind, Kind::Flatten)
        });

    let match_arms = variants.iter().map(|(variant, attrs)| {
        let sub_name = attrs.cased_name();
        let variant_name = &variant.ident;
        let constructor_block = match variant.fields {
            Named(ref fields) => args::gen_constructor(&fields.named, attrs),
            Unit => quote!(),
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                quote!( ( <#ty as clap::FromArgMatches>::from_arg_matches(arg_matches).unwrap() ) )
            }
            Unnamed(..) => abort_call_site!("{}: tuple enums are not supported", variant.ident),
        };

        quote! {
            Some((#sub_name, arg_matches)) => {
                Some(#name :: #variant_name #constructor_block)
            }
        }
    });
    let child_subcommands = flatten_variants.iter().map(|(variant, _attrs)| {
        let variant_name = &variant.ident;
        match variant.fields {
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                quote! {
                    if let Some(res) = <#ty as clap::FromArgMatches>::from_arg_matches(arg_matches) {
                        return Some(#name :: #variant_name (res));
                    }
                }
            }
            _ => abort!(
                variant,
                "`flatten` is usable only with single-typed tuple variants"
            ),
        }
    });

    let wildcard = match ext_subcmd {
        Some((span, var_name, str_ty, values_of)) => quote_spanned! { span=>
                ::std::option::Option::Some(#name::#var_name(
                    ::std::iter::once(#str_ty::from(other))
                    .chain(
                        sub_arg_matches.#values_of("").into_iter().flatten().map(#str_ty::from)
                    )
                    .collect::<::std::vec::Vec<_>>()
                ))
        },

        None => quote!(None),
    };

    quote! {
        fn from_arg_matches(arg_matches: &clap::ArgMatches) -> Option<Self> {
            match arg_matches.subcommand() {
                #( #match_arms, )*
                ::std::option::Option::Some((other, sub_arg_matches)) => {
                    #( #child_subcommands )else*

                    #wildcard
                }
                ::std::option::Option::None => ::std::option::Option::None,
            }
        }
    }
}

fn gen_update_from_arg_matches(
    name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
    parent_attribute: &Attrs,
) -> TokenStream {
    use syn::Fields::*;

    let (flatten, variants): (Vec<_>, Vec<_>) = variants
        .iter()
        .filter_map(|variant| {
            let attrs = Attrs::from_variant(
                variant,
                parent_attribute.casing(),
                parent_attribute.env_casing(),
            );

            match &*attrs.kind() {
                // Fallback to `from_arg_matches`
                Kind::ExternalSubcommand => None,
                _ => Some((variant, attrs)),
            }
        })
        .partition(|(_, attrs)| {
            let kind = attrs.kind();
            matches!(&*kind, Kind::Flatten)
        });

    let subcommands = variants.iter().map(|(variant, attrs)| {
        let sub_name = attrs.cased_name();
        let variant_name = &variant.ident;
        let (pattern, updater) = match variant.fields {
            Named(ref fields) => {
                let (fields, update): (Vec<_>, Vec<_>) = fields
                    .named
                    .iter()
                    .map(|field| {
                        let attrs = Attrs::from_field(
                            field,
                            parent_attribute.casing(),
                            parent_attribute.env_casing(),
                        );
                        let field_name = field.ident.as_ref().unwrap();
                        (
                            quote!( ref mut #field_name ),
                            args::gen_updater(&fields.named, &attrs, false),
                        )
                    })
                    .unzip();
                (quote!( { #( #fields, )* }), quote!( { #( #update )* } ))
            }
            Unit => (quote!(), quote!({})),
            Unnamed(ref fields) => {
                if fields.unnamed.len() == 1 {
                    (
                        quote!((ref mut arg)),
                        quote!(clap::FromArgMatches::update_from_arg_matches(
                            arg,
                            sub_arg_matches
                        )),
                    )
                } else {
                    abort_call_site!("{}: tuple enums are not supported", variant.ident)
                }
            }
        };

        quote! {
            (#sub_name, #name :: #variant_name #pattern) => {
                let arg_matches = sub_arg_matches;
                #updater
            }
        }
    });

    let child_subcommands = flatten.iter().map(|(variant, _attrs)| {
        let variant_name = &variant.ident;
        match variant.fields {
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                quote! {
                    if <#ty as clap::Subcommand>::has_subcommand(name) {
                        if let #name :: #variant_name (child) = s {
                            <#ty as clap::FromArgMatches>::update_from_arg_matches(child, arg_matches);
                            return;
                        }
                    }
                }
            }
            _ => abort!(
                variant,
                "`flatten` is usable only with single-typed tuple variants"
            ),
        }
    });

    quote! {
        fn update_from_arg_matches<'b>(
            &mut self,
            arg_matches: &clap::ArgMatches,
        ) {
            if let Some((name, sub_arg_matches)) = arg_matches.subcommand() {
                match (name, self) {
                    #( #subcommands ),*
                    (other_name, s) => {
                        #( #child_subcommands )*
                        if let Some(sub) = <Self as clap::FromArgMatches>::from_arg_matches(arg_matches) {
                            *s = sub;
                        }
                    }
                }
            }
        }
    }
}
