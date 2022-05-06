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
    attrs::{Attrs, Kind, Name, DEFAULT_CASING, DEFAULT_ENV_CASING},
    derives::args,
    dummies,
    utils::{is_simple_ty, subty_if_name, Sp},
};

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataEnum, DeriveInput,
    FieldsUnnamed, Generics, Token, Variant,
};

pub fn derive_subcommand(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    dummies::subcommand(ident);

    match input.data {
        Data::Enum(ref e) => gen_for_enum(ident, &input.generics, &input.attrs, e),
        _ => abort_call_site!("`#[derive(Subcommand)]` only supports enums"),
    }
}

pub fn gen_for_enum(
    enum_name: &Ident,
    generics: &Generics,
    attrs: &[Attribute],
    e: &DataEnum,
) -> TokenStream {
    let from_arg_matches = gen_from_arg_matches_for_enum(enum_name, generics, attrs, e);

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

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #from_arg_matches

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
        impl #impl_generics clap::Subcommand for #enum_name #ty_generics #where_clause {
            fn augment_subcommands <'b>(__clap_app: clap::Command<'b>) -> clap::Command<'b> {
                #augmentation
            }
            fn augment_subcommands_for_update <'b>(__clap_app: clap::Command<'b>) -> clap::Command<'b> {
                #augmentation_update
            }
            fn has_subcommand(__clap_name: &str) -> bool {
                #has_subcommand
            }
        }
    }
}

