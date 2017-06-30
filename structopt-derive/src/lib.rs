// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

//! How to `derive(StructOpt)`
//!
//! First, look at an example:
//!
//! ```ignore
//! #[derive(StructOpt)]
//! #[structopt(name = "example", about = "An example of StructOpt usage.")]
//! struct Opt {
//!     #[structopt(short = "d", long = "debug", help = "Activate debug mode")]
//!     debug: bool,
//!     #[structopt(short = "s", long = "speed", help = "Set speed", default_value = "42")]
//!     speed: f64,
//!     #[structopt(help = "Input file")]
//!     input: String,
//!     #[structopt(help = "Output file, stdout if not present")]
//!     output: Option<String>,
//! }
//! ```
//!
//! So, `derive(StructOpt)` do the job, and `structopt` attribute is
//! used for additional parameters.
//!
//! First, define a struct, whatever its name.  This structure will
//! correspond to a `clap::App`.  Every method of `clap::App` in the
//! form of `fn function_name(self, &str)` can be use in the form of
//! attributes.  Our example call for example something like
//! `app.about("An example of StructOpt usage.")`.  There is some
//! special attributes:
//!
//!   - `name`: correspond to the creation of the `App` object. Our
//!     example does `clap::App::new("example")`.  Default to
//!     the crate name given by cargo.
//!   - `version`: default to the crate version given by cargo.
//!   - `author`: default to the crate version given by cargo.
//!   - `about`: default to the crate version given by cargo.
//!
//! Then, each field of the struct correspond to a `clap::Arg`.  As
//! for the struct attributes, every method of `clap::Arg` in the form
//! of `fn function_name(self, &str)` can be use in the form of
//! attributes.  The `name` attribute can be used to customize the
//! `Arg::with_name()` call (default to the field name).
//!
//! The type of the field gives the kind of argument:
//!
//! Type                 | Effect            | Added method call to `clap::Arg`
//! ---------------------|-------------------|--------------------------------------
//! `bool`               | `true` if present | `.takes_value(false).multiple(false)`
//! `u64`                | number of params  | `.takes_value(false).multiple(true)`
//! `Option<T: FromStr>` | optional argument | `.takes_value(true).multiple(false)`
//! `Vec<T: FromStr>`    | list of arguments | `.takes_value(true).multiple(true)`
//! `T: FromStr` | required argument | `.takes_value(true).multiple(false).required(!has_default)`
//!
//! The `FromStr` trait is used to convert the argument to the given
//! type, and the `Arg::validator` method is setted to a method using
//! `FromStr::Error::description()`.
//!
//! Thus, the `speed` argument is generated as:
//!
//! ```ignore
//! clap::Arg::with_name("speed")
//!     .takes_value(true)
//!     .multiple(false)
//!     .required(false)
//!     .validator(parse_validator::<f64>)
//!     .short("s")
//!     .long("debug")
//!     .help("Set speed")
//!     .default_value("42")
//! ```

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;

/// Generates the `StructOpt` impl.
#[proc_macro_derive(StructOpt, attributes(structopt))]
pub fn structopt(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_structopt(&ast);
    gen.parse().unwrap()
}

#[derive(Copy, Clone)]
enum Ty {
    Bool,
    U64,
    Vec,
    Option,
    Other,
}

