/// Completion support for zsh
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Zsh;

impl crate::dynamic::Completer for Zsh {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.zsh")
    }
    fn write_registration(
        &self,
        _name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let bin = shlex::quote(bin);
        let completer = shlex::quote(completer);
        let script = r#"#compdef BIN
function _clap_dynamic_completer() {
    export _CLAP_COMPLETE_INDEX=$(expr $CURRENT - 1)
    export _CLAP_IFS=$'\n'

    local completions=("${(@f)$(COMPLETER complete --shell zsh -- ${words} 2>/dev/null)}")

    if [[ -n $completions ]]; then
        compadd -a completions
    fi
}

compdef _clap_dynamic_completer BIN"#
            .replace("COMPLETER", &completer)
            .replace("BIN", &bin);

        writeln!(buf, "{script}")?;
        Ok(())
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<std::ffi::OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index: usize = std::env::var("_CLAP_COMPLETE_INDEX")
            .ok()
            .and_then(|i| i.parse().ok())
            .unwrap_or_default();
        let ifs: Option<String> = std::env::var("_CLAP_IFS").ok().and_then(|i| i.parse().ok());

        // If the current word is empty, add an empty string to the args
        let mut args = args.clone();
        if args.len() == index {
            args.push("".into());
        }
        let completions = crate::dynamic::complete(cmd, args, index, current_dir)?;

        for (i, candidate) in completions.iter().enumerate() {
            if i != 0 {
                write!(buf, "{}", ifs.as_deref().unwrap_or("\n"))?;
            }
            write!(buf, "{}", candidate.get_content().to_string_lossy())?;
        }
        Ok(())
    }
}
