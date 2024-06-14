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

/// NOTE: borrow the idea from Parser.rs::ParseState to record the state during completion.s
#[derive(Debug, PartialEq, Eq, Clone)]
enum CompletionState {
    /// Completion value done, there is no state to record.
    ValueDone,

    /// Completing a optional flag
    Opt(clap::Arg),

    /// Completing a positional argument
    Pos(usize),

    /// Error during completing parse
    Unknown,
}

// TODO: Index the positional argument and support more subcommnad completion. Consider things as follows:
// 1. `allow_missing_positional`
// 2. long_flag, short_flag and alias for subcommand
// 3. `allow_external_subcommands`
// 4. `args_conflicts_with_subcommands`
// 5. `subcommand_precedence_over_arg`
// 6. `multicall`

/// Complete the given command 
pub fn complete(
    cmd: &mut clap::Command,
    args: Vec<OsString>,
    arg_index: usize,
    current_dir: Option<&std::path::Path>,
) -> Result<Vec<(OsString, Option<StyledStr>)>, std::io::Error> {
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

    // TODO: Multicall support => We should do something in glue script code.
    if !cmd.is_no_binary_name_set() {
        raw_args.next_os(&mut cursor);
    }

    let mut current_cmd = &*cmd;
    let mut pos_index = 1;
    let mut is_escaped = false;
    let mut state = CompletionState::Unknown;
    while let Some(arg) = raw_args.next(&mut cursor) {
        if cursor == target_cursor {
            match state {
                CompletionState::ValueDone | CompletionState::Unknown => {
                    return complete_arg(&arg, current_cmd, current_dir, pos_index, is_escaped);
                }
                CompletionState::Opt(opt) => {
                    return Ok(complete_arg_value(arg.to_value(), &opt, current_dir));
                }
                CompletionState::Pos(pos) => {
                    if let Some(positional) =
                        cmd.get_positionals().find(|p| p.get_index() == Some(pos))
                    {
                        return Ok(complete_arg_value(arg.to_value(), positional, current_dir));
                    }
                }
            }
        }

        debug!("complete::next: Begin parsing '{:?}'", arg.to_value_os(),);

        if let Ok(value) = arg.to_value() {
            if let Some(next_cmd) = current_cmd.find_subcommand(value) {
                current_cmd = next_cmd;
                state = CompletionState::ValueDone;
                pos_index = 1;
                continue;
            }
        }

        if is_escaped {
            pos_index += 1;
            state = CompletionState::Pos(pos_index);
        } else {
            if arg.is_long() {
                if let Some((flag, value)) = arg.to_long() {
                    if let Ok(flag) = flag {
                        state = if let None = value {
                            let opt = current_cmd
                                .get_arguments()
                                .find(|a| a.get_long() == Some(flag));
                            if let Some(opt) = opt {
                                // HACK: Assuming knowledge of `--flag=value` will be split into `--flag` `=` `value` in bash.
                                // And it will not be split in other shells.
                                // It will limit the completion for the situation that `=` is a value of an optional flag in other shells.
                                if let Some(equal) = raw_args.peek(&cursor) {
                                    if equal.is_equal() {
                                        raw_args.next(&mut cursor);
                                    }
                                }
                                CompletionState::Opt(opt.clone())
                            } else {
                                CompletionState::Unknown
                            }
                        } else {
                            CompletionState::ValueDone
                        }
                    }
                }
            } else if arg.is_escape() {
                is_escaped = true;
                state = CompletionState::Pos(pos_index);
            } else if arg.is_negative_number() {
                state = CompletionState::ValueDone;
            } else if arg.is_short() {
                if let Some(short) = arg.to_short() {
                    let mut short = short.clone();
                    // HACK: Not consider `-fhg` now. During parsing, we assume that ShortFlags are in the format of `-fbar` or `-f=bar`.
                    let opt = short.next_flag();
                    state = if let Some(opt) = opt {
                        if let Ok(opt) = opt {
                            let opt = current_cmd
                                .get_arguments()
                                .find(|a| a.get_short() == Some(opt));
                            if let Some(opt) = opt {
                                // HACK: Assuming knowledge of `-f=value` will be split into `-f` `=` `value` in bash.
                                // And it will not be split in other shells.
                                // It will limit the completion for the situation that `=` is a value of an optional flag in other shells.
                                if let Some(equal) = raw_args.peek(&cursor) {
                                    if equal.is_equal() {
                                        raw_args.next(&mut cursor);
                                    }
                                }
                                CompletionState::Opt(opt.clone())
                            } else {
                                CompletionState::Unknown
                            }
                        } else {
                            CompletionState::Unknown
                        }
                    } else {
                         CompletionState::Unknown
                    }
                }
            } else if arg.is_stdio() {
            } else if arg.is_empty() {
            } else {
                pos_index += 1;
                state = CompletionState::Pos(pos_index);
            }
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "no completion generated",
    ))
}

