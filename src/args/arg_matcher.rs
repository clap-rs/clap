use vec_map::VecMap;

use args::{ArgMatches, MatchedArg, SubCommand};
use std::collections::hash_map::{Entry, Keys, Iter};

pub struct ArgMatcher<'ar>(ArgMatches<'ar, 'ar>);

impl<'ar> ArgMatcher<'ar> {
    pub fn new() -> Self {
        ArgMatcher(ArgMatches::new())
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

    pub fn insert(&mut self, name: &'ar str) {
        self.0.args.insert(name, MatchedArg::new());
    }

    pub fn contains(&self, arg: &str) -> bool {
        self.0.args.contains_key(arg)
    }

    pub fn is_empty(&self) -> bool {
        self.0.args.is_empty()
    }

    pub fn values_of(&self, arg: &str) -> Option<Vec<&str>> {
        self.0.values_of(arg)
    }

    pub fn usage(&mut self, usage: String) {
        self.0.usage = Some(usage);
    }

    pub fn arg_names(&self) -> Keys<&'ar str, MatchedArg> {
        self.0.args.keys()
    }

    pub fn entry(&mut self, arg: &'ar str) -> Entry<&'ar str, MatchedArg> {
        self.0.args.entry(arg)
    }

    pub fn subcommand(&mut self, sc: SubCommand<'ar, 'ar>) {
        self.0.subcommand = Some(Box::new(sc));
    }

    pub fn subcommand_name(&self) -> Option<&str> {
        self.0.subcommand_name()
    }

    pub fn iter(&self) -> Iter<&'ar str, MatchedArg> {
        self.0.args.iter()
    }

    pub fn inc_occurrence_of(&mut self, arg: &'ar str) {
        if let Some(a) = self.get_mut(arg) {
            a.occurrences += 1;
            return;
        }
        self.insert(arg);
    }

    pub fn inc_occurrences_of(&mut self, args: &[&'ar str]) {
        for arg in args {
            self.inc_occurrence_of(arg);
        }
    }

    pub fn add_val_to(&mut self, arg: &'ar str, val: String) {
        let ma = self.entry(arg).or_insert(MatchedArg {
            // occurrences will be incremented on getting a value
            occurrences: 0,
            values: Some(VecMap::new()),
        });
        if let Some(ref mut vals) = ma.values {
            let len = vals.len() + 1;
            vals.insert(len, val);
            ma.occurrences += 1;
        }
    }
}

impl<'ar> Into<ArgMatches<'ar, 'ar>> for ArgMatcher<'ar> {
    fn into(self) -> ArgMatches<'ar, 'ar> {
        self.0
    }
}
