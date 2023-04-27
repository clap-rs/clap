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
    attrs::{Attrs, Kind, Name, ParserKind, DEFAULT_CASING, DEFAULT_ENV_CASING},
    dummies,
    utils::{inner_type, is_simple_ty, sub_type, Sp, Ty},
};

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Data, DataStruct,
    DeriveInput, Field, Fields, Generics, Type,
};

pub fn derive_args(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    dummies::args(ident);

    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => gen_for_struct(ident, &input.generics, &fields.named, &input.attrs),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => gen_for_struct(
            ident,
            &input.generics,
            &Punctuated::<Field, Comma>::new(),
            &input.attrs,
        ),
        _ => abort_call_site!("`#[derive(Args)]` only supports non-tuple structs"),
    }
}

pub fn gen_for_struct(
    struct_name: &Ident,
    generics: &Generics,
    fields: &Punctuated<Field, Comma>,
    attrs: &[Attribute],
) -> TokenStream {
    let from_arg_matches = gen_from_arg_matches_for_struct(struct_name, generics, fields, attrs);

    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Derived(struct_name.clone()),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );
    let app_var = Ident::new("__clap_app", Span::call_site());
    let augmentation = gen_augment(fields, &app_var, &attrs, false);
    let augmentation_update = gen_augment(fields, &app_var, &attrs, true);

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
            clippy::almost_swapped,
        )]
        impl #impl_generics clap::Args for #struct_name #ty_generics #where_clause {
            fn augment_args<'b>(#app_var: clap::Command<'b>) -> clap::Command<'b> {
                #augmentation
            }
            fn augment_args_for_update<'b>(#app_var: clap::Command<'b>) -> clap::Command<'b> {
                #augmentation_update
            }
        }
    }
}

pub fn gen_from_arg_matches_for_struct(
    struct_name: &Ident,
    generics: &Generics,
    fields: &Punctuated<Field, Comma>,
    attrs: &[Attribute],
) -> TokenStream {
    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Derived(struct_name.clone()),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );

    let constructor = gen_constructor(fields, &attrs);
    let updater = gen_updater(fields, &attrs, true);
    let raw_deprecated = raw_deprecated();

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
            clippy::almost_swapped,
        )]
        impl #impl_generics clap::FromArgMatches for #struct_name #ty_generics #where_clause {
            fn from_arg_matches(__clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }

            fn from_arg_matches_mut(__clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<Self, clap::Error> {
                #raw_deprecated
                let v = #struct_name #constructor;
                ::std::result::Result::Ok(v)
            }

            fn update_from_arg_matches(&mut self, __clap_arg_matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }

            fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut clap::ArgMatches) -> ::std::result::Result<(), clap::Error> {
                #raw_deprecated
                #updater
                ::std::result::Result::Ok(())
            }
        }
    }
}

