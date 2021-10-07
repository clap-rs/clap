use std::fmt::Display;
use std::str::FromStr;

/// Community supported auto-generated completion script avilable.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Contrib {
    /// Fig autocomplete
    Fig,
}

impl Contrib {
    /// A list of supported shells in `[&'static str]` form.
    pub fn variants() -> [&'static str; 1] {
        ["fig"]
    }
}

impl Display for Contrib {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Contrib::Fig => write!(f, "fig"),
        }
    }
}

impl FromStr for Contrib {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "fig" => Ok(Contrib::Fig),
            _ => Err(String::from("[valid values: fig]")),
        }
    }
}
