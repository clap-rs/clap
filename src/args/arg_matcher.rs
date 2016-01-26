use std::ffi::OsStr;
use std::collections::hash_map::{Entry, Iter};
use std::ops::Deref;

use vec_map::VecMap;

use args::{ArgMatches, MatchedArg, SubCommand};

pub struct ArgMatcher<'a>(pub ArgMatches<'a>);

impl<'a> ArgMatcher<'a> {
    pub fn new() -> Self {
        ArgMatcher(ArgMatches::default())
    }

    pub fn get_mut(&mut self, arg: &str) -> Option<&mut MatchedArg> {
        self.0.args.get_mut(arg)
    }

    pub fn get(&self, arg: &str) -> Option<&MatchedArg> {
        self.0.args.get(arg)
    }

    pub fn remove(&mut self, arg: &str) {
        self.0.args.remove(arg);
    }

    pub fn insert(&mut self, name: &'a str) {
        self.0.args.insert(name, MatchedArg::new());
    }

    pub fn contains(&self, arg: &str) -> bool {
        self.0.args.contains_key(arg)
    }

    pub fn is_empty(&self) -> bool {
        self.0.args.is_empty()
    }

    pub fn usage(&mut self, usage: String) {
        self.0.usage = Some(usage);
    }

    pub fn arg_names(&'a self) -> Vec<&'a str> {
        self.0.args.keys().map(Deref::deref).collect()
    }

    pub fn entry(&mut self, arg: &'a str) -> Entry<&'a str, MatchedArg> {
        self.0.args.entry(arg)
    }

    pub fn subcommand(&mut self, sc: SubCommand<'a>) {
        self.0.subcommand = Some(Box::new(sc));
    }

    pub fn subcommand_name(&self) -> Option<&str> {
        self.0.subcommand_name()
    }

    pub fn iter(&self) -> Iter<&str, MatchedArg> {
        self.0.args.iter()
    }

    pub fn inc_occurrence_of(&mut self, arg: &'a str) {
        if let Some(a) = self.get_mut(arg) {
            debugln!("+1 to {}'s occurrences", arg);
            a.occurs += 1;
            return;
        }
        self.insert(arg);
    }

    pub fn inc_occurrences_of(&mut self, args: &[&'a str]) {
        for arg in args {
            self.inc_occurrence_of(arg);
        }
    }

    pub fn add_val_to(&mut self, arg: &'a str, val: &OsStr) {
        let ma = self.entry(arg).or_insert(MatchedArg {
            occurs: 0,
            vals: VecMap::new(),
        });
        let len = ma.vals.len() + 1;
        ma.vals.insert(len, val.to_owned());
    }
}

impl<'a> Into<ArgMatches<'a>> for ArgMatcher<'a> {
    fn into(self) -> ArgMatches<'a> {
        self.0
    }
}
