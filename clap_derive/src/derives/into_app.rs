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

use super::{spanned::Sp, Attrs, GenOutput, Name, DEFAULT_CASING, DEFAULT_ENV_CASING};

pub fn derive_into_app(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    use syn::Data::*;

    let struct_name = &input.ident;
    let inner_impl = match input.data {
        Struct(syn::DataStruct { .. }) => {
            gen_into_app_impl_for_struct(struct_name, &input.attrs).tokens
        }
        // @TODO impl into_app for enums?
        // Enum(ref e) => clap_for_enum_impl(struct_name, &e.variants, &input.attrs),
        _ => panic!("clap_derive only supports non-tuple structs"), // and enums"),
    };

    quote!(#inner_impl)
}

pub fn gen_into_app_impl_for_struct(name: &syn::Ident, attrs: &[syn::Attribute]) -> GenOutput {
    let into_app_fn = gen_into_app_fn_for_struct(attrs);
    let into_app_fn_tokens = into_app_fn.tokens;

    let tokens = quote! {
        impl ::clap::IntoApp for #name {
            #into_app_fn_tokens
        }

        impl<'b> Into<::clap::App<'b>> for #name {
            fn into(self) -> ::clap::App<'b> {
                use ::clap::IntoApp;
                <#name as ::clap::IntoApp>::into_app()
            }
        }
    };

    GenOutput {
        tokens,
        attrs: into_app_fn.attrs,
    }
}

pub fn gen_into_app_fn_for_struct(struct_attrs: &[syn::Attribute]) -> GenOutput {
    let gen = gen_app_builder(struct_attrs);
    let app_tokens = gen.tokens;

    let tokens = quote! {
        fn into_app<'b>() -> ::clap::App<'b> {
            Self::augment_app(#app_tokens)
        }
    };

    GenOutput {
        tokens,
        attrs: gen.attrs,
    }
}

pub fn gen_app_builder(attrs: &[syn::Attribute]) -> GenOutput {
    let name = env::var("CARGO_PKG_NAME").ok().unwrap_or_default();

    let attrs = Attrs::from_struct(
        proc_macro2::Span::call_site(),
        attrs,
        Name::Assigned(syn::LitStr::new(&name, proc_macro2::Span::call_site())),
        Sp::call_site(DEFAULT_CASING),
        Sp::call_site(DEFAULT_ENV_CASING),
    );
    let tokens = {
        let name = attrs.cased_name();
        quote!(::clap::App::new(#name))
    };

    GenOutput { tokens, attrs }
}

pub fn gen_into_app_impl_for_enum(name: &syn::Ident, attrs: &[syn::Attribute]) -> GenOutput {
    let into_app_fn = gen_into_app_fn_for_enum(attrs);
    let into_app_fn_tokens = into_app_fn.tokens;

    let tokens = quote! {
        impl ::clap::IntoApp for #name {
            #into_app_fn_tokens
        }

        impl<'b> Into<::clap::App<'b>> for #name {
            fn into(self) -> ::clap::App<'b> {
                use ::clap::IntoApp;
                <#name as ::clap::IntoApp>::into_app()
            }
        }
    };

    GenOutput {
        tokens,
        attrs: into_app_fn.attrs,
    }
}

pub fn gen_into_app_fn_for_enum(enum_attrs: &[syn::Attribute]) -> GenOutput {
    let gen = gen_app_builder(enum_attrs);
    let app_tokens = gen.tokens;

    let tokens = quote! {
        fn into_app<'b>() -> ::clap::App<'b> {
            let app = #app_tokens
                .setting(::clap::AppSettings::SubcommandRequiredElseHelp);
            Self::augment_app(app)
        }
    };

    GenOutput {
        tokens,
        attrs: gen.attrs,
    }
}
