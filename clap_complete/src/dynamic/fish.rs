//! Complete commands within fish

// For fish the behaviour should match the default as close as possible
// 1. grouping shorthand completions i.e. -a<TAB> should show other shorthands to end up with
//    -ams...
// 2. only complete options when one - is typed
// Due to https://github.com/fish-shell/fish-shell/issues/7943 we need to implement this our self
//

use std::{
    ffi::OsString,
    fmt::Display,
    io::{stdout, Write},
    iter,
};

use clap::Args;
use clap_lex::RawOsString;

use super::{complete, completions_for_arg, Completer, CompletionContext};

/// Dynamic completion for Fish
pub struct Fish;

impl Completer for Fish {
    type CompleteArgs = CompleteArgs;

    fn completion_script(cmd: &mut clap::Command) -> Vec<u8> {
        let name = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());
        format!(
            r#"complete -x -c {name} -a "{name} complete --previous=(commandline --current-process --tokenize --cut-at-cursor) --current (commandline --current-token)""#
        ).into_bytes()
    }

    fn file_name(name: &str) -> String {
        format!("{name}.fish")
    }

    fn try_complete(
        CompleteArgs {
            commandline,
            current,
        }: Self::CompleteArgs,
        cmd: &mut clap::Command,
    ) -> clap::error::Result<()> {
        let CompletionContext {
            value_for,
            options,
            subcommands,
        } = complete(cmd, commandline)?;

        let mut completions = Vec::new();
        let current = RawOsString::new(current);

        // fish only shows flags, when the user currently types a flag
        if !options.is_empty() && current.starts_with("--") {
            for option in options {
                // TODO maybe only offer aliases when the user currently is typing one
                // This could easily inflate the number of completions when a lot of aliases are
                // used, on the other hand this can be achieved by using hidden aliases
                if let Some(longs) = option.get_long_and_visible_aliases() {
                    for long in longs {
                        add_completion(
                            &mut completions,
                            format!("--{long}").as_bytes(),
                            option.get_help(),
                        )?;
                    }
                }
            }
            // TODO display non option values, in case the arg allows values starting with `--`
        } else if !options.is_empty() && current.starts_with('-') {
            // TODO implement flag joining i.e. show `-er` when the user typed `-e`
            for option in options {
                if let Some(shorts) = option.get_short_and_visible_aliases() {
                    for short in shorts {
                        add_completion(
                            &mut completions,
                            format!("-{short}").as_bytes(),
                            option.get_help(),
                        )?;
                    }
                }
            }
            // TODO display non option values, in case the arg allows values starting with `-`
        } else if let Some(value_for) = value_for {
            for (item, help) in completions_for_arg(value_for, &current)? {
                add_completion(&mut completions, item.as_raw_bytes(), help)?;
            }
        } else {
            for subcommand in subcommands {
                for alias in
                    iter::once(subcommand.get_name()).chain(subcommand.get_visible_aliases())
                {
                    add_completion(&mut completions, alias.as_bytes(), subcommand.get_about())?;
                }
            }
        }

        stdout().write_all(&completions)?;
        Ok(())
    }
}

fn add_completion(
    completions: &mut Vec<u8>,
    item: &[u8],
    help: Option<impl Display>,
) -> clap::error::Result<()> {
    // TODO do some escaping if necessary
    completions.write_all(item)?;
    if let Some(help) = help {
        writeln!(completions, "\t{help}")?;
    } else {
        writeln!(completions)?;
    }
    Ok(())
}

#[derive(Args, Clone, Debug)]
/// Arguments for Fish Completion
pub struct CompleteArgs {
    /// commandline tokens before the cursor
    #[arg(long, allow_hyphen_values = true, value_delimiter = '\n')]
    commandline: Vec<OsString>,
    /// token containing the cursor
    // TODO maybe use RawOsString here, but clap would need to support it
    #[arg(long, allow_hyphen_values = true)]
    current: OsString,
}
