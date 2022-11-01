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
    Arg, Command, Id,
};
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

    fn get_help(&self) -> Option<&StyledStr> {
        self.arg.get_help()
    }

    fn get_id(&self) -> &Id {
        self.arg.get_id()
    }

    fn get_possible_values(&self) -> Vec<PossibleValue> {
        self.arg.get_possible_values()
    }

    fn get_short_and_visiable_aliases(&self) -> Option<Vec<char>> {
        self.arg.get_short_and_visible_aliases()
    }

    fn get_long_and_visiable_aliases(&self) -> Option<Vec<&str>> {
        self.arg.get_long_and_visible_aliases()
    }

    fn is_positional(&self) -> bool {
        self.arg.is_positional()
    }

    fn is_required_set(&self) -> bool {
        self.arg.is_required_set()
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

            if !self.get_possible_values().is_empty() {
                s.push_str(format!(r#"@"nu-complete {} {}""#, self.name, self.get_id()).as_str())
            }
        }

        if let Some(help) = self.get_help() {
            s.push_str(format!("\t# {}", help).as_str());
        }

        s.push('\n');
    }

    fn get_values_completion(&self) -> Option<String> {
        let possible_values = self.get_possible_values();
        if possible_values.is_empty() {
            return None;
        }

        let mut s = format!(
            r#"  def "nu-complete {} {}" [] {{"#,
            self.name,
            self.get_id()
        );
        s.push_str("\n    [");

        for value in &possible_values {
            s.push_str(format!(r#" "{}""#, value.get_name()).as_str());
        }

        s.push_str(" ]\n  }\n\n");

        Some(s)
    }
}

impl ToString for Argument<'_, '_> {
    fn to_string(&self) -> String {
        let mut s = String::new();

        if self.is_positional() {
            s.push_str(format!("    {}", self.get_id()).as_str());

            if !self.is_required_set() {
                s.push('?');
            }

            self.append_type_and_help(&mut s);

            return s;
        }

        let shorts = self.get_short_and_visiable_aliases();
        let longs = self.get_long_and_visiable_aliases();

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
    let bin_name = cmd.get_bin_name().expect("Failed to get bin name");

    for v in cmd
        .get_arguments()
        .filter_map(|arg| Argument::new(arg, bin_name).get_values_completion())
    {
        completions.push_str(&v);
    }

    if let Some(about) = cmd.get_about() {
        completions.push_str(format!("  # {}\n", about).as_str());
    }

    let name = if is_subcommand {
        format!(r#""{}""#, bin_name)
    } else {
        bin_name.into()
    };

    completions.push_str(format!("  export extern {} [\n", name).as_str());

    let s: String = cmd
        .get_arguments()
        .map(|arg| Argument::new(arg, bin_name).to_string())
        .collect();

    completions.push_str(&s);
    completions.push_str("  ]\n\n");

    if is_subcommand {
        for sub in cmd.get_subcommands() {
            generate_completion(completions, sub, true);
        }
    }
}
