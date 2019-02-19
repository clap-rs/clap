use std::ops::Index;

use crate::Arg;
use crate::build::arg::{ArgId, Position};

pub trait Find<By> {
    type Output;
    fn find(&self, by: By) -> Option<<Self as Find<By>>::Output>;
}

pub trait FindMut<By> {
    type Output;
    fn find(&mut self, by: By) -> Option<<Self as FindMut<By>>::Output>;
}

pub struct Args<'help> {
    inner: Vec<Arg<'help>>,
}

impl<'help> Args<'help> {
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
        self.flags().any(|x| x.is_flag())
    }
    pub fn has_options(&self) -> bool {
        self.options().any(|x| x.is_option())
    }
    pub fn has_positionals(&self) -> bool {
        self.positionals().any(|x| x.is_positional())
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
impl<'help> Args<'help> {
    pub fn args(&self) -> impl Iterator<Item=&Arg> {
        self.inner.iter()
    }
    pub fn flags(&self) -> impl Iterator<Item=&Arg> {
        self.args().filter(|x| x.is_flat())
    }
    pub fn options(&self) -> impl Iterator<Item=&Arg> {
        self.args().filter(|x| x.is_option())
    }
    pub fn positionals(&self) -> impl Iterator<Item=&Arg> {
        self.args().filter(|x| x.is_positional())
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

impl<'a, 'help> Index<usize> for Args<'help> {
    type Output = &'a Arg<'help>;

    fn index(&'a self, index: usize) -> &'a Arg<'help> {
        self.inner[index]
    }
}
