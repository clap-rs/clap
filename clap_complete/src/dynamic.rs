//! Complete commands within shells

/// Complete commands within bash
pub mod bash {
    use std::ffi::OsString;
    use std::io::Write;

    use unicode_xid::UnicodeXID;

    #[derive(clap::Subcommand)]
    #[clap(hide = true)]
    #[allow(missing_docs)]
    #[derive(Clone, Debug)]
    pub enum CompleteCommand {
        /// Register shell completions for this program
        Complete(CompleteArgs),
    }

    #[derive(clap::Args)]
    #[clap(group = clap::ArgGroup::new("complete").multiple(true).conflicts_with("register"))]
    #[allow(missing_docs)]
    #[derive(Clone, Debug)]
    pub struct CompleteArgs {
        /// Path to write completion-registration to
        #[clap(long, required = true, parse(from_os_str))]
        register: Option<std::path::PathBuf>,

        #[clap(
            long,
            required = true,
            value_name = "COMP_CWORD",
            hide_short_help = true,
            group = "complete"
        )]
        index: Option<usize>,

        #[clap(
            long,
            required = true,
            value_name = "COMP_CWORD",
            hide_short_help = true,
            group = "complete"
        )]
        ifs: Option<String>,

        #[clap(
            long = "type",
            required = true,
            arg_enum,
            hide_short_help = true,
            group = "complete"
        )]
        comp_type: Option<CompType>,

        #[clap(long, hide_short_help = true, group = "complete")]
        space: bool,

        #[clap(
            long,
            conflicts_with = "space",
            hide_short_help = true,
            group = "complete"
        )]
        no_space: bool,

        #[clap(raw = true, hide_short_help = true, group = "complete")]
        comp_words: Vec<OsString>,
    }

    impl CompleteCommand {
        /// Process the completion request
        pub fn complete(&self, cmd: &mut clap::Command) -> std::convert::Infallible {
            self.try_complete(cmd).unwrap_or_else(|e| e.exit());
            std::process::exit(0)
        }

        /// Process the completion request
        pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::Result<()> {
            debug!("CompleteCommand::try_complete: {:?}", self);
            let CompleteCommand::Complete(args) = self;
            if let Some(out_path) = args.register.as_deref() {
                let mut buf = Vec::new();
                let name = cmd.get_name();
                let bin = cmd.get_bin_name().unwrap_or(cmd.get_name());
                register(name, [bin], bin, &Behavior::default(), &mut buf)?;
                if out_path == std::path::Path::new("-") {
                    std::io::stdout().write(&buf)?;
                } else {
                    if out_path.is_dir() {
                        let out_path = out_path.join(file_name(name));
                        std::fs::write(out_path, buf)?;
                    } else {
                        std::fs::write(out_path, buf)?;
                    }
                }
            } else {
                let index = args.index.unwrap_or_default();
                let comp_type = args.comp_type.unwrap_or_default();
                let space = match (args.space, args.no_space) {
                    (true, false) => Some(true),
                    (false, true) => Some(false),
                    (true, true) => {
                        unreachable!("`--space` and `--no-space` set, clap should prevent this")
                    }
                    (false, false) => None,
                }
                .unwrap();
                let current_dir = std::env::current_dir().ok();
                let completions = complete(
                    cmd,
                    args.comp_words.clone(),
                    index,
                    comp_type,
                    space,
                    current_dir.as_deref(),
                )?;

                let mut buf = Vec::new();
                for completion in &completions {
                    write!(&mut buf, "{}", completion.to_string_lossy())?;
                    write!(&mut buf, "{}", args.ifs.as_deref().unwrap_or("\n"))?;
                }
                std::io::stdout().write(&buf)?;
            }

            Ok(())
        }
    }

    /// The recommended file name for the registration code
    pub fn file_name(name: &str) -> String {
        format!("{}.bash", name)
    }

    /// Define the completion behavior
    pub enum Behavior {
        /// Bare bones behavior
        Minimal,
        /// Fallback to readline behavior when no matches are generated
        Readline,
        /// Customize bash's completion behavior
        Custom(String),
    }

    impl Default for Behavior {
        fn default() -> Self {
            Self::Readline
        }
    }

    /// Generate code to register the dynamic completion
    pub fn register(
        name: &str,
        executables: impl IntoIterator<Item = impl AsRef<str>>,
        completer: &str,
        behavior: &Behavior,
        buf: &mut dyn Write,
    ) -> Result<(), std::io::Error> {
        let escaped_name = name.replace("-", "_");
        debug_assert!(
            escaped_name.chars().all(|c| c.is_xid_continue()),
            "`name` must be an identifier, got `{}`",
            escaped_name
        );
        let mut upper_name = escaped_name.clone();
        upper_name.make_ascii_uppercase();

        let executables = executables
            .into_iter()
            .map(|s| shlex::quote(s.as_ref()).into_owned())
            .collect::<Vec<_>>()
            .join(" ");

        let options = match behavior {
            Behavior::Minimal => "-o nospace -o bashdefault",
            Behavior::Readline => "-o nospace -o default -o bashdefault",
            Behavior::Custom(c) => c.as_str(),
        };

        let completer = shlex::quote(completer);

        let script = r#"
_clap_complete_NAME() {
    local IFS=$'\013'
    local SUPPRESS_SPACE=0
    if compopt +o nospace 2> /dev/null; then
        SUPPRESS_SPACE=1
    fi
    if [[ ${SUPPRESS_SPACE} == 1 ]]; then
        SPACE_ARG="--no-space"
    else
        SPACE_ARG="--space"
    fi
    COMPREPLY=( $("COMPLETER" complete --index ${COMP_CWORD} --type ${COMP_TYPE} ${SPACE_ARG} --ifs="$IFS" -- "${COMP_WORDS[@]}") )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    elif [[ $SUPPRESS_SPACE == 1 ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
        compopt -o nospace
    fi
}
complete OPTIONS -F _clap_complete_NAME EXECUTABLES
"#
        .replace("NAME", &escaped_name)
        .replace("EXECUTABLES", &executables)
        .replace("OPTIONS", options)
        .replace("COMPLETER", &completer)
        .replace("UPPER", &upper_name);

        writeln!(buf, "{}", script)?;
        Ok(())
    }

    /// Type of completion attempted that caused a completion function to be called
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[non_exhaustive]
    pub enum CompType {
        /// Normal completion
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

    impl clap::ArgEnum for CompType {
        fn value_variants<'a>() -> &'a [Self] {
            &[
                Self::Normal,
                Self::Successive,
                Self::Alternatives,
                Self::Unmodified,
                Self::Menu,
            ]
        }
        fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::PossibleValue<'a>> {
            match self {
                Self::Normal => {
                    let value = "9";
                    debug_assert_eq!(b'\t'.to_string(), value);
                    Some(
                        clap::PossibleValue::new(value)
                            .alias("normal")
                            .help("Normal completion"),
                    )
                }
                Self::Successive => {
                    let value = "63";
                    debug_assert_eq!(b'?'.to_string(), value);
                    Some(
                        clap::PossibleValue::new(value)
                            .alias("successive")
                            .help("List completions after successive tabs"),
                    )
                }
                Self::Alternatives => {
                    let value = "33";
                    debug_assert_eq!(b'!'.to_string(), value);
                    Some(
                        clap::PossibleValue::new(value)
                            .alias("alternatives")
                            .help("List alternatives on partial word completion"),
                    )
                }
                Self::Unmodified => {
                    let value = "64";
                    debug_assert_eq!(b'@'.to_string(), value);
                    Some(
                        clap::PossibleValue::new(value)
                            .alias("unmodified")
                            .help("List completions if the word is not unmodified"),
                    )
                }
                Self::Menu => {
                    let value = "37";
                    debug_assert_eq!(b'%'.to_string(), value);
                    Some(
                        clap::PossibleValue::new(value)
                            .alias("menu")
                            .help("Menu completion"),
                    )
                }
            }
        }
    }

    impl Default for CompType {
        fn default() -> Self {
            Self::Normal
        }
    }

    /// Complete the command specified
    pub fn complete(
        cmd: &mut clap::Command,
        args: Vec<std::ffi::OsString>,
        arg_index: usize,
        _comp_type: CompType,
        _trailing_space: bool,
        current_dir: Option<&std::path::Path>,
    ) -> Result<Vec<std::ffi::OsString>, std::io::Error> {
        cmd.build();

        let raw_args = clap_lex::RawArgs::new(args.into_iter());
        let mut cursor = raw_args.cursor();
        let mut target_cursor = raw_args.cursor();
        raw_args.seek(
            &mut target_cursor,
            clap_lex::SeekFrom::Start(arg_index as u64),
        );
        // As we loop, `cursor` will always be pointing to the next item
        raw_args.next_os(&mut target_cursor);

        // TODO: Multicall support
        if !cmd.is_no_binary_name_set() {
            raw_args.next_os(&mut cursor);
        }

        let mut current_cmd = &*cmd;
        while let Some(arg) = raw_args.next(&mut cursor) {
            if cursor == target_cursor {
                return complete_new(current_cmd, current_dir);
            }
            if let Ok(value) = arg.to_value() {
                if let Some(next_cmd) = current_cmd.find_subcommand(value) {
                    current_cmd = next_cmd;
                }
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No completion generated",
        ))
    }

    fn complete_new(
        cmd: &clap::Command,
        current_dir: Option<&std::path::Path>,
    ) -> Result<Vec<std::ffi::OsString>, std::io::Error> {
        let mut completions = Vec::new();

        completions.extend(
            crate::generator::utils::longs_and_visible_aliases(cmd)
                .into_iter()
                .map(|f| format!("--{}", f).into()),
        );
        completions.extend(
            crate::generator::utils::shorts_and_visible_aliases(cmd)
                .into_iter()
                .map(|f| format!("-{}", f).into()),
        );

        let mut positionals = Vec::new();
        positionals.extend(
            cmd.get_positionals()
                .flat_map(|p| p.get_possible_values())
                .flatten()
                .map(|p| p.get_name().into()),
        );
        let hints = cmd
            .get_positionals()
            .map(|p| p.get_value_hint())
            .map(|h| {
                if h == clap::ValueHint::Unknown {
                    clap::ValueHint::AnyPath
                } else {
                    h
                }
            })
            .collect::<std::collections::HashSet<_>>();
        for hint in hints {
            match hint {
                clap::ValueHint::Unknown => unreachable!("Filtered out"),
                clap::ValueHint::Other => {
                    // Should not complete
                }
                clap::ValueHint::AnyPath => {
                    positionals.extend(complete_path(current_dir, |_| true));
                }
                clap::ValueHint::FilePath => {
                    positionals.extend(complete_path(current_dir, |p| p.is_file()));
                }
                clap::ValueHint::DirPath => {
                    positionals.extend(complete_path(current_dir, |p| p.is_dir()));
                }
                clap::ValueHint::ExecutablePath => {
                    use is_executable::IsExecutable;
                    positionals.extend(complete_path(current_dir, |p| p.is_executable()));
                }
                clap::ValueHint::CommandName
                | clap::ValueHint::CommandString
                | clap::ValueHint::CommandWithArguments
                | clap::ValueHint::Username
                | clap::ValueHint::Hostname
                | clap::ValueHint::Url
                | clap::ValueHint::EmailAddress => {
                    // No completion implementation
                }
                _ => {
                    // Safe-ish fallback
                    positionals.extend(complete_path(current_dir, |_| true));
                }
            }
        }
        positionals.sort();
        positionals.dedup();
        completions.extend(positionals);

        completions.extend(all_subcommands(cmd).into_iter().map(|s| s));

        Ok(completions)
    }

    fn complete_path(
        current_dir: Option<&std::path::Path>,
        is_wanted: impl Fn(&std::path::Path) -> bool,
    ) -> Vec<OsString> {
        let mut completions = Vec::new();

        let current_dir = match current_dir {
            Some(current_dir) => current_dir,
            None => {
                return Vec::new();
            }
        };
        for entry in std::fs::read_dir(current_dir)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
        {
            let mut path = entry.path();
            if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                path.push(""); // Ensure trailing `/`
                let suggestion = path.strip_prefix(current_dir).unwrap();
                completions.push(suggestion.as_os_str().to_owned());
            } else {
                if is_wanted(&path) {
                    let suggestion = path.strip_prefix(current_dir).unwrap();
                    completions.push(suggestion.as_os_str().to_owned());
                }
            }
        }

        completions
    }

    fn all_subcommands(cmd: &clap::Command) -> Vec<OsString> {
        debug!("all_subcommands");

        let mut scs = crate::generator::utils::all_subcommands(cmd)
            .iter()
            .map(|x| OsString::from(&x.0))
            .collect::<Vec<_>>();
        scs.sort();
        scs.dedup();
        scs
    }
}
