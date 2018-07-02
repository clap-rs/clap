// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use proc_macro2;
use quote;
use syn;
use syn::punctuated;
use syn::token;

pub fn derive_arg_enum(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    unimplemented!()

    // let from_str_block = impl_from_str(ast)?;
    // let variants_block = impl_variants(ast)?;

    // quote! {
    //     #from_str_block
    //     #variants_block
    // }
}

/*
fn impl_from_str(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    let is_case_sensitive = ast.attrs.iter().any(|v| v.name() == "case_sensitive");
    let variants = variants(ast)?;

    let strings = variants
        .iter()
        .map(|ref variant| String::from(variant.ident.as_ref()))
        .collect::<Vec<_>>();

    // All of these need to be iterators.
    let ident_slice = [ident.clone()];
    let idents = ident_slice.iter().cycle();

    let for_error_message = strings.clone();

    let condition_function_slice = [match is_case_sensitive {
        true => quote! { str::eq },
        false => quote! { ::std::ascii::AsciiExt::eq_ignore_ascii_case },
    }];
    let condition_function = condition_function_slice.iter().cycle();

    Ok(quote! {
        impl ::std::str::FromStr for #ident {
            type Err = String;

            fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
                match input {
                    #(val if #condition_function(val, #strings) => Ok(#idents::#variants),)*
                    _ => Err({
                        let v = #for_error_message;
                        format!("valid values: {}",
                            v.join(" ,"))
                    }),
                }
            }
        }
    })
}

fn impl_variants(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    let variants = variants(ast)?
        .iter()
        .map(|ref variant| String::from(variant.ident.as_ref()))
        .collect::<Vec<_>>();
    let length = variants.len();

    Ok(quote! {
        impl #ident {
            fn variants() -> [&'static str; #length] {
                #variants
            }
        }
    })
}

fn variants(ast: &syn::DeriveInput) -> &punctuated::Punctuated<syn::Variant, token::Comma> {
    use syn::Data::*;

    match ast.data {
        Enum(ref data) => data.variants,
        _ => panic!("Only enums are supported for deriving the ArgEnum trait"),
    }
}
*/
