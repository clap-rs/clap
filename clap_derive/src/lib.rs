// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
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

#![doc(html_logo_url = "https://raw.githubusercontent.com/clap-rs/clap/master/assets/clap.png")]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attrs;
mod derives;
mod dummies;
mod parse;
mod utils;

/// Generates the `ArgEnum` impl.
#[proc_macro_derive(ArgEnum, attributes(clap))]
#[proc_macro_error]
pub fn arg_enum(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::derive_arg_enum(&input).into()
}

/// Generates the `Parser` implementation.
///
/// This is far less verbose than defining the `clap::Command` struct manually,
/// receiving an instance of `clap::ArgMatches` from conducting parsing, and then
/// implementing a conversion code to instantiate an instance of the user
/// context struct.
#[proc_macro_derive(Parser, attributes(clap, structopt))]
#[proc_macro_error]
pub fn parser(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::derive_parser(&input).into()
}

/// Generates the `Subcommand` impl.
#[proc_macro_derive(Subcommand, attributes(clap))]
#[proc_macro_error]
pub fn subcommand(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::derive_subcommand(&input).into()
}

/// Generates the `Args` impl.
#[proc_macro_derive(Args, attributes(clap))]
#[proc_macro_error]
pub fn args(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::derive_args(&input).into()
}

/// Adds an argument to the main function, the argument type must implement [`clap::Parser`][clap::Parser]
/// ## Examples
/// ```rust
///
/// #[derive(clap::Parser)]
/// #[clap(name = "demo")]
/// struct Context {
///     /// More verbose output
///     #[clap(long)]
///     verbose: bool,
///     /// An optional name
///     #[clap(short, long)]
///     name: Option<String>,
/// }
///
///
/// #[clap::main]
/// fn main(args: Context) {
///     println!("{:?}", args.name);
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let asyncness = &input.sig.asyncness;
    let attrs = &input.attrs;

    if name != "main" {
        abort_call_site!("only fn main can be tagged with #[clap::main]");
    }
    let end = match ret {
        syn::ReturnType::Default => quote! {.unwrap()},
        _ => quote! {?},
    };
    let inputs = &input.sig.inputs;
    let result = match inputs.len() {
        0 => {
            quote! {
                #(#attrs)*
                #asyncness fn main() #ret {
                    #body
                }
            }
        }
        1 => {
            let arg = match inputs.first().unwrap() {
                syn::FnArg::Typed(arg) => arg,
                _ => {
                    abort_call_site!("fn main should take a fully formed argument");
                }
            };
            let arg_name = &arg.pat;
            let arg_type = &arg.ty;
            quote! {
                #(#attrs)*
                #asyncness fn main() #ret {
                    let #arg_name = <#arg_type as clap::Parser>::try_parse()#end;
                    #body
                }

            }
        }
        _ => {
            abort_call_site!("fn main can take 0 or 1 arguments");
        }
    };
    result.into()
}
