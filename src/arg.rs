//! Rule, Accumulator, Matcher
//!
//! Rule:        Runtime parsing rules
//! Accumulator: Accumulated values collected through parsing
//! Matcher:     Glue the rules atop the accumulator

use ClapError;

//

pub struct Rule<'a> {
    pub name: &'a str,
    pub short: Option<char>,
    pub long: Option<&'a str>,
    pub description: Option<&'a str>,

    pub max_occurrences: usize,  // -vvv 3 occurrences of -v
    pub required: bool,          // <foo>
    pub values_collected: Vec<&'a str>, // --foo a b c; foo takes 3 arguments
//  conflicts: Vec<CowStr>,  // -fb foo and bar conflict
//  requires: Vec<CowStr>,   // --dump-config requires --config
}

impl<'a> Rule<'a> {
    pub fn with_name(name: &'a str) -> Self {
        Rule {
            name: name,
            short: None,
            long: None,
            description: None,

            max_occurrences: 1,
            required: false,
            values_collected: Vec::new(),
        }
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub fn long(mut self, long: &'a str) -> Self {
        self.long = Some(long);
        self
    }

    pub fn description(mut self, desc: &'a str) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn multiple(self) -> Self {
        self.max_occurrences(0)
    }

    pub fn takes_value(mut self, val: &'a str) -> Self {
        self.values_collected.push(val);
        self
    }

    pub fn takes_value_n_times(mut self, val: &'a str, n: usize) -> Self {
        for _ in (0..n) {
            self.values_collected.push(val);
        }
        self
    }

    pub fn takes_value_unnamed(mut self) -> Self {
        self.values_collected.push(self.name);
        self
    }

    pub fn takes_value_unnamed_n_times(mut self, n: usize) -> Self {
        for _ in (0..n) {
            self.values_collected.push(self.name);
        }
        self
    }

    pub fn max_occurrences(mut self, n: usize) -> Self {
        self.max_occurrences = n;
        self
    }
}

// Accumulated matches

// Matched return type
#[derive(Default)]
pub struct Accumulator {
    occurrences: usize,
    values: Vec<String>,
}

impl Accumulator {
    pub fn get_occurrences(&self) -> usize {
        self.occurrences
    }

    pub fn get_vec<'a>(&'a self) -> &'a Vec<String> {
        &self.values
    }
}

// Apply rules against collected results

pub struct Matcher<'a> {
    pub rule: &'a Rule<'a>,
    pub accumulator: Accumulator,
}

impl<'a> Matcher<'a> {
    pub fn with_rule(ar: &'a Rule) -> Self {
        Matcher {
            rule: ar,
            accumulator: Default::default()
        }
    }

    // Call whenever found
    pub fn handle<I, T>(&mut self, it: &mut I) -> Result<(), ClapError<'a>>
        where I: Iterator<Item=T>, T: AsRef<str>
    {
        match *self.rule {
            Rule { max_occurrences: m, .. } if m > 0 && self.accumulator.occurrences >= m =>
                return Err(ClapError::TooManyInstances(self.rule.name)),
            _ => self.accumulator.occurrences += 1,
        }

        for _ in self.rule.values_collected.iter() {
            match it.next() {
                Some(val) => self.accumulator.values.push(val.as_ref().to_owned()),
                None => return Err(ClapError::ExpectedValue(self.rule.name)),
            }
        }

        Ok(())
    }
}

#[test]
fn flag_found_3_times() {
    let ref ar = Rule::with_name("foo").multiple();
    let mut am = Matcher::with_rule(ar);
    let ref mut it = Vec::<&str>::new().into_iter(); // Empty iterator
    assert_eq!(it.len(), 0);
    assert!(am.handle(it).is_ok());
    assert!(am.handle(it).is_ok());
    assert!(am.handle(it).is_ok());
    assert_eq!(am.accumulator.get_occurrences(), 3);
    assert_eq!(it.len(), 0);
}

#[test]
fn arg_found_3_times() {
    let ref ar = Rule::with_name("foo").takes_value_unnamed_n_times(1).multiple();
    let mut am = Matcher::with_rule(ar);
    let ref mut it = vec!["foo", "bar", "baz"].into_iter();
    assert_eq!(it.len(), 3); // iterator contains 3 entries
    assert!(am.handle(it).is_ok());
    assert_eq!(&*am.accumulator.get_vec(), &["foo"]);
    assert!(am.handle(it).is_ok());
    assert_eq!(&*am.accumulator.get_vec(), &["foo", "bar"]);
    assert_eq!(it.len(), 1); // iterator is almost starved
    assert!(am.handle(it).is_ok());
    assert_eq!(&*am.accumulator.get_vec(), &["foo", "bar", "baz"]);
    assert_eq!(it.len(), 0); // iterator has been starved
}
