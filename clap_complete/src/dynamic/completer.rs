use std::ffi::OsStr;
use std::ffi::OsString;

use clap::builder::StyledStr;
use clap_lex::OsStrExt as _;

/// Shell-specific completions
pub trait Completer {
    /// The recommended file name for the registration code
    fn file_name(&self, name: &str) -> String;
    /// Register for completions
    fn write_registration(
        &self,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error>;
    /// Complete the given command
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error>;
}

/// Complete the given command
pub fn complete(
    cmd: &mut clap::Command,
    args: Vec<OsString>,
    arg_index: usize,
    current_dir: Option<&std::path::Path>,
) -> Result<Vec<CompletionCandidate>, std::io::Error> {
    cmd.build();

    let raw_args = clap_lex::RawArgs::new(args);
    let mut cursor = raw_args.cursor();
    let mut target_cursor = raw_args.cursor();
    raw_args.seek(
        &mut target_cursor,
        clap_lex::SeekFrom::Start(arg_index as u64),
    );
    // As we loop, `cursor` will always be pointing to the next item
    raw_args.next_os(&mut target_cursor);

    // TODO: Multicall support
    if !cmd.is_no_binary_name_set() {
        raw_args.next_os(&mut cursor);
    }

    let mut current_cmd = &*cmd;
    let mut pos_index = 1;
    let mut is_escaped = false;
    let mut state = ParseState::ValueDone;
    while let Some(arg) = raw_args.next(&mut cursor) {
        if cursor == target_cursor {
            return complete_arg(&arg, current_cmd, current_dir, pos_index, state);
        }

        debug!("complete::next: Begin parsing '{:?}'", arg.to_value_os(),);

        if let Ok(value) = arg.to_value() {
            if let Some(next_cmd) = current_cmd.find_subcommand(value) {
                current_cmd = next_cmd;
                pos_index = 1;
                state = ParseState::ValueDone;
                continue;
            }
        }

        if is_escaped {
            pos_index += 1;
            state = ParseState::Pos(pos_index);
        } else if arg.is_escape() {
            is_escaped = true;
            state = ParseState::ValueDone;
        } else if let Some((flag, value)) = arg.to_long() {
            if let Ok(flag) = flag {
                let opt = current_cmd.get_arguments().find(|a| {
                    let longs = a.get_long_and_visible_aliases();
                    let is_find = longs.map(|v| {
                        let mut iter = v.into_iter();
                        let s = iter.find(|s| *s == flag);
                        s.is_some()
                    });
                    is_find.unwrap_or(false)
                });
                state = match opt.map(|o| o.get_action()) {
                    Some(clap::ArgAction::Set) | Some(clap::ArgAction::Append) => {
                        if value.is_some() {
                            ParseState::ValueDone
                        } else {
                            ParseState::Opt(opt.unwrap().clone())
                        }
                    }
                    Some(clap::ArgAction::SetTrue) | Some(clap::ArgAction::SetFalse) => {
                        ParseState::ValueDone
                    }
                    Some(clap::ArgAction::Count) => ParseState::ValueDone,
                    Some(clap::ArgAction::Version) => ParseState::ValueDone,
                    Some(clap::ArgAction::Help)
                    | Some(clap::ArgAction::HelpLong)
                    | Some(clap::ArgAction::HelpShort) => ParseState::ValueDone,
                    Some(_) => ParseState::ValueDone,
                    None => ParseState::ValueDone,
                };
            } else {
                state = ParseState::ValueDone;
            }
        } else if let Some(mut short) = arg.to_short() {
            let mut takes_value = false;
            loop {
                if let Some(Ok(opt)) = short.next_flag() {
                    let opt = current_cmd.get_arguments().find(|a| {
                        let shorts = a.get_short_and_visible_aliases();
                        let is_find = shorts.map(|v| {
                            let mut iter = v.into_iter();
                            let c = iter.find(|c| *c == opt);
                            c.is_some()
                        });
                        is_find.unwrap_or(false)
                    });

                    state = match opt.map(|o| o.get_action()) {
                        Some(clap::ArgAction::Set) | Some(clap::ArgAction::Append) => {
                            takes_value = true;
                            if short.next_value_os().is_some() {
                                ParseState::ValueDone
                            } else {
                                ParseState::Opt(opt.unwrap().clone())
                            }
                        }
                        Some(clap::ArgAction::SetTrue) | Some(clap::ArgAction::SetFalse) => {
                            ParseState::ValueDone
                        }
                        Some(clap::ArgAction::Count) => ParseState::ValueDone,
                        Some(clap::ArgAction::Version) => ParseState::ValueDone,
                        Some(clap::ArgAction::Help)
                        | Some(clap::ArgAction::HelpShort)
                        | Some(clap::ArgAction::HelpLong) => ParseState::ValueDone,
                        Some(_) => ParseState::ValueDone,
                        None => ParseState::ValueDone,
                    };

                    if takes_value {
                        break;
                    }
                } else {
                    state = ParseState::ValueDone;
                    break;
                }
            }
        } else {
            match state {
                ParseState::ValueDone | ParseState::Pos(_) => {
                    pos_index += 1;
                    state = ParseState::ValueDone;
                }
                ParseState::Opt(_) => {
                    state = ParseState::ValueDone;
                }
            }
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "no completion generated",
    ))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ParseState {
    /// Parsing a value done, there is no state to record.
    ValueDone,

    /// Parsing a positional argument after `--`
    Pos(usize),

    /// Parsing a optional flag argument
    Opt(clap::Arg),
}

fn complete_arg(
    arg: &clap_lex::ParsedArg<'_>,
    cmd: &clap::Command,
    current_dir: Option<&std::path::Path>,
    pos_index: usize,
    state: ParseState,
) -> Result<Vec<CompletionCandidate>, std::io::Error> {
    debug!(
        "complete_arg: arg={:?}, cmd={:?}, current_dir={:?}, pos_index={:?}, state={:?}",
        arg,
        cmd.get_name(),
        current_dir,
        pos_index,
        state
    );
    let mut completions = Vec::<CompletionCandidate>::new();

    match state {
        ParseState::ValueDone => {
            if let Some((flag, value)) = arg.to_long() {
                if let Ok(flag) = flag {
                    if let Some(value) = value {
                        if let Some(arg) = cmd.get_arguments().find(|a| a.get_long() == Some(flag))
                        {
                            completions.extend(
                                complete_arg_value(value.to_str().ok_or(value), arg, current_dir)
                                    .into_iter()
                                    .map(|comp| {
                                        CompletionCandidate::new(format!(
                                            "--{}={}",
                                            flag,
                                            comp.get_content().to_string_lossy()
                                        ))
                                        .help(comp.get_help().cloned())
                                        .visible(comp.is_visible())
                                    }),
                            );
                        }
                    } else {
                        completions.extend(longs_and_visible_aliases(cmd).into_iter().filter(
                            |comp| {
                                comp.get_content()
                                    .starts_with(format!("--{}", flag).as_str())
                            },
                        ));

                        completions.extend(hidden_longs_aliases(cmd).into_iter().filter(|comp| {
                            comp.get_content()
                                .starts_with(format!("--{}", flag).as_str())
                        }))
                    }
                }
            } else if arg.is_escape() || arg.is_stdio() || arg.is_empty() {
                // HACK: Assuming knowledge of is_escape / is_stdio
                completions.extend(longs_and_visible_aliases(cmd));

                completions.extend(hidden_longs_aliases(cmd));
            }

            if arg.is_empty() || arg.is_stdio() || arg.is_short() {
                let dash_or_arg = if arg.is_empty() {
                    "-".into()
                } else {
                    arg.to_value_os().to_string_lossy()
                };
                // HACK: Assuming knowledge of is_stdio
                completions.extend(
                    shorts_and_visible_aliases(cmd)
                        .into_iter()
                        // HACK: Need better `OsStr` manipulation
                        .map(|comp| {
                            CompletionCandidate::new(format!(
                                "{}{}",
                                dash_or_arg,
                                comp.get_content().to_string_lossy()
                            ))
                            .help(comp.get_help().cloned())
                            .visible(true)
                        }),
                );
            }

            if let Some(positional) = cmd
                .get_positionals()
                .find(|p| p.get_index() == Some(pos_index))
            {
                completions.extend(complete_arg_value(arg.to_value(), positional, current_dir));
            }

            if let Ok(value) = arg.to_value() {
                completions.extend(complete_subcommand(value, cmd));
            }
        }
        ParseState::Pos(_) => {
            if let Some(positional) = cmd
                .get_positionals()
                .find(|p| p.get_index() == Some(pos_index))
            {
                completions.extend(complete_arg_value(arg.to_value(), positional, current_dir));
            }
        }
        ParseState::Opt(opt) => {
            completions.extend(complete_arg_value(arg.to_value(), &opt, current_dir));
        }
    }
    if completions.iter().any(|a| a.is_visible()) {
        completions.retain(|a| a.is_visible())
    }

    Ok(completions)
}

fn complete_arg_value(
    value: Result<&str, &OsStr>,
    arg: &clap::Arg,
    current_dir: Option<&std::path::Path>,
) -> Vec<CompletionCandidate> {
    let mut values = Vec::new();
    debug!("complete_arg_value: arg={arg:?}, value={value:?}");

    if let Some(possible_values) = possible_values(arg) {
        if let Ok(value) = value {
            values.extend(possible_values.into_iter().filter_map(|p| {
                let name = p.get_name();
                name.starts_with(value).then(|| {
                    CompletionCandidate::new(OsString::from(name))
                        .help(p.get_help().cloned())
                        .visible(!p.is_hide_set())
                })
            }));
        }
    } else {
        let value_os = match value {
            Ok(value) => OsStr::new(value),
            Err(value_os) => value_os,
        };
        match arg.get_value_hint() {
            clap::ValueHint::Other => {
                // Should not complete
            }
            clap::ValueHint::Unknown | clap::ValueHint::AnyPath => {
                values.extend(complete_path(value_os, current_dir, |_| true));
            }
            clap::ValueHint::FilePath => {
                values.extend(complete_path(value_os, current_dir, |p| p.is_file()));
            }
            clap::ValueHint::DirPath => {
                values.extend(complete_path(value_os, current_dir, |p| p.is_dir()));
            }
            clap::ValueHint::ExecutablePath => {
                use is_executable::IsExecutable;
                values.extend(complete_path(value_os, current_dir, |p| p.is_executable()));
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
                values.extend(complete_path(value_os, current_dir, |_| true));
            }
        }
        values.sort();
    }

    values
}

fn complete_path(
    value_os: &OsStr,
    current_dir: Option<&std::path::Path>,
    is_wanted: impl Fn(&std::path::Path) -> bool,
) -> Vec<CompletionCandidate> {
    let mut completions = Vec::new();

    let current_dir = match current_dir {
        Some(current_dir) => current_dir,
        None => {
            // Can't complete without a `current_dir`
            return Vec::new();
        }
    };
    let (existing, prefix) = value_os
        .split_once("\\")
        .unwrap_or((OsStr::new(""), value_os));
    let root = current_dir.join(existing);
    debug!("complete_path: root={root:?}, prefix={prefix:?}");
    let prefix = prefix.to_string_lossy();

    for entry in std::fs::read_dir(&root)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
    {
        let raw_file_name = entry.file_name();
        if !raw_file_name.starts_with(&prefix) {
            continue;
        }

        if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
            let path = entry.path();
            let mut suggestion = pathdiff::diff_paths(&path, current_dir).unwrap_or(path);
            suggestion.push(""); // Ensure trailing `/`
            completions.push(
                CompletionCandidate::new(suggestion.as_os_str().to_owned())
                    .help(None)
                    .visible(true),
            );
        } else {
            let path = entry.path();
            if is_wanted(&path) {
                let suggestion = pathdiff::diff_paths(&path, current_dir).unwrap_or(path);
                completions.push(
                    CompletionCandidate::new(suggestion.as_os_str().to_owned())
                        .help(None)
                        .visible(true),
                );
            }
        }
    }

    completions
}

fn complete_subcommand(value: &str, cmd: &clap::Command) -> Vec<CompletionCandidate> {
    debug!(
        "complete_subcommand: cmd={:?}, value={:?}",
        cmd.get_name(),
        value
    );

    let mut scs = subcommands(cmd)
        .into_iter()
        .filter(|x| x.content.starts_with(value))
        .collect::<Vec<_>>();
    scs.sort();
    scs.dedup();
    scs
}

/// Gets all the long options, their visible aliases and flags of a [`clap::Command`] with formatted `--` prefix.
/// Includes `help` and `version` depending on the [`clap::Command`] settings.
fn longs_and_visible_aliases(p: &clap::Command) -> Vec<CompletionCandidate> {
    debug!("longs: name={}", p.get_name());

    p.get_arguments()
        .filter_map(|a| {
            a.get_long_and_visible_aliases().map(|longs| {
                longs.into_iter().map(|s| {
                    CompletionCandidate::new(format!("--{}", s.to_string()))
                        .help(a.get_help().cloned())
                        .visible(!a.is_hide_set())
                })
            })
        })
        .flatten()
        .collect()
}

/// Gets all the long hidden aliases and flags of a [`clap::Command`].
fn hidden_longs_aliases(p: &clap::Command) -> Vec<CompletionCandidate> {
    debug!("longs: name={}", p.get_name());

    p.get_arguments()
        .filter_map(|a| {
            a.get_aliases().map(|longs| {
                longs.into_iter().map(|s| {
                    CompletionCandidate::new(format!("--{}", s.to_string()))
                        .help(a.get_help().cloned())
                        .visible(false)
                })
            })
        })
        .flatten()
        .collect()
}

/// Gets all the short options, their visible aliases and flags of a [`clap::Command`].
/// Includes `h` and `V` depending on the [`clap::Command`] settings.
fn shorts_and_visible_aliases(p: &clap::Command) -> Vec<CompletionCandidate> {
    debug!("shorts: name={}", p.get_name());

    p.get_arguments()
        .filter_map(|a| {
            a.get_short_and_visible_aliases().map(|shorts| {
                shorts.into_iter().map(|s| {
                    CompletionCandidate::new(s.to_string())
                        .help(a.get_help().cloned())
                        .visible(!a.is_hide_set())
                })
            })
        })
        .flatten()
        .collect()
}

/// Get the possible values for completion
fn possible_values(a: &clap::Arg) -> Option<Vec<clap::builder::PossibleValue>> {
    if !a.get_num_args().expect("built").takes_values() {
        None
    } else {
        a.get_value_parser()
            .possible_values()
            .map(|pvs| pvs.collect())
    }
}

/// Gets subcommands of [`clap::Command`] in the form of `("name", "bin_name")`.
///
/// Subcommand `rustup toolchain install` would be converted to
/// `("install", "rustup toolchain install")`.
fn subcommands(p: &clap::Command) -> Vec<CompletionCandidate> {
    debug!("subcommands: name={}", p.get_name());
    debug!("subcommands: Has subcommands...{:?}", p.has_subcommands());
    p.get_subcommands()
        .flat_map(|sc| {
            sc.get_name_and_visible_aliases()
                .into_iter()
                .map(|s| {
                    CompletionCandidate::new(s.to_string())
                        .help(sc.get_about().cloned())
                        .visible(!sc.is_hide_set())
                })
                .chain(sc.get_aliases().into_iter().map(|s| {
                    CompletionCandidate::new(s.to_string())
                        .help(sc.get_about().cloned())
                        .visible(false)
                }))
        })
        .collect()
}

/// A completion candidate definition
///
/// This makes it easier to add more fields to completion candidate,
/// rather than using `(OsString, Option<StyledStr>)` or `(String, Option<StyledStr>)` to represent a completion candidate
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompletionCandidate {
    /// Main completion candidate content
    content: OsString,

    /// Help message with a completion candidate
    help: Option<StyledStr>,

    /// Whether the completion candidate is visible
    visible: bool,
}

impl CompletionCandidate {
    /// Create a new completion candidate
    pub fn new(content: impl Into<OsString>) -> Self {
        let content = content.into();
        Self {
            content,
            ..Default::default()
        }
    }

    /// Set the help message of the completion candidate
    pub fn help(mut self, help: Option<StyledStr>) -> Self {
        self.help = help;
        self
    }

    /// Set the visibility of the completion candidate
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Get the content of the completion candidate
    pub fn get_content(&self) -> &OsStr {
        &self.content
    }

    /// Get the help message of the completion candidate
    pub fn get_help(&self) -> Option<&StyledStr> {
        self.help.as_ref()
    }

    /// Get the visibility of the completion candidate
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}
