use std::hint::unreachable_unchecked;
use std::ffi::OsStr;
use std::mem;

#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
use std::os::unix::ffi::OsStrExt;

#[cfg(all(feature = "debug", any(target_os = "windows", target_arch = "wasm32")))]
use util::OsStrExt3;
use util::OsStrExt2;
use util::OsSplit;

use crate::parse::{ArgPrediction, HyphenStyle, ParseCtx};

pub enum RawArgKind {
    Key,
    Value,
    Unknown,
}

pub struct RawArg<'a>(pub(crate) &'a OsStr);

impl<'a> RawArg<'a> {
    pub fn make_prediction(&self, ctx: &ParseCtx) -> ArgPrediction {
        let hs = HyphenStyle::from(self);
        match ctx {
            ParseCtx::Initial | ParseCtx::ArgAcceptsVal => {
                match hs {
                    HyphenStyle::Single | HyphenStyle::Double => ArgPrediction::Key,
                    HyphenStyle::SingleOnly | HyphenStyle::None => ArgPrediction::PossibleValue,
                    HyphenStyle::DoubleOnly => ArgPrediction::TrailingValuesSignal,
                }
            },
            ParseCtx::TrailingValues | ParseCtx::ArgRequiresValue => { ArgPrediction::Value },

            _ => unreachable!(),
        }
    }
}

#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
impl<'a> OsStrExt3 for RawArg<'a> {
    fn from_bytes(b: &[u8]) -> &Self {
        unsafe { mem::transmute(b) }
    }
    fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }
}
#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
impl<'a> OsStrExt for RawArg<'a> {
    fn from_bytes(b: &[u8]) -> &Self {
        unsafe { mem::transmute(b) }
    }
    fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }
}

impl<'a> OsStrExt2 for RawArg<'a> {
    fn starts_with(&self, s: &[u8]) -> bool { self.0.starts_with(s) }

    fn contains_byte(&self, byte: u8) -> bool { self.0.contains_byte(byte) }

    fn split_at_byte(&self, byte: u8) -> (&OsStr, &OsStr) { self.0.split_at_byte(byte) }

    fn trim_left_matches(&self, byte: u8) -> &OsStr { self.0.trim_left_matches(byte) }

    fn split_at(&self, i: usize) -> (&OsStr, &OsStr) { self.0.split_at(i) }

    fn split(&self, b: u8) -> OsSplit { self.0.split(b) }
}

impl<'a> From<&'a OsStr> for RawArg<'a> {
    fn from(oss: &'a OsStr) -> Self {
        RawArg(oss)
    }
}