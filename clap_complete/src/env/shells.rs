use std::ffi::OsString;
use std::str::FromStr;

use super::EnvCompleter;

/// Bash completion adapter
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Bash;

impl EnvCompleter for Bash {
    fn name(&self) -> &'static str {
        "bash"
    }
    fn is(&self, name: &str) -> bool {
        name == "bash"
    }
    fn write_registration(
        &self,
        var: &str,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let escaped_name = name.replace('-', "_");

        let completer =
            shlex::try_quote(completer).unwrap_or(std::borrow::Cow::Borrowed(completer));

        let script = r#"
_clap_complete_NAME() {
    local _CLAP_COMPLETE_INDEX=${COMP_CWORD}
    local _CLAP_COMPLETE_COMP_TYPE=${COMP_TYPE}
    if compopt +o nospace 2> /dev/null; then
        local _CLAP_COMPLETE_SPACE=false
    else
        local _CLAP_COMPLETE_SPACE=true
    fi
    local words=("${COMP_WORDS[@]}")
    local _clap_head_len=0
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        # Reassemble the word list by walking COMP_WORDS and COMP_LINE in
        # lockstep.  COMP_WORDS is split on both whitespace (IFS) and
        # COMP_WORDBREAKS characters; we want to undo only the COMP_WORDBREAKS
        # splits so that tokens like --opt=val are presented to the clap engine
        # as a single word.  We distinguish the two kinds of split by inspecting
        # the gap text between consecutive COMP_WORDS entries in COMP_LINE:
        #
        #   - If the first character of the gap is whitespace, the split was
        #     caused by a word boundary and must be kept.
        #   - Otherwise (gap is empty or starts with a non-whitespace character)
        #     the split was caused by COMP_WORDBREAKS alone and the two tokens
        #     are glued back together.
        #
        # COMP_LINE is used only as a read-only string; it is never eval'd.
        local _clap_line="${COMP_LINE:0:$COMP_POINT}"
        # Build the token list to reassemble.  COMP_WORDS is indexed up to
        # COMP_CWORD inclusive, but bash populates it from the full COMP_LINE,
        # so COMP_WORDS[COMP_CWORD] may be a word that lies after the cursor.
        # $2 tells us the actual word at the cursor (quote-stripped, truncated
        # at COMP_POINT); when it is non-empty, patch it into the slice.
        # When $2 is empty and the line ends with whitespace the cursor sits in
        # trailing whitespace: COMP_WORDS[COMP_CWORD] is the next word, not the
        # current one, so drop it; the trailing-whitespace check after the loop
        # will push the empty current word.
        local _clap_comp_words=("${COMP_WORDS[@]:0:COMP_CWORD+1}")
        if [[ -n "$2" ]]; then
            _clap_comp_words[COMP_CWORD]="$2"
        elif [[ "${_clap_line}" =~ [[:space:]]$ ]]; then
            _clap_comp_words=("${COMP_WORDS[@]:0:COMP_CWORD}")
        fi
        local _clap_words=()
        local _cword
        local _clap_pos=0
        local _clap_w _clap_gap _clap_rest _clap_gap_ch
        for _clap_w in "${_clap_comp_words[@]}"; do
            # The gap is the text in _clap_line between the previous token and
            # this one.  "${var%%"tok"*}" removes the longest suffix of var
            # that matches "tok"*, leaving the prefix before the first "tok".
            _clap_rest="${_clap_line:_clap_pos}"
            _clap_gap="${_clap_rest%%"${_clap_w}"*}"
            _clap_gap_ch="${_clap_gap:0:1}"
            if [[ ${#_clap_words[@]} -eq 0 || "${_clap_gap_ch}" == [[:space:]] ]]; then
                # No previous word yet, or the gap starts with whitespace:
                # this is a genuine word boundary — start a new word.
                _clap_words+=("${_clap_w}")
            else
                # Gap is empty or starts with a non-whitespace character:
                # a COMP_WORDBREAKS split — glue onto the previous word.
                _clap_words[${#_clap_words[@]}-1]+="${_clap_w}"
            fi
            _clap_pos=$(( _clap_pos + ${#_clap_gap} + ${#_clap_w} ))
        done
        # If the line up to the cursor ends with whitespace the current word
        # is empty.  COMP_WORDS omits it; push an explicit empty entry.
        if [[ "${_clap_line}" =~ [[:space:]]$ ]]; then
            _clap_words+=("")
        fi
        _cword=$(( ${#_clap_words[@]} - 1 ))
        _CLAP_COMPLETE_INDEX=$_cword
        words=("${_clap_words[@]}")
        # Bash splices each COMPREPLY entry in place of $2, which is only
        # the suffix of the current word after its last COMP_WORDBREAKS
        # character (the "tail").  Strip the head from every candidate so
        # that bash inserts the full token into the command line.
        local _clap_tail="${words[_cword]##*["${COMP_WORDBREAKS}"]}"
        _clap_head_len=$(( ${#words[_cword]} - ${#_clap_tail} ))
    fi
    # Capture output and exit code separately: `local var=$(cmd)` always
    # returns 0 because `local` itself succeeds, masking cmd's exit code.
    local _clap_out
    _clap_out=$( \
        _CLAP_COMPLETE_INDEX="$_CLAP_COMPLETE_INDEX" \
        _CLAP_COMPLETE_COMP_TYPE="$_CLAP_COMPLETE_COMP_TYPE" \
        _CLAP_COMPLETE_SPACE="$_CLAP_COMPLETE_SPACE" \
        VAR="bash" \
        "COMPLETER" -- "${words[@]}" \
    )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    else
        local _clap_completions=() _clap_c
        if [[ -n "$_clap_out" ]]; then
            mapfile -t _clap_completions <<< "$_clap_out"
        fi
        COMPREPLY=()
        for _clap_c in "${_clap_completions[@]}"; do
            COMPREPLY+=("${_clap_c:_clap_head_len}")
        done
        if [[ $_CLAP_COMPLETE_SPACE == false ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
            compopt -o nospace
        fi
    fi
}
if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -o nospace -o bashdefault -o nosort -F _clap_complete_NAME BIN
else
    complete -o nospace -o bashdefault -F _clap_complete_NAME BIN
fi
"#
        .replace("NAME", &escaped_name)
        .replace("BIN", bin)
        .replace("COMPLETER", &completer)
        .replace("VAR", var);

        writeln!(buf, "{script}")?;
        Ok(())
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index: usize = std::env::var("_CLAP_COMPLETE_INDEX")
            .ok()
            .and_then(|i| i.parse().ok())
            .unwrap_or_default();
        let _comp_type: CompType = std::env::var("_CLAP_COMPLETE_COMP_TYPE")
            .ok()
            .and_then(|i| i.parse().ok())
            .unwrap_or_default();
        let _space: Option<bool> = std::env::var("_CLAP_COMPLETE_SPACE")
            .ok()
            .and_then(|i| i.parse().ok());
        let ifs: Option<String> = std::env::var("_CLAP_IFS").ok().and_then(|i| i.parse().ok());
        let completions = crate::engine::complete(cmd, args, index, current_dir)?;

        for (i, candidate) in completions.iter().enumerate() {
            if i != 0 {
                write!(buf, "{}", ifs.as_deref().unwrap_or("\n"))?;
            }
            write!(buf, "{}", candidate.get_value().to_string_lossy())?;
        }
        Ok(())
    }
}

/// Type of completion attempted that caused a completion function to be called
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
enum CompType {
    /// Normal completion
    #[default]
    Normal,
    /// List completions after successive tabs
    Successive,
    /// List alternatives on partial word completion
    Alternatives,
    /// List completions if the word is not unmodified
    Unmodified,
    /// Menu completion
    Menu,
}

impl FromStr for CompType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "9" => Ok(Self::Normal),
            "63" => Ok(Self::Successive),
            "33" => Ok(Self::Alternatives),
            "64" => Ok(Self::Unmodified),
            "37" => Ok(Self::Menu),
            _ => Err(format!("unsupported COMP_TYPE `{s}`")),
        }
    }
}

/// Elvish completion adapter
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Elvish;

impl EnvCompleter for Elvish {
    fn name(&self) -> &'static str {
        "elvish"
    }
    fn is(&self, name: &str) -> bool {
        name == "elvish"
    }
    fn write_registration(
        &self,
        var: &str,
        _name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let bin = shlex::try_quote(bin).unwrap_or(std::borrow::Cow::Borrowed(bin));
        let completer =
            shlex::try_quote(completer).unwrap_or(std::borrow::Cow::Borrowed(completer));

        let script = r#"
set edit:completion:arg-completer[BIN] = { |@words|
    var index = (count $words)
    set index = (- $index 1)

    put (env _CLAP_IFS="\n" _CLAP_COMPLETE_INDEX=(to-string $index) VAR="elvish" COMPLETER -- $@words) | to-lines
}
"#
        .replace("COMPLETER", &completer)
        .replace("BIN", &bin)
        .replace("VAR", var);

        writeln!(buf, "{script}")?;
        Ok(())
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index: usize = std::env::var("_CLAP_COMPLETE_INDEX")
            .ok()
            .and_then(|i| i.parse().ok())
            .unwrap_or_default();
        let ifs: Option<String> = std::env::var("_CLAP_IFS").ok().and_then(|i| i.parse().ok());
        let completions = crate::engine::complete(cmd, args, index, current_dir)?;

        for (i, candidate) in completions.iter().enumerate() {
            if i != 0 {
                write!(buf, "{}", ifs.as_deref().unwrap_or("\n"))?;
            }
            write!(buf, "{}", candidate.get_value().to_string_lossy())?;
        }
        Ok(())
    }
}

/// Fish completion adapter
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Fish;

impl EnvCompleter for Fish {
    fn name(&self) -> &'static str {
        "fish"
    }
    fn is(&self, name: &str) -> bool {
        name == "fish"
    }
    fn write_registration(
        &self,
        var: &str,
        _name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let bin = fish_quote(bin);
        let completer = fish_quote_for_eval(completer);

        writeln!(
            buf,
            r#"complete --keep-order --exclusive --command {bin} --arguments "({var}=fish {completer} -- (commandline --current-process --tokenize --cut-at-cursor) (commandline --current-token))""#
        )
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index = args.len() - 1;
        let completions = crate::engine::complete(cmd, args, index, current_dir)?;

        for candidate in completions {
            write!(buf, "{}", candidate.get_value().to_string_lossy())?;
            if let Some(help) = candidate.get_help() {
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

/// Quote `s` for embedding inside fish's `--arguments` value.
///
/// Fish parses `--arguments` twice: once when sourcing the registration (the
/// outer double-quoted string) and again when evaluating the embedded command
/// substitution at completion time. Each special character must therefore
/// survive two rounds of unescaping, which is what this helper does that
/// [`fish_quote`] does not.
fn fish_quote_for_eval(s: &str) -> std::borrow::Cow<'_, str> {
    if !fish_needs_quoting(s) {
        return std::borrow::Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for c in s.chars() {
        match c {
            '\\' => out.push_str(r"\\\\"),
            '\'' => out.push_str(r"\\'"),
            '"' => out.push_str(r#"\""#),
            '$' => out.push_str(r"\$"),
            _ => out.push(c),
        }
    }
    out.push('\'');
    std::borrow::Cow::Owned(out)
}

/// Quote `s` for fish's first-pass parser.
///
/// Used for the `--command` value, where fish reads the token once when
/// sourcing the registration and never re-evaluates it. Single-quoting plus
/// escaping `\` and `'` is sufficient.
fn fish_quote(s: &str) -> std::borrow::Cow<'_, str> {
    if !fish_needs_quoting(s) {
        return std::borrow::Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for c in s.chars() {
        match c {
            '\\' => out.push_str(r"\\"),
            '\'' => out.push_str(r"\'"),
            _ => out.push(c),
        }
    }
    out.push('\'');
    std::borrow::Cow::Owned(out)
}

fn fish_needs_quoting(s: &str) -> bool {
    s.is_empty()
        || s.chars().any(|c| {
            !(c.is_ascii_alphanumeric()
                || matches!(c, '/' | '_' | '-' | '.' | ',' | '+' | '=' | ':' | '@'))
        })
}

/// Powershell completion adapter
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Powershell;

impl EnvCompleter for Powershell {
    fn name(&self) -> &'static str {
        "powershell"
    }
    fn is(&self, name: &str) -> bool {
        name == "powershell" || name == "powershell_ise"
    }
    fn write_registration(
        &self,
        var: &str,
        _name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let bin = shlex::try_quote(bin).unwrap_or(std::borrow::Cow::Borrowed(bin));
        let completer =
            shlex::try_quote(completer).unwrap_or(std::borrow::Cow::Borrowed(completer));

        // `completer` may or may not be surrounded by double quotes, enclosing
        // the expression in a here-string ensures the whole thing is
        // interpreted as the first argument to the call operator
        writeln!(
            buf,
            r#"
Register-ArgumentCompleter -Native -CommandName {bin} -ScriptBlock {{
    param($wordToComplete, $commandAst, $cursorPosition)

    $prev = $env:{var};
    $env:{var} = "powershell";

    $args = $commandAst.Extent.Text
    $args = $args.Substring(0, [math]::Min($cursorPosition, $args.Length));
    if ($wordToComplete -eq "") {{
        $args += " ''";
    }}

    $results = Invoke-Expression @"
& {completer} -- $args
"@;
    if ($null -eq $prev) {{
        Remove-Item Env:\{var};
    }} else {{
        $env:{var} = $prev;
    }}
    $results | ForEach-Object {{
        $split = $_.Split("`t");
        $cmd = $split[0];

        if ($split.Length -eq 2) {{
            $help = $split[1];
        }}
        else {{
            $help = $split[0];
        }}

        [System.Management.Automation.CompletionResult]::new($cmd, $cmd, 'ParameterValue', $help)
    }}
}};
        "#
        )
    }

    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
        current_dir: Option<&std::path::Path>,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let index = args.len() - 1;
        let completions = crate::engine::complete(cmd, args, index, current_dir)?;

        for candidate in completions {
            write!(buf, "{}", candidate.get_value().to_string_lossy())?;
            if let Some(help) = candidate.get_help() {
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

/// Zsh completion adapter
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Zsh;

impl EnvCompleter for Zsh {
    fn name(&self) -> &'static str {
        "zsh"
    }
    fn is(&self, name: &str) -> bool {
        name == "zsh"
    }
    fn write_registration(
        &self,
        var: &str,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let escaped_name = name.replace('-', "_");
        let bin = shlex::try_quote(bin).unwrap_or(std::borrow::Cow::Borrowed(bin));
        let completer =
            shlex::try_quote(completer).unwrap_or(std::borrow::Cow::Borrowed(completer));

        let script = r#"#compdef BIN
function _clap_dynamic_completer_NAME() {
    local _CLAP_COMPLETE_INDEX=$(expr $CURRENT - 1)
    local _CLAP_IFS=$'\n'

    local completions=("${(@f)$( \
        _CLAP_IFS="$_CLAP_IFS" \
        _CLAP_COMPLETE_INDEX="$_CLAP_COMPLETE_INDEX" \
        VAR="zsh" \
        COMPLETER -- "${words[@]}" 2>/dev/null \
    )}")

    if [[ -n $completions ]]; then
        local -a dirs=()
        local -a other=()
        local completion
        for completion in $completions; do
            local value="${completion%%:*}"
            if [[ "$value" == */ ]]; then
                local dir_no_slash="${value%/}"
                if [[ "$completion" == *:* ]]; then
                    local desc="${completion#*:}"
                    dirs+=("$dir_no_slash:$desc")
                else
                    dirs+=("$dir_no_slash")
                fi
            else
                other+=("$completion")
            fi
        done
        [[ -n $dirs ]] && _describe -V 'values' dirs -S '/' -r '/'
        [[ -n $other ]] && _describe -V 'values' other
    fi
}

compdef _clap_dynamic_completer_NAME BIN"#
            .replace("NAME", &escaped_name)
            .replace("COMPLETER", &completer)
            .replace("BIN", &bin)
            .replace("VAR", var);

        writeln!(buf, "{script}")?;
        Ok(())
    }
    fn write_complete(
        &self,
        cmd: &mut clap::Command,
        args: Vec<OsString>,
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
        let completions = crate::engine::complete(cmd, args, index, current_dir)?;

        for (i, candidate) in completions.iter().enumerate() {
            if i != 0 {
                write!(buf, "{}", ifs.as_deref().unwrap_or("\n"))?;
            }
            write!(
                buf,
                "{}",
                Self::escape_value(&candidate.get_value().to_string_lossy())
            )?;
            if let Some(help) = candidate.get_help() {
                write!(
                    buf,
                    ":{}",
                    Self::escape_help(help.to_string().lines().next().unwrap_or_default())
                )?;
            }
        }
        Ok(())
    }
}

impl Zsh {
    /// Escape value string
    fn escape_value(string: &str) -> String {
        string.replace('\\', "\\\\").replace(':', "\\:")
    }

    /// Escape help string
    fn escape_help(string: &str) -> String {
        string.replace('\\', "\\\\")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snapbox::IntoData as _;
    use snapbox::assert_data_eq;

    // This test verifies that fish shell path quoting works with or without spaces in the path.
    #[test]
    #[cfg(all(unix, feature = "unstable-dynamic"))]
    #[cfg(feature = "unstable-shell-tests")]
    fn fish_env_completer_path_quoting_works() {
        // Returns the dynamic registration line for the fish shell, for example:
        // complete --keep-order --exclusive --command my-bin --arguments "(COMPLETE=fish /path/to/my-bin ... )"
        let get_fish_registration = |completer_bin: &str| {
            let mut buf = Vec::new();
            let fish = Fish;
            fish.write_registration(
                "IGNORED_VAR",
                "ignored-name",
                "/ignored/bin",
                completer_bin,
                &mut buf,
            )
            .expect("write_registration failed");
            String::from_utf8(buf).expect("Invalid UTF-8")
        };

        let script = get_fish_registration("completer");
        assert_data_eq!(
            script.trim(),
            snapbox::str![r#"complete [..] "([..] completer [..])""#]
        );

        let script = get_fish_registration("/path/completer");
        assert_data_eq!(
            script.trim(),
            snapbox::str![r#"complete [..] "([..] /path/completer [..])""#]
        );

        let script = get_fish_registration("/path with a space/completer");
        assert_data_eq!(
            script.trim(),
            snapbox::str![r#"complete [..] "([..] '/path with a space/completer' [..])""#]
        );
    }

    #[test]
    #[cfg(all(unix, feature = "unstable-dynamic"))]
    fn fish_env_completer_path_with_backslash() {
        let mut buf = Vec::new();
        Fish.write_registration("V", "n", "/ignored/bin", "/p/dyn\\amic/foo", &mut buf)
            .expect("write_registration failed");
        let script = String::from_utf8(buf).expect("Invalid UTF-8");
        assert_data_eq!(
            script,
            snapbox::str![[r#"
complete --keep-order --exclusive --command /ignored/bin --arguments "(V=fish '/p/dyn\\\\amic/foo' -- (commandline --current-process --tokenize --cut-at-cursor) (commandline --current-token))"

"#]]
            .raw()
        );
    }

    #[test]
    #[cfg(all(unix, feature = "unstable-dynamic"))]
    fn fish_env_command_name_with_backslash() {
        let mut buf = Vec::new();
        Fish.write_registration("V", "n", "dyn\\amic", "/p/completer", &mut buf)
            .expect("write_registration failed");
        let script = String::from_utf8(buf).expect("Invalid UTF-8");
        assert_data_eq!(
            script,
            snapbox::str![[r#"
complete --keep-order --exclusive --command 'dyn\\amic' --arguments "(V=fish /p/completer -- (commandline --current-process --tokenize --cut-at-cursor) (commandline --current-token))"

"#]]
            .raw()
        );
    }

    #[test]
    #[cfg(all(unix, feature = "unstable-dynamic"))]
    fn fish_env_completer_path_with_dollar() {
        let mut buf = Vec::new();
        Fish.write_registration("V", "n", "/ignored/bin", "/p/$var/c", &mut buf)
            .expect("write_registration failed");
        let script = String::from_utf8(buf).expect("Invalid UTF-8");
        assert_data_eq!(
            script,
            snapbox::str![[r#"
complete --keep-order --exclusive --command /ignored/bin --arguments "(V=fish '/p/\$var/c' -- (commandline --current-process --tokenize --cut-at-cursor) (commandline --current-token))"

"#]]
            .raw()
        );
    }
}
