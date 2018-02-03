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

#[derive(Debug, Clone, Copy)]
enum AttrSource { Struct, Field, }

#[derive(Debug, PartialEq)]
enum Parser {
    /// Parse an option to using a `fn(&str) -> T` function. The function should never fail.
    FromStr,
    /// Parse an option to using a `fn(&str) -> Result<T, E>` function. The error will be
    /// converted to a string using `.to_string()`.
    TryFromStr,
    /// Parse an option to using a `fn(&OsStr) -> T` function. The function should never fail.
    FromOsStr,
    /// Parse an option to using a `fn(&OsStr) -> Result<T, OsString>` function.
    TryFromOsStr,
    /// Counts the number of flag occurrences. Parses using a `fn(u64) -> T` function. The function
    /// should never fail.
    FromOccurrences,
}

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

    let doc_comments: Vec<String> = attrs.iter()
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
                Some(text.into())
            } else {
                None
            }
        })
        .collect();

    let doc_comments = if doc_comments.is_empty() {
        None
    } else {
        // Clap's `App` has an `about` method to set a description,
        // it's `Field`s have a `help` method instead.
        if let AttrSource::Struct = attr_source {
            Some(("about".into(), doc_comments.join(" ").into()))
        } else {
            Some(("help".into(), doc_comments.join(" ").into()))
        }
    };

    Box::new(doc_comments.into_iter().chain(settings_attrs))
}

