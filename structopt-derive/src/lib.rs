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

#[proc_macro_derive(StructOpt)]
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

fn impl_structopt(ast: &syn::MacroInput) -> quote::Tokens {
    use syn::{Body, VariantData};
    let name = &ast.ident;
    let s = if let Body::Struct(VariantData::Struct(ref s)) = ast.body {
        s
    } else {
        panic!("Only struct is supported")
    };
    let args = s.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let modifier = match ty(&f.ty) {
            Ty::Bool => quote! {
                .max_values(0)
                .takes_value(false)
                .multiple(false)
            },
            Ty::U64 => quote! {
                .max_values(0)
                .takes_value(false)
                .multiple(true)
            },
            Ty::Option => quote! {
                .takes_value(true)
                .multiple(false)
            },
            Ty::Vec => quote! {
                .takes_value(true)
                .multiple(true)
            },
            Ty::Other => quote!{
                .takes_value(true)
                .multiple(false)
                .required(true)
            },
        };
        quote! {
            .arg(Arg::with_name(stringify!(#ident))
                 .long(stringify!(#ident))
                 #modifier)
        }
    });
    let fields = s.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let convert = match ty(&f.ty) {
            Ty::Bool => quote!(is_present(stringify!(#ident))),
            Ty::U64 => quote!(occurrences_of(stringify!(#ident))),
            Ty::Option => quote! {
                value_of(stringify!(#ident))
                    .as_ref()
                    .map(|s| s.parse().unwrap())
            },
            Ty::Vec => quote! {
                values_of(stringify!(#ident))
                    .map(|v| v.map(|s| s.parse().unwrap()).collect())
                    .unwrap_or_else(Vec::new)
            },
            Ty::Other => quote! {
                value_of(stringify!(#ident))
                    .as_ref()
                    .unwrap()
                    .parse()
                    .unwrap()
            },
        };
        quote! {
            #ident: matches.#convert,
        }
    });

    quote! {
        impl StructOpt for #name {
            fn clap<'a, 'b>() -> clap::App<'a, 'b> {
                use ::clap::Arg;
                app_from_crate!()
                    #( #args )*
            }
            fn from_clap(app: clap::App) -> Self {
                let matches = app.get_matches();
                #name {
                    #( #fields )*
                }
            }
        }
    }
}
