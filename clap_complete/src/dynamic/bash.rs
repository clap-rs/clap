//! Complete commands within bash

use std::ffi::OsString;
use std::io::Write;

use unicode_xid::UnicodeXID;

#[derive(clap::Subcommand)]
#[command(hide = true)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum CompleteCommand {
    /// Register shell completions for this program
    Complete(CompleteArgs),
}

#[derive(clap::Args)]
#[command(group = clap::ArgGroup::new("complete").multiple(true).conflicts_with("register"))]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct CompleteArgs {
    /// Path to write completion-registration to
    #[arg(long, required = true)]
    register: Option<std::path::PathBuf>,

    #[arg(raw = true, hide_short_help = true, group = "complete")]
    comp_words: Vec<OsString>,
}

impl CompleteCommand {
    /// Process the completion request
    pub fn complete(&self, cmd: &mut clap::Command) -> std::convert::Infallible {
        self.try_complete(cmd).unwrap_or_else(|e| e.exit());
        std::process::exit(0)
    }

    /// Process the completion request
    pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        debug!("CompleteCommand::try_complete: {self:?}");
        let CompleteCommand::Complete(args) = self;
        if let Some(out_path) = args.register.as_deref() {
            let mut buf = Vec::new();
            let name = cmd.get_name();
            let bin = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());
            register(name, [bin], bin, &Behavior::default(), &mut buf)?;
            if out_path == std::path::Path::new("-") {
                std::io::stdout().write_all(&buf)?;
            } else if out_path.is_dir() {
                let out_path = out_path.join(file_name(name));
                std::fs::write(out_path, buf)?;
            } else {
                std::fs::write(out_path, buf)?;
            }
        } else {
            let index: usize = std::env::var("_CLAP_COMPLETE_INDEX")
                .ok()
                .and_then(|i| i.parse().ok())
                .unwrap_or_default();
            let _comp_type: CompType = std::env::var("_CLAP_COMPLETE_COMP_TYPE")
                .ok()
                .and_then(|i| i.parse().ok())
                .unwrap_or_default();
            let _space: Option<bool> = std::env::var("_CLAP_COMPLETE_SPACE")
                .ok()
                .and_then(|i| i.parse().ok());
            let ifs: Option<String> = std::env::var("_CLAP_COMPLETE_IFS")
                .ok()
                .and_then(|i| i.parse().ok());
            let current_dir = std::env::current_dir().ok();
            let completions =
                super::complete(cmd, args.comp_words.clone(), index, current_dir.as_deref())?;

            let mut buf = Vec::new();
            for (i, completion) in completions.iter().enumerate() {
                if i != 0 {
                    write!(&mut buf, "{}", ifs.as_deref().unwrap_or("\n"))?;
                }
                write!(&mut buf, "{}", completion.to_string_lossy())?;
            }
            std::io::stdout().write_all(&buf)?;
        }

        Ok(())
    }
}

/// The recommended file name for the registration code
pub fn file_name(name: &str) -> String {
    format!("{name}.bash")
}

/// Define the completion behavior
pub enum Behavior {
    /// Bare bones behavior
    Minimal,
    /// Fallback to readline behavior when no matches are generated
    Readline,
    /// Customize bash's completion behavior
    Custom(String),
}

impl Default for Behavior {
    fn default() -> Self {
        Self::Readline
    }
}

/// Generate code to register the dynamic completion
pub fn register(
    name: &str,
    executables: impl IntoIterator<Item = impl AsRef<str>>,
    completer: &str,
    behavior: &Behavior,
    buf: &mut dyn Write,
) -> Result<(), std::io::Error> {
    let escaped_name = name.replace('-', "_");
    debug_assert!(
        escaped_name.chars().all(|c| c.is_xid_continue()),
        "`name` must be an identifier, got `{escaped_name}`"
    );
    let mut upper_name = escaped_name.clone();
    upper_name.make_ascii_uppercase();

    let executables = executables
        .into_iter()
        .map(|s| shlex::quote(s.as_ref()).into_owned())
        .collect::<Vec<_>>()
        .join(" ");

    let options = match behavior {
        Behavior::Minimal => "-o nospace -o bashdefault",
        Behavior::Readline => "-o nospace -o default -o bashdefault",
        Behavior::Custom(c) => c.as_str(),
    };

    let completer = shlex::quote(completer);

    let script = r#"
_clap_complete_NAME() {
    export _CLAP_COMPLETE_INDEX=${COMP_CWORD}
    export _CLAP_COMPLETE_COMP_TYPE=${COMP_TYPE}
    if compopt +o nospace 2> /dev/null; then
        export _CLAP_COMPLETE_SPACE=false
    else
        export _CLAP_COMPLETE_SPACE=true
    fi
    export _CLAP_COMPLETE_IFS=$'\013'
    COMPREPLY=( $("COMPLETER" complete -- "${COMP_WORDS[@]}") )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    elif [[ $SUPPRESS_SPACE == 1 ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
        compopt -o nospace
    fi
}
complete OPTIONS -F _clap_complete_NAME EXECUTABLES
"#
    .replace("NAME", &escaped_name)
    .replace("EXECUTABLES", &executables)
    .replace("OPTIONS", options)
    .replace("COMPLETER", &completer)
    .replace("UPPER", &upper_name);

    writeln!(buf, "{script}")?;
    Ok(())
}

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

impl std::str::FromStr for CompType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use clap::ValueEnum as _;
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

impl Default for CompType {
    fn default() -> Self {
        Self::Normal
    }
}
