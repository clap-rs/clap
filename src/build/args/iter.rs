mod flags;
mod options;
mod positionals;

pub use self::flags::{Flags, FlagsMut};
pub use self::options::{Options, OptionsMut};
pub use self::positionals::{Positionals, PositionalsMut};

use std::slice::{Iter, IterMut};
use std::iter::Filter;

use INTERNAL_ERROR_MESSAGE;
use crate::Arg;
use crate::build::args::{ArgId, Position, Short, Long};

pub trait QueryArgs<'help> {
    fn find_by_id(&self, arg: ArgId) -> Option<&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_short(&self, short: Short) -> Option<&Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_long(&self, long: Long) -> Option<&Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_position(&self, pos: Position) -> Option<&Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn visible(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn hidden(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn global(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn required(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
}

pub trait QueryArgsMut<'help> {
    fn find_by_id_mut(&mut self, id: ArgId) -> Option<&mut Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_short_mut(&mut self, short: Short) -> Option<&mut Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_long_mut(&mut self, long: Long) -> Option<&mut Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn find_by_position_mut(&mut self, pos: Position) -> Option<&mut Arg<'help>>{ panic!(INTERNAL_ERROR_MESSAGE) }
    fn visible_mut(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn hidden_mut(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn global_mut(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
    fn required_mut(&self) -> impl Iterator<Item=&Arg<'help>> { panic!(INTERNAL_ERROR_MESSAGE) }
}

