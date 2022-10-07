//! Complete commands within shells

pub mod bash;
pub mod fish;

use std::{
    ffi::OsString,
    io::{self, Write},
    path::{self, Path, PathBuf},
};

use clap::{
    builder::StyledStr, Arg, Args, Command, CommandFactory, Error, Parser, Subcommand, ValueEnum,
};
use clap_lex::{ArgCursor, RawArgs, RawOsStr, RawOsString};

use bash::Bash;
use fish::Fish;

#[derive(Parser, Clone, Debug)]
#[command(hide = true)]
/// Subcommand to trigger completions
///
/// To add to a [`Command`] either:
/// - use [`Subcommand::augment_subcommands`]
/// - use `#[command(flatten)]` when adding to an enum deriving [`Subcommand`]
///
/// Afterwards completions can be manually triggered by calling [`CompleteCommand::complete`].
pub enum CompleteCommand {
    /// Register shell completions for this program
    #[command(subcommand)]
    Complete(CompleteShell),
}

impl CompleteCommand {
    /// Process the completion request, exit on errors
    pub fn complete(self, cmd: &mut Command) -> std::convert::Infallible {
        self.try_complete(cmd).unwrap_or_else(|e| e.exit());
        std::process::exit(0)
    }

    /// Process the completion request, return errors
    pub fn try_complete(self, cmd: &mut Command) -> clap::error::Result<()> {
        debug!("CompleteCommand::try_complete: {:?}", self);
        let CompleteCommand::Complete(complete) = self;
        match complete {
            CompleteShell::Bash(args) => <Bash as Completer>::try_complete(args, cmd),
            CompleteShell::Fish(args) => <Fish as Completer>::try_complete(args, cmd),
            CompleteShell::Register(RegisterArgs { path, shell }) => {
                let script = shell.completion_script(cmd);
                if path == Path::new("-") {
                    io::stdout().write_all(&script)?;
                } else if path.is_dir() {
                    let path = path.join(shell.file_name(cmd.get_name()));
                    std::fs::write(path, script)?;
                } else {
                    std::fs::write(path, script)?;
                }
                Ok(())
            }
        }
    }
}

/// Trait completing an App
///
/// Allows to be used like this on an item deriving [`Parser`]
///
/// **NOTE:** This parses any invocation with the subcommand `complete` present making this
/// conflict with any command expecting `complete` as a valid argument or subcommand.
///
/// ```
/// use std::path::PathBuf;
///
/// use clap::Parser;
/// use clap_complete::dynamic::Completeable;
///
/// #[derive(Parser)]
/// struct Opts {
///     file: Option<PathBuf>
/// }
///
/// fn main() {
///     Opts::complete_or_parse();
/// }
/// ```
pub trait Completeable {
    /// Either trigger dynamic completion or parse arguments as T
    ///
    /// This is a drop-in replacement for [`Parser::parse()`].
    fn complete_or_parse() -> Self
    where
        Self: Parser,
    {
        Self::complete();
        Self::parse()
    }
    /// Either trigger dynamic completion or parse try to parse arguments as T
    ///
    /// This is a drop-in replacement for [`Parser::try_parse()`].
    fn try_complete_or_parse() -> Result<Self, Error>
    where
        Self: Parser,
    {
        Self::try_complete()?;
        Self::try_parse()
    }
    /// Trigger dynamic completion if the `complete` subcommand is present do nothing if not
    ///
    /// Exits on failing completions
    fn complete()
    where
        Self: CommandFactory,
    {
        if let Ok(c) = CompleteCommand::try_parse() {
            c.complete(&mut Self::command());
        }
    }
    /// Trigger dynamic completion if the `complete` subcommand is present do nothing if not
    ///
    /// Errors, if completions fail for any reason.
    fn try_complete() -> Result<(), Error>
    where
        Self: CommandFactory,
    {
        if let Ok(c) = CompleteCommand::try_parse() {
            c.try_complete(&mut Self::command())?;
        }
        Ok(())
    }
}

impl<T: CommandFactory> Completeable for T {}

#[derive(Subcommand, Clone, Debug)]
#[command(hide = true)]
#[allow(missing_docs)]
// Subcommand for all the shells, so each can have their own options
pub enum CompleteShell {
    Bash(bash::CompleteArgs),
    Fish(fish::CompleteArgs),
    /// Only exception is Register, which outputs the completion script for a shell
    Register(RegisterArgs),
}

#[derive(Args, Clone, Debug)]
/// Arguments for registering dynamic completions
pub struct RegisterArgs {
    /// Path to write completion-registration to
    #[arg(long, short)]
    path: PathBuf,
    /// Shell to generate completions for
    #[arg(long, short)]
    shell: Shell,
}

/// Shell with dynamic completion available.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
#[non_exhaustive]
pub enum Shell {
    /// Bourne Again SHell (bash)
    Bash,
    /// Friendly Interactive SHell (fish)
    Fish,
}

