/// Completion support for Fish
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Fish;

impl crate::dynamic::Completer for Fish {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.fish")
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
        writeln!(
            buf,
            r#"complete -x -c {bin} -a "("'{completer}'" complete --shell fish -- (commandline --current-process --tokenize --cut-at-cursor) (commandline --current-token))""#
        )
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<std::ffi::OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index = args.len() - 1;
        let completions = crate::dynamic::complete(cmd, args, index, current_dir)?;

        for (completion, help) in completions {
            write!(buf, "{}", completion.to_string_lossy())?;
            if let Some(help) = help {
                write!(
                    buf,
                    "\t{}",
                    help.to_string().lines().next().unwrap_or_default()
                )?;
            }
            writeln!(buf)?;
        }
        Ok(())
    }
}
