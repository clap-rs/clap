use unicode_xid::UnicodeXID;

use crate::dynamic::registrar::Registrar;

/// Bash autocomplete file generation.
#[derive(clap::Args, Clone, Debug)]
pub struct BashGenerateArgs {}

impl BashGenerateArgs {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Registrar for BashGenerateArgs {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.bash")
    }

    fn write_registration(
        &self,
        name: &str,
        _bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let escaped_name = name.replace('-', "_");
        debug_assert!(
            escaped_name.chars().all(|c| c.is_xid_continue()),
            "`name` must be an identifier, got `{escaped_name}`"
        );
        let mut upper_name = escaped_name.clone();
        upper_name.make_ascii_uppercase();

        let completer = shlex::quote(completer);

        let script = r#"
_clap_complete_NAME() {
    export IFS=$'\013'
    local SUPPRESS_SPACE=0
    if compopt +o nospace 2> /dev/null; then
        SUPPRESS_SPACE=1
    fi
    if [[ ${SUPPRESS_SPACE} == 1 ]]; then
        SPACE_ARG="--no-space"
    else
        SPACE_ARG="--space"
    fi

    COMPREPLY=( $("COMPLETER" complete bash --index ${COMP_CWORD} --type ${COMP_TYPE} ${SPACE_ARG} --ifs="$IFS" -- "${COMP_WORDS[@]}") )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    elif [[ $SUPPRESS_SPACE == 1 ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
        compopt -o nospace
    fi
}
complete -o nosort -o noquote -o nospace -F _clap_complete_NAME BIN
"#
        .replace("NAME", &escaped_name)
        .replace("COMPLETER", &completer)
        .replace("UPPER", &upper_name);

        writeln!(buf, "{script}")?;
        Ok(())
    }
}
