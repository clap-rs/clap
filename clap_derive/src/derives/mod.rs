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
pub mod arg_enum;
pub mod attrs;
mod clap;
mod from_argmatches;
mod into_app;
pub mod parse;
pub mod spanned;
pub mod ty;

// pub use self::arg_enum::derive_arg_enum;
pub use self::attrs::{
    Attrs, CasingStyle, GenOutput, Kind, Name, Parser, ParserKind, DEFAULT_CASING,
    DEFAULT_ENV_CASING,
};
pub use self::clap::derive_clap;
pub use self::from_argmatches::derive_from_argmatches;
pub use self::into_app::derive_into_app;
pub use self::ty::{sub_type, Ty};
