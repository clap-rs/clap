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

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::{format_ident, quote, quote_spanned};
use syn::ext::IdentExt;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Data, DataStruct, DeriveInput, Field,
    Fields, Generics, Path,
};

use crate::dummies;
use crate::item::{Item, Kind, Name};
use crate::utils::{inner_type, sub_type, Sp, Ty};

pub fn derive_args(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;

    dummies::args(ident);

    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => {
            let name = Name::Derived(ident.clone());
            let item = Item::from_args_struct(input, name);
            let fields = fields
                .named
                .iter()
                .map(|field| {
                    let item = Item::from_args_field(field, item.casing(), item.env_casing());
                    (field, item)
                })
                .collect::<Vec<_>>();
            gen_for_struct(&item, ident, &input.generics, &fields)
        }
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            let name = Name::Derived(ident.clone());
            let item = Item::from_args_struct(input, name);
            let fields = Punctuated::<Field, Comma>::new();
            let fields = fields
                .iter()
                .map(|field| {
                    let item = Item::from_args_field(field, item.casing(), item.env_casing());
                    (field, item)
                })
                .collect::<Vec<_>>();
            gen_for_struct(&item, ident, &input.generics, &fields)
        }
        _ => abort_call_site!("`#[derive(Args)]` only supports non-tuple structs"),
    }
}

pub fn gen_for_struct(
    item: &Item,
    item_name: &Ident,
    generics: &Generics,
    fields: &[(&Field, Item)],
) -> TokenStream {
    if !matches!(&*item.kind(), Kind::Command(_)) {
        abort! { item.kind().span(),
            "`{}` cannot be used with `command`",
            item.kind().name(),
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let clap_path = item.clap_path();

    let constructor = gen_constructor(fields, clap_path);
    let updater = gen_updater(fields, true, clap_path);
    let raw_deprecated = raw_deprecated();

    let app_var = Ident::new("__clap_app", Span::call_site());
    let augmentation = gen_augment(fields, &app_var, item, false, clap_path);
    let augmentation_update = gen_augment(fields, &app_var, item, true, clap_path);

    let group_id = if item.skip_group() {
        quote!(None)
    } else {
        let group_id = item.ident().unraw().to_string();
        quote!(Some(#clap_path::Id::from(#group_id)))
    };

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
        impl #impl_generics #clap_path::FromArgMatches for #item_name #ty_generics #where_clause {
            fn from_arg_matches(__clap_arg_matches: &#clap_path::ArgMatches) -> ::std::result::Result<Self, #clap_path::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }

            fn from_arg_matches_mut(__clap_arg_matches: &mut #clap_path::ArgMatches) -> ::std::result::Result<Self, #clap_path::Error> {
                #raw_deprecated
                let v = #item_name #constructor;
                ::std::result::Result::Ok(v)
            }

            fn update_from_arg_matches(&mut self, __clap_arg_matches: &#clap_path::ArgMatches) -> ::std::result::Result<(), #clap_path::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }

            fn update_from_arg_matches_mut(&mut self, __clap_arg_matches: &mut #clap_path::ArgMatches) -> ::std::result::Result<(), #clap_path::Error> {
                #raw_deprecated
                #updater
                ::std::result::Result::Ok(())
            }
        }

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
        impl #impl_generics #clap_path::Args for #item_name #ty_generics #where_clause {
            fn group_id() -> Option<#clap_path::Id> {
                #group_id
            }
            fn augment_args<'b>(#app_var: #clap_path::Command) -> #clap_path::Command {
                #augmentation
            }
            fn augment_args_for_update<'b>(#app_var: #clap_path::Command) -> #clap_path::Command {
                #augmentation_update
            }
        }
    }
}

