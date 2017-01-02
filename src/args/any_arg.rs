// Std
use std::rc::Rc;
use std::fmt as std_fmt;
use std::ffi::{OsStr, OsString};

// Third Party
use vec_map::{self, VecMap};

// Internal
use args::settings::ArgSettings;
use args::ArgKind;

#[doc(hidden)]
pub trait AnyArg<'n, 'e>: std_fmt::Display {
    fn name(&self) -> &'n str;
    fn id(&self) -> usize;
    fn overrides(&self) -> Option<&[&'e str]>;
    fn aliases(&self) -> Option<Vec<&'e str>>;
    fn requires(&self) -> Option<&[(Option<&'e str>, &'n str)]>;
    fn blacklist(&self) -> Option<&[&'e str]>;
    fn required_unless(&self) -> Option<&[&'e str]>;
    fn is_set(&self, ArgSettings) -> bool;
    fn set(&mut self, ArgSettings);
    fn has_switch(&self) -> bool;
    fn max_vals(&self) -> Option<u64>;
    fn min_vals(&self) -> Option<u64>;
    fn num_vals(&self) -> Option<u64>;
    fn possible_vals(&self) -> Option<&[&'e str]>;
    fn validator(&self) -> Option<&Rc<Fn(String) -> Result<(), String>>>;
    fn validator_os(&self) -> Option<&Rc<Fn(&OsStr) -> Result<(), OsString>>>;
    fn short(&self) -> Option<char>;
    fn long(&self) -> Option<&'e str>;
    fn val_delim(&self) -> Option<char>;
    fn takes_value(&self) -> bool;
    fn val_names(&self) -> Option<&VecMap<&'e str>>;
    fn help(&self) -> Option<&'e str>;
    fn default_val(&self) -> Option<&'n str>;
    fn default_vals_ifs(&self) -> Option<vec_map::Values<(&'n str, Option<&'e str>, &'e str)>>;
    fn longest_filter(&self) -> bool;
    fn kind(&self) -> ArgKind;
    fn val_terminator(&self) -> Option<&'e str>;
}

pub trait DispOrder {
    fn disp_ord(&self) -> usize;
}
