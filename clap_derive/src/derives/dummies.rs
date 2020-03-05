//! Dummy implementations that we emit along with an error.

use proc_macro2::Ident;
use proc_macro_error::append_dummy;
use quote::quote;

pub fn clap_struct(name: &Ident) {
    into_app(name);
    from_arg_matches(name);
    append_dummy(quote!( impl ::clap::Clap for #name {} ));
}

pub fn clap_enum(name: &Ident) {
    into_app(name);
    from_arg_matches(name);
    subcommand(name);
    append_dummy(quote!( impl ::clap::Clap for #name {} ));
}

pub fn into_app(name: &Ident) {
    append_dummy(quote! {
        impl ::clap::IntoApp for #name {
            fn into_app<'b>() -> ::clap::App {
                unimplemented!()
            }
            fn augment_clap<'b>(_app: ::clap::App) -> ::clap::App {
                unimplemented!()
            }
        }
    });
}

pub fn from_arg_matches(name: &Ident) {
    append_dummy(quote! {
        impl ::clap::FromArgMatches for #name {
            fn from_arg_matches(_m: &::clap::ArgMatches) -> Self {
                unimplemented!()
            }
        }
    });
}

pub fn subcommand(name: &Ident) {
    append_dummy(quote! {
        impl ::clap::Subcommand for #name {
            fn from_subcommand(_name: &str, _matches: Option<&::clap::ArgMatches>) -> Option<Self> {
                unimplemented!()
            }
            fn augment_subcommands(_app: ::clap::App) -> ::clap::App {
                unimplemented!()
            }
        }
    });
}
