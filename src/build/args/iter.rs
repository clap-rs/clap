mod flags;
mod options;
mod positionals;

pub use self::flags::{Flags, FlagsMut};
pub use self::options::{Options, OptionsMut};
pub use self::positionals::{Positionals, PositionalsMut};

use std::slice::{Iter, IterMut};
use std::iter::Filter;

use crate::INTERNAL_ERROR_MESSAGE;
use crate::Arg;
use crate::build::args::{ArgId, Position};

pub trait QueryArgs<'help> {
    fn find_by_id(&self, Argid) -> Option<&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_short(&self, char) -> Option<&Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_long(&self, &str) -> Option<&Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_position(&self, Position) -> Option<&Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn visible(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn hidden(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn global(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn required(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
}

pub trait QueryArgsMut<'help> {
    fn find_by_id_mut(&mut self, Argid) -> Option<&mut Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_short_mut(&mut self, char) -> Option<&mut Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_long_mut(&mut self, &str) -> Option<&mut Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_position_mut(&mut self, Position) -> Option<&mut Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn visible_mut(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn hidden_mut(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn global_mut(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn required_mut(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
}

pub struct Args<'help> {
    iter: Iter<'help, Arg<'help>>,
}

impl<'help> Args<'help> {
    pub fn flags(&self) -> Flags<'help> {
        Flags { iter: self.iter.filter(|x| x.is_flag()) }
    }
    pub fn options(&self) -> Options<'help> {
        Options { iter: self.iter.filter(|x| x.is_option()) }
    }
    pub fn positionals(&self) -> Positionals<'help> {
        Positionals { iter: self.iter.filter(|x| x.is_positional()) }
    }
}

impl<'help> QueryArgs<'help> for Args<'help> {
    fn find_by_id(&self, id: ArgId) -> Option<&Arg<'help>> {
        self.inner.find(|x| x.id == id)
    }
    fn find_by_short(&self, s: char) -> Option<&Arg<'help>> {
        self.inner.find(|x| x.uses_short(s))
    }
    fn find_by_long(&self, l: &str) -> Option<&Arg<'help>> {
        self.inner.find(|x| x.uses_long(l))
    }
    fn find_by_position(&self, p: Position) -> Option<&Arg<'help>> {
        self.inner.find(|x| x.uses_position(p))
    }
    fn visible(&self) -> impl Iterator<Item=&Arg<'help>> {self.inner.filter(|x| x.is_visible()) }
    fn hidden(&self) -> impl Iterator<Item=&Arg<'help>> { self.inner.filter(|x| !x.is_visible())}
    fn global(&self) -> impl Iterator<Item=&Arg<'help>> { self.inner.filter(|x| x.is_global()) }
    fn required(&self) -> impl Iterator<Item=&Arg<'help>> {self.inner.filter(|x| x.is_required())  }
}

pub struct ArgsMut<'help> {
    iter: IterMut<'help, Arg<'help>>,
}

impl<'help> ArgsMut<'help> {
    pub fn flags_mut(&self) -> FlagsMut<'help> {
        FlagsMut { iter: self.iter.filter(|x| x.is_flag()) }
    }
    pub fn options_mut(&self) -> OptionsMut<'help> {
        OptionsMut { iter: self.iter.filter(|x| x.is_option()) }
    }
    pub fn positionals_mut(&self) -> PositionalsMut<'help> {
        PositionalsMut { iter: self.iter.filter(|x| x.is_positional()) }
    }
}

impl<'help> QueryArgsMut<'help> for ArgsMut<'help> {
    fn find_by_id_mut(&mut self, id: ArgId) -> Option<&mut Arg<'help>> {
        self.inner.find(|x| x.id == id)
    }
    fn find_by_short_mut(&mut self, s: char) -> Option<&mut Arg<'help>> {
        self.inner.find(|x| x.uses_short(s))
    }
    fn find_by_long_mut(&mut self, l: &str) -> Option<&mut Arg<'help>> {
        self.inner.find(|x| x.uses_long(l))
    }
    fn find_by_position_mut(&mut self, p: Position) -> Option<&mut Arg<'help>> {
        self.inner.find(|x| x.uses_position(p))
    }
    fn visible_mut(&mut self) -> impl Iterator<Item=&mut Arg<'help>> {self.inner.filter(|x| x.is_visible()) }
    fn hidden_mut(&mut self) -> impl Iterator<Item=&mut Arg<'help>> { self.inner.filter(|x| !x.is_visible())}
    fn global_mut(&mut self) -> impl Iterator<Item=&mut Arg<'help>> { self.inner.filter(|x| x.is_global()) }
    fn required_mut(&mut self) -> impl Iterator<Item=&mut Arg<'help>> {self.inner.filter(|x| x.is_required())  }
}