fn from_attr_or_env(attrs: &[(Ident, Lit)], key: &str, env: &str) -> String {
    let default = std::env::var(env).unwrap_or("".into());
    attrs.iter()
        .filter(|&&(ref i, _)| i.as_ref() == key)
        .last()
        .and_then(|&(_, ref l)| match *l {
            Lit::Str(ref s, _) => Some(s.clone()),
            _ => None
        })
        .unwrap_or(default)
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

fn get_default_parser() -> (Parser, quote::Tokens) {
    (Parser::TryFromStr, quote!(::std::str::FromStr::from_str))
}

fn get_parser(field: &Field) -> Option<(Parser, quote::Tokens)> {
    field.attrs.iter()
        .flat_map(|attr| {
            if let MetaItem::List(ref i, ref l) = attr.value {
                if i == "structopt" {
                    return &**l;
                }
            }
            &[]
        })
        .filter_map(|attr| {
            if let NestedMetaItem::MetaItem(MetaItem::List(ref i, ref l)) = *attr {
                if i == "parse" {
                    return l.first();
                }
            }
            None
        })
        .map(|attr| {
            match *attr {
                NestedMetaItem::MetaItem(MetaItem::NameValue(ref i, Lit::Str(ref v, _))) => {
                    let function = parse_path(v).expect("parser function path");
                    let parser = match i.as_ref() {
                        "from_str" => Parser::FromStr,
                        "try_from_str" => Parser::TryFromStr,
                        "from_os_str" => Parser::FromOsStr,
                        "try_from_os_str" => Parser::TryFromOsStr,
                        "from_occurrences" => Parser::FromOccurrences,
                        _ => panic!("unsupported parser {}", i)
                    };

                    (parser, quote!(#function))
                }
                NestedMetaItem::MetaItem(MetaItem::Word(ref i)) => {
                    match i.as_ref() {
                        "from_str" => (Parser::FromStr, quote!(::std::convert::From::from)),
                        "try_from_str" => (Parser::TryFromStr, quote!(::std::str::FromStr::from_str)),
                        "from_os_str" => (Parser::FromOsStr, quote!(::std::convert::From::from)),
                        "try_from_os_str" => panic!("cannot omit parser function name with `try_from_os_str`"),
                        "from_occurrences" => (Parser::FromOccurrences, quote!({|v| v as _})),
                        _ => panic!("unsupported parser {}", i)
                    }
                }
                _ => panic!("unknown value parser specification"),
            }
        })
        .next()
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
        .filter(|&field| is_subcommand(field))
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
        .filter(|&field| !is_subcommand(field))
        .map(|field| {
            let name = gen_name(field);
            let mut cur_type = ty(&field.ty);
            let convert_type = match cur_type {
                Ty::Vec | Ty::Option => sub_type(&field.ty).unwrap_or(&field.ty),
                _ => &field.ty,
            };

            let mut occurences = false;
            let parser = get_parser(field);
            if let Some((ref parser, _)) = parser {
                cur_type = convert_with_custom_parse(cur_type);
                occurences = *parser == Parser::FromOccurrences;
            }

            let validator = match parser.unwrap_or_else(get_default_parser) {
                (Parser::TryFromStr, f) => quote! {
                    .validator(|s| {
                        #f(&s)
                            .map(|_: #convert_type| ())
                            .map_err(|e| e.to_string())
                    })
                },
                (Parser::TryFromOsStr, f) => quote! {
                    .validator_os(|s| #f(&s).map(|_: #convert_type| ()))
                },
                _ => quote! {},
            };

            let modifier = match cur_type {
                Ty::Bool => quote!( .takes_value(false).multiple(false) ),
                Ty::Option => quote!( .takes_value(true).multiple(false) #validator ),
                Ty::Vec => quote!( .takes_value(true).multiple(true) #validator ),
                Ty::Other if occurences => quote!( .takes_value(false).multiple(true) ),
                Ty::Other => {
                    let required = extract_attrs(&field.attrs, AttrSource::Field)
                        .find(|&(ref i, _)| i.as_ref() == "default_value"
                              || i.as_ref() == "default_value_raw")
                        .is_none();
                    quote!( .takes_value(true).multiple(false).required(#required) #validator )
                },
            };
            let from_attr = extract_attrs(&field.attrs, AttrSource::Field)
                .filter(|&(ref i, _)| i.as_ref() != "name")
                .map(|(i, l)| gen_attr_call(&i, &l));
            quote!( .arg(::structopt::clap::Arg::with_name(stringify!(#name)) #modifier #(#from_attr)*) )
        });

    quote! {{
        let #app_var = #app_var #( #args )* ;
        #( #subcmds )*
        #app_var
    }}
}

/// Interpret the value of `*_raw` attributes as code and the rest as strings.
fn gen_attr_call(key: &syn::Ident, val: &syn::Lit) -> quote::Tokens {
    if let Lit::Str(ref val, _) = *val {
        let key = key.as_ref();
        if key.ends_with("_raw") {
            let key = Ident::from(&key[..(key.len() - 4)]);
            // Call method without quoting the string
            let ts = syn::parse_token_trees(val)
                .expect(&format!("bad parameter {} = {}: the parameter must be valid rust code", key, val));
            return quote!(.#key(#(#ts)*));
        }
    }
    quote!(.#key(#val))
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
            let real_ty = &field.ty;
            let mut cur_type = ty(real_ty);
            let parser = get_parser(field);
            if parser.is_some() {
                cur_type = convert_with_custom_parse(cur_type);
            }

            let parser = parser.unwrap_or_else(get_default_parser);
            let (value_of, values_of, parse) = match parser {
                (Parser::FromStr, f) => (
                    quote!(value_of),
                    quote!(values_of),
                    f,
                ),
                (Parser::TryFromStr, f) => (
                    quote!(value_of),
                    quote!(values_of),
                    quote!(|s| #f(s).unwrap()),
                ),
                (Parser::FromOsStr, f) => (
                    quote!(value_of_os),
                    quote!(values_of_os),
                    f,
                ),
                (Parser::TryFromOsStr, f) => (
                    quote!(value_of_os),
                    quote!(values_of_os),
                    quote!(|s| #f(s).unwrap()),
                ),
                (Parser::FromOccurrences, f) => (
                    quote!(occurrences_of),
                    quote!(),
                    f,
                ),
            };

            let occurences = parser.0 == Parser::FromOccurrences;
            let field_value = match cur_type {
                Ty::Bool => quote!(matches.is_present(stringify!(#name))),
                Ty::Option => quote! {
                    matches.#value_of(stringify!(#name))
                        .as_ref()
                        .map(#parse)
                },
                Ty::Vec => quote! {
                    matches.#values_of(stringify!(#name))
                        .map(|v| v.map(#parse).collect())
                        .unwrap_or_else(Vec::new)
                },
                Ty::Other if occurences => quote! {
                    #parse(matches.#value_of(stringify!(#name)))
                },
                Ty::Other => quote! {
                    matches.#value_of(stringify!(#name))
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
        fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
            #struct_name #field_block
        }
    }
}

fn format_author(raw_authors: String) -> String {
    raw_authors.replace(":", ", ")
}

fn method_if_arg(method: &str, arg: &str) -> Option<quote::Tokens> {
    if arg.is_empty() {
        None
    } else {
        let method: Ident = method.into();
        Some(quote!(.#method(#arg)))
    }
}

fn gen_clap(attrs: &[Attribute]) -> quote::Tokens {
    let attrs: Vec<_> = extract_attrs(attrs, AttrSource::Struct).collect();
    let name: Lit = from_attr_or_env(&attrs, "name", "CARGO_PKG_NAME").into();
    let version = from_attr_or_env(&attrs, "version", "CARGO_PKG_VERSION");
    let version = method_if_arg("version", &version);
    let author = format_author(from_attr_or_env(&attrs, "author", "CARGO_PKG_AUTHORS"));
    let author = method_if_arg("author", &author);
    let about = from_attr_or_env(&attrs, "about", "CARGO_PKG_DESCRIPTION");
    let about = method_if_arg("about", &about);
    let settings = attrs.iter()
        .filter(|&&(ref i, _)| !["name", "version", "author", "about"].contains(&i.as_ref()))
        .map(|&(ref i, ref l)| gen_attr_call(i, l))
        .collect::<Vec<_>>();

    quote! {
        ::structopt::clap::App::new(#name)
            #version
            #author
            #about
            #( #settings )*
    }
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
        let name = extract_attrs(&variant.attrs, AttrSource::Struct)
            .filter_map(|attr| match attr {
                (ref i, Lit::Str(ref s, ..)) if i == "name" =>
                    Some(s.to_string()),
                _ => None
            })
            .next()
            .unwrap_or_else(|| variant.ident.to_string());
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
        let from_attr = extract_attrs(&variant.attrs, AttrSource::Struct)
            .filter(|&(ref i, _)| i != "name")
            .map(|(i, l)| gen_attr_call(&i, &l));

        quote! {
            .subcommand({
                let #app_var = ::structopt::clap::SubCommand::with_name( #name )
                    #( #from_attr )* ;
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
        #[doc(hidden)]
        fn from_clap(matches: &::structopt::clap::ArgMatches) -> Self {
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
                    Some(s.to_string()),
                _ => None
            })
            .next()
            .unwrap_or_else(|| variant.ident.as_ref().to_string());
        let variant_name = &variant.ident;
        let constructor_block = match variant.data {
            VariantData::Struct(ref fields) => gen_constructor(fields),
            VariantData::Unit => quote!(),
            VariantData::Tuple(ref fields) if fields.len() == 1 => {
                let ty = &fields[0];
                quote!( ( <#ty as ::structopt::StructOpt>::from_clap(matches) ) )
            }
            VariantData::Tuple(..) => panic!("{}: tuple enum are not supported", variant.ident),
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
