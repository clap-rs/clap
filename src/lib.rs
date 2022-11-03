//! Generates [Nushell](https://github.com/nushell/nushell) completions for [`clap`](https://github.com/clap-rs/clap) based CLIs
//!
//! ## Example
//!
//! ```
//! use clap::Command;
//! use clap_complete::generate;
//! use clap_complete_nushell::Nushell;
//! use std::io;
//!
//! let mut cmd = Command::new("myapp")
//!     .subcommand(Command::new("test").subcommand(Command::new("config")))
//!     .subcommand(Command::new("hello"));
//!
//! generate(Nushell, &mut cmd, "myapp", &mut io::stdout());
//! ```

use clap::{Arg, ArgAction, Command};
use clap_complete::Generator;

/// Generate Nushell complete file
pub struct Nushell;

impl Generator for Nushell {
    fn file_name(&self, name: &str) -> String {
        format!("{}.nu", name)
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn std::io::Write) {
        let mut completions = String::new();

        completions.push_str("module completions {\n\n");

        generate_completion(&mut completions, cmd, false);

        for sub in cmd.get_subcommands() {
            generate_completion(&mut completions, sub, true);
        }

        completions.push_str("}\n\n");
        completions.push_str("use completions *\n");

        buf.write_all(completions.as_bytes())
            .expect("Failed to write to generated file")
    }
}

struct Argument<'a, 'b> {
    arg: &'a Arg,
    name: &'b str,
}

impl<'a, 'b> Argument<'a, 'b> {
    fn new(arg: &'a Arg, name: &'b str) -> Self {
        Self { arg, name }
    }

    fn takes_values(&self) -> bool {
        self.arg
            .get_num_args()
            .map(|r| r.takes_values())
            .unwrap_or(false)
    }

    fn append_type_and_help(&self, s: &mut String) {
        if self.takes_values() {
            s.push_str(": string");

            if !self.arg.get_possible_values().is_empty() {
                s.push_str(
                    format!(r#"@"nu-complete {} {}""#, self.name, self.arg.get_id()).as_str(),
                )
            }
        }

        if let Some(help) = self.arg.get_help() {
            let max: usize = 30;
            let mut width = 0;
            if let Some(line) = s.lines().last() {
                width = max.saturating_sub(line.len());
            }
            s.push_str(format!("{:>width$}# {}", ' ', help,).as_str());
        }

        s.push('\n');
    }

    fn get_values_completion(&self) -> Option<String> {
        let possible_values = self.arg.get_possible_values();
        if possible_values.is_empty() {
            return None;
        }

        let mut s = format!(
            r#"  def "nu-complete {} {}" [] {{"#,
            self.name,
            self.arg.get_id()
        );
        s.push_str("\n    [");

        for value in &possible_values {
            let name = value.get_name();
            if name.contains(|c: char| c.is_whitespace()) {
                s.push_str(format!(r#" "\"{}\"""#, name).as_str());
            } else {
                s.push_str(format!(r#" "{}""#, name).as_str());
            }
        }

        s.push_str(" ]\n  }\n\n");

        Some(s)
    }
}

impl ToString for Argument<'_, '_> {
    fn to_string(&self) -> String {
        let mut s = String::new();

        if self.arg.is_positional() {
            // rest arguments
            if matches!(self.arg.get_action(), ArgAction::Append) {
                s.push_str(format!("    ...{}", self.arg.get_id()).as_str());
            } else {
                s.push_str(format!("    {}", self.arg.get_id()).as_str());

                if !self.arg.is_required_set() {
                    s.push('?');
                }
            }

            self.append_type_and_help(&mut s);

            return s;
        }

        let shorts = self.arg.get_short_and_visible_aliases();
        let longs = self.arg.get_long_and_visible_aliases();

        match shorts {
            Some(shorts) => match longs {
                Some(longs) => {
                    // short options and long options
                    s.push_str(
                        format!(
                            "    --{}(-{})",
                            longs.first().expect("At least one long option expected"),
                            shorts.first().expect("At lease one short option expected")
                        )
                        .as_str(),
                    );
                    self.append_type_and_help(&mut s);

                    // long alias
                    for long in longs.iter().skip(1) {
                        s.push_str(format!("    --{}", long).as_str());
                        self.append_type_and_help(&mut s);
                    }

                    // short alias
                    for short in shorts.iter().skip(1) {
                        s.push_str(format!("    -{}", short).as_str());
                        self.append_type_and_help(&mut s);
                    }
                }
                None => {
                    // short options only
                    for short in shorts {
                        s.push_str(format!("    -{}", short).as_str());
                        self.append_type_and_help(&mut s);
                    }
                }
            },
            None => match longs {
                Some(longs) => {
                    // long options only
                    for long in longs {
                        s.push_str(format!("    --{}", long).as_str());
                        self.append_type_and_help(&mut s);
                    }
                }
                None => unreachable!("No short or long optioin found"),
            },
        }

        s
    }
}

fn generate_completion(completions: &mut String, cmd: &Command, is_subcommand: bool) {
    let name = cmd.get_bin_name().expect("Failed to get bin name");

    for value in cmd
        .get_arguments()
        .filter_map(|arg| Argument::new(arg, name).get_values_completion())
    {
        completions.push_str(&value);
    }

    if let Some(about) = cmd.get_about() {
        completions.push_str(format!("  # {}\n", about).as_str());
    }

    if is_subcommand {
        completions.push_str(format!("  export extern \"{}\" [\n", name).as_str());
    } else {
        completions.push_str(format!("  export extern {} [\n", name).as_str());
    }

    for s in cmd
        .get_arguments()
        .map(|arg| Argument::new(arg, name).to_string())
    {
        completions.push_str(&s);
    }

    completions.push_str("  ]\n\n");

    if is_subcommand {
        for sub in cmd.get_subcommands() {
            generate_completion(completions, sub, true);
        }
    }
}