fn gen_from_arg_matches_for_enum(
    name: &Ident,
    generics: &Generics,
    attrs: &[Attribute],
    e: &DataEnum,
) -> TokenStream {
    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Derived(name.clone()),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );

    let from_arg_matches = gen_from_arg_matches(name, &e.variants, &attrs);
    let update_from_arg_matches = gen_update_from_arg_matches(name, &e.variants, &attrs);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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
        impl #impl_generics clap::FromArgMatches for #name #ty_generics #where_clause {
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

    let app_var = Ident::new("__clap_app", Span::call_site());

    let subcommands: Vec<_> = variants
        .iter()
        .filter_map(|variant| {
            let attrs = Attrs::from_variant(
                variant,
                parent_attribute.casing(),
                parent_attribute.env_casing(),
            );
            let kind = attrs.kind();

            match &*kind {
                Kind::Skip(_) => None,

                Kind::ExternalSubcommand => {
                    let ty = match variant.fields {
                        Unnamed(ref fields) if fields.unnamed.len() == 1 => &fields.unnamed[0].ty,

                        _ => abort!(
                            variant,
                            "The enum variant marked with `external_subcommand` must be \
                             a single-typed tuple, and the type must be either `Vec<String>` \
                             or `Vec<OsString>`."
                        ),
                    };
                    let subcommand = match subty_if_name(ty, "Vec") {
                        Some(subty) => {
                            if is_simple_ty(subty, "OsString") {
                                quote_spanned! { kind.span()=>
                                    let #app_var = #app_var.allow_external_subcommands(true).allow_invalid_utf8_for_external_subcommands(true);
                                }
                            } else {
                                quote_spanned! { kind.span()=>
                                    let #app_var = #app_var.allow_external_subcommands(true);
                                }
                            }
                        }

                        None => abort!(
                            ty.span(),
                            "The type must be `Vec<_>` \
                             to be used with `external_subcommand`."
                        ),
                    };
                    Some(subcommand)
                }

                Kind::Flatten => match variant.fields {
                    Unnamed(FieldsUnnamed { ref unnamed, .. }) if unnamed.len() == 1 => {
                        let ty = &unnamed[0];
                        let old_heading_var = format_ident!("__clap_old_heading");
                        let next_help_heading = attrs.next_help_heading();
                        let next_display_order = attrs.next_display_order();
                        let subcommand = if override_required {
                            quote! {
                                let #old_heading_var = #app_var.get_next_help_heading();
                                let #app_var = #app_var #next_help_heading #next_display_order;
                                let #app_var = <#ty as clap::Subcommand>::augment_subcommands_for_update(#app_var);
                                let #app_var = #app_var.next_help_heading(#old_heading_var);
                            }
                        } else {
                            quote! {
                                let #old_heading_var = #app_var.get_next_help_heading();
                                let #app_var = #app_var #next_help_heading #next_display_order;
                                let #app_var = <#ty as clap::Subcommand>::augment_subcommands(#app_var);
                                let #app_var = #app_var.next_help_heading(#old_heading_var);
                            }
                        };
                        Some(subcommand)
                    }
                    _ => abort!(
                        variant,
                        "`flatten` is usable only with single-typed tuple variants"
                    ),
                },

                Kind::Subcommand(_) => {
                    let subcommand_var = Ident::new("__clap_subcommand", Span::call_site());
                    let arg_block = match variant.fields {
                        Named(_) => {
                            abort!(variant, "non single-typed tuple enums are not supported")
                        }
                        Unit => quote!( #subcommand_var ),
                        Unnamed(FieldsUnnamed { ref unnamed, .. }) if unnamed.len() == 1 => {
                            let ty = &unnamed[0];
                            if override_required {
                                quote_spanned! { ty.span()=>
                                    {
                                        <#ty as clap::Subcommand>::augment_subcommands_for_update(#subcommand_var)
                                    }
                                }
                            } else {
                                quote_spanned! { ty.span()=>
                                    {
                                        <#ty as clap::Subcommand>::augment_subcommands(#subcommand_var)
                                    }
                                }
                            }
                        }
                        Unnamed(..) => {
                            abort!(variant, "non single-typed tuple enums are not supported")
                        }
                    };

                    let name = attrs.cased_name();
                    let initial_app_methods = attrs.initial_top_level_methods();
                    let final_from_attrs = attrs.final_top_level_methods();
                    let subcommand = quote! {
                        let #app_var = #app_var.subcommand({
                            let #subcommand_var = clap::Command::new(#name);
                            let #subcommand_var = #subcommand_var #initial_app_methods;
                            let #subcommand_var = #arg_block;
                            #[allow(deprecated)]
                            let #subcommand_var = #subcommand_var.setting(clap::AppSettings::SubcommandRequiredElseHelp);
                            #subcommand_var #final_from_attrs
                        });
                    };
                    Some(subcommand)
                }

                _ => {
                    let subcommand_var = Ident::new("__clap_subcommand", Span::call_site());
                    let sub_augment = match variant.fields {
                        Named(ref fields) => {
                            // Defer to `gen_augment` for adding cmd methods
                            args::gen_augment(&fields.named, &subcommand_var, &attrs, override_required)
                        }
                        Unit => {
                            let arg_block = quote!( #subcommand_var );
                            let initial_app_methods = attrs.initial_top_level_methods();
                            let final_from_attrs = attrs.final_top_level_methods();
                            quote! {
                                let #subcommand_var = #subcommand_var #initial_app_methods;
                                let #subcommand_var = #arg_block;
                                #subcommand_var #final_from_attrs
                            }
                        },
                        Unnamed(FieldsUnnamed { ref unnamed, .. }) if unnamed.len() == 1 => {
                            let ty = &unnamed[0];
                            let arg_block = if override_required {
                                quote_spanned! { ty.span()=>
                                    {
                                        <#ty as clap::Args>::augment_args_for_update(#subcommand_var)
                                    }
                                }
                            } else {
                                quote_spanned! { ty.span()=>
                                    {
                                        <#ty as clap::Args>::augment_args(#subcommand_var)
                                    }
                                }
                            };
                            let initial_app_methods = attrs.initial_top_level_methods();
                            let final_from_attrs = attrs.final_top_level_methods();
                            quote! {
                                let #subcommand_var = #subcommand_var #initial_app_methods;
                                let #subcommand_var = #arg_block;
                                #subcommand_var #final_from_attrs
                            }
                        }
                        Unnamed(..) => {
                            abort!(variant, "non single-typed tuple enums are not supported")
                        }
                    };

                    let name = attrs.cased_name();
                    let subcommand = quote! {
                        let #app_var = #app_var.subcommand({
                            let #subcommand_var = clap::Command::new(#name);
                            #sub_augment
                        });
                    };
                    Some(subcommand)
                }
            }
        })
        .collect();

    let initial_app_methods = parent_attribute.initial_top_level_methods();
    let final_app_methods = parent_attribute.final_top_level_methods();
    quote! {
            let #app_var = #app_var #initial_app_methods;
            #( #subcommands )*;
            #app_var #final_app_methods
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

    let subcommands = variants.iter().map(|(_variant, attrs)| {
        let sub_name = attrs.cased_name();
        quote! {
            if #sub_name == __clap_name {
                return true
            }
        }
    });
    let child_subcommands = flatten_variants
        .iter()
        .map(|(variant, _attrs)| match variant.fields {
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                quote! {
                    if <#ty as clap::Subcommand>::has_subcommand(__clap_name) {
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
            #( #subcommands )*

            #( #child_subcommands )else*

            false
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

    let subcommand_name_var = format_ident!("__clap_name");
    let sub_arg_matches_var = format_ident!("__clap_sub_arg_matches");
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
                        "The enum variant marked with `external_subcommand` must be \
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

    let subcommands = variants.iter().map(|(variant, attrs)| {
        let sub_name = attrs.cased_name();
        let variant_name = &variant.ident;
        let constructor_block = match variant.fields {
            Named(ref fields) => args::gen_constructor(&fields.named, attrs),
            Unit => quote!(),
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                quote!( ( <#ty as clap::FromArgMatches>::from_arg_matches(__clap_arg_matches)? ) )
            }
            Unnamed(..) => abort_call_site!("{}: tuple enums are not supported", variant.ident),
        };

        if cfg!(feature = "unstable-v4") {
            quote! {
                if #sub_name == #subcommand_name_var && !#sub_arg_matches_var.is_present("") {
                    return ::std::result::Result::Ok(#name :: #variant_name #constructor_block)
                }
            }
        } else {
            quote! {
                if #sub_name == #subcommand_name_var {
                    return ::std::result::Result::Ok(#name :: #variant_name #constructor_block)
                }
            }
        }
    });
    let child_subcommands = flatten_variants.iter().map(|(variant, _attrs)| {
        let variant_name = &variant.ident;
        match variant.fields {
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                quote! {
                    if <#ty as clap::Subcommand>::has_subcommand(__clap_name) {
                        let __clap_res = <#ty as clap::FromArgMatches>::from_arg_matches(__clap_arg_matches)?;
                        return ::std::result::Result::Ok(#name :: #variant_name (__clap_res));
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
                ::std::result::Result::Ok(#name::#var_name(
                    ::std::iter::once(#str_ty::from(#subcommand_name_var))
                    .chain(
                        #sub_arg_matches_var.#values_of("").into_iter().flatten().map(#str_ty::from)
                    )
                    .collect::<::std::vec::Vec<_>>()
                ))
        },

        None => quote! {
            ::std::result::Result::Err(clap::Error::raw(clap::ErrorKind::UnrecognizedSubcommand, format!("The subcommand '{}' wasn't recognized", #subcommand_name_var)))
        },
    };

    quote! {
        fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error> {
            if let Some((#subcommand_name_var, #sub_arg_matches_var)) = __clap_arg_matches.subcommand() {
                {
                    let __clap_arg_matches = #sub_arg_matches_var;
                    #( #subcommands )*
                }

                #( #child_subcommands )else*

                #wildcard
            } else {
                ::std::result::Result::Err(clap::Error::raw(clap::ErrorKind::MissingSubcommand, "A subcommand is required but one was not provided."))
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
                        quote!((ref mut __clap_arg)),
                        quote!(clap::FromArgMatches::update_from_arg_matches(
                            __clap_arg,
                            __clap_sub_arg_matches
                        )?),
                    )
                } else {
                    abort_call_site!("{}: tuple enums are not supported", variant.ident)
                }
            }
        };

        quote! {
            #name :: #variant_name #pattern if #sub_name == __clap_name => {
                let __clap_arg_matches = __clap_sub_arg_matches;
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
                    if <#ty as clap::Subcommand>::has_subcommand(__clap_name) {
                        if let #name :: #variant_name (child) = s {
                            <#ty as clap::FromArgMatches>::update_from_arg_matches(child, __clap_arg_matches)?;
                            return ::std::result::Result::Ok(());
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
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            if let Some((__clap_name, __clap_sub_arg_matches)) = __clap_arg_matches.subcommand() {
                match self {
                    #( #subcommands ),*
                    s => {
                        #( #child_subcommands )*
                        *s = <Self as clap::FromArgMatches>::from_arg_matches(__clap_arg_matches)?;
                    }
                }
            }
            ::std::result::Result::Ok(())
        }
    }
}
