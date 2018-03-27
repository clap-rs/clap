// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate is custom derive for StructOpt. It should not be used
//! directly. See [structopt documentation](https://docs.rs/structopt)
//! for the usage of `#[derive(StructOpt)]`.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

#[macro_use]
mod macros;

mod attrs;

use proc_macro::TokenStream;
use syn::*;
use syn::punctuated::Punctuated;
use syn::token::{Comma};
use attrs::{Attrs, Parser};

/// Generates the `StructOpt` impl.
#[proc_macro_derive(StructOpt, attributes(structopt))]
pub fn structopt(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_structopt(&input);
    gen.into()
}

#[derive(Copy, Clone, PartialEq)]
enum Ty {
    Bool,
    Vec,
    Option,
    Other,
}

fn ty(t: &syn::Type) -> Ty {
    if let syn::Type::Path(TypePath { path: syn::Path { ref segments, .. }, .. }) = *t {
        match segments.iter().last().unwrap().ident.as_ref() {
            "bool" => Ty::Bool,
            "Option" => Ty::Option,
            "Vec" => Ty::Vec,
            _ => Ty::Other,
        }
    } else {
        Ty::Other
    }
}

fn sub_type(t: &syn::Type) -> Option<&syn::Type> {
    let segs = match *t {
        syn::Type::Path(TypePath { path: syn::Path { ref segments, .. }, .. }) => segments,
        _ => return None,
    };
    match *segs.iter().last().unwrap() {
        PathSegment {
            arguments: PathArguments::AngleBracketed(
                AngleBracketedGenericArguments { ref args, .. }
            ),
            ..
        } if args.len() == 1 => {
            if let GenericArgument::Type(ref ty) = args[0] {
                Some(ty)
            } else {
                None
            }
        }
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
fn gen_augmentation(fields: &Punctuated<Field, Comma>, app_var: &Ident) -> quote::Tokens {
    let subcmds: Vec<quote::Tokens> = fields.iter()
        .filter(|&field| Attrs::from_field(&field).is_subcommand())
        .map(|field| {
            let cur_type = ty(&field.ty);
            let subcmd_type = match (cur_type, sub_type(&field.ty)) {
                (Ty::Option, Some(sub_type)) => sub_type,
                _ => &field.ty
            };
            let required = if cur_type == Ty::Option {
                my_quote!()
            } else {
                my_quote!( let #app_var = #app_var.setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp); )
            };

            my_quote!{
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
                (Parser::TryFromStr, ref f) => my_quote! {
                    .validator(|s| {
                        #f(&s)
                            .map(|_: #convert_type| ())
                            .map_err(|e| e.to_string())
                    })
                },
                (Parser::TryFromOsStr, ref f) => my_quote! {
                    .validator_os(|s| #f(&s).map(|_: #convert_type| ()))
                },
                _ => my_quote!(),
            };

            let modifier = match cur_type {
                Ty::Bool => my_quote!( .takes_value(false).multiple(false) ),
                Ty::Option => my_quote!( .takes_value(true).multiple(false) #validator ),
                Ty::Vec => my_quote!( .takes_value(true).multiple(true) #validator ),
                Ty::Other if occurences => my_quote!( .takes_value(false).multiple(true) ),
                Ty::Other => {
                    let required = !attrs.has_method("default_value");
                    my_quote!( .takes_value(true).multiple(false).required(#required) #validator )
                },
            };
            let methods = attrs.methods();
            let name = attrs.name();
            Some(my_quote!(.arg(::structopt::clap::Arg::with_name(#name)#modifier#methods)))
        });

    my_quote! {{
        let #app_var = #app_var #( #args )* ;
        #( #subcmds )*
        #app_var
    }}
}

fn gen_constructor(fields: &Punctuated<Field, Comma>) -> quote::Tokens {
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
                Ty::Option => my_quote!(),
                _ => my_quote!( .unwrap() )
            };
            my_quote!(#field_name: #subcmd_type::from_subcommand(matches.subcommand())#unwrapper)
        } else {
            let real_ty = &field.ty;
            let mut cur_type = ty(real_ty);
            if attrs.has_custom_parser() {
                cur_type = convert_with_custom_parse(cur_type);
            }

            use Parser::*;
            let (value_of, values_of, parse) = match *attrs.parser() {
                (FromStr, ref f) => (my_quote!(value_of), my_quote!(values_of), f.clone()),
                (TryFromStr, ref f) =>
                    (my_quote!(value_of), my_quote!(values_of), my_quote!(|s| #f(s).unwrap())),
                (FromOsStr, ref f) =>
                    (my_quote!(value_of_os), my_quote!(values_of_os), f.clone()),
                (TryFromOsStr, ref f) =>
                    (my_quote!(value_of_os), my_quote!(values_of_os), my_quote!(|s| #f(s).unwrap())),
                (FromOccurrences, ref f) => (my_quote!(occurrences_of), my_quote!(), f.clone()),
            };

            let occurences = attrs.parser().0 == Parser::FromOccurrences;
            let name = attrs.name();
            let field_value = match cur_type {
                Ty::Bool => my_quote!(matches.is_present(#name)),
                Ty::Option => my_quote! {
                    matches.#value_of(#name)
                        .as_ref()
                        .map(#parse)
                },
                Ty::Vec => my_quote! {
                    matches.#values_of(#name)
                        .map(|v| v.map(#parse).collect())
                        .unwrap_or_else(Vec::new)
                },
                Ty::Other if occurences => my_quote! {
                    #parse(matches.#value_of(#name))
                },
                Ty::Other => my_quote! {
                    matches.#value_of(#name)
                        .map(#parse)
                        .unwrap()
                },
            };

            my_quote!( #field_name: #field_value )
        }
    });

    my_quote! {{
        #( #fields ),*
    }}
}

fn gen_from_clap(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> quote::Tokens {
    let field_block = gen_constructor(fields);

    my_quote! {
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
    my_quote!(::structopt::clap::App::new(#name)#methods)
}

fn gen_clap_struct(struct_attrs: &[Attribute]) -> quote::Tokens {
    let gen = gen_clap(struct_attrs);
    my_quote! {
        fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
            let app = #gen;
            Self::augment_clap(app)
        }
    }
}

fn gen_augment_clap(fields: &Punctuated<Field, Comma>) -> quote::Tokens {
    let app_var: Ident = "app".into();
    let augmentation = gen_augmentation(fields, &app_var);
    my_quote! {
        pub fn augment_clap<'a, 'b>(#app_var: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
            #augmentation
        }
    }
}

fn gen_clap_enum(enum_attrs: &[Attribute]) -> quote::Tokens {
    let gen = gen_clap(enum_attrs);
    my_quote! {
        fn clap<'a, 'b>() -> ::structopt::clap::App<'a, 'b> {
            let app = #gen
                .setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp);
            Self::augment_clap(app)
        }
    }
}

fn gen_augment_clap_enum(variants: &Punctuated<Variant, Comma>) -> quote::Tokens {
    use syn::Fields::*;

    let subcommands = variants.iter().map(|variant| {
        let name = variant.ident.as_ref().to_string();
        let attrs = Attrs::from_struct(&variant.attrs, name);
        let app_var: Ident = "subcommand".into();
        let arg_block = match variant.fields {
            Named(ref fields) => gen_augmentation(&fields.named, &app_var),
            Unit => my_quote!( #app_var ),
            Unnamed(FieldsUnnamed { ref unnamed, .. }) if unnamed.len() == 1 => {
                let ty = &unnamed[0];
                my_quote! {
                    {
                        let #app_var = #ty::augment_clap(#app_var);
                        if #ty::is_subcommand() {
                            #app_var.setting(::structopt::clap::AppSettings::SubcommandRequiredElseHelp)
                        } else {
                            #app_var
                        }
                    }
                }
            }
            Unnamed(..) => panic!("{}: tuple enum are not supported", variant.ident),
        };

        let name = attrs.name();
        let from_attrs = attrs.methods();
        my_quote! {
            .subcommand({
                let #app_var = ::structopt::clap::SubCommand::with_name(#name);
                let #app_var = #arg_block;
                #app_var#from_attrs
            })
        }
    });

    my_quote! {
        pub fn augment_clap<'a, 'b>(app: ::structopt::clap::App<'a, 'b>) -> ::structopt::clap::App<'a, 'b> {
            app #( #subcommands )*
        }
    }
}

fn gen_from_clap_enum(name: &Ident) -> quote::Tokens {
    my_quote! {
        fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
            #name ::from_subcommand(matches.subcommand())
                .unwrap()
        }
    }
}

fn gen_from_subcommand(name: &Ident, variants: &Punctuated<Variant, Comma>) -> quote::Tokens {
    use syn::Fields::*;

    let match_arms = variants.iter().map(|variant| {
        let attrs = Attrs::from_struct(&variant.attrs, variant.ident.as_ref().to_string());
        let sub_name = attrs.name();
        let variant_name = &variant.ident;
        let constructor_block = match variant.fields {
            Named(ref fields) => gen_constructor(&fields.named),
            Unit => my_quote!(),
            Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed[0];
                my_quote!( ( <#ty as ::structopt::StructOpt>::from_clap(matches) ) )
            }
            Unnamed(..) =>
                panic!("{}: tuple enum are not supported", variant.ident),
        };

        my_quote! {
            (#sub_name, Some(matches)) =>
                Some(#name :: #variant_name #constructor_block)
        }
    });

    my_quote! {
        pub fn from_subcommand<'a, 'b>(sub: (&'b str, Option<&'b ::structopt::clap::ArgMatches<'a>>)) -> Option<Self> {
            match sub {
                #( #match_arms ),*,
                _ => None
            }
        }
    }
}

fn impl_structopt_for_struct(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
    attrs: &[Attribute]
) -> quote::Tokens {
    let clap = gen_clap_struct(attrs);
    let augment_clap = gen_augment_clap(fields);
    let from_clap = gen_from_clap(name, fields);

    my_quote! {
        #[allow(unused_variables)]
        impl ::structopt::StructOpt for #name {
            #clap
            #from_clap
        }

        #[allow(dead_code, unreachable_code)]
        #[doc(hidden)]
        impl #name {
            #augment_clap
            pub fn is_subcommand() -> bool { false }
        }
    }
}

fn impl_structopt_for_enum(
    name: &Ident,
    variants: &Punctuated<Variant, Comma>,
    attrs: &[Attribute]
) -> quote::Tokens {
    let clap = gen_clap_enum(attrs);
    let augment_clap = gen_augment_clap_enum(variants);
    let from_clap = gen_from_clap_enum(name);
    let from_subcommand = gen_from_subcommand(name, variants);

    my_quote! {
        impl ::structopt::StructOpt for #name {
            #clap
            #from_clap
        }

        #[allow(unused_variables, dead_code, unreachable_code)]
        #[doc(hidden)]
        impl #name {
            #augment_clap
            #from_subcommand
            pub fn is_subcommand() -> bool { true }
        }
    }
}

fn impl_structopt(input: &DeriveInput) -> quote::Tokens {
    use syn::Data::*;

    let struct_name = &input.ident;
    let inner_impl = match input.data {
        Struct(DataStruct { fields: syn::Fields::Named(ref fields), .. }) =>
            impl_structopt_for_struct(struct_name, &fields.named, &input.attrs),
        Enum(ref e) =>
            impl_structopt_for_enum(struct_name, &e.variants, &input.attrs),
        _ => panic!("structopt only supports non-tuple structs and enums")
    };

    my_quote!(#inner_impl)
}