fn complete_arg(
    arg: &clap_lex::ParsedArg<'_>,
    cmd: &clap::Command,
    current_dir: Option<&std::path::Path>,
    pos_index: usize,
    is_escaped: bool,
) -> Result<Vec<(OsString, Option<StyledStr>)>, std::io::Error> {
    debug!(
        "complete_arg: arg={:?}, cmd={:?}, current_dir={:?}, pos_index={}, is_escaped={}",
        arg,
        cmd.get_name(),
        current_dir,
        pos_index,
        is_escaped
    );
    let mut completions = Vec::new();
    if !is_escaped {
        if arg.is_long() {
            if let Some((flag, value)) = arg.to_long() {
                if let Ok(flag) = flag {
                    if let Some(arg) = cmd.get_arguments().find(|a| a.get_long() == Some(flag)) {
                        if let Some(value) = value {
                            completions.extend(
                                complete_arg_value(value.to_str().ok_or(value), arg, current_dir)
                                    .into_iter()
                                    .map(|(os, help)| {
                                        (
                                            format!("--{}={}", flag, os.to_string_lossy()).into(),
                                            help,
                                        )
                                    }),
                            );
                        }
                    } else {
                        if let Some(_) = value {
                        } else {
                            completions.extend(
                                longs_and_visible_aliases(cmd).into_iter().filter_map(
                                    |(f, help)| {
                                        f.starts_with(flag).then(|| (format!("--{f}").into(), help))
                                    },
                                ),
                            );
                        }
                    }
                }
            }
        } else if arg.is_escape() {
            // HACK: Assuming knowledge of is_escape
            completions.extend(
                longs_and_visible_aliases(cmd)
                    .into_iter()
                    .map(|(f, help)| (format!("--{f}").into(), help)),
            );
        } else if arg.is_negative_number() {
        } else if arg.is_short() {
            // HACK: Assuming knowledge of -f<TAB>` and `-f=<TAB>` to complete the value of `-f`
            if let Some(short) = arg.to_short() {
                let mut short = short.clone();
                let opt = short.next_flag();
                if let Some(opt) = opt {
                    if let Ok(opt) = opt {
                        if let Some(arg) = cmd.get_arguments().find(|a| a.get_short() == Some(opt))
                        {
                            if let Some(equal) = short.peek_next_flag() {
                                if let Ok(equal) = equal {
                                    if equal == '=' {
                                        short.next_flag();
                                    }
                                }
                            }
                            if let Some(value) = short.next_value_os() {
                                completions.extend(
                                    complete_arg_value(
                                        value.to_str().ok_or(value),
                                        arg,
                                        current_dir,
                                    )
                                    .into_iter()
                                    .map(|(f, help)| {
                                        (format!("-{}{}", opt, f.to_string_lossy()).into(), help)
                                    }),
                                )
                            }
                        }
                    }
                }
            }
        } else if arg.is_stdio() {
            // HACK: Assuming knowledge of is_stdio
            completions.extend(
                longs_and_visible_aliases(cmd)
                    .into_iter()
                    .map(|(f, help)| (format!("--{f}").into(), help)),
            );
            // HACK: Assuming knowledge of is_stdio / is_escape
            completions.extend(
                shorts_and_visible_aliases(cmd)
                    .into_iter()
                    .map(|(f, help)| (format!("-{}", f).into(), help)),
            );
        } else if arg.is_empty() {
            // NOTE: Do nothing for empty arg.
        }
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

    Ok(completions)
}

fn complete_arg_value(
    value: Result<&str, &OsStr>,
    arg: &clap::Arg,
    current_dir: Option<&std::path::Path>,
) -> Vec<(OsString, Option<StyledStr>)> {
    let mut values = Vec::new();
    debug!("complete_arg_value: arg={arg:?}, value={value:?}");

    if let Some(possible_values) = possible_values(arg) {
        if let Ok(value) = value {
            values.extend(possible_values.into_iter().filter_map(|p| {
                let name = p.get_name();
                name.starts_with(value)
                    .then(|| (name.into(), p.get_help().cloned()))
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
) -> Vec<(OsString, Option<StyledStr>)> {
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
            completions.push((suggestion.as_os_str().to_owned(), None));
        } else {
            let path = entry.path();
            if is_wanted(&path) {
                let suggestion = pathdiff::diff_paths(&path, current_dir).unwrap_or(path);
                completions.push((suggestion.as_os_str().to_owned(), None));
            }
        }
    }

    completions
}

// TODO: support more subcommands alias completion.
fn complete_subcommand(value: &str, cmd: &clap::Command) -> Vec<(OsString, Option<StyledStr>)> {
    debug!(
        "complete_subcommand: cmd={:?}, value={:?}",
        cmd.get_name(),
        value
    );

    let mut scs = subcommands(cmd)
        .into_iter()
        .filter(|x| x.0.starts_with(value))
        .map(|x| (OsString::from(&x.0), x.1))
        .collect::<Vec<_>>();
    scs.sort();
    scs.dedup();
    scs
}

/// Gets all the long options, their visible aliases and flags of a [`clap::Command`].
/// Includes `help` and `version` depending on the [`clap::Command`] settings.
fn longs_and_visible_aliases(p: &clap::Command) -> Vec<(String, Option<StyledStr>)> {
    debug!("longs: name={}", p.get_name());

    p.get_arguments()
        .filter_map(|a| {
            a.get_long_and_visible_aliases().map(|longs| {
                longs
                    .into_iter()
                    .map(|s| (s.to_string(), a.get_help().cloned()))
            })
        })
        .flatten()
        .collect()
}

/// Gets all the short options, their visible aliases and flags of a [`clap::Command`].
/// Includes `h` and `V` depending on the [`clap::Command`] settings.
fn shorts_and_visible_aliases(p: &clap::Command) -> Vec<(char, Option<StyledStr>)> {
    debug!("shorts: name={}", p.get_name());

    p.get_arguments()
        .filter_map(|a| {
            a.get_short_and_visible_aliases()
                .map(|shorts| shorts.into_iter().map(|s| (s, a.get_help().cloned())))
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

// TODO: support more subcommands alias completion.
/// Gets subcommands of [`clap::Command`] in the form of `("name", "bin_name")`.
///
/// Subcommand `rustup toolchain install` would be converted to
/// `("install", "rustup toolchain install")`.
fn subcommands(p: &clap::Command) -> Vec<(String, Option<StyledStr>)> {
    debug!("subcommands: name={}", p.get_name());
    debug!("subcommands: Has subcommands...{:?}", p.has_subcommands());

    p.get_subcommands()
        .map(|sc| (sc.get_name().to_string(), sc.get_about().cloned()))
        .collect()
}