/// Generate a block of code to add arguments/subcommands corresponding to
/// the `fields` to an cmd.
pub fn gen_augment(
    fields: &Punctuated<Field, Comma>,
    app_var: &Ident,
    parent_attribute: &Attrs,
    override_required: bool,
) -> TokenStream {
    let mut subcmds = fields.iter().filter_map(|field| {
        let attrs = Attrs::from_field(
            field,
            parent_attribute.casing(),
            parent_attribute.env_casing(),
        );
        let kind = attrs.kind();
        if let Kind::Subcommand(ty) = &*kind {
            let subcmd_type = match (**ty, sub_type(&field.ty)) {
                (Ty::Option, Some(sub_type)) => sub_type,
                _ => &field.ty,
            };
            let required = if **ty == Ty::Option {
                quote!()
            } else {
                quote_spanned! { kind.span()=>
                    #[allow(deprecated)]
                    let #app_var = #app_var.setting(
                        clap::AppSettings::SubcommandRequiredElseHelp
                    );
                }
            };

            let span = field.span();
            let ts = if override_required {
                quote! {
                    let #app_var = <#subcmd_type as clap::Subcommand>::augment_subcommands_for_update( #app_var );
                }
            } else{
                quote! {
                    let #app_var = <#subcmd_type as clap::Subcommand>::augment_subcommands( #app_var );
                    #required
                }
            };
            Some((span, ts))
        } else {
            None
        }
    });
    let subcmd = subcmds.next().map(|(_, ts)| ts);
    if let Some((span, _)) = subcmds.next() {
        abort!(
            span,
            "multiple subcommand sets are not allowed, that's the second"
        );
    }

    let args = fields.iter().filter_map(|field| {
        let attrs = Attrs::from_field(
            field,
            parent_attribute.casing(),
            parent_attribute.env_casing(),
        );
        let kind = attrs.kind();
        match &*kind {
            Kind::Subcommand(_)
            | Kind::Skip(_)
            | Kind::FromGlobal(_)
            | Kind::ExternalSubcommand => None,
            Kind::Flatten => {
                let ty = &field.ty;
                let old_heading_var = format_ident!("__clap_old_heading");
                let next_help_heading = attrs.next_help_heading();
                let next_display_order = attrs.next_display_order();
                if override_required {
                    Some(quote_spanned! { kind.span()=>
                        let #old_heading_var = #app_var.get_next_help_heading();
                        let #app_var = #app_var #next_help_heading #next_display_order;
                        let #app_var = <#ty as clap::Args>::augment_args_for_update(#app_var);
                        let #app_var = #app_var.next_help_heading(#old_heading_var);
                    })
                } else {
                    Some(quote_spanned! { kind.span()=>
                        let #old_heading_var = #app_var.get_next_help_heading();
                        let #app_var = #app_var #next_help_heading #next_display_order;
                        let #app_var = <#ty as clap::Args>::augment_args(#app_var);
                        let #app_var = #app_var.next_help_heading(#old_heading_var);
                    })
                }
            }
            Kind::Arg(ty) => {
                let convert_type = inner_type(&field.ty);

                let parser = attrs.parser(&field.ty);

                let value_parser = attrs.value_parser(&field.ty);
                let action = attrs.action(&field.ty);
                let func = &parser.func;

                let mut occurrences = false;
                let mut flag = false;
                let validator = match *parser.kind {
                    _ if attrs.ignore_parser() || attrs.is_enum() => quote!(),
                    ParserKind::TryFromStr => quote_spanned! { func.span()=>
                        .validator(|s| {
                            #func(s)
                            .map(|_: #convert_type| ())
                        })
                    },
                    ParserKind::TryFromOsStr => quote_spanned! { func.span()=>
                        .validator_os(|s| #func(s).map(|_: #convert_type| ()))
                    },
                    ParserKind::FromStr | ParserKind::FromOsStr => quote!(),
                    ParserKind::FromFlag => {
                        flag = true;
                        quote!()
                    }
                    ParserKind::FromOccurrences => {
                        occurrences = true;
                        quote!()
                    }
                };
                let parse_deprecation = match *parser.kind {
                    _ if !attrs.explicit_parser() || cfg!(not(feature = "deprecated")) => quote!(),
                    ParserKind::FromStr => quote_spanned! { func.span()=>
                        #[deprecated(since = "3.2.0", note = "Replaced with `#[clap(value_parser = ...)]`")]
                        fn parse_from_str() {
                        }
                        parse_from_str();
                    },
                    ParserKind::TryFromStr => quote_spanned! { func.span()=>
                        #[deprecated(since = "3.2.0", note = "Replaced with `#[clap(value_parser = ...)]`")]
                        fn parse_try_from_str() {
                        }
                        parse_try_from_str();
                    },
                    ParserKind::FromOsStr => quote_spanned! { func.span()=>
                        #[deprecated(since = "3.2.0", note = "Replaced with `#[clap(value_parser)]` for `PathBuf` or `#[clap(value_parser = ...)]` with a custom `TypedValueParser`")]
                        fn parse_from_os_str() {
                        }
                        parse_from_os_str();
                    },
                    ParserKind::TryFromOsStr => quote_spanned! { func.span()=>
                        #[deprecated(since = "3.2.0", note = "Replaced with `#[clap(value_parser = ...)]` with a custom `TypedValueParser`")]
                        fn parse_try_from_os_str() {
                        }
                        parse_try_from_os_str();
                    },
                    ParserKind::FromFlag => quote_spanned! { func.span()=>
                        #[deprecated(since = "3.2.0", note = "Replaced with `#[clap(action = ArgAction::SetTrue)]`")]
                        fn parse_from_flag() {
                        }
                        parse_from_flag();
                    },
                    ParserKind::FromOccurrences => quote_spanned! { func.span()=>
                        #[deprecated(since = "3.2.0", note = "Replaced with `#[clap(action = ArgAction::Count)]` with a field type of `u8`")]
                        fn parse_from_occurrences() {
                        }
                        parse_from_occurrences();
                    },
                };

                let value_name = attrs.value_name();
                let possible_values = if attrs.is_enum() && !attrs.ignore_parser() {
                    gen_value_enum_possible_values(convert_type)
                } else {
                    quote!()
                };

                let implicit_methods = match **ty {
                    Ty::Option => {
                        quote_spanned! { ty.span()=>
                            .takes_value(true)
                            .value_name(#value_name)
                            #possible_values
                            #validator
                            #value_parser
                            #action
                        }
                    }

                    Ty::OptionOption => quote_spanned! { ty.span()=>
                        .takes_value(true)
                        .value_name(#value_name)
                        .min_values(0)
                        .max_values(1)
                        .multiple_values(false)
                        #possible_values
                        #validator
                        #value_parser
                        #action
                    },

                    Ty::OptionVec => {
                        if attrs.ignore_parser() {
                            if attrs.is_positional() {
                                quote_spanned! { ty.span()=>
                                    .takes_value(true)
                                    .value_name(#value_name)
                                    .multiple_values(true)  // action won't be sufficient for getting multiple
                                    #possible_values
                                    #validator
                                    #value_parser
                                    #action
                                }
                            } else {
                                quote_spanned! { ty.span()=>
                                    .takes_value(true)
                                    .value_name(#value_name)
                                    #possible_values
                                    #validator
                                    #value_parser
                                    #action
                                }
                            }
                        } else {
                            quote_spanned! { ty.span()=>
                                .takes_value(true)
                                .value_name(#value_name)
                                .multiple_occurrences(true)
                                #possible_values
                                #validator
                                #value_parser
                                #action
                            }
                        }
                    }

                    Ty::Vec => {
                        if attrs.ignore_parser() {
                            if attrs.is_positional() {
                                quote_spanned! { ty.span()=>
                                    .takes_value(true)
                                    .value_name(#value_name)
                                    .multiple_values(true)  // action won't be sufficient for getting multiple
                                    #possible_values
                                    #validator
                                    #value_parser
                                    #action
                                }
                            } else {
                                quote_spanned! { ty.span()=>
                                    .takes_value(true)
                                    .value_name(#value_name)
                                    #possible_values
                                    #validator
                                    #value_parser
                                    #action
                                }
                            }
                        } else {
                            quote_spanned! { ty.span()=>
                                .takes_value(true)
                                .value_name(#value_name)
                                .multiple_occurrences(true)
                                #possible_values
                                #validator
                                #value_parser
                                #action
                            }
                        }
                    }

                    Ty::Other if occurrences => quote_spanned! { ty.span()=>
                        .multiple_occurrences(true)
                    },

                    Ty::Other if flag => quote_spanned! { ty.span()=>
                        .takes_value(false)
                    },

                    Ty::Other => {
                        let required = attrs.find_default_method().is_none() && !override_required;
                        // `ArgAction::takes_values` is assuming `ArgAction::default_value` will be
                        // set though that won't always be true but this should be good enough,
                        // otherwise we'll report an "arg required" error when unwrapping.
                        let action_value = action.args();
                        quote_spanned! { ty.span()=>
                            .takes_value(true)
                            .value_name(#value_name)
                            .required(#required && #action_value.takes_values())
                            #possible_values
                            #validator
                            #value_parser
                            #action
                        }
                    }
                };

                let id = attrs.id();
                let explicit_methods = attrs.field_methods(true);

                Some(quote_spanned! { field.span()=>
                    let #app_var = #app_var.arg({
                        #parse_deprecation

                        #[allow(deprecated)]
                        let arg = clap::Arg::new(#id)
                            #implicit_methods;

                        let arg = arg
                            #explicit_methods;
                        arg
                    });
                })
            }
        }
    });

    let initial_app_methods = parent_attribute.initial_top_level_methods();
    let final_app_methods = parent_attribute.final_top_level_methods();
    quote! {{
        let #app_var = #app_var #initial_app_methods;
        #( #args )*
        #subcmd
        #app_var #final_app_methods
    }}
}

fn gen_value_enum_possible_values(ty: &Type) -> TokenStream {
    quote_spanned! { ty.span()=>
        .possible_values(<#ty as clap::ValueEnum>::value_variants().iter().filter_map(clap::ValueEnum::to_possible_value))
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
        let arg_matches = format_ident!("__clap_arg_matches");
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
                match **ty {
                    Ty::Option => {
                        quote_spanned! { kind.span()=>
                            #field_name: {
                                if #arg_matches.subcommand_name().map(<#subcmd_type as clap::Subcommand>::has_subcommand).unwrap_or(false) {
                                    Some(<#subcmd_type as clap::FromArgMatches>::from_arg_matches_mut(#arg_matches)?)
                                } else {
                                    None
                                }
                            }
                        }
                    },
                    _ => {
                        quote_spanned! { kind.span()=>
                            #field_name: {
                                <#subcmd_type as clap::FromArgMatches>::from_arg_matches_mut(#arg_matches)?
                            }
                        }
                    },
                }
            }

            Kind::Flatten => quote_spanned! { kind.span()=>
                #field_name: clap::FromArgMatches::from_arg_matches_mut(#arg_matches)?
            },

            Kind::Skip(val) => match val {
                None => quote_spanned!(kind.span()=> #field_name: Default::default()),
                Some(val) => quote_spanned!(kind.span()=> #field_name: (#val).into()),
            },

            Kind::Arg(ty) | Kind::FromGlobal(ty) => {
                gen_parsers(&attrs, ty, field_name, field, None)
            }
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
        let arg_matches = format_ident!("__clap_arg_matches");

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

                let updater = quote_spanned! { ty.span()=>
                    <#subcmd_type as clap::FromArgMatches>::update_from_arg_matches_mut(#field_name, #arg_matches)?;
                };

                let updater = match **ty {
                    Ty::Option => quote_spanned! { kind.span()=>
                        if let Some(#field_name) = #field_name.as_mut() {
                            #updater
                        } else {
                            *#field_name = Some(<#subcmd_type as clap::FromArgMatches>::from_arg_matches_mut(
                                #arg_matches
                            )?);
                        }
                    },
                    _ => quote_spanned! { kind.span()=>
                        #updater
                    },
                };

                quote_spanned! { kind.span()=>
                    {
                        #access
                        #updater
                    }
                }
            }

            Kind::Flatten => quote_spanned! { kind.span()=> {
                    #access
                    clap::FromArgMatches::update_from_arg_matches_mut(#field_name, #arg_matches)?;
                }
            },

            Kind::Skip(_) => quote!(),

            Kind::Arg(ty) | Kind::FromGlobal(ty) => gen_parsers(&attrs, ty, field_name, field, Some(&access)),
        }
    });

    quote! {
        #( #fields )*
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

    let parser = attrs.parser(&field.ty);
    let func = &parser.func;
    let span = parser.kind.span();
    let convert_type = inner_type(&field.ty);
    let id = attrs.id();
    let mut flag = false;
    let mut occurrences = false;
    let (get_one, get_many, deref, mut parse) = match *parser.kind {
        _ if attrs.ignore_parser() => (
            quote_spanned!(span=> remove_one::<#convert_type>),
            quote_spanned!(span=> remove_many::<#convert_type>),
            quote!(|s| s),
            quote_spanned!(func.span()=> |s| ::std::result::Result::Ok::<_, clap::Error>(s)),
        ),
        FromOccurrences => {
            occurrences = true;
            (
                quote_spanned!(span=> occurrences_of),
                quote!(),
                quote!(|s| ::std::ops::Deref::deref(s)),
                func.clone(),
            )
        }
        FromFlag => {
            flag = true;
            (
                quote!(),
                quote!(),
                quote!(|s| ::std::ops::Deref::deref(s)),
                func.clone(),
            )
        }
        FromStr => (
            quote_spanned!(span=> get_one::<String>),
            quote_spanned!(span=> get_many::<String>),
            quote!(|s| ::std::ops::Deref::deref(s)),
            quote_spanned!(func.span()=> |s| ::std::result::Result::Ok::<_, clap::Error>(#func(s))),
        ),
        TryFromStr => (
            quote_spanned!(span=> get_one::<String>),
            quote_spanned!(span=> get_many::<String>),
            quote!(|s| ::std::ops::Deref::deref(s)),
            quote_spanned!(func.span()=> |s| #func(s).map_err(|err| clap::Error::raw(clap::ErrorKind::ValueValidation, format!("Invalid value for {}: {}", #id, err)))),
        ),
        FromOsStr => (
            quote_spanned!(span=> get_one::<::std::ffi::OsString>),
            quote_spanned!(span=> get_many::<::std::ffi::OsString>),
            quote!(|s| ::std::ops::Deref::deref(s)),
            quote_spanned!(func.span()=> |s| ::std::result::Result::Ok::<_, clap::Error>(#func(s))),
        ),
        TryFromOsStr => (
            quote_spanned!(span=> get_one::<::std::ffi::OsString>),
            quote_spanned!(span=> get_many::<::std::ffi::OsString>),
            quote!(|s| ::std::ops::Deref::deref(s)),
            quote_spanned!(func.span()=> |s| #func(s).map_err(|err| clap::Error::raw(clap::ErrorKind::ValueValidation, format!("Invalid value for {}: {}", #id, err)))),
        ),
    };
    if attrs.is_enum() && !attrs.ignore_parser() {
        let ci = attrs.ignore_case();

        parse = quote_spanned! { convert_type.span()=>
            |s| <#convert_type as clap::ValueEnum>::from_str(s, #ci).map_err(|err| clap::Error::raw(clap::ErrorKind::ValueValidation, format!("Invalid value for {}: {}", #id, err)))
        }
    }

    // Give this identifier the same hygiene
    // as the `arg_matches` parameter definition. This
    // allows us to refer to `arg_matches` within a `quote_spanned` block
    let arg_matches = format_ident!("__clap_arg_matches");

    let field_value = match **ty {
        Ty::Option => {
            quote_spanned! { ty.span()=>
                #arg_matches.#get_one(#id)
                    .map(#deref)
                    .map(#parse)
                    .transpose()?
            }
        }

        Ty::OptionOption => quote_spanned! { ty.span()=>
            if #arg_matches.contains_id(#id) {
                Some(
                    #arg_matches.#get_one(#id)
                        .map(#deref)
                        .map(#parse).transpose()?
                )
            } else {
                None
            }
        },

        Ty::OptionVec => quote_spanned! { ty.span()=>
            if #arg_matches.contains_id(#id) {
                Some(#arg_matches.#get_many(#id)
                    .map(|v| v.map(#deref).map::<::std::result::Result<#convert_type, clap::Error>, _>(#parse).collect::<::std::result::Result<Vec<_>, clap::Error>>())
                    .transpose()?
                    .unwrap_or_else(Vec::new))
            } else {
                None
            }
        },

        Ty::Vec => {
            quote_spanned! { ty.span()=>
                #arg_matches.#get_many(#id)
                    .map(|v| v.map(#deref).map::<::std::result::Result<#convert_type, clap::Error>, _>(#parse).collect::<::std::result::Result<Vec<_>, clap::Error>>())
                    .transpose()?
                    .unwrap_or_else(Vec::new)
            }
        }

        Ty::Other if occurrences => quote_spanned! { ty.span()=>
            #parse(
                #arg_matches.#get_one(#id)
            )
        },

        Ty::Other if flag => {
            if update.is_some() && is_simple_ty(&field.ty, "bool") {
                quote_spanned! { ty.span()=>
                    *#field_name || #arg_matches.is_present(#id)
                }
            } else {
                quote_spanned! { ty.span()=>
                    #parse(#arg_matches.is_present(#id))
                }
            }
        }

        Ty::Other => {
            quote_spanned! { ty.span()=>
                #arg_matches.#get_one(#id)
                    .map(#deref)
                    .ok_or_else(|| clap::Error::raw(clap::ErrorKind::MissingRequiredArgument, format!("The following required argument was not provided: {}", #id)))
                    .and_then(#parse)?
            }
        }
    };

    if let Some(access) = update {
        quote_spanned! { field.span()=>
            if #arg_matches.contains_id(#id) {
                #access
                *#field_name = #field_value
            }
        }
    } else {
        quote_spanned!(field.span()=> #field_name: #field_value )
    }
}

#[cfg(feature = "raw-deprecated")]
pub fn raw_deprecated() -> TokenStream {
    quote! {}
}

#[cfg(not(feature = "raw-deprecated"))]
pub fn raw_deprecated() -> TokenStream {
    quote! {
        #![allow(deprecated)]  // Assuming any deprecation in here will be related to a deprecation in `Args`

    }
}