/// Generate a block of code to add arguments/subcommands corresponding to
/// the `fields` to an cmd.
pub fn gen_augment(
    fields: &[(&Field, Item)],
    app_var: &Ident,
    parent_item: &Item,
    override_required: bool,
    clap_path: &Path,
) -> TokenStream {
    let mut subcommand_specified = false;
    let args = fields.iter().filter_map(|(field, item)| {
        let kind = item.kind();
        match &*kind {
            Kind::Command(_)
            | Kind::Value
            | Kind::Skip(_, _)
            | Kind::FromGlobal(_)
            | Kind::ExternalSubcommand => None,
            Kind::Subcommand(ty) => {
                if subcommand_specified {
                    abort!(field.span(), "`#[command(subcommand)]` can only be used once per container");
                }
                subcommand_specified = true;

                let subcmd_type = match (**ty, sub_type(&field.ty)) {
                    (Ty::Option, Some(sub_type)) => sub_type,
                    _ => &field.ty,
                };
                let implicit_methods = if **ty == Ty::Option || override_required {
                    quote!()
                } else {
                    quote_spanned! { kind.span()=>
                        .subcommand_required(true)
                        .arg_required_else_help(true)
                    }
                };

                Some(quote! {
                    let #app_var = <#subcmd_type as #clap_path::Subcommand>::augment_subcommands( #app_var );
                    let #app_var = #app_var
                        #implicit_methods;
                })
            }
            Kind::Flatten(ty) => {
                let inner_type = match (**ty, sub_type(&field.ty)) {
                    (Ty::Option, Some(sub_type)) => sub_type,
                    _ => &field.ty,
                };

                let next_help_heading = item.next_help_heading();
                let next_display_order = item.next_display_order();
                if override_required {
                    Some(quote_spanned! { kind.span()=>
                        let #app_var = #app_var
                            #next_help_heading
                            #next_display_order;
                        let #app_var = <#inner_type as #clap_path::Args>::augment_args_for_update(#app_var);
                    })
                } else {
                    Some(quote_spanned! { kind.span()=>
                        let #app_var = #app_var
                            #next_help_heading
                            #next_display_order;
                        let #app_var = <#inner_type as #clap_path::Args>::augment_args(#app_var);
                    })
                }
            }
            Kind::Arg(ty) => {
                let value_parser = item.value_parser(&field.ty, clap_path);
                let action = item.action(&field.ty, clap_path);
                let value_name = item.value_name();

                let implicit_methods = match **ty {
                    Ty::Unit => {
                        quote_spanned! { ty.span()=>
                            .value_name(#value_name)
                            #value_parser
                            #action
                        }
                    }
                    Ty::Option => {
                        quote_spanned! { ty.span()=>
                            .value_name(#value_name)
                            #value_parser
                            #action
                        }
                    }

                    Ty::OptionOption => quote_spanned! { ty.span()=>
                        .value_name(#value_name)
                        .num_args(0..=1)
                        #value_parser
                        #action
                    },

                    Ty::OptionVec => {
                        if item.is_positional() {
                            quote_spanned! { ty.span()=>
                                .value_name(#value_name)
                                .num_args(1..)  // action won't be sufficient for getting multiple
                                #value_parser
                                #action
                            }
                        } else {
                            quote_spanned! { ty.span()=>
                                .value_name(#value_name)
                                #value_parser
                                #action
                            }
                        }
                    }

                    Ty::Vec => {
                        if item.is_positional() {
                            quote_spanned! { ty.span()=>
                                .value_name(#value_name)
                                .num_args(1..)  // action won't be sufficient for getting multiple
                                #value_parser
                                #action
                            }
                        } else {
                            quote_spanned! { ty.span()=>
                                .value_name(#value_name)
                                #value_parser
                                #action
                            }
                        }
                    }

                    Ty::Other => {
                        let required = item.find_default_method().is_none() && !override_required;
                        // `ArgAction::takes_values` is assuming `ArgAction::default_value` will be
                        // set though that won't always be true but this should be good enough,
                        // otherwise we'll report an "arg required" error when unwrapping.
                        let action_value = action.args();
                        quote_spanned! { ty.span()=>
                            .value_name(#value_name)
                            .required(#required && #action_value.takes_values())
                            #value_parser
                            #action
                        }
                    }
                };

                let id = item.id();
                let explicit_methods = item.field_methods();
                let deprecations = if !override_required {
                    item.deprecations()
                } else {
                    quote!()
                };

                Some(quote_spanned! { field.span()=>
                    let #app_var = #app_var.arg({
                        #deprecations

                        #[allow(deprecated)]
                        let arg = #clap_path::Arg::new(#id)
                            #implicit_methods;

                        let arg = arg
                            #explicit_methods;
                        arg
                    });
                })
            }
        }
    });

    let deprecations = if !override_required {
        parent_item.deprecations()
    } else {
        quote!()
    };
    let initial_app_methods = parent_item.initial_top_level_methods();
    let final_app_methods = parent_item.final_top_level_methods();
    let group_app_methods = if parent_item.skip_group() {
        quote!()
    } else {
        let group_id = parent_item.ident().unraw().to_string();
        let literal_group_members = fields
            .iter()
            .filter_map(|(_field, item)| {
                let kind = item.kind();
                if matches!(*kind, Kind::Arg(_)) {
                    Some(item.id())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let literal_group_members_len = literal_group_members.len();
        let mut literal_group_members = quote! {{
            let members: [#clap_path::Id; #literal_group_members_len] = [#( #clap_path::Id::from(#literal_group_members) ),* ];
            members
        }};
        // HACK: Validation isn't ready yet for nested arg groups, so just don't populate the group in
        // that situation
        let possible_group_members_len = fields
            .iter()
            .filter(|(_field, item)| {
                let kind = item.kind();
                matches!(*kind, Kind::Flatten(_))
            })
            .count();
        if 0 < possible_group_members_len {
            literal_group_members = quote! {{
                let members: [#clap_path::Id; 0] = [];
                members
            }};
        }

        quote!(
            .group(
                #clap_path::ArgGroup::new(#group_id)
                    .multiple(true)
                    .args(#literal_group_members)
            )
        )
    };
    quote! {{
        #deprecations
        let #app_var = #app_var
            #initial_app_methods
            #group_app_methods
            ;
        #( #args )*
        #app_var #final_app_methods
    }}
}

pub fn gen_constructor(fields: &[(&Field, Item)], clap_path: &Path) -> TokenStream {
    let fields = fields.iter().map(|(field, item)| {
        let field_name = field.ident.as_ref().unwrap();
        let kind = item.kind();
        let arg_matches = format_ident!("__clap_arg_matches");
        match &*kind {
            Kind::Command(_)
            | Kind::Value
            | Kind::ExternalSubcommand => {
                abort! { kind.span(),
                    "`{}` cannot be used with `arg`",
                    kind.name(),
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
                                if #arg_matches.subcommand_name().map(<#subcmd_type as #clap_path::Subcommand>::has_subcommand).unwrap_or(false) {
                                    Some(<#subcmd_type as #clap_path::FromArgMatches>::from_arg_matches_mut(#arg_matches)?)
                                } else {
                                    None
                                }
                            }
                        }
                    },
                    Ty::Other => {
                        quote_spanned! { kind.span()=>
                            #field_name: {
                                <#subcmd_type as #clap_path::FromArgMatches>::from_arg_matches_mut(#arg_matches)?
                            }
                        }
                    },
                    Ty::Unit |
                    Ty::Vec |
                    Ty::OptionOption |
                    Ty::OptionVec => {
                        abort!(
                            ty.span(),
                            "{} types are not supported for subcommand",
                            ty.as_str()
                        );
                    }
                }
            }

            Kind::Flatten(ty) => {
                let inner_type = match (**ty, sub_type(&field.ty)) {
                    (Ty::Option, Some(sub_type)) => sub_type,
                    _ => &field.ty,
                };
                match **ty {
                    Ty::Other => {
                        quote_spanned! { kind.span()=>
                            #field_name: <#inner_type as #clap_path::FromArgMatches>::from_arg_matches_mut(#arg_matches)?
                        }
                    },
                    Ty::Option => {
                        quote_spanned! { kind.span()=>
                            #field_name: {
                                let group_id = <#inner_type as #clap_path::Args>::group_id()
                                    .expect("`#[arg(flatten)]`ed field type implements `Args::group_id`");
                                if #arg_matches.contains_id(group_id.as_str()) {
                                    Some(
                                        <#inner_type as #clap_path::FromArgMatches>::from_arg_matches_mut(#arg_matches)?
                                    )
                                } else {
                                    None
                                }
                            }
                        }
                    },
                    Ty::Unit |
                    Ty::Vec |
                    Ty::OptionOption |
                    Ty::OptionVec => {
                        abort!(
                            ty.span(),
                            "{} types are not supported for flatten",
                            ty.as_str()
                        );
                    }
                }
            },

            Kind::Skip(val, _) => match val {
                None => quote_spanned!(kind.span()=> #field_name: Default::default()),
                Some(val) => quote_spanned!(kind.span()=> #field_name: (#val).into()),
            },

            Kind::Arg(ty) | Kind::FromGlobal(ty) => {
                gen_parsers(item, ty, field_name, field, None, clap_path)
            }
        }
    });

    quote! {{
        #( #fields ),*
    }}
}

pub fn gen_updater(fields: &[(&Field, Item)], use_self: bool, clap_path: &Path) -> TokenStream {
    let fields = fields.iter().map(|(field, item)| {
        let field_name = field.ident.as_ref().unwrap();
        let kind = item.kind();

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
            Kind::Command(_)
            | Kind::Value
            | Kind::ExternalSubcommand => {
                abort! { kind.span(),
                    "`{}` cannot be used with `arg`",
                    kind.name(),
                }
            }
            Kind::Subcommand(ty) => {
                let subcmd_type = match (**ty, sub_type(&field.ty)) {
                    (Ty::Option, Some(sub_type)) => sub_type,
                    _ => &field.ty,
                };

                let updater = quote_spanned! { ty.span()=>
                    <#subcmd_type as #clap_path::FromArgMatches>::update_from_arg_matches_mut(#field_name, #arg_matches)?;
                };

                let updater = match **ty {
                    Ty::Option => quote_spanned! { kind.span()=>
                        if let Some(#field_name) = #field_name.as_mut() {
                            #updater
                        } else {
                            *#field_name = Some(<#subcmd_type as #clap_path::FromArgMatches>::from_arg_matches_mut(
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

            Kind::Flatten(ty) => {
                let inner_type = match (**ty, sub_type(&field.ty)) {
                    (Ty::Option, Some(sub_type)) => sub_type,
                    _ => &field.ty,
                };

                let updater = quote_spanned! { ty.span()=>
                    <#inner_type as #clap_path::FromArgMatches>::update_from_arg_matches_mut(#field_name, #arg_matches)?;
                };

                let updater = match **ty {
                    Ty::Option => quote_spanned! { kind.span()=>
                        if let Some(#field_name) = #field_name.as_mut() {
                            #updater
                        } else {
                            *#field_name = Some(<#inner_type as #clap_path::FromArgMatches>::from_arg_matches_mut(
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
            },

            Kind::Skip(_, _) => quote!(),

            Kind::Arg(ty) | Kind::FromGlobal(ty) => gen_parsers(item, ty, field_name, field, Some(&access), clap_path),
        }
    });

    quote! {
        #( #fields )*
    }
}

fn gen_parsers(
    item: &Item,
    ty: &Sp<Ty>,
    field_name: &Ident,
    field: &Field,
    update: Option<&TokenStream>,
    clap_path: &Path,
) -> TokenStream {
    let span = ty.span();
    let convert_type = inner_type(&field.ty);
    let id = item.id();
    let get_one = quote_spanned!(span=> remove_one::<#convert_type>);
    let get_many = quote_spanned!(span=> remove_many::<#convert_type>);
    let deref = quote!(|s| s);
    let parse = quote_spanned!(span=> |s| ::std::result::Result::Ok::<_, #clap_path::Error>(s));

    // Give this identifier the same hygiene
    // as the `arg_matches` parameter definition. This
    // allows us to refer to `arg_matches` within a `quote_spanned` block
    let arg_matches = format_ident!("__clap_arg_matches");

    let field_value = match **ty {
        Ty::Unit => {
            quote_spanned! { ty.span()=>
                ()
            }
        }

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
                    .map(|v| v.map(#deref).map::<::std::result::Result<#convert_type, #clap_path::Error>, _>(#parse).collect::<::std::result::Result<Vec<_>, #clap_path::Error>>())
                    .transpose()?
                    .unwrap_or_else(Vec::new))
            } else {
                None
            }
        },

        Ty::Vec => {
            quote_spanned! { ty.span()=>
                #arg_matches.#get_many(#id)
                    .map(|v| v.map(#deref).map::<::std::result::Result<#convert_type, #clap_path::Error>, _>(#parse).collect::<::std::result::Result<Vec<_>, #clap_path::Error>>())
                    .transpose()?
                    .unwrap_or_else(Vec::new)
            }
        }

        Ty::Other => {
            quote_spanned! { ty.span()=>
                #arg_matches.#get_one(#id)
                    .map(#deref)
                    .ok_or_else(|| #clap_path::Error::raw(#clap_path::error::ErrorKind::MissingRequiredArgument, format!("The following required argument was not provided: {}", #id)))
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