fn ty(t: &syn::Ty) -> Ty {
    if let syn::Ty::Path(None, syn::Path { segments: ref segs, .. }) = *t {
        match segs.last().unwrap().ident.as_ref() {
            "bool" => Ty::Bool,
            "u64" => Ty::U64,
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

#[derive(Debug, Clone, Copy)]
enum AttrSource { Struct, Field, }

fn extract_attrs<'a>(attrs: &'a [Attribute], attr_source: AttrSource) -> Box<Iterator<Item = (Ident, Lit)> + 'a> {
    let settings_attrs = attrs.iter()
        .filter_map(|attr| match attr.value {
            MetaItem::List(ref i, ref v) if i.as_ref() == "structopt" => Some(v),
            _ => None,
        }).flat_map(|v| v.iter().filter_map(|mi| match *mi {
            NestedMetaItem::MetaItem(MetaItem::NameValue(ref i, ref l)) =>
                Some((i.clone(), l.clone())),
            _ => None,
        }));

    let doc_comments = attrs.iter()
        .filter_map(move |attr| {
            if let Attribute {
                value: MetaItem::NameValue(ref name, Lit::Str(ref value, StrStyle::Cooked)),
                is_sugared_doc: true,
                ..
            } = *attr {
                if name != "doc" { return None; }
                let text = value.trim_left_matches("//!")
                    .trim_left_matches("///")
                    .trim_left_matches("/*!")
                    .trim_left_matches("/**")
                    .trim();

                // Clap's `App` has an `about` method to set a description,
                // it's `Field`s have a `help` method instead.
                if let AttrSource::Struct = attr_source {
                    Some(("about".into(), text.into()))
                } else {
                    Some(("help".into(), text.into()))
                }
            } else {
                None
            }
        });

    Box::new(doc_comments.chain(settings_attrs))
}

fn from_attr_or_env(attrs: &[(Ident, Lit)], key: &str, env: &str) -> Lit {
    let default = std::env::var(env).unwrap_or("".into());
    attrs.iter()
        .filter(|&&(ref i, _)| i.as_ref() == key)
        .last()
        .map(|&(_, ref l)| l.clone())
        .unwrap_or_else(|| Lit::Str(default, StrStyle::Cooked))
}

fn is_subcommand(field: &Field) -> bool {
    field.attrs.iter()
        .map(|attr| &attr.value)
        .any(|meta| if let MetaItem::List(ref i, ref l) = *meta {
            if i != "structopt" { return false; }
            match l.first() {
                Some(&NestedMetaItem::MetaItem(MetaItem::Word(ref inner))) => inner == "subcommand",
                _ => false
            }
        } else {
          false
        })
}

/// Generate a block of code to add arguments/subcommands corresponding to
/// the `fields` to an app.
fn gen_augmentation(fields: &[Field], app_var: &Ident) -> quote::Tokens {
    let subcmds: Vec<quote::Tokens> = fields.iter()
        .filter(|&field| is_subcommand(field))
        .map(|field| {
            let cur_type = ty(&field.ty);
            let subcmd_type = match (cur_type, sub_type(&field.ty)) {
                (Ty::Option, Some(sub_type)) => sub_type,
                _ => &field.ty
            };

            quote!( let #app_var = #subcmd_type ::augment_clap( #app_var ); )
        })
        .collect();
    let args = fields.iter()
        .filter(|&field| !is_subcommand(field))
        .map(|field| {
            let name = gen_name(field);
            let cur_type = ty(&field.ty);
            let convert_type = match cur_type {
                Ty::Vec | Ty::Option => sub_type(&field.ty).unwrap_or(&field.ty),
                _ => &field.ty,
            };
            let validator = quote! {
                validator(|s| s.parse::<#convert_type>()
                          .map(|_| ())
                          .map_err(|e| e.description().into()))
            };
            let modifier = match cur_type {
                Ty::Bool => quote!( .takes_value(false).multiple(false) ),
                Ty::U64 => quote!( .takes_value(false).multiple(true) ),
                Ty::Option => quote!( .takes_value(true).multiple(false).#validator ),
                Ty::Vec => quote!( .takes_value(true).multiple(true).#validator ),
                Ty::Other => {
                    let required = extract_attrs(&field.attrs, AttrSource::Field)
                        .find(|&(ref i, _)| i.as_ref() == "default_value")
                        .is_none();
                    quote!( .takes_value(true).multiple(false).required(#required).#validator )
                },
            };
            let from_attr = extract_attrs(&field.attrs, AttrSource::Field)
                .filter(|&(ref i, _)| i.as_ref() != "name")
                .map(|(i, l)| quote!(.#i(#l)));
            quote!( .arg(_structopt::clap::Arg::with_name(stringify!(#name)) #modifier #(#from_attr)*) )
        });

    assert!(subcmds.len() <= 1, "cannot have more than one nested subcommand");

    quote! {{
        use std::error::Error;
        let #app_var = #app_var #( #args )* ;
        #( #subcmds )*
        #app_var
    }}
}

fn gen_constructor(fields: &[Field]) -> quote::Tokens {
    let fields = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let name = gen_name(field);
        if is_subcommand(field) {
            let cur_type = ty(&field.ty);
            let subcmd_type = match (cur_type, sub_type(&field.ty)) {
                (Ty::Option, Some(sub_type)) => sub_type,
                _ => &field.ty
            };
            let unwrapper = match cur_type {
                Ty::Option => quote!(),
                _ => quote!( .unwrap() )
            };
            quote!( #field_name: #subcmd_type ::from_subcommand(matches.subcommand()) #unwrapper )
        } else {
            let convert = match ty(&field.ty) {
                Ty::Bool => quote!(is_present(stringify!(#name))),
                Ty::U64 => quote!(occurrences_of(stringify!(#name))),
                Ty::Option => quote! {
                    value_of(stringify!(#name))
                        .as_ref()
                        .map(|s| s.parse().unwrap())
                },
                Ty::Vec => quote! {
                    values_of(stringify!(#name))
                        .map(|v| v.map(|s| s.parse().unwrap()).collect())
                        .unwrap_or_else(Vec::new)
                },
                Ty::Other => quote! {
                    value_of(stringify!(#name))
                        .as_ref()
                        .unwrap()
                        .parse()
                        .unwrap()
                },
            };
            quote!( #field_name: matches.#convert )
        }
    });

    quote! {{
        #( #fields ),*
    }}
}

fn gen_name(field: &Field) -> Ident {
    extract_attrs(&field.attrs, AttrSource::Field)
        .filter(|&(ref i, _)| i.as_ref() == "name")
        .last()
        .and_then(|(_, ref l)| match l {
            &Lit::Str(ref s, _) => Some(Ident::new(s.clone())),
            _ => None,
        })
        .unwrap_or(field.ident.as_ref().unwrap().clone())
}

fn gen_from_clap(struct_name: &Ident, fields: &[Field]) -> quote::Tokens {
    let field_block = gen_constructor(fields);

    quote! {
        fn from_clap(matches: _structopt::clap::ArgMatches) -> Self {
            #struct_name #field_block
        }
    }
}

fn gen_clap(struct_attrs: &[Attribute], subcmd_required: bool) -> quote::Tokens {
    let struct_attrs: Vec<_> = extract_attrs(struct_attrs, AttrSource::Struct).collect();
    let name = from_attr_or_env(&struct_attrs, "name", "CARGO_PKG_NAME");
    let version = from_attr_or_env(&struct_attrs, "version", "CARGO_PKG_VERSION");
    let author = from_attr_or_env(&struct_attrs, "author", "CARGO_PKG_AUTHORS");
    let about = from_attr_or_env(&struct_attrs, "about", "CARGO_PKG_DESCRIPTION");
    let setting = if subcmd_required {
        quote!( .setting(_structopt::clap::AppSettings::SubcommandRequired) )
    } else {
        quote!()
    };

    quote! {
        fn clap<'a, 'b>() -> _structopt::clap::App<'a, 'b> {
            let app = _structopt::clap::App::new(#name)
                .version(#version)
                .author(#author)
                .about(#about)
                #setting
                ;
            Self::augment_clap(app)
        }
    }
}

fn gen_augment_clap(fields: &[Field]) -> quote::Tokens {
    let app_var = Ident::new("app");
    let augmentation = gen_augmentation(fields, &app_var);
    quote! {
        fn augment_clap<'a, 'b>(#app_var: _structopt::clap::App<'a, 'b>) -> _structopt::clap::App<'a, 'b> {
            #augmentation
        }
    }
}

fn gen_clap_enum(enum_attrs: &[Attribute]) -> quote::Tokens {
    let enum_attrs: Vec<_> = extract_attrs(enum_attrs, AttrSource::Struct).collect();
    let name = from_attr_or_env(&enum_attrs, "name", "CARGO_PKG_NAME");
    let version = from_attr_or_env(&enum_attrs, "version", "CARGO_PKG_VERSION");
    let author = from_attr_or_env(&enum_attrs, "author", "CARGO_PKG_AUTHORS");
    let about = from_attr_or_env(&enum_attrs, "about", "CARGO_PKG_DESCRIPTION");

    quote! {
        fn clap<'a, 'b>() -> _structopt::clap::App<'a, 'b> {
            let app = _structopt::clap::App::new(#name)
                .version(#version)
                .author(#author)
                .about(#about)
                .setting(_structopt::clap::AppSettings::SubcommandRequired);
            Self::augment_clap(app)
        }
    }
}

fn gen_augment_clap_enum(variants: &[Variant]) -> quote::Tokens {
    let subcommands = variants.iter().map(|variant| {
        let name = extract_attrs(&variant.attrs, AttrSource::Struct)
            .filter_map(|attr| match attr {
                (ref i, Lit::Str(ref s, ..)) if i == "name" => 
                    Some(Ident::new(s as &str)),
                _ => None
            })
            .next()
            .unwrap_or_else(|| variant.ident.clone());
        let app_var = Ident::new("subcommand");
        let arg_block = match variant.data {
            VariantData::Struct(ref fields) => gen_augmentation(fields, &app_var),
            _ => unreachable!()
        };

        quote! {
            .subcommand({
                let #app_var = _structopt::clap::SubCommand::with_name( stringify!(#name) );
                #arg_block
            })
        }
    });
   
    quote! {
        fn augment_clap<'a, 'b>(app: _structopt::clap::App<'a, 'b>) -> _structopt::clap::App<'a, 'b> {
            app #( #subcommands )*
        }
    }
}

fn gen_from_clap_enum(name: &Ident) -> quote::Tokens {
    quote! {
        fn from_clap(matches: _structopt::clap::ArgMatches) -> Self {
            #name ::from_subcommand(matches.subcommand())
                .unwrap()
        }
    }
}

fn gen_from_subcommand(name: &Ident, variants: &[Variant]) -> quote::Tokens {
    let match_arms = variants.iter().map(|variant| {
        let sub_name = extract_attrs(&variant.attrs, AttrSource::Struct)
            .filter_map(|attr| match attr {
                (ref i, Lit::Str(ref s, ..)) if i == "name" => 
                    Some(Ident::new(s as &str)),
                _ => None
            })
            .next()
            .unwrap_or_else(|| variant.ident.clone());
        let variant_name = &variant.ident;
        let constructor_block = match variant.data {
            VariantData::Struct(ref fields) => gen_constructor(fields),
            _ => unreachable!()
        };
        
        quote! {
            (stringify!(#sub_name), Some(matches)) =>
                Some(#name :: #variant_name #constructor_block)
        }
    });

    quote! {
        fn from_subcommand<'a, 'b>(sub: (&'b str, Option<&'b _structopt::clap::ArgMatches<'a>>)) -> Option<Self> {
            match sub {
                #( #match_arms ),*,
                _ => None
            }
        }
    }
}

fn impl_structopt_for_struct(name: &Ident, fields: &[Field], attrs: &[Attribute]) -> quote::Tokens {
    let subcmd_required = fields.iter().any(is_subcommand);
    let clap = gen_clap(attrs, subcmd_required);
    let augment_clap = gen_augment_clap(fields);
    let from_clap = gen_from_clap(name, fields);

    quote! {
        impl _structopt::StructOpt for #name {
            #clap
            #augment_clap
            #from_clap
        }
    }
}

fn impl_structopt_for_enum(name: &Ident, variants: &[Variant], attrs: &[Attribute]) -> quote::Tokens {
    if variants.iter().any(|variant| {
            if let VariantData::Struct(..) = variant.data { false } else { true }
        })
    {
        panic!("enum variants must use curly braces");
    }

    let clap = gen_clap_enum(attrs);
    let augment_clap = gen_augment_clap_enum(variants);
    let from_clap = gen_from_clap_enum(name);
    let from_subcommand = gen_from_subcommand(name, variants);

    quote! {
        impl _structopt::StructOpt for #name {
            #clap
            #augment_clap
            #from_clap
        }

        impl #name {
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

    let dummy_const = Ident::new(format!("_IMPL_STRUCTOPT_FOR_{}", struct_name));
    quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_imports)]
        const #dummy_const: () = {
            extern crate structopt as _structopt;
            use structopt::StructOpt;
            #inner_impl
        };
    }
}
