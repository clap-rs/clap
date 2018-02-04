// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! This crate is custom derive for StructOpt. It should not be used
//! directly. See [structopt documentation](https://docs.rs/structopt)
//! for the usage of `#[derive(StructOpt)]`.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod attrs;

use proc_macro::TokenStream;
use syn::*;
use attrs::{Attrs, Parser};

/// Generates the `StructOpt` impl.
#[proc_macro_derive(StructOpt, attributes(structopt))]
pub fn structopt(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_structopt(&ast);
    gen.parse().unwrap()
}

#[derive(Copy, Clone, PartialEq)]
enum Ty {
    Bool,
    Vec,
    Option,
    Other,
}

fn ty(t: &syn::Ty) -> Ty {
    if let syn::Ty::Path(None, syn::Path { segments: ref segs, .. }) = *t {
        match segs.last().unwrap().ident.as_ref() {
            "bool" => Ty::Bool,
            "Option" => Ty::Option,
            "Vec" => Ty::Vec,
            _ => Ty::Other,
        }
    } else {
        Ty::Other
    }
}

fn sub_type(t: &syn::Ty) -> Option<&syn::Ty> {
    let segs = match *t {
        syn::Ty::Path(None, syn::Path { ref segments, .. }) => segments,
        _ => return None,
    };
    match *segs.last().unwrap() {
        PathSegment {
            parameters: PathParameters::AngleBracketed(
                AngleBracketedParameterData { ref types, .. }),
            ..
        } if !types.is_empty() => Some(&types[0]),
            _ => None,
    }
}

fn convert_with_custom_parse(cur_type: Ty) -> Ty {
    match cur_type {
        Ty::Bool => Ty::Other,
        rest => rest,
    }
}

