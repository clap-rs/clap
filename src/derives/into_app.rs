// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
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
use std::env;

use proc_macro2;
use syn;

use derives::Attrs;

pub fn derive_into_app(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    use syn::Data::*;

    let struct_name = &input.ident;
    let inner_impl = match input.data {
        Struct(syn::DataStruct { .. }) => gen_into_app_impl_for_struct(struct_name, &input.attrs),
        // @TODO impl into_app for enums?
        // Enum(ref e) => clap_for_enum_impl(struct_name, &e.variants, &input.attrs),
        _ => panic!("clap_derive only supports non-tuple structs"), // and enums"),
    };

    quote!(#inner_impl)
}

pub fn gen_into_app_impl_for_struct(
    name: &syn::Ident,
    attrs: &[syn::Attribute],
) -> proc_macro2::TokenStream {
    let into_app_fn = gen_into_app_fn_for_struct(attrs);

    quote! {
        impl ::clap::IntoApp for #name {
            #into_app_fn
        }

        impl<'a, 'b> Into<::clap::App<'a, 'b>> for #name {
            fn into(self) -> ::clap::App<'a, 'b> {
                <#name as ::clap::IntoApp>::into_app()
            }
        }
    }
}

pub fn gen_into_app_fn_for_struct(struct_attrs: &[syn::Attribute]) -> proc_macro2::TokenStream {
    let app = gen_app_builder(struct_attrs);
    quote! {
        fn into_app<'a, 'b>() -> ::clap::App<'a, 'b> {
            Self::augment_app(#app)
        }
    }
}

pub fn gen_app_builder(attrs: &[syn::Attribute]) -> proc_macro2::TokenStream {
    let name = env::var("CARGO_PKG_NAME")
        .ok()
        .unwrap_or_else(String::default);
    let attrs = Attrs::from_struct(attrs, name);
    let name = attrs.name();
    let methods = attrs.methods();
    quote!(::clap::App::new(#name)#methods)
}

pub fn gen_into_app_impl_for_enum(
    name: &syn::Ident,
    attrs: &[syn::Attribute],
) -> proc_macro2::TokenStream {
    let into_app_fn = gen_into_app_fn_for_enum(attrs);

    quote! {
        impl ::clap::IntoApp for #name {
            #into_app_fn
        }

        impl<'a, 'b> Into<::clap::App<'a, 'b>> for #name {
            fn into(self) -> ::clap::App<'a, 'b> {
                <#name as ::clap::IntoApp>::into_app()
            }
        }
    }
}

pub fn gen_into_app_fn_for_enum(enum_attrs: &[syn::Attribute]) -> proc_macro2::TokenStream {
    let gen = gen_app_builder(enum_attrs);
    quote! {
        fn into_app<'a, 'b>() -> ::clap::App<'a, 'b> {
            let app = #gen
                .setting(::clap::AppSettings::SubcommandRequiredElseHelp);
            Self::augment_app(app)
        }
    }
}
