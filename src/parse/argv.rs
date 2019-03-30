use std::env;

pub trait ArgV : Iterator<Item=T> where T: AsRef<OsStr> { }

impl<T> ArgV for Vec<T> where T: AsRef<OsStr> { }
impl<T> ArgV for env::Args { }
impl<T> ArgV for env::ArgsOs { }
impl<'a, T> ArgV for &'a [T] where T: AsRef<OsStr> { }

macro_rules! impl_argv_arr {
    ($($n:expr)+) => {
        $(impl<T> ArgV for [T; $n] where T: AsRef<OsStr> { })+
    }
}

impl_argv_arr! {
     1  2  3  4  5  6  7  8  9 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32
}
