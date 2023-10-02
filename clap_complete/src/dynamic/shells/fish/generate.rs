use crate::dynamic::Registrar;

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct FishGenerateArgs {}

impl Registrar for FishGenerateArgs {
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
            r#"complete -x -c {bin} -a "("'{completer}'" complete fish -- (commandline --current-process --tokenize --cut-at-cursor) (commandline --current-token))""#
        )
    }
}
