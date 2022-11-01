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

use clap::{
    builder::{PossibleValue, StyledStr},
    Arg, Command,
};
use clap_complete::Generator;

/// Generate Nushell complete file
pub struct Nushell;

enum Argument<'a> {
    Short(Vec<char>),
    Long(Vec<&'a str>),
    ShortAndLong(Vec<char>, Vec<&'a str>),
    Positional(bool),
}

struct ArgumentLine<'a, 'b> {
    id: &'a str,
    name: &'b str,
    arg: Argument<'a>,
    takes_values: bool,
    possible_values: Vec<PossibleValue>,
    help: Option<&'a StyledStr>,
}

impl<'a, 'b> ArgumentLine<'a, 'b> {
    fn new(arg: &'a Arg, name: &'b str) -> Self {
        let id = arg.get_id().as_str();

        let takes_values = arg
            .get_num_args()
            .map(|v| v.takes_values())
            .unwrap_or(false);

        let possible_values = arg.get_possible_values();

        let help = arg.get_help();

        if arg.is_positional() {
            let required = arg.is_required_set();
            let arg = Argument::Positional(required);

            return Self {
                id,
                name,
                arg,
                takes_values,
                possible_values,
                help,
            };
        }

        let shorts = arg.get_short_and_visible_aliases();
        let longs = arg.get_long_and_visible_aliases();

        match shorts {
            Some(shorts) => match longs {
                Some(longs) => Self {
                    id,
                    name,
                    arg: Argument::ShortAndLong(shorts, longs),
                    takes_values,
                    possible_values,
                    help,
                },
                None => Self {
                    id,
                    name,
                    arg: Argument::Short(shorts),
                    takes_values,
                    possible_values,
                    help,
                },
            },
            None => match longs {
                Some(longs) => Self {
                    id,
                    name,
                    arg: Argument::Long(longs),
                    takes_values,
                    possible_values,
                    help,
                },
                None => unreachable!("No short or long option found"),
            },
        }
    }

    fn generate_value_hints(&self) -> Option<String> {
        if self.possible_values.is_empty() {
            return None;
        }

        let mut s = format!(r#"  def "nu-complete {} {}" [] {{"#, self.name, self.id);
        s.push_str("\n    [");

        for value in &self.possible_values {
            s.push_str(format!(r#" "{}""#, value.get_name()).as_str());
        }

        s.push_str(" ]\n  }\n\n");

        Some(s)
    }

    fn append_type_and_help(&self, s: &mut String) {
        if self.takes_values {
            s.push_str(": string");

            if !self.possible_values.is_empty() {
                s.push_str(format!(r#"@"nu-complete {} {}""#, self.name, self.id).as_str())
            }
        }

        if let Some(help) = self.help {
            s.push_str(format!("\t# {}", help).as_str());
        }

        s.push('\n');
    }
}

impl ToString for ArgumentLine<'_, '_> {
    fn to_string(&self) -> String {
        let mut s = String::new();

        match &self.arg {
            Argument::Short(shorts) => {
                for short in shorts {
                    s.push_str(format!("    -{}", short).as_str());
                    self.append_type_and_help(&mut s);
                }
            }
            Argument::Long(longs) => {
                for long in longs {
                    s.push_str(format!("    --{}", long).as_str());
                    self.append_type_and_help(&mut s);
                }
            }
            Argument::ShortAndLong(shorts, longs) => {
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
            Argument::Positional(required) => {
                s.push_str(format!("    {}", self.id).as_str());

                if !*required {
                    s.push('?');
                }

                self.append_type_and_help(&mut s);
            }
        }

        s
    }
}

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

fn generate_completion(completions: &mut String, cmd: &Command, is_subcommand: bool) {
    let bin_name = cmd.get_bin_name().expect("Failed to get bin name");

    for hint in cmd
        .get_arguments()
        .filter_map(|arg| ArgumentLine::new(arg, bin_name).generate_value_hints())
    {
        completions.push_str(&hint);
    }

    let name = if is_subcommand {
        format!(r#""{}""#, bin_name)
    } else {
        bin_name.into()
    };

    if let Some(about) = cmd.get_about() {
        completions.push_str(format!("  # {}\n", about).as_str());
    }

    completions.push_str(format!("  export extern {} [\n", name).as_str());

    let s: String = cmd
        .get_arguments()
        .map(|arg| ArgumentLine::new(arg, bin_name).to_string())
        .collect();

    completions.push_str(&s);
    completions.push_str("  ]\n\n");

    // For sub-subcommands
    if is_subcommand {
        for sub in cmd.get_subcommands() {
            generate_completion(completions, sub, true);
        }
    }
}
