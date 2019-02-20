use std::slice::Iter;
use std::iter::Filter;

use crate::Arg;
use crate::build::args::{ArgId, Position};

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
    pub fn find(&self, id: ArgId) -> Option<&Arg<'help>> {
        self.args().find(|x| x.id == id)
    }
    pub fn find_short(&self, s: char) -> Option<&Arg<'help>> {
        self.args().find(|x| x.uses_short(s))
    }
    pub fn find_long(&self, l: &str) -> Option<&Arg<'help>> {
        self.args().find(|x| x.uses_long(l))
    }
    pub fn find_position(&self, p: Position) -> Option<&Arg<'help>> {
        self.positionals().find(|x| x.uses_position(p))
    }
}

pub struct Flags<'help> {
    iter: Filter<Iter<'help, Arg<'help>>, fn(&&Arg) -> bool>,
}

pub struct Options<'help> {
    iter: Filter<Iter<'help, Arg<'help>>, fn(&&Arg) -> bool>,
}

pub struct Positionals<'help> {
    iter: Filter<Iter<'help, Arg<'help>>, fn(&&Arg) -> bool>,
}
