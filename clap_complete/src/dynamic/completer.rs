use std::ffi::OsStr;
use std::ffi::OsString;

use clap_lex::OsStrExt as _;

/// Specifies the number of dashes an argument must already
/// contain to complete short and long options.
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum ShowOptions {
    /// One `-` for short, two `--` for long options.
    ExactDash,
    /// At least one `-` to show options.
    MinOneDash,
    /// Always complete options.
    Always,
}

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
    /// Complete the command
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<std::ffi::OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error>;
}

/// Complete the command specified
pub fn complete(
    cmd: &mut clap::Command,
    args: Vec<std::ffi::OsString>,
    arg_index: usize,
    current_dir: Option<&std::path::Path>,
    show_options: ShowOptions,
) -> Result<Vec<std::ffi::OsString>, std::io::Error> {
    cmd.build();

    let raw_args = clap_lex::RawArgs::new(args.into_iter());
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
    while let Some(arg) = raw_args.next(&mut cursor) {
        if cursor == target_cursor {
            return complete_arg(
                &arg,
                current_cmd,
                current_dir,
                pos_index,
                is_escaped,
                show_options,
            );
        }

        debug!("complete::next: Begin parsing '{:?}'", arg.to_value_os(),);

        if let Ok(value) = arg.to_value() {
            if let Some(next_cmd) = current_cmd.find_subcommand(value) {
                current_cmd = next_cmd;
                pos_index = 0;
                continue;
            }
        }

        if is_escaped {
            pos_index += 1;
        } else if arg.is_escape() {
            is_escaped = true;
        } else if let Some(_long) = arg.to_long() {
        } else if let Some(_short) = arg.to_short() {
        } else {
            pos_index += 1;
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
    show_options: ShowOptions,
) -> Result<Vec<std::ffi::OsString>, std::io::Error> {
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
        if let Some((flag, value)) = arg.to_long() {
            if let Ok(flag) = flag {
                if let Some(value) = value {
                    if let Some(arg) = cmd.get_arguments().find(|a| a.get_long() == Some(flag)) {
                        completions.extend(
                            complete_arg_value(value.to_str().ok_or(value), arg, current_dir)
                                .into_iter()
                                .map(|os| {
                                    // HACK: Need better `OsStr` manipulation
                                    format!("--{}={}", flag, os.to_string_lossy()).into()
                                }),
                        )
                    }
                } else {
                    completions.extend(
                        crate::generator::utils::longs_and_visible_aliases(cmd)
                            .into_iter()
                            .filter_map(|f| f.starts_with(flag).then(|| format!("--{f}").into())),
                    );
                }
            }
        } else if arg.is_escape()
            || (show_options >= ShowOptions::MinOneDash && arg.is_stdio())
            || (show_options == ShowOptions::Always && arg.is_empty())
        {
            // HACK: Assuming knowledge of is_escape / is_stdio
            completions.extend(
                crate::generator::utils::longs_and_visible_aliases(cmd)
                    .into_iter()
                    .map(|f| format!("--{f}").into()),
            );
        }

        if arg.is_stdio()
            || arg.is_short()
            || (arg.is_empty() && show_options == ShowOptions::Always)
        {
            let dash_or_arg = if arg.is_empty() {
                "-".into()
            } else {
                arg.to_value_os().to_string_lossy()
            };
            // HACK: Assuming knowledge of is_stdio
            completions.extend(
                crate::generator::utils::shorts_and_visible_aliases(cmd)
                    .into_iter()
                    // HACK: Need better `OsStr` manipulation
                    .map(|f| format!("{}{}", dash_or_arg, f).into()),
            );
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
) -> Vec<OsString> {
    let mut values = Vec::new();
    debug!("complete_arg_value: arg={arg:?}, value={value:?}");

    if let Some(possible_values) = crate::generator::utils::possible_values(arg) {
        if let Ok(value) = value {
            values.extend(possible_values.into_iter().filter_map(|p| {
                let name = p.get_name();
                name.starts_with(value).then(|| name.into())
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
) -> Vec<OsString> {
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
            completions.push(suggestion.as_os_str().to_owned());
        } else {
            let path = entry.path();
            if is_wanted(&path) {
                let suggestion = pathdiff::diff_paths(&path, current_dir).unwrap_or(path);
                completions.push(suggestion.as_os_str().to_owned());
            }
        }
    }

    completions
}

fn complete_subcommand(value: &str, cmd: &clap::Command) -> Vec<OsString> {
    debug!(
        "complete_subcommand: cmd={:?}, value={:?}",
        cmd.get_name(),
        value
    );

    let mut scs = crate::generator::utils::subcommands(cmd)
        .into_iter()
        .filter(|x| x.0.starts_with(value))
        .map(|x| OsString::from(&x.0))
        .collect::<Vec<_>>();
    scs.sort();
    scs.dedup();
    scs
}
