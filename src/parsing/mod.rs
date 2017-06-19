#[macro_use]
mod macros;
mod arg_matcher;
mod osstringext;
mod parser;
mod strext;
mod validator;

// pub use self::any_arg::AnyArg;
pub use self::arg_matcher::ArgMatcher;
#[cfg(target_os = "windows")]
pub use self::osstringext::OsStrExt3;
pub use self::osstringext::OsStrExt2;
// pub use self::validator::Validator;
pub use self::parser::{Parser, ParseResult};

pub trait DispOrder {
    fn disp_ord(&self) -> usize;
}
