use std::slice::{Iter, IterMut};
use std::iter::Filter;

use crate::Arg;
use crate::build::args::{ArgId, Position};
use super::{QueryArgs, QueryArgsMut};

pub struct Positionals<'help> {
    iter: Filter<Iter<'help, Arg<'help>>, fn(&&Arg) -> bool>,
}
impl<'help> QueryArgs<'help> for Positionals<'help> {
    fn find_by_id(&self, id: ArgId) -> Option<&Arg<'help>> {
        self.inner.find(|x| x.id == id)
    }
    fn visible(&self) -> impl Iterator<Item=&Arg<'help>> {self.inner.filter(|x| x.is_visible()) }
    fn hidden(&self) -> impl Iterator<Item=&Arg<'help>> { self.inner.filter(|x| !x.is_visible())}
    fn global(&self) -> impl Iterator<Item=&Arg<'help>> { self.inner.filter(|x| x.is_global()) }
    fn required(&self) -> impl Iterator<Item=&Arg<'help>> {self.inner.filter(|x| x.is_required())  }
}

pub struct PositionalsMut<'help> {
    iter: Filter<IterMut<'help, Arg<'help>>, fn(&&Arg) -> bool>,
}
impl<'help> QueryArgsMut<'help> for PositionalsMut<'help> {
    fn find_by_id_mut(&mut self, id: ArgId) -> Option<&mut Arg<'help>> {
        self.inner.find(|x| x.id == id)
    }
    fn find_by_position_mut(&mut self, p: Position) -> Option<&mut Arg<'help>> {
        self.inner.find(|x| x.uses_position(p))
    }
    fn visible_mut(&mut self) -> impl Iterator<Item=&mut Arg<'help>> {self.inner.filter(|x| x.is_visible()) }
    fn hidden_mut(&mut self) -> impl Iterator<Item=&mut Arg<'help>> { self.inner.filter(|x| !x.is_visible())}
    fn global_mut(&mut self) -> impl Iterator<Item=&mut Arg<'help>> { self.inner.filter(|x| x.is_global()) }
    fn required_mut(&mut self) -> impl Iterator<Item=&mut Arg<'help>> {self.inner.filter(|x| x.is_required())  }
}

