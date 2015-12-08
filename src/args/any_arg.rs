use std::rc::Rc;

use args::settings::ArgSettings;

#[doc(hidden)]
pub trait AnyArg<'n> {
    fn name(&self) -> &'n str;
    fn overrides(&self) -> Option<&[&'n str]>;
    fn requires(&self) -> Option<&[&'n str]>;
    fn blacklist(&self) -> Option<&[&'n str]>;
    fn is_set(&self, ArgSettings) -> bool;
    fn set(&mut self, ArgSettings);
    fn has_switch(&self) -> bool;
    fn max_vals(&self) -> Option<u8>;
    fn min_vals(&self) -> Option<u8>;
    fn num_vals(&self) -> Option<u8>;
    fn possible_vals(&self) -> Option<&[&'n str]>;
    fn validator(&self) -> Option<&Rc<Fn(String) -> Result<(), String>>>;
    fn short(&self) -> Option<char>;
    fn long(&self) -> Option<&'n str>;
}
