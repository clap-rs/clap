use std::str::FromStr;

/// Define the completion behavior
#[derive(Debug, Clone)]
pub enum Behavior {
    /// Bare bones behavior
    Minimal,
    /// Fallback to readline behavior when no matches are generated
    Readline,
    /// Customize bash's completion behavior
    Custom(String),
}

impl FromStr for Behavior {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "minimal" => Ok(Self::Minimal),
            "readline" => Ok(Self::Readline),
            _ => Ok(Self::Custom(s.to_owned())),
        }
    }
}

impl Default for Behavior {
    fn default() -> Self {
        Self::Readline
    }
}
