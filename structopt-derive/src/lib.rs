// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;

#[proc_macro_derive(StructOpt, attributes(structopt))]
pub fn structopt(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_structopt(&ast);
    gen.parse().unwrap()
}

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

fn extract_attrs<'a>(attrs: &'a [Attribute]) -> Box<Iterator<Item = (&'a Ident, &'a Lit)> + 'a> {
    let iter = attrs.iter()
        .filter_map(|attr| match attr.value {
            MetaItem::List(ref i, ref v) if i.as_ref() == "structopt" => Some(v),
            _ => None,
        }).flat_map(|v| v.iter().filter_map(|mi| match *mi {
            NestedMetaItem::MetaItem(MetaItem::NameValue(ref i, ref l)) => Some((i, l)),
            _ => None,
        }));
    Box::new(iter)
}

fn from_attr_or(attrs: &[(&Ident, &Lit)], key: &str, default: &str) -> Lit {
    attrs.iter()
        .find(|&&(i, _)| i.as_ref() == key)
        .map(|&(_, l)| l.clone())
        .unwrap_or_else(|| Lit::Str(default.into(), StrStyle::Cooked))
}

fn gen_name(field: &Field) -> Ident {
    extract_attrs(&field.attrs)
        .find(|&(i, _)| i.as_ref() == "name")
        .and_then(|(_, l)| match *l {
            Lit::Str(ref s, _) => Some(Ident::new(s.clone())),
            _ => None,
        })
        .unwrap_or(field.ident.as_ref().unwrap().clone())
}

fn gen_from_clap(struct_name: &Ident, s: &[Field]) -> quote::Tokens {
    let fields = s.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let name = gen_name(field);
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
        quote!( #field_name: matches.#convert, )
    });
    quote! {
        fn from_clap(matches: _clap::ArgMatches) -> Self {
            #struct_name {
                #( #fields )*
            }
        }
    }
}

fn gen_clap(ast: &DeriveInput, s: &[Field]) -> quote::Tokens {
    let struct_attrs: Vec<_> = extract_attrs(&ast.attrs).collect();
    let name = from_attr_or(&struct_attrs, "name", env!("CARGO_PKG_NAME"));
    let version = from_attr_or(&struct_attrs, "version", env!("CARGO_PKG_VERSION"));
    let author = from_attr_or(&struct_attrs, "author", env!("CARGO_PKG_AUTHORS"));
    let about = from_attr_or(&struct_attrs, "about", env!("CARGO_PKG_DESCRIPTION"));

    let args = s.iter().map(|field| {
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
                let required = extract_attrs(&field.attrs)
                    .find(|&(i, _)| i.as_ref() == "default_value")
                    .is_none();
                quote!( .takes_value(true).multiple(false).required(#required).#validator )
            },
        };
        let from_attr = extract_attrs(&field.attrs)
            .filter(|&(i, _)| i.as_ref() != "name")
            .map(|(i, l)| quote!(.#i(#l)));
        quote!( .arg(_clap::Arg::with_name(stringify!(#name)) #modifier #(#from_attr)*) )
    });
    quote! {
        fn clap<'a, 'b>() -> _clap::App<'a, 'b> {
            use std::error::Error;
            _clap::App::new(#name)
                .version(#version)
                .author(#author)
                .about(#about)
                #( #args )*
        }
    }
}

fn impl_structopt(ast: &syn::DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;
    let s = match ast.body {
        Body::Struct(VariantData::Struct(ref s)) => s,
        _ => panic!("Only struct is supported"),
    };

    let clap = gen_clap(ast, s);
    let from_clap = gen_from_clap(struct_name, s);
    let dummy_const = Ident::new(format!("_IMPL_STRUCTOPT_FOR_{}", struct_name));
    quote! {
        #[allow(non_upper_case_globals, unused_attributes, unused_imports)]
        const #dummy_const: () = {
            extern crate clap as _clap;
            extern crate structopt as _structopt;
            impl _structopt::StructOpt for #struct_name {
                #clap
                #from_clap
            }
        };
    }
}
