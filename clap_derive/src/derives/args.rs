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
    attrs::{Attrs, Kind, Name, ParserKind, DEFAULT_CASING, DEFAULT_ENV_CASING},
    dummies,
    utils::{sub_type, subty_if_name, Sp, Ty},
};

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::{quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Data, DataStruct,
    DeriveInput, Field, Fields, Type,
};

pub fn derive_args(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    dummies::args(ident);

    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => gen_for_struct(ident, &fields.named, &input.attrs),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => gen_for_struct(ident, &Punctuated::<Field, Comma>::new(), &input.attrs),
        _ => abort_call_site!("`#[derive(Args)]` only supports non-tuple structs"),
    }
}

pub fn gen_for_struct(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
    attrs: &[Attribute],
) -> TokenStream {
    let from_arg_matches = gen_from_arg_matches_for_struct(struct_name, fields, attrs);

    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Derived(struct_name.clone()),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );
    let app_var = Ident::new("app", Span::call_site());
    let augmentation = gen_augment(fields, &app_var, &attrs, false);
    let augmentation_update = gen_augment(fields, &app_var, &attrs, true);

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
        impl clap::Args for #struct_name {
            fn augment_args<'b>(#app_var: clap::App<'b>) -> clap::App<'b> {
                #augmentation
            }
            fn augment_args_for_update<'b>(#app_var: clap::App<'b>) -> clap::App<'b> {
                #augmentation_update
            }
        }
    }
}

pub fn gen_from_arg_matches_for_struct(
    struct_name: &Ident,
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
        impl clap::FromArgMatches for #struct_name {
            fn from_arg_matches(arg_matches: &clap::ArgMatches) -> Option<Self> {
                let v = #struct_name #constructor;
                Some(v)
            }

            fn update_from_arg_matches(&mut self, arg_matches: &clap::ArgMatches) {
                #updater
            }
        }
    }
}

