use std::ops::Index;

use crate::Arg;
use crate::build::args::{ArgId, Position};
use crate::{Flags, Args, Positionals, Options};

pub struct ArgsVec<'help> {
    inner: Vec<Arg<'help>>,
}

impl<'help> ArgsVec<'help> {
    pub fn contains_short(&self, s: char) -> bool {
        self.args().any(|x| x.uses_short(c))
    }
    pub fn contains_long(&self, l: &str) -> bool {
        self.args().any(|x| x.uses_long(l))
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn has_flags(&self) -> bool {
        self.args().flags().count() > 0
    }
    pub fn has_options(&self) -> bool {
        self.args().options().count() > 0
    }
    pub fn has_positionals(&self) -> bool {
        self.args().positionals().count() > 0
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn push(&mut self, arg: Arg<'help>) {
        self.inner.push(arg)
    }
    pub fn remove(&mut self, id: ArgId) -> Option<Arg<'help>> {
        let opt_i = self.args().enumerate().find(|(x, i)| x.id == id).map(|x| x.0);
        if let Some(i) = opt_i {
            Some(self.inner.swap_remove(i))
        } else {
            None
        }
    }
    pub fn find_mut(&mut self, id: ArgId) -> Option<&mut Arg<'help>> {
        self.args_mut().find(|x| x.id == id)
    }
    pub fn find_short_mut(&mut self, s: char) -> Option<&mut Arg<'help>> {
        self.args_mut().find(|x| x.uses_short(s))
    }
    pub fn find_long_mut(&mut self, l: &str) -> Option<&mut Arg<'help>> {
        self.args_mut().find(|x| x.uses_long(l))
    }
    pub fn find_position_mut(&mut self, p: Position) -> Option<&mut Arg<'help>> {
        self.positionals_mut().find(|x| x.uses_position(p))
    }
}

// Iterator Getters
impl<'help> ArgsVec<'help> {
    pub fn args(&self) -> Args<'help>{
        self.inner.iter()
    }
    pub fn flags(&self) -> Flags<'help> {
        self.args().flags()
    }
    pub fn options(&self) -> Options<'help> {
        self.args().options()
    }
    pub fn positionals(&self) -> Positionals<'help> {
        self.args().positionals()
    }
    pub fn args_mut(&mut self) -> impl Iterator<Item=&mut Arg> {
        self.inner.iter_mut()
    }
    pub fn flags_mut(&mut self) -> impl Iterator<Item=&mut Arg> {
        self.args_mut().filter(|x| x.is_flat())
    }
    pub fn options_mut(&mut self) -> impl Iterator<Item=&mut Arg> {
        self.args_mut().filter(|x| x.is_option())
    }
    pub fn positionals_mut(&mut self) -> impl Iterator<Item=&mut Arg> {
        self.args_mut().filter(|x| x.is_positional())
    }
}

impl<'a, 'help> Index<usize> for ArgsVec<'help> {
    type Output = &'a Arg<'help>;

    fn index(&'a self, index: usize) -> &'a Arg<'help> {
        self.inner[index]
    }
}
