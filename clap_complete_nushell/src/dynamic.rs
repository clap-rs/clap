use clap::Command;
use clap_complete::env::EnvCompleter;
use std::ffi::OsString;
use std::io::{Error, Write};
use std::path::Path;

impl EnvCompleter for super::Nushell {
    fn name(&self) -> &'static str {
        "nushell"
    }

    fn is(&self, name: &str) -> bool {
        name.eq_ignore_ascii_case("nushell") || name.eq_ignore_ascii_case("nu")
    }

    fn write_registration(
        &self,
        var: &str,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn Write,
    ) -> Result<(), Error> {
        writeln!(
            buf,
            r#"
# External completer for {name}
#
# This module can either be `source`d for a simplified installation or loaded as a module (`use`)
# to be integrated into your existing external completer setup.
#
# Example 1 (simplified installation):
# ```
# use {name}-completer.nu # (replace with path to this file)
# {name}-completer install
# ```
#
# Example 2 (integrate with custom external completer):
# ```
# use {name}-completer.nu # (replace with path to this file)
# $env.config.completions.external.enable = true
# $env.config.completions.external.completer = {{ |spans|
#   if ({name}-completer handles $spans) {{
#     {name}-completer complete $spans
#   }} else {{
#     # any other external completers
#   }}
# }}
# ```

# Workaround for https://github.com/nushell/nushell/issues/8483
def expand-alias []: list -> list {{
    let spans = $in
    if ($spans | length) == 0 {{
        return $spans
    }}

    let expanded_alias = (scope aliases | where name == $spans.0 | get --ignore-errors 0.expansion)
    if $expanded_alias != null  {{
        # put the first word of the expanded alias first in the span
        $spans | skip 1 | prepend ($expanded_alias | split row " " | take 1)
    }} else {{
        $spans
    }}
}}

# Determines whether the completer for {name} is supposed to handle the command line
export def handles [
    spans: list # The spans that were passed to the external completer closure
]: nothing -> bool {{
    ($spans | expand-alias | get --ignore-errors 0) == r#'{bin}'#
}}

# Performs the completion for {name}
export def complete [
    spans: list # The spans that were passed to the external completer closure
]: nothing -> list {{
    {var}=nushell ^r#'{completer}'# -- ...$spans | from json
}}

# Installs this module as an external completer for {name} globally.
#
# For commands other {name}, it will fall back to whatever external completer
# was defined previously (if any).
export def --env install []: nothing -> nothing {{
    $env.config = $env.config
      | upsert completions.external.enable true
      | upsert completions.external.completer {{ |original_config|
          let previous_completer = $original_config
            | get --ignore-errors completions.external.completer
            | default {{ |spans| null }}
          {{ |spans|
            if (handles $spans) {{
                complete $spans
            }} else {{
                do $previous_completer $spans
            }}
          }}
      }}
}}
"#
        )
    }

    fn write_complete(
        &self,
        cmd: &mut Command,
        args: Vec<OsString>,
        current_dir: Option<&Path>,
        buf: &mut dyn Write,
    ) -> Result<(), Error> {
        let idx = (args.len() - 1).max(0);
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