impl Shell {
    /// Return completion script
    fn completion_script(&self, cmd: &mut Command) -> Vec<u8> {
        match self {
            Shell::Bash => Bash::completion_script(cmd),
            Shell::Fish => Fish::completion_script(cmd),
        }
    }
    /// The recommended file name for the registration code
    fn file_name(&self, name: &str) -> String {
        match self {
            Shell::Bash => Bash::file_name(name),
            Shell::Fish => Fish::file_name(name),
        }
    }
}

/// dynamic completions
pub trait Completer {
    /// Arguments used by the shells dynamic completions
    type CompleteArgs: Args;
    /// Return completion script
    fn completion_script(cmd: &mut Command) -> Vec<u8>;
    /// The recommended file name for the registration code
    fn file_name(name: &str) -> String;
    // TODO maybe also have a function returning the expected file path for SYSTEM/USER
    // installation e.g. for fish /etc/fish/completions and ~/.config/fish/completions/
    /// Process the completion request
    fn try_complete(args: Self::CompleteArgs, cmd: &mut Command) -> clap::error::Result<()>;
}

/// All information relevant to producing the completions
#[derive(Default, Debug)]
pub struct CompletionContext<'a> {
    /// The value that could be expected
    value_for: Option<&'a Arg>,
    /// The flags that could be expected
    ///
    /// Does not contain options that are already present or that conflict with other args.
    /// Is empty when `value_for` is an option expecting a value
    options: Vec<&'a Arg>,
    /// The subcommands that could be expected
    subcommands: Vec<&'a Command>,
}
impl<'a> CompletionContext<'a> {
    fn from_option(opt: &'a Arg) -> Self {
        Self {
            value_for: Some(opt),
            ..Default::default()
        }
    }
    fn from_positional(positional: &'a Arg, options: Vec<&'a Arg>) -> Self {
        Self {
            value_for: Some(positional),
            options,
            ..Default::default()
        }
    }
    fn from_optional_positional(
        positional: &'a Arg,
        options: Vec<&'a Arg>,
        subcommands: Vec<&'a Command>,
    ) -> Self {
        Self {
            value_for: Some(positional),
            options,
            subcommands,
        }
    }
    fn empty() -> Self {
        Self::default()
    }
}

/// Complete the command specified
pub fn complete(
    cmd: &mut Command,
    args: impl IntoIterator<Item = impl Into<OsString>>,
) -> io::Result<CompletionContext> {
    cmd.build();
    let raw_args = RawArgs::new(args);
    let cursor = &mut raw_args.cursor();
    // TODO: Multicall support
    if !cmd.is_no_binary_name_set() {
        raw_args.next_os(cursor);
    }
    complete_internal(cmd, raw_args, cursor)
}

/// Recursive completions over subcommands
fn complete_internal<'a>(
    cmd: &'a Command,
    raw_args: RawArgs,
    cursor: &mut ArgCursor,
) -> io::Result<CompletionContext<'a>> {
    let mut positionals = cmd.get_positionals().peekable();
    let mut opts: Vec<_> = cmd.get_arguments().filter(|a| !a.is_positional()).collect();

    let mut opt_expecting_value = None;

    // --
    let is_escaped = false;

    while let Some(arg) = raw_args.next(cursor) {
        if !(is_escaped
            // TODO Decide what to do when positional allows hyphen but option does not
            || is_some_and(positionals.peek().copied(), Arg::is_allow_hyphen_values_set)
            || is_some_and(opt_expecting_value, Arg::is_allow_hyphen_values_set))
        {
            // Ignoring non utf8 flags
            if let Some((flag, value)) = arg.to_long() {
                if let Ok(flag) = flag {
                    // Manual drain_filter().next()
                    if let Some(opt) = opts.iter().position(|o| {
                        is_some_and(o.get_long(), |long| long == flag)
                            || is_some_and(o.get_all_aliases(), |aliases| {
                                aliases.iter().any(|&alias| alias == flag)
                            })
                    }) {
                        let opt = opts.remove(opt);
                        let conflicts = cmd.get_arg_conflicts_with(opt);
                        opts.retain(|o| !conflicts.contains(o));
                        // TODO support multiple arguments, delimiter etc.
                        opt_expecting_value = (opt.get_num_args().is_some()
                            && !opt.is_require_equals_set()
                            && value.is_none())
                        .then(|| opt);
                    }
                }
                continue;
            }
            // TODO support negative numbers
            if let Some(flags) = arg.to_short() {
                // Ignoring non utf8 flags
                for flag in flags.flatten() {
                    // Manual drain_filter().next()
                    if let Some(opt) = opts.iter().position(|o| {
                        is_some_and(o.get_short(), |short| short == flag)
                            || is_some_and(o.get_all_short_aliases(), |aliases| {
                                aliases.iter().any(|&alias| alias == flag)
                            })
                    }) {
                        let opt = opts.remove(opt);
                        let conflicts = cmd.get_arg_conflicts_with(opt);
                        opts.retain(|o| !conflicts.contains(o));
                        // TODO support multiple arguments, delimiter etc.
                        // TODO Do short flags support =?, what about -avalue
                        // What if it is what about -ma where a takes a value? even valid?
                        opt_expecting_value = opt.get_num_args().is_some().then(|| opt);
                    }
                }
                continue;
            }
        }
        if opt_expecting_value.is_some() {
            opt_expecting_value = None;
        } else if let Some(positional) = positionals.peek() {
            // TODO Handle multiple value positionals
            if positional.is_required_set() {
                positionals.next();
            } else if let Some(subcmd) = cmd.find_subcommand(arg.to_value_os().to_os_str()) {
                // TODO Handle global options
                return complete_internal(subcmd, raw_args, cursor);
            } else {
                positionals.next();
            }
        } else if let Some(subcmd) = cmd.find_subcommand(arg.to_value_os().to_os_str()) {
            // TODO Handle global options
            return complete_internal(subcmd, raw_args, cursor);
        }
    }

    if let Some(opt) = opt_expecting_value {
        return Ok(CompletionContext::from_option(opt));
    } else if let Some(positional) = positionals.next() {
        if positional.is_required_set() {
            return Ok(CompletionContext::from_positional(positional, opts));
        } else {
            return Ok(CompletionContext::from_optional_positional(
                positional,
                opts,
                cmd.get_subcommands().collect(),
            ));
        }
    }
    Ok(CompletionContext::empty())
}

