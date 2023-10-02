/// Type of completion attempted that caused a completion function to be called
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompType {
    /// Normal completion
    Normal,
    /// List completions after successive tabs
    Successive,
    /// List alternatives on partial word completion
    Alternatives,
    /// List completions if the word is not unmodified
    Unmodified,
    /// Menu completion
    Menu,
}

impl std::str::FromStr for CompType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "9" => Ok(Self::Normal),
            "63" => Ok(Self::Successive),
            "33" => Ok(Self::Alternatives),
            "64" => Ok(Self::Unmodified),
            "37" => Ok(Self::Menu),
            _ => Err(format!("unsupported COMP_TYPE `{}`", s)),
        }
    }
}

impl Default for CompType {
    fn default() -> Self {
        Self::Normal
    }
}

impl clap::ValueEnum for CompType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Normal,
            Self::Successive,
            Self::Alternatives,
            Self::Unmodified,
            Self::Menu,
        ]
    }
    fn to_possible_value(&self) -> ::std::option::Option<clap::builder::PossibleValue> {
        match self {
            Self::Normal => {
                let value = "9";
                debug_assert_eq!(b'\t'.to_string(), value);
                Some(
                    clap::builder::PossibleValue::new(value)
                        .alias("normal")
                        .help("Normal completion"),
                )
            }
            Self::Successive => {
                let value = "63";
                debug_assert_eq!(b'?'.to_string(), value);
                Some(
                    clap::builder::PossibleValue::new(value)
                        .alias("successive")
                        .help("List completions after successive tabs"),
                )
            }
            Self::Alternatives => {
                let value = "33";
                debug_assert_eq!(b'!'.to_string(), value);
                Some(
                    clap::builder::PossibleValue::new(value)
                        .alias("alternatives")
                        .help("List alternatives on partial word completion"),
                )
            }
            Self::Unmodified => {
                let value = "64";
                debug_assert_eq!(b'@'.to_string(), value);
                Some(
                    clap::builder::PossibleValue::new(value)
                        .alias("unmodified")
                        .help("List completions if the word is not unmodified"),
                )
            }
            Self::Menu => {
                let value = "37";
                debug_assert_eq!(b'%'.to_string(), value);
                Some(
                    clap::builder::PossibleValue::new(value)
                        .alias("menu")
                        .help("Menu completion"),
                )
            }
        }
    }
}
