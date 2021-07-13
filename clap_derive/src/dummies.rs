//! Dummy implementations that we emit along with an error.

use proc_macro2::Ident;
use proc_macro_error::append_dummy;
use quote::quote;

pub fn clap_struct(name: &Ident) {
    into_app(name);
    from_arg_matches(name);
    append_dummy(quote!( impl clap::Clap for #name {} ));
}

pub fn clap_enum(name: &Ident) {
    into_app(name);
    from_arg_matches(name);
    subcommand(name);
    arg_enum(name);
    append_dummy(quote!( impl clap::Clap for #name {} ));
}

pub fn into_app(name: &Ident) {
    append_dummy(quote! {
        impl clap::IntoApp for #name {
            fn into_app<'b>() -> clap::App<'b> {
                unimplemented!()
            }
            fn augment_clap<'b>(_app: clap::App<'b>) -> clap::App<'b> {
                unimplemented!()
            }
            fn into_app_for_update<'b>() -> clap::App<'b> {
                unimplemented!()
            }
            fn augment_clap_for_update<'b>(_app: clap::App<'b>) -> clap::App<'b> {
                unimplemented!()
            }
        }
    });
}

pub fn from_arg_matches(name: &Ident) {
    append_dummy(quote! {
        impl clap::FromArgMatches for #name {
            fn try_from_arg_matches(_m: &clap::ArgMatches) -> clap::Result<Self> {
                unimplemented!()
            }
            fn try_update_from_arg_matches(
                &mut self,
                matches: &clap::ArgMatches
            ) -> clap::Result<()> {
                unimplemented!()
            }
        }
    });
}

pub fn subcommand(name: &Ident) {
    append_dummy(quote! {
        impl clap::Subcommand for #name {
            fn from_subcommand(_sub: Option<(&str, &clap::ArgMatches)>) -> clap::Result<Self> {
                unimplemented!()
            }
            fn update_from_subcommand(
                &mut self,
                _sub: Option<(&str, &clap::ArgMatches)>
            ) -> clap::Result<()> {
                unimplemented!()
            }
            fn augment_subcommands(_app: clap::App<'_>) -> clap::App<'_> {
                unimplemented!()
            }
            fn augment_subcommands_for_update(_app: clap::App<'_>) -> clap::App<'_> {
                unimplemented!()
            }
        }
    });
}

pub fn arg_enum(name: &Ident) {
    append_dummy(quote! {
        impl clap::ArgEnum for #name {
            const VARIANTS: &'static [&'static str] = &[];
            fn from_str(_input: &str, _case_insensitive: bool) -> Result<Self, String> {
                unimplemented!()
            }
        }
    })
}