/// Generate a block of code to add arguments/subcommands corresponding to
/// the `fields` to an app.
fn gen_augmentation(fields: &[Field], app_var: &Ident) -> quote::Tokens {
    let subcmds: Vec<quote::Tokens> = fields.iter()
        .filter(|&field| Attrs::from_field(&field).is_subcommand())
        .map(|field| {
            let cur_type = ty(&field.ty);
            let subcmd_type = match (cur_type, sub_type(&field.ty)) {
                (Ty::Option, Some(sub_type)) => sub_type,
                _ => &field.ty
            };
            let required = if cur_type == Ty::Option {
                quote!()
            } else {
                quote!( let #app_var = #app_var.setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp); )
            };

            quote!{
                let #app_var = #subcmd_type ::augment_clap( #app_var );
                #required
            }
        })
        .collect();

    assert!(subcmds.len() <= 1, "cannot have more than one nested subcommand");

    let args = fields.iter()
        .filter_map(|field| {
            let attrs = Attrs::from_field(field);
            if attrs.is_subcommand() { return None; }
            let mut cur_type = ty(&field.ty);
            let convert_type = match cur_type {
                Ty::Vec | Ty::Option => sub_type(&field.ty).unwrap_or(&field.ty),
                _ => &field.ty,
            };

            let occurences = attrs.parser().0 == Parser::FromOccurrences;
            if attrs.has_custom_parser() {
                cur_type = convert_with_custom_parse(cur_type);
            }

            let validator = match *attrs.parser() {
                (Parser::TryFromStr, ref f) => quote! {
                    .validator(|s| {
                        #f(&s)
                            .map(|_: #convert_type| ())
                            .map_err(|e| e.to_string())
                    })
                },
                (Parser::TryFromOsStr, ref f) => quote! {
                    .validator_os(|s| #f(&s).map(|_: #convert_type| ()))
                },
                _ => quote!(),
            };

            let modifier = match cur_type {
                Ty::Bool => quote!( .takes_value(false).multiple(false) ),
                Ty::Option => quote!( .takes_value(true).multiple(false) #validator ),
                Ty::Vec => quote!( .takes_value(true).multiple(true) #validator ),
                Ty::Other if occurences => quote!( .takes_value(false).multiple(true) ),
                Ty::Other => {
                    let required = attrs.has_method("default_value");
                    quote!( .takes_value(true).multiple(false).required(#required) #validator )
                },
            };
            let methods = attrs.methods();
            let name = attrs.name();
            Some(quote!(.arg(::structopt::clap::Arg::with_name(#name)#modifier#methods)))
        });

    quote! {{
        let #app_var = #app_var #( #args )* ;
        #( #subcmds )*
        #app_var
    }}
}

fn gen_constructor(fields: &[Field]) -> quote::Tokens {
    let fields = fields.iter().map(|field| {
        let attrs = Attrs::from_field(field);
        let field_name = field.ident.as_ref().unwrap();
        if attrs.is_subcommand() {
            let cur_type = ty(&field.ty);
            let subcmd_type = match (cur_type, sub_type(&field.ty)) {
                (Ty::Option, Some(sub_type)) => sub_type,
                _ => &field.ty
            };
            let unwrapper = match cur_type {
                Ty::Option => quote!(),
                _ => quote!( .unwrap() )
            };
            quote!(#field_name: #subcmd_type::from_subcommand(matches.subcommand())#unwrapper)
        } else {
            let real_ty = &field.ty;
            let mut cur_type = ty(real_ty);
            if attrs.has_custom_parser() {
                cur_type = convert_with_custom_parse(cur_type);
            }

            use Parser::*;
            let (value_of, values_of, parse) = match *attrs.parser() {
                (FromStr, ref f) => (quote!(value_of), quote!(values_of), f.clone()),
                (TryFromStr, ref f) =>
                    (quote!(value_of), quote!(values_of), quote!(|s| #f(s).unwrap())),
                (FromOsStr, ref f) =>
                    (quote!(value_of_os), quote!(values_of_os), f.clone()),
                (TryFromOsStr, ref f) =>
                    (quote!(value_of_os), quote!(values_of_os), quote!(|s| #f(s).unwrap())),
                (FromOccurrences, ref f) => (quote!(occurrences_of), quote!(), f.clone()),
            };

            let occurences = attrs.parser().0 == Parser::FromOccurrences;
            let name = attrs.name();
            let field_value = match cur_type {
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
    });

    quote! {{
        #( #fields ),*
    }}
}

fn gen_from_clap(struct_name: &Ident, fields: &[Field]) -> quote::Tokens {
    let field_block = gen_constructor(fields);

    quote! {
        fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
            #struct_name #field_block
        }
    }
}

fn gen_clap(attrs: &[Attribute]) -> quote::Tokens {
    let name = std::env::var("CARGO_PKG_NAME").ok().unwrap_or_else(String::default);
    let attrs = Attrs::from_struct(attrs, name);
    let name = attrs.name();
    let methods = attrs.methods();
    quote!(::structopt::clap::App::new(#name)#methods)
}

fn gen_clap_struct(struct_attrs: &[Attribute]) -> quote::Tokens {
    let gen = gen_clap(struct_attrs);

    quote! {
        fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
            let app = #gen;
            Self::augment_clap(app)
        }
    }
}

fn gen_augment_clap(fields: &[Field]) -> quote::Tokens {
    let app_var = Ident::new("app");
    let augmentation = gen_augmentation(fields, &app_var);
    quote! {
        pub fn augment_clap<'a, 'b>(#app_var: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
            #augmentation
        }
    }
}

fn gen_clap_enum(enum_attrs: &[Attribute]) -> quote::Tokens {
    let gen = gen_clap(enum_attrs);
    quote! {
        fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
            let app = #gen
                .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp);
            Self::augment_clap(app)
        }
    }
}

fn gen_augment_clap_enum(variants: &[Variant]) -> quote::Tokens {
    let subcommands = variants.iter().map(|variant| {
        let attrs = Attrs::from_struct(&variant.attrs, variant.ident.to_string());
        let app_var = Ident::new("subcommand");
        let arg_block = match variant.data {
            VariantData::Struct(ref fields) => gen_augmentation(fields, &app_var),
            VariantData::Unit => quote!( #app_var ),
            VariantData::Tuple(ref fields) if fields.len() == 1 => {
                let ty = &fields[0];
                quote!(#ty::augment_clap(#app_var))
            }
            VariantData::Tuple(..) => panic!("{}: tuple enum are not supported", variant.ident),
        };

        let name = attrs.name();
        let from_attrs = attrs.methods();
        quote! {
            .subcommand({
                let #app_var = ::structopt::clap::SubCommand::with_name(#name)#from_attrs;
                #arg_block
            })
        }
    });

    quote! {
        pub fn augment_clap<'a, 'b>(app: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
            app #( #subcommands )*
        }
    }
}

fn gen_from_clap_enum(name: &Ident) -> quote::Tokens {
    quote! {
        fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
            #name ::from_subcommand(matches.subcommand())
                .unwrap()
        }
    }
}

fn gen_from_subcommand(name: &Ident, variants: &[Variant]) -> quote::Tokens {
    let match_arms = variants.iter().map(|variant| {
        let attrs = Attrs::from_struct(&variant.attrs, variant.ident.as_ref().to_string());
        let sub_name = attrs.name();
        let variant_name = &variant.ident;
        let constructor_block = match variant.data {
            VariantData::Struct(ref fields) => gen_constructor(fields),
            VariantData::Unit => quote!(),
            VariantData::Tuple(ref fields) if fields.len() == 1 => {
                let ty = &fields[0];
                quote!( ( <#ty as ::structopt::StructOpt>::from_clap(matches) ) )
            }
            VariantData::Tuple(..) =>
                panic!("{}: tuple enum are not supported", variant.ident),
        };

        quote! {
            (#sub_name, Some(matches)) =>
                Some(#name :: #variant_name #constructor_block)
        }
    });

    quote! {
        #[doc(hidden)]
        pub fn from_subcommand<'a, 'b>(sub: (&'b str, Option<&'b ::structopt::clap::ArgMatches<'a>>)) -> Option<Self> {
            match sub {
                #( #match_arms ),*,
                _ => None
            }
        }
    }
}

fn impl_structopt_for_struct(name: &Ident, fields: &[Field], attrs: &[Attribute]) -> quote::Tokens {
    let clap = gen_clap_struct(attrs);
    let augment_clap = gen_augment_clap(fields);
    let from_clap = gen_from_clap(name, fields);

    quote! {
        #[allow(unused_variables)]
        impl ::structopt::StructOpt for #name {
            #clap
            #from_clap
        }

        impl #name {
            #augment_clap
        }
    }
}

fn impl_structopt_for_enum(name: &Ident, variants: &[Variant], attrs: &[Attribute]) -> quote::Tokens {
    let clap = gen_clap_enum(attrs);
    let augment_clap = gen_augment_clap_enum(variants);
    let from_clap = gen_from_clap_enum(name);
    let from_subcommand = gen_from_subcommand(name, variants);

    quote! {
        impl ::structopt::StructOpt for #name {
            #clap
            #from_clap
        }

        #[allow(unused_variables)]
        impl #name {
            #augment_clap
            #from_subcommand
        }
    }
}

fn impl_structopt(ast: &DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;
    let inner_impl = match ast.body {
        Body::Struct(VariantData::Struct(ref fields)) =>
            impl_structopt_for_struct(struct_name, fields, &ast.attrs),
        Body::Enum(ref variants) =>
            impl_structopt_for_enum(struct_name, variants, &ast.attrs),
        _ => panic!("structopt only supports non-tuple structs and enums")
    };

    quote!(#inner_impl)
}
