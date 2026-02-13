//! Implements dynamic completion for Nushell.
//!
//! There is no direct equivalent of other shells' `source $(COMPLETE=... your-clap-bin)` in nushell,
//! because code being sourced must exist at parse-time.
//!
//! One way to get close to that is to split the completion integration into two parts:
//!   1. a minimal part that goes into `env.nu`, which updates the actual completion integration
//!   2. the completion integration, which is placed into the user's autoload directory
//!
//! To install the completion integration, the user runs
//! ```nu
//! COMPLETE=nushell your-clap-bin | save --raw --force --append $nu.env-path
//! ```

// Std
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::io::{Error, Write};
use std::path::Path;

// External
use clap::Command;
use clap_complete::env::EnvCompleter;

/// Generate integration for dynamic completion in Nushell
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Nushell;

impl EnvCompleter for Nushell {
    fn name(&self) -> &'static str {
        "nushell"
    }

    fn is(&self, name: &str) -> bool {
        name == "nushell"
    }

    fn write_registration(
        &self,
        var: &str,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn Write,
    ) -> Result<(), Error> {
        let mode_var = ModeVar(var).to_string();
        if std::env::var_os(&mode_var).as_ref().map(|x| x.as_os_str())
            == Some(OsStr::new("integration"))
        {
            write_completion_script(var, name, bin, completer, buf)
        } else {
            write_refresh_completion_integration(var, name, completer, buf)
        }
    }

    fn write_complete(
        &self,
        cmd: &mut Command,
        args: Vec<OsString>,
        current_dir: Option<&Path>,
        buf: &mut dyn Write,
    ) -> Result<(), Error> {
        let idx = args.len().saturating_sub(1).max(0);
        let candidates = clap_complete::engine::complete(cmd, args, idx, current_dir)?;
        let mut strbuf = String::new();
        {
            let mut records = write_json::array(&mut strbuf);
            for candidate in candidates {
                let mut record = records.object();
                record.string("value", candidate.get_value().to_string_lossy().as_ref());
                if let Some(help) = candidate.get_help() {
                    record.string("description", &help.to_string()[..]);
                }
            }
        }
        write!(buf, "{strbuf}")
    }
}

struct ModeVar<'a>(&'a str);

impl<'a> Display for ModeVar<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "_{0}__mode", self.0)
    }
}

fn write_refresh_completion_integration(
    var: &str,
    name: &str,
    completer: &str,
    buf: &mut dyn Write,
) -> Result<(), Error> {
    let mode = ModeVar(var);
    writeln!(
        buf,
        r#"
# Refresh completer integration for {name} (must be in env.nu)
do {{
  # Search for existing script to avoid duplicates in case autoload dirs change
  let completer_script_name = '{name}-completer.nu'
  let autoload_dir = $nu.user-autoload-dirs
    | where {{ path join $completer_script_name | path exists }}
    | get 0 --optional
    | default ($nu.user-autoload-dirs | get 0 --optional)
  mkdir $autoload_dir

  let completer_path = ($autoload_dir | path join $completer_script_name)
  {var}=nushell {mode}=integration ^r#'{completer}'# | save --raw --force $completer_path
}}
        "#
    )
}

fn write_completion_script(
    var: &str,
    name: &str,
    _bin: &str,
    completer: &str,
    buf: &mut dyn Write,
) -> Result<(), Error> {
    writeln!(
        buf,
        r#"
# Performs the completion for {name}
def {name}-completer [
    spans: list<string> # The spans that were passed to the external completer closure
]: nothing -> list {{
    {var}=nushell ^r#'{completer}'# -- ...$spans | from json
}}

@complete {name}-completer
def --wrapped {name} [...args] {{
  ^r#'{completer}'# ...$args
}}
"#
    )
}
