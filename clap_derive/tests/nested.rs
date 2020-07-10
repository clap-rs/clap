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

use clap::Clap;

// Tests that clap_derive properly detects an `Option` field
// that results from a macro expansion
#[test]
fn use_option() {
    macro_rules! expand_ty {
        ($name:ident: $ty:ty) => {
            #[derive(Clap)]
            struct Outer {
                #[clap(short, long)]
                #[allow(dead_code)]
                $name: $ty,
            }
        };
    }

    expand_ty!(my_field: Option<String>);
}
