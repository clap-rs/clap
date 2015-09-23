//! App, CollectionMatcher
//!
//! App:               Implementation of CollectionMatcher
//! CollectionMatcher: Trait providing a generic argument parser

use std::collections::hash_map::HashMap;

use {ClapError, Rule, Accumulator};
use arg::Matcher;

pub trait CollectionMatcher<'a> {
    fn iter_rules(&self) -> &Vec<Rule>;

    fn matches<I, T: 'a>(&'a self, it: &'a mut I) -> Result<CollectedMatches<'a>, ClapError<'a>>
        where I: Iterator<Item=T>, T: AsRef<str>
    {
        let rules = self.iter_rules().iter();
        let mut index: HashMap<Index, usize> = HashMap::new();
        let mut matches: Vec<Matcher> = Vec::with_capacity(rules.len());
        let mut positional_counter = 0;
        for (i, r) in rules.enumerate() {
            r.long.map(|l| index.insert(Index::Long(l), i));
            r.short.map(|s| index.insert(Index::Short(s), i));
            if r.long.is_none() && r.short.is_none() {
                assert!(r.max_occurrences == 1); // repeating would be useful
                assert!(r.values_collected.len() == 1);
                index.insert(Index::Positional(positional_counter), i);
                positional_counter += 1;
            }
            matches.push(Matcher::with_rule(r));
        }
        let mut positional_only = false; positional_counter = 0;
        while let Some(word) = it.next() {
            match word.as_ref() {
                "--" if !positional_only => positional_only = true,
                x if !positional_only && x.starts_with("--") => { // --foo=val ?
                    let long = &x[2..];
                    match index.get(&Index::Long(long)) {
                        Some(i) => try!(matches[*i].handle(it)),
                        None => return Err(ClapError::UnexpectedLong(long.to_owned())),
                    }
                },
                x if !positional_only && x.starts_with("-") =>
                    for short in (x[1..]).chars() {
                        match index.get(&Index::Short(short)) {
                            Some(i) => try!(matches[*i].handle(it)),
                            None => return Err(ClapError::UnexpectedShort(short.to_owned())),
                        }
                    },
                val => {
                    match index.get(&Index::Positional(positional_counter)) {
                        Some(i) => try!(matches[*i].handle(&mut vec![val].into_iter())),
                        None => return Err(ClapError::UnexpectedPositional(val.to_owned())),
                    }
                    positional_counter += 1;
                },
            }
        };

        // [ ] validate
        // [ ] conflicts
        // [ ] requires
        // [x] required
        for r in matches.iter() {
            debug_assert_eq!(r.accumulator.get_vec().len(),
                r.rule.values_collected.len() * r.accumulator.get_occurrences());
            if r.rule.required && r.accumulator.get_occurrences() < 1 {
                return Err(ClapError::ExpectedValue(r.rule.name))
            }
        }

        Ok(matches.into_iter().map(|m| (m.rule.name, m.accumulator)).collect())
    }
}

// No usage related meta: author, description, version. They belong in a usage builder
pub struct App<'a>(Vec<Rule<'a>>);

pub type CollectedMatches<'a> = HashMap<&'a str, Accumulator>;

#[derive(Hash, PartialEq, Eq)]
enum Index<'a> {
    Short(char),
    Long(&'a str),
    Positional(usize),
}

impl<'a> CollectionMatcher<'a> for App<'a> {
    fn iter_rules(&self) -> &Vec<Rule> {
        &self.0
    }
}

impl<'a> App<'a> {
    pub fn with_rules(vec: Vec<Rule<'a>>) -> Self {
        App(vec)
    }
}

// Will build the above App with appropriate meta for USAGE string generation

#[derive(Default)]
pub struct AppBuilder<'a> {
    rules: Vec<Rule<'a>>,
    about: Option<&'a str>,
    author: Option<&'a str>,
}

impl<'a> AppBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn author(mut self, author: &'a str) -> Self {
        self.author = Some(author);
        self
    }

    pub fn about(mut self, about: &'a str) -> Self {
        self.about = Some(about);
        self
    }


    pub fn args(mut self, args: Vec<Rule<'a>>) -> Self {
        self.rules.extend(args);
        self
    }
}

impl<'a> From<AppBuilder<'a>> for App<'a> {
    fn from(ab: AppBuilder<'a>) -> Self {
        App(ab.rules)
    }
}
