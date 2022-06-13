//! Dummy implementations that we emit along with an error.

use proc_macro2::Ident;
use proc_macro_error::append_dummy;
use quote::quote;

pub fn parser_struct(name: &Ident) {
    into_app(name);
    args(name);
    append_dummy(quote!( impl clap::Parser for #name {} ));
}

pub fn parser_enum(name: &Ident) {
    into_app(name);
    subcommand(name);
    append_dummy(quote!( impl clap::Parser for #name {} ));
}

pub fn into_app(name: &Ident) {
    append_dummy(quote! {
        #[allow(deprecated)]
        impl clap::CommandFactory for #name {
            fn into_app<'b>() -> clap::Command<'b> {
                unimplemented!()
            }
            fn into_app_for_update<'b>() -> clap::Command<'b> {
                unimplemented!()
            }
        }
    });
}

pub fn from_arg_matches(name: &Ident) {
    append_dummy(quote! {
        impl clap::FromArgMatches for #name {
            fn from_arg_matches(_m: &clap::ArgMatches) -> ::std::result::Result<Self, clap::Error> {
                unimplemented!()
            }
            fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> ::std::result::Result<(), clap::Error>{
                unimplemented!()
            }
        }
    });
}

pub fn subcommand(name: &Ident) {
    from_arg_matches(name);
    append_dummy(quote! {
        impl clap::Subcommand for #name {
            fn augment_subcommands(_cmd: clap::Command<'_>) -> clap::Command<'_> {
                unimplemented!()
            }
            fn augment_subcommands_for_update(_cmd: clap::Command<'_>) -> clap::Command<'_> {
                unimplemented!()
            }
            fn has_subcommand(name: &str) -> bool {
                unimplemented!()
            }
        }
    });
}

pub fn args(name: &Ident) {
    from_arg_matches(name);
    append_dummy(quote! {
        impl clap::Args for #name {
            fn augment_args(_cmd: clap::Command<'_>) -> clap::Command<'_> {
                unimplemented!()
            }
            fn augment_args_for_update(_cmd: clap::Command<'_>) -> clap::Command<'_> {
                unimplemented!()
            }
        }
    });
}

pub fn value_enum(name: &Ident) {
    append_dummy(quote! {
        impl clap::ValueEnum for #name {
            fn value_variants<'a>() -> &'a [Self]{
                unimplemented!()
            }
            fn from_str(_input: &str, _ignore_case: bool) -> ::std::result::Result<Self, String> {
                unimplemented!()
            }
            fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::PossibleValue<'a>>{
                unimplemented!()
            }
        }
    })
}
