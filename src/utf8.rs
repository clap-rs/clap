// use std::ffi::OsStr;
// use std::borrow::Cow;
//
// pub trait Utf8Rule { type Out; fn into(&OsStr) -> <Self as Utf8Rule>::Out; }
//
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct Strict<'a>;
// impl<'a> Utf8Rule for Strict<'a> { type Out = &'a str; }
//
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct Lossy<'a>;
// impl<'a> Utf8Rule for Lossy<'a> { type Out = Cow<'a, str>; }
//
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct AllowInvalid<'a>;
// impl<'a> Utf8Rule for AllowInvalid<'a> { type Out = &'a OsStr; }
//
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub enum Utf8 {
//     Strict,
//     Lossy,
//     AllowInvalid,
// }
//
// impl Utf8 {
//     pub fn into<U: UtfRule>(&self) -> U::Out {
//         match *self {
//             Utf::Strict => Strict::,
//             Utf::Lossy =>,
//             Utf::AllowInvalid =>,
//         }
//     }
// }

pub const INVALID_UTF8: &'static str = "unexpected invalid UTF-8 code point";
