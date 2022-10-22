//! Generates [Nushell](https://github.com/nushell/nushell) completions for [`clap`](https://github.com/clap-rs/clap) based CLIs

use clap::Command;
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

fn generate_completion(completions: &mut String, cmd: &Command, is_subcommand: bool) {
    if let Some(about) = cmd.get_about() {
        completions.push_str(format!("  # {}\n", about).as_str());
    }

    let bin_name = cmd.get_bin_name().expect("Failed to get bin name");

    if is_subcommand {
        completions.push_str(format!("  export extern \"{}\" [\n", bin_name).as_str());
    } else {
        completions.push_str(format!("  export extern {} [\n", bin_name).as_str());
    }

    let mut s = String::new();
    for arg in cmd.get_arguments() {
        if arg.is_positional() {
            s.push_str(format!("    {}", arg.get_id()).as_str());
            if !arg.is_required_set() {
                s.push('?');
            }
        }

        let long = arg.get_long();
        if let Some(opt) = long {
            s.push_str(format!("    --{}", opt).as_str());
        }

        let short = arg.get_short();
        if let Some(opt) = short {
            if long.is_some() {
                s.push_str(format!("(-{})", opt).as_str());
            } else {
                s.push_str(format!("    -{}", opt).as_str());
            }
        }

        if let Some(v) = arg.get_num_args() {
            if v.takes_values() {
                // TODO: add more types?
                // TODO: add possible values?
                s.push_str(": string");
            }
        }

        if let Some(msg) = arg.get_help() {
            if arg.is_positional() || long.is_some() || short.is_some() {
                s.push_str(format!("\t# {}", msg).as_str());
            }
        }

        s.push('\n');
    }

    completions.push_str(&s);
    completions.push_str("  ]\n\n");

    // For sub-subcommands
    if is_subcommand {
        for sub in cmd.get_subcommands() {
            generate_completion(completions, sub, true);
        }
    }
}
