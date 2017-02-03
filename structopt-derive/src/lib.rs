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
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = impl_structopt(&ast);
    gen.parse().unwrap()
}

fn impl_structopt(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl StructOpt for #name {
            fn clap<'a, 'b>() -> clap::App<'a, 'b> {
                app_from_crate!()
            }
            fn from_clap(app: clap::App) -> Self {
                let _ = app.get_matches();
                Self::default()
            }
        }
    }
}