fn is_some_and<T>(option: Option<T>, fun: impl Fn(T) -> bool) -> bool {
    option.map_or(false, fun)
}

fn completions_for_arg(
    arg: &Arg,
    value: &RawOsStr,
) -> io::Result<Vec<(RawOsString, Option<StyledStr>)>> {
    // TODO take current token to complete subdirectories
    let mut values = Vec::new();
    debug!("complete_arg_value: arg={:?}, value={:?}", arg, value);

    if let Some(possible_values) = crate::generator::utils::possible_values(arg) {
        values.extend(possible_values.into_iter().filter_map(|p| {
            let name = RawOsStr::from_str(p.get_name());
            name.starts_with_os(value)
                .then(|| (name.to_owned(), p.get_help().map(ToOwned::to_owned)))
        }));
    } else {
        match arg.get_value_hint() {
            clap::ValueHint::Other => {
                // Should not complete
            }
            clap::ValueHint::Unknown | clap::ValueHint::AnyPath => {
                values.extend(complete_path(value, |_| true));
            }
            clap::ValueHint::FilePath => {
                values.extend(complete_path(value, |p| p.is_file()));
            }
            clap::ValueHint::DirPath => {
                values.extend(complete_path(value, |p| p.is_dir()));
            }
            clap::ValueHint::ExecutablePath => {
                use is_executable::IsExecutable;
                values.extend(complete_path(value, |p| p.is_executable()));
            }
            clap::ValueHint::CommandName
            | clap::ValueHint::CommandString
            | clap::ValueHint::CommandWithArguments
            | clap::ValueHint::Username
            | clap::ValueHint::Hostname
            | clap::ValueHint::Url
            | clap::ValueHint::EmailAddress => {
                // No completion implementation
            }
            _ => {
                // Safe-ish fallback
                values.extend(complete_path(value, |_| true));
            }
        }
        values.sort();
    }

    Ok(values)
}

fn complete_path(
    path: &clap_lex::RawOsStr,
    is_wanted: impl Fn(&Path) -> bool,
) -> Vec<(RawOsString, Option<StyledStr>)> {
    let mut completions = Vec::new();

    // `/` on Unix and `/`+`\` on Windows
    let complete_entries = path.is_empty() || path.to_str_lossy().ends_with(path::is_separator);

    let value = PathBuf::from(path.to_os_str().to_os_string());
    let (value, prefix) = if complete_entries {
        (value, RawOsString::from_string(String::new()))
    } else {
        (
            if let Some(parent) = value.parent() {
                parent.to_owned()
            } else {
                return Vec::new();
            },
            RawOsString::new(value.file_name().unwrap_or_default().to_os_string()),
        )
    };

    let root = if value.is_absolute() {
        value
    } else {
        let current_dir = if let Ok(current_dir) = std::env::current_dir() {
            current_dir
        } else {
            debug!("complete_path: no relative path completions without current_dir");
            // Can't complete without a `current_dir`
            return Vec::new();
        };

        current_dir.join(value)
    };

    debug!("complete_path: root={:?}, prefix={:?}", root, prefix);

    // Ignores all errors
    for entry in std::fs::read_dir(&root).into_iter().flatten().flatten() {
        let raw_file_name = clap_lex::RawOsString::new(entry.file_name());
        if !raw_file_name.starts_with_os(&prefix) {
            continue;
        }

        if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
            let path = entry.path();
            let mut suggestion = pathdiff::diff_paths(&path, &root).unwrap_or(path);
            suggestion.push(""); // Ensure trailing `/`
            completions.push((RawOsString::new(suggestion.as_os_str().to_owned()), None));
        } else {
            let path = entry.path();
            if is_wanted(&path) {
                let suggestion = pathdiff::diff_paths(&path, &root).unwrap_or(path);
                completions.push((RawOsString::new(suggestion.as_os_str().to_owned()), None));
            }
        }
    }

    completions
}
