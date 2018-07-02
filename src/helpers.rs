// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use errors::*;
use syn::Body::Enum;
use syn::{DeriveInput, Variant};

pub fn variants(ast: &DeriveInput) -> Result<&Vec<Variant>> {
    match ast.body {
        Enum(ref variants) => Ok(variants),
        _ => Err(ErrorKind::WrongBodyType("enum"))?,
    }
}
