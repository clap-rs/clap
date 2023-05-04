use std::collections::BTreeMap;

use clap::{arg, command, ArgGroup, ArgMatches, Command};

fn main() {
    let matches = cli().get_matches();
    let values = Value::from_matches(&matches);
    println!("{values:#?}");
}

fn cli() -> Command {
    command!()
        .group(ArgGroup::new("tests").multiple(true))
        .next_help_heading("TESTS")
        .args([
            arg!(--empty "File is empty and is either a regular file or a directory").group("tests"),
            arg!(--name <NAME> "Base of file name (the path with the leading directories removed) matches shell pattern pattern").group("tests"),
        ])
        .group(ArgGroup::new("operators").multiple(true))
        .next_help_heading("OPERATORS")
        .args([
            arg!(-o - -or "expr2 is not evaluate if exp1 is true").group("operators"),
            arg!(-a - -and "Same as `expr1 expr1`").group("operators"),
        ])
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Value {
    Bool(bool),
    String(String),
}

impl Value {
    pub fn from_matches(matches: &ArgMatches) -> Vec<(clap::Id, Self)> {
        let mut values = BTreeMap::new();
        for id in matches.ids() {
            if matches.try_get_many::<clap::Id>(id.as_str()).is_ok() {
                // ignore groups
                continue;
            }
            let value_source = matches
                .value_source(id.as_str())
                .expect("id came from matches");
            if value_source != clap::parser::ValueSource::CommandLine {
                // Any other source just gets tacked on at the end (like default values)
                continue;
            }
            if Self::extract::<String>(matches, id, &mut values) {
                continue;
            }
            if Self::extract::<bool>(matches, id, &mut values) {
                continue;
            }
            unimplemented!("unknown type for {}: {:?}", id, matches);
        }
        values.into_values().collect::<Vec<_>>()
    }

    fn extract<T: Clone + Into<Value> + Send + Sync + 'static>(
        matches: &ArgMatches,
        id: &clap::Id,
        output: &mut BTreeMap<usize, (clap::Id, Self)>,
    ) -> bool {
        match matches.try_get_many::<T>(id.as_str()) {
            Ok(Some(values)) => {
                for (value, index) in values.zip(
                    matches
                        .indices_of(id.as_str())
                        .expect("id came from matches"),
                ) {
                    output.insert(index, (id.clone(), value.clone().into()));
                }
                true
            }
            Ok(None) => {
                unreachable!("`ids` only reports what is present")
            }
            Err(clap::parser::MatchesError::UnknownArgument { .. }) => {
                unreachable!("id came from matches")
            }
            Err(clap::parser::MatchesError::Downcast { .. }) => false,
            Err(_) => {
                unreachable!("id came from matches")
            }
        }
    }
}

impl From<String> for Value {
    fn from(other: String) -> Self {
        Self::String(other)
    }
}

impl From<bool> for Value {
    fn from(other: bool) -> Self {
        Self::Bool(other)
    }
}
