use crate::{
    attrs::{Attrs, Kind, Name, DEFAULT_CASING, DEFAULT_ENV_CASING},
    derives::{from_arg_matches, into_app},
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

pub fn gen_for_enum(name: &Ident, attrs: &[Attribute], e: &DataEnum) -> TokenStream {
    let attrs = Attrs::from_struct(
        Span::call_site(),
        attrs,
        Name::Derived(name.clone()),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );

    let augment_subcommands = gen_augment("augment_subcommands", &e.variants, &attrs, false);
    let augment_subcommands_for_update =
        gen_augment("augment_subcommands_for_update", &e.variants, &attrs, true);
    let from_subcommand = gen_from_subcommand(name, &e.variants, &attrs);
    let update_from_subcommand = gen_update_from_subcommand(name, &e.variants, &attrs);

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
        impl clap::Subcommand for #name {
            #augment_subcommands
            #from_subcommand
            #augment_subcommands_for_update
            #update_from_subcommand
        }
    }
}

fn gen_augment(
    fn_name: &str,
    variants: &Punctuated<Variant, Token![,]>,
    parent_attribute: &Attrs,
    override_required: bool,
) -> TokenStream {
    use syn::Fields::*;

    let subcommands: Vec<_> = variants
        .iter()
        .map(|variant| {
            let attrs = Attrs::from_struct(
                variant.span(),
                &variant.attrs,
                Name::Derived(variant.ident.clone()),
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
                        quote! {
                            let app = <#ty as clap::Subcommand>::augment_subcommands(app);
                        }
                    }
                    _ => abort!(
                        variant,
                        "`flatten` is usable only with single-typed tuple variants"
                    ),
                },

                _ => {
                    let app_var = Ident::new("subcommand", Span::call_site());
                    let arg_block = match variant.fields {
                        Named(ref fields) => into_app::gen_app_augmentation(
                            &fields.named,
                            &app_var,
                            &attrs,
                            override_required,
                        ),
                        Unit => quote!( #app_var ),
                        Unnamed(FieldsUnnamed { ref unnamed, .. }) if unnamed.len() == 1 => {
                            let ty = &unnamed[0];
                            quote_spanned! { ty.span()=>
                                {
                                    <#ty as clap::IntoApp>::augment_clap(#app_var)
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
    let fn_name = Ident::new(fn_name, Span::call_site());
    quote! {
        fn #fn_name <'b>(app: clap::App<'b>) -> clap::App<'b> {
            let app = app #app_methods;
            #( #subcommands )*;
            app #version
        }
    }
}

fn gen_from_subcommand(
    name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
    parent_attribute: &Attrs,
) -> TokenStream {
    use syn::Fields::*;

    let mut ext_subcmd = None;

    let (flatten_variants, variants): (Vec<_>, Vec<_>) = variants
        .iter()
        .filter_map(|variant| {
            let attrs = Attrs::from_struct(
                variant.span(),
                &variant.attrs,
                Name::Derived(variant.ident.clone()),
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
            Named(ref fields) => from_arg_matches::gen_constructor(&fields.named, &attrs),
            Unit => quote!(),
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                quote!( ( <#ty as clap::FromArgMatches>::from_arg_matches(arg_matches) ) )
            }
            Unnamed(..) => abort_call_site!("{}: tuple enums are not supported", variant.ident),
        };

        quote! {
            Some((#sub_name, arg_matches)) => {
                Ok(#name :: #variant_name #constructor_block)
            }
        }
    });
    let child_subcommands = flatten_variants.iter().map(|(variant, _attrs)| {
        let variant_name = &variant.ident;
        match variant.fields {
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                quote! {
                    match <#ty as clap::Subcommand>::from_subcommand(other) {
                        Ok(res) => return Ok(#name :: #variant_name (res)),
                        Err(clap::Error {
                            kind: clap::ErrorKind::UnrecognizedSubcommand,
                            ..
                        }) => {}
                        Err(e) => return Err(e),
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
            None => Err(clap::Error::with_description(
                format!("The subcommand wasn't recognized"),
                clap::ErrorKind::UnrecognizedSubcommand
            )),

            Some((external, arg_matches)) => {
                ::std::result::Result::Ok(#name::#var_name(
                    ::std::iter::once(#str_ty::from(external))
                    .chain(
                        arg_matches.#values_of("").into_iter().flatten().map(#str_ty::from)
                    )
                    .collect::<::std::vec::Vec<_>>()
                ))
            }
        },

        None => quote!(_ => Err(clap::Error::with_description(
            format!("The subcommand wasn't recognized"),
            clap::ErrorKind::UnrecognizedSubcommand
        ))),
    };

    quote! {
        fn from_subcommand(subcommand: Option<(&str, &clap::ArgMatches)>) -> clap::Result<Self> {
            match subcommand {
                #( #match_arms, )*
                other => {
                    #( #child_subcommands )else*

                    match other {
                        #wildcard
                    }
                }
            }
        }
    }
}

fn gen_update_from_subcommand(
    name: &Ident,
    variants: &Punctuated<Variant, Token![,]>,
    parent_attribute: &Attrs,
) -> TokenStream {
    use syn::Fields::*;

    let (flatten_variants, variants): (Vec<_>, Vec<_>) = variants
        .iter()
        .filter_map(|variant| {
            let attrs = Attrs::from_struct(
                variant.span(),
                &variant.attrs,
                Name::Derived(variant.ident.clone()),
                parent_attribute.casing(),
                parent_attribute.env_casing(),
            );

            if let Kind::ExternalSubcommand = &*attrs.kind() {
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
                            from_arg_matches::gen_updater(&fields.named, &attrs, false),
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
                            arg_matches
                        )),
                    )
                } else {
                    abort_call_site!("{}: tuple enums are not supported", variant.ident)
                }
            }
        };

        quote! {
            (#sub_name, #name :: #variant_name #pattern) => { #updater }
        }
    });

    let child_subcommands = flatten_variants.iter().map(|(variant, attrs)| {
        let sub_name = attrs.cased_name();
        let variant_name = &variant.ident;
        let (pattern, updater) = match variant.fields {
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                (
                    quote!((ref mut arg)),
                    quote! {
                        <#ty as clap::Subcommand>::update_from_subcommand(arg, Some((name, arg_matches)))?;
                    },
                )
            }
            _ => abort!(
                variant,
                "`flatten` is usable only with single-typed tuple variants"
            ),
        };
        quote! {
            (#sub_name, #name :: #variant_name #pattern) => { #updater }
        }
    });

    quote! {
        fn update_from_subcommand<'b>(
            &mut self,
            subcommand: Option<(&str, &clap::ArgMatches)>
        ) -> clap::Result<()> {
            if let Some((name, arg_matches)) = subcommand {
                match (name, self) {
                    #( #subcommands ),*
                    #( #child_subcommands ),*
                    (_, s) => match <Self as clap::Subcommand>::from_subcommand(Some((name, arg_matches))) {
                        Ok(sub) => *s = sub,
                        Err(clap::Error { kind: clap::ErrorKind::UnrecognizedSubcommand, .. }) => (),
                        Err(e) => return Err(e),
                    }
                }
            }
            Ok(())
        }
    }
}
