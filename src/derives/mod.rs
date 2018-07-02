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
use syn;

pub mod arg_enum;
pub mod attrs;
mod clap;

pub use self::arg_enum::derive_arg_enum;
pub use self::clap::derive_clap;

fn sub_type(t: &syn::Type) -> Option<&syn::Type> {
    let segs = match *t {
        syn::Type::Path(syn::TypePath {
            path: syn::Path { ref segments, .. },
            ..
        }) => segments,
        _ => return None,
    };
    match *segs.iter().last().unwrap() {
        syn::PathSegment {
            arguments:
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    ref args, ..
                }),
            ..
        } if args.len() == 1 =>
        {
            if let syn::GenericArgument::Type(ref ty) = args[0] {
                Some(ty)
            } else {
                None
            }
        }
        _ => None,
    }
}
