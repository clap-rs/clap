use clap::{App, AppSettings, Arg, ValueHint};
use clap_generate::{generate, generators::*};
use std::fmt;

mod bash;
mod elvish;
mod fig;
mod fish;
mod powershell;
mod zsh;

#[derive(PartialEq, Eq)]
pub struct PrettyString<'a>(pub &'a str);

impl<'a> fmt::Debug for PrettyString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0)
    }
}

macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        pretty_assertions::assert_eq!(PrettyString($left), PrettyString($right));
    };
}

pub fn common<G: Generator>(gen: G, app: &mut App, name: &str, fixture: &str) {
    let mut buf = vec![];
    generate(gen, app, name, &mut buf);
    let string = String::from_utf8(buf).unwrap();

    assert_eq!(&string, fixture);
}
