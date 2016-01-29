use std::rc::Rc;
use std::fmt::Display;

use args::settings::ArgSettings;

#[doc(hidden)]
pub trait AnyArg<'n, 'e>: Display {
    fn name(&self) -> &'n str;
    fn overrides(&self) -> Option<&[&'e str]>;
    fn requires(&self) -> Option<&[&'e str]>;
    fn blacklist(&self) -> Option<&[&'e str]>;
    fn is_set(&self, ArgSettings) -> bool;
    fn set(&mut self, ArgSettings);
    fn has_switch(&self) -> bool;
    fn max_vals(&self) -> Option<u8>;
    fn min_vals(&self) -> Option<u8>;
    fn num_vals(&self) -> Option<u8>;
    fn possible_vals(&self) -> Option<&[&'e str]>;
    fn validator(&self) -> Option<&Rc<Fn(String) -> Result<(), String>>>;
    fn short(&self) -> Option<char>;
    fn long(&self) -> Option<&'e str>;
    fn val_delim(&self) -> Option<char>;
}