/// Generate a block of code to add arguments/subcommands corresponding to
/// the `fields` to an app.
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
                Some(quote_spanned! { kind.span()=>
                    let #app_var = <#ty as clap::Args>::augment_args(#app_var);
                })
            }
            Kind::Arg(ty) => {
                let convert_type = match **ty {
                    Ty::Vec | Ty::Option => sub_type(&field.ty).unwrap_or(&field.ty),
                    Ty::OptionOption | Ty::OptionVec => {
                        sub_type(&field.ty).and_then(sub_type).unwrap_or(&field.ty)
                    }
                    _ => &field.ty,
                };

                let occurrences = *attrs.parser().kind == ParserKind::FromOccurrences;
                let flag = *attrs.parser().kind == ParserKind::FromFlag;

                let parser = attrs.parser();
                let func = &parser.func;

                let validator = match *parser.kind {
                    _ if attrs.is_enum() => quote!(),
                    ParserKind::TryFromStr => quote_spanned! { func.span()=>
                        .validator(|s| {
                            #func(s)
                            .map(|_: #convert_type| ())
                        })
                    },
                    ParserKind::TryFromOsStr => quote_spanned! { func.span()=>
                        .validator_os(|s| #func(s).map(|_: #convert_type| ()))
                    },
                    _ => quote!(),
                };

                let value_name = attrs.value_name();

                let modifier = match **ty {
                    Ty::Bool => quote!(),

                    Ty::Option => {
                        let mut possible_values = quote!();

                        if attrs.is_enum() {
                            if let Some(subty) = subty_if_name(&field.ty, "Option") {
                                possible_values = gen_arg_enum_possible_values(subty);
                            }
                        };

                        quote_spanned! { ty.span()=>
                            .takes_value(true)
                            .value_name(#value_name)
                            #possible_values
                            #validator
                        }
                    }

                    Ty::OptionOption => quote_spanned! { ty.span()=>
                        .takes_value(true)
                        .value_name(#value_name)
                        .min_values(0)
                        .max_values(1)
                        .multiple_values(false)
                        #validator
                    },

                    Ty::OptionVec => quote_spanned! { ty.span()=>
                        .takes_value(true)
                        .value_name(#value_name)
                        .multiple_values(true)
                        .min_values(0)
                        #validator
                    },

                    Ty::Vec => {
                        let mut possible_values = quote!();

                        if attrs.is_enum() {
                            if let Some(subty) = subty_if_name(&field.ty, "Vec") {
                                possible_values = gen_arg_enum_possible_values(subty);
                            }
                        };

                        quote_spanned! { ty.span()=>
                            .takes_value(true)
                            .value_name(#value_name)
                            .multiple_values(true)
                            #possible_values
                            #validator
                        }
                    }

                    Ty::Other if occurrences => quote_spanned! { ty.span()=>
                        .multiple_occurrences(true)
                    },

                    Ty::Other if flag => quote_spanned! { ty.span()=>
                        .takes_value(false)
                        .multiple_values(false)
                    },

                    Ty::Other => {
                        let required = !attrs.has_method("default_value") && !override_required;
                        let mut possible_values = quote!();

                        if attrs.is_enum() {
                            possible_values = gen_arg_enum_possible_values(&field.ty);
                        };

                        quote_spanned! { ty.span()=>
                            .takes_value(true)
                            .value_name(#value_name)
                            .required(#required)
                            #possible_values
                            #validator
                        }
                    }
                };

                let name = attrs.cased_name();
                let methods = attrs.field_methods();

                Some(quote_spanned! { field.span()=>
                    let #app_var = #app_var.arg(
                        clap::Arg::new(#name)
                            #modifier
                            #methods
                    );
                })
            }
        }
    });

    let app_methods = parent_attribute.top_level_methods();
    let version = parent_attribute.version();
    quote! {{
        #( #args )*
        let #app_var = #app_var#app_methods;
        #subcmd
        #app_var#version
    }}
}

fn gen_arg_enum_possible_values(ty: &Type) -> TokenStream {
    quote_spanned! { ty.span()=>
        .possible_values(&<#ty as clap::ArgEnum>::VARIANTS)
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
        let arg_matches = quote! { arg_matches };
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
                    _ => quote_spanned!( ty.span()=> .expect("app should verify subcommand is required") ),
                };
                quote_spanned! { kind.span()=>
                    #field_name: {
                        <#subcmd_type as clap::FromArgMatches>::from_arg_matches(#arg_matches)
                        #unwrapper
                    }
                }
            }

            Kind::Flatten => quote_spanned! { kind.span()=>
                #field_name: clap::FromArgMatches::from_arg_matches(#arg_matches).unwrap()
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
        let arg_matches = quote! { arg_matches };

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
                    <#subcmd_type as clap::FromArgMatches>::update_from_arg_matches(#field_name, #arg_matches);
                };

                let updater = match **ty {
                    Ty::Option => quote_spanned! { kind.span()=>
                        if let Some(#field_name) = #field_name.as_mut() {
                            #updater
                        } else {
                            *#field_name = <#subcmd_type as clap::FromArgMatches>::from_arg_matches(
                                #arg_matches
                            )
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
                    clap::FromArgMatches::update_from_arg_matches(#field_name, #arg_matches);
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
    // Use `quote!` to give this identifier the same hygiene
    // as the `arg_matches` parameter definition. This
    // allows us to refer to `arg_matches` within a `quote_spanned` block
    let arg_matches = quote! { arg_matches };

    let field_value = match **ty {
        Ty::Bool => {
            if update.is_some() {
                quote_spanned! { ty.span()=>
                    *#field_name || #arg_matches.is_present(#name)
                }
            } else {
                quote_spanned! { ty.span()=>
                    #arg_matches.is_present(#name)
                }
            }
        }

        Ty::Option => {
            if attrs.is_enum() {
                if let Some(subty) = subty_if_name(&field.ty, "Option") {
                    parse = gen_arg_enum_parse(subty, attrs);
                }
            }

            quote_spanned! { ty.span()=>
                #arg_matches.#value_of(#name)
                    .map(#parse)
            }
        }

        Ty::OptionOption => quote_spanned! { ty.span()=>
            if #arg_matches.is_present(#name) {
                Some(#arg_matches.#value_of(#name).map(#parse))
            } else {
                None
            }
        },

        Ty::OptionVec => quote_spanned! { ty.span()=>
            if #arg_matches.is_present(#name) {
                Some(#arg_matches.#values_of(#name)
                     .map(|v| v.map(#parse).collect())
                     .unwrap_or_else(Vec::new))
            } else {
                None
            }
        },

        Ty::Vec => {
            if attrs.is_enum() {
                if let Some(subty) = subty_if_name(&field.ty, "Vec") {
                    parse = gen_arg_enum_parse(subty, attrs);
                }
            }

            quote_spanned! { ty.span()=>
                #arg_matches.#values_of(#name)
                    .map(|v| v.map(#parse).collect())
                    .unwrap_or_else(Vec::new)
            }
        }

        Ty::Other if occurrences => quote_spanned! { ty.span()=>
            #parse(#arg_matches.#value_of(#name))
        },

        Ty::Other if flag => quote_spanned! { ty.span()=>
            #parse(#arg_matches.is_present(#name))
        },

        Ty::Other => {
            if attrs.is_enum() {
                parse = gen_arg_enum_parse(&field.ty, attrs);
            }

            quote_spanned! { ty.span()=>
                #arg_matches.#value_of(#name)
                    .map(#parse)
                    .expect("app should verify arg is required")
            }
        }
    };

    if let Some(access) = update {
        quote_spanned! { field.span()=>
            if #arg_matches.is_present(#name) {
                #access
                *#field_name = #field_value
            }
        }
    } else {
        quote_spanned!(field.span()=> #field_name: #field_value )
    }
}

fn gen_arg_enum_parse(ty: &Type, attrs: &Attrs) -> TokenStream {
    let ci = attrs.case_insensitive();

    quote_spanned! { ty.span()=>
        |s| <#ty as clap::ArgEnum>::from_str(s, #ci).expect("app should verify the choice was valid")
    }
}
