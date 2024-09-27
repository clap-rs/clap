use clap::builder::PossibleValue;
use snapbox::prelude::*;

pub(crate) fn basic_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .arg(
            clap::Arg::new("config")
                .short('c')
                .global(true)
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("v")
                .short('v')
                .conflicts_with("config")
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand(
            clap::Command::new("test")
                .about("Subcommand\nwith a second line")
                .arg(
                    clap::Arg::new("debug")
                        .short('d')
                        .action(clap::ArgAction::Count),
                ),
        )
}

pub(crate) fn feature_sample_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .version("3.0")
        .propagate_version(true)
        .about("Tests completions")
        .arg(
            clap::Arg::new("file")
                .value_hint(clap::ValueHint::FilePath)
                .help("some input file"),
        )
        .arg(
            clap::Arg::new("config")
                .action(clap::ArgAction::Count)
                .help("some config file")
                .short('c')
                .visible_short_alias('C')
                .long("config")
                .visible_alias("conf"),
        )
        .arg(clap::Arg::new("choice").value_parser(["first", "second"]))
        .subcommand(
            clap::Command::new("test").about("tests things").arg(
                clap::Arg::new("case")
                    .long("case")
                    .action(clap::ArgAction::Set)
                    .help("the case to test"),
            ),
        )
}

pub(crate) fn special_commands_command(name: &'static str) -> clap::Command {
    feature_sample_command(name)
        .subcommand(
            clap::Command::new("some_cmd")
                .about("tests other things")
                .arg(
                    clap::Arg::new("config")
                        .long("config")
                        .hide(true)
                        .action(clap::ArgAction::Set)
                        .require_equals(true)
                        .help("the other case to test"),
                )
                .arg(clap::Arg::new("path").num_args(1..)),
        )
        .subcommand(clap::Command::new("some-cmd-with-hyphens").alias("hyphen"))
        .subcommand(clap::Command::new("some-hidden-cmd").hide(true))
}

pub(crate) fn quoting_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .version("3.0")
        .arg(
            clap::Arg::new("single-quotes")
                .long("single-quotes")
                .action(clap::ArgAction::SetTrue)
                .help("Can be 'always', 'auto', or 'never'"),
        )
        .arg(
            clap::Arg::new("double-quotes")
                .long("double-quotes")
                .action(clap::ArgAction::SetTrue)
                .help("Can be \"always\", \"auto\", or \"never\""),
        )
        .arg(
            clap::Arg::new("backticks")
                .long("backticks")
                .action(clap::ArgAction::SetTrue)
                .help("For more information see `echo test`"),
        )
        .arg(
            clap::Arg::new("backslash")
                .long("backslash")
                .action(clap::ArgAction::SetTrue)
                .help("Avoid '\\n'"),
        )
        .arg(
            clap::Arg::new("brackets")
                .long("brackets")
                .action(clap::ArgAction::SetTrue)
                .help("List packages [filter]"),
        )
        .arg(
            clap::Arg::new("expansions")
                .long("expansions")
                .action(clap::ArgAction::SetTrue)
                .help("Execute the shell command with $SHELL"),
        )
        .subcommands([
            clap::Command::new("cmd-single-quotes").about("Can be 'always', 'auto', or 'never'"),
            clap::Command::new("cmd-double-quotes")
                .about("Can be \"always\", \"auto\", or \"never\""),
            clap::Command::new("cmd-backticks").about("For more information see `echo test`"),
            clap::Command::new("cmd-backslash").about("Avoid '\\n'"),
            clap::Command::new("cmd-brackets").about("List packages [filter]"),
            clap::Command::new("cmd-expansions").about("Execute the shell command with $SHELL"),
        ])
}

pub(crate) fn aliases_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .version("3.0")
        .about("testing bash completions")
        .arg(
            clap::Arg::new("flag")
                .short('f')
                .visible_short_alias('F')
                .long("flag")
                .action(clap::ArgAction::SetTrue)
                .visible_alias("flg")
                .help("cmd flag"),
        )
        .arg(
            clap::Arg::new("option")
                .short('o')
                .visible_short_alias('O')
                .long("option")
                .visible_alias("opt")
                .help("cmd option")
                .action(clap::ArgAction::Set),
        )
        .arg(clap::Arg::new("positional"))
}

pub(crate) fn sub_subcommands_command(name: &'static str) -> clap::Command {
    feature_sample_command(name).subcommand(
        clap::Command::new("some_cmd")
            .about("top level subcommand")
            .visible_alias("some_cmd_alias")
            .subcommand(
                clap::Command::new("sub_cmd").about("sub-subcommand").arg(
                    clap::Arg::new("config")
                        .long("config")
                        .action(clap::ArgAction::Set)
                        .value_parser([
                            PossibleValue::new("Lest quotes, aren't escaped.")
                                .help("help,with,comma"),
                            PossibleValue::new("Second to trigger display of options"),
                        ])
                        .help("the other case to test"),
                ),
            ),
    )
}

pub(crate) fn value_hint_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .arg(
            clap::Arg::new("choice")
                .long("choice")
                .action(clap::ArgAction::Set)
                .value_parser(["bash", "fish", "zsh"]),
        )
        .arg(
            clap::Arg::new("unknown")
                .long("unknown")
                .value_hint(clap::ValueHint::Unknown),
        )
        .arg(
            clap::Arg::new("other")
                .long("other")
                .value_hint(clap::ValueHint::Other),
        )
        .arg(
            clap::Arg::new("path")
                .long("path")
                .short('p')
                .value_hint(clap::ValueHint::AnyPath),
        )
        .arg(
            clap::Arg::new("file")
                .long("file")
                .short('f')
                .value_hint(clap::ValueHint::FilePath),
        )
        .arg(
            clap::Arg::new("dir")
                .long("dir")
                .short('d')
                .value_hint(clap::ValueHint::DirPath),
        )
        .arg(
            clap::Arg::new("exe")
                .long("exe")
                .short('e')
                .value_hint(clap::ValueHint::ExecutablePath),
        )
        .arg(
            clap::Arg::new("cmd_name")
                .long("cmd-name")
                .value_hint(clap::ValueHint::CommandName),
        )
        .arg(
            clap::Arg::new("cmd")
                .long("cmd")
                .short('c')
                .value_hint(clap::ValueHint::CommandString),
        )
        .arg(
            clap::Arg::new("command_with_args")
                .action(clap::ArgAction::Set)
                .num_args(1..)
                .trailing_var_arg(true)
                .value_hint(clap::ValueHint::CommandWithArguments),
        )
        .arg(
            clap::Arg::new("user")
                .short('u')
                .long("user")
                .value_hint(clap::ValueHint::Username),
        )
        .arg(
            clap::Arg::new("host")
                .short('H')
                .long("host")
                .value_hint(clap::ValueHint::Hostname),
        )
        .arg(
            clap::Arg::new("url")
                .long("url")
                .value_hint(clap::ValueHint::Url),
        )
        .arg(
            clap::Arg::new("email")
                .long("email")
                .value_hint(clap::ValueHint::EmailAddress),
        )
}

pub(crate) fn value_terminator_command(name: &'static str) -> clap::Command {
    clap::Command::new(name).arg(
        clap::Arg::new("arguments")
            .help("multi-valued argument with a value terminator")
            .num_args(1..)
            .value_terminator(";"),
    )
}

pub(crate) fn two_multi_valued_arguments_command(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .arg(
            clap::Arg::new("first")
                .help("first multi-valued argument")
                .num_args(1..),
        )
        .arg(
            clap::Arg::new("second")
                .help("second multi-valued argument")
                .raw(true),
        )
}

pub(crate) fn subcommand_last(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .arg(clap::Arg::new("free").last(true))
        .subcommands([clap::Command::new("foo"), clap::Command::new("bar")])
}

pub(crate) fn assert_matches(
    expected: impl IntoData,
    gen: impl clap_complete::Generator,
    mut cmd: clap::Command,
    name: &'static str,
) {
    let mut buf = vec![];
    clap_complete::generate(gen, &mut cmd, name, &mut buf);

    snapbox::Assert::new()
        .action_env(snapbox::assert::DEFAULT_ACTION_ENV)
        .normalize_paths(false)
        .eq(buf, expected);
}

#[cfg(feature = "unstable-shell-tests")]
pub(crate) fn register_example<R: completest::RuntimeBuilder>(context: &str, name: &str) {
    use completest::Runtime as _;

    let scratch = snapbox::dir::DirRoot::mutable_temp().unwrap();
    let scratch_path = scratch.path().unwrap();

    let shell_name = R::name();
    let home = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/snapshots/home")
        .join(context)
        .join(name)
        .join(shell_name);
    println!("Compiling");
    let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let bin_path = snapbox::cmd::compile_example(
        name,
        [
            "--manifest-path",
            manifest_path.to_str().unwrap(),
            // Unconditionally include to avoid completion file tests failing based on the how
            // `cargo test` is invoked
            "--features=unstable-dynamic",
        ],
    )
    .unwrap();
    println!("Compiled");
    let bin_root = bin_path.parent().unwrap().to_owned();

    let mut registration = std::process::Command::new(&bin_path);
    match context {
        "static" => registration.args([format!("--generate={shell_name}")]),
        "dynamic-command" => registration.args(["complete", shell_name]),
        "dynamic-env" => registration.env("COMPLETE", shell_name),
        _ => unreachable!("unsupported context {}", context),
    };
    let registration = registration.output().unwrap();
    assert!(
        registration.status.success(),
        "{}",
        String::from_utf8_lossy(&registration.stderr)
    );
    let registration = std::str::from_utf8(&registration.stdout).unwrap();
    assert!(!registration.is_empty());

    let mut runtime = R::new(bin_root, scratch_path.to_owned()).unwrap();

    runtime.register(name, registration).unwrap();

    snapbox::assert_subset_eq(home, scratch_path);

    scratch.close().unwrap();
}

#[cfg(feature = "unstable-shell-tests")]
pub(crate) fn load_runtime<R: completest::RuntimeBuilder>(
    context: &str,
    name: &str,
) -> Box<dyn completest::Runtime>
where
    <R as completest::RuntimeBuilder>::Runtime: 'static,
{
    let shell_name = R::name();
    let home = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/snapshots/home")
        .join(context)
        .join(name)
        .join(shell_name);
    let scratch = snapbox::dir::DirRoot::mutable_temp()
        .unwrap()
        .with_template(&home)
        .unwrap();
    let home = scratch.path().unwrap().to_owned();
    println!("Compiling");
    let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let bin_path = snapbox::cmd::compile_example(
        name,
        [
            "--manifest-path",
            manifest_path.to_str().unwrap(),
            // Unconditionally include to avoid completion file tests failing based on the how
            // `cargo test` is invoked
            "--features=unstable-dynamic",
        ],
    )
    .unwrap();
    println!("Compiled");
    let bin_root = bin_path.parent().unwrap().to_owned();

    let runtime = R::with_home(bin_root, home).unwrap();

    Box::new(ScratchRuntime {
        _scratch: scratch,
        runtime: Box::new(runtime),
    })
}

#[cfg(feature = "unstable-shell-tests")]
#[derive(Debug)]
struct ScratchRuntime {
    _scratch: snapbox::dir::DirRoot,
    runtime: Box<dyn completest::Runtime>,
}

#[cfg(feature = "unstable-shell-tests")]
impl completest::Runtime for ScratchRuntime {
    fn home(&self) -> &std::path::Path {
        self.runtime.home()
    }

    fn register(&mut self, name: &str, content: &str) -> std::io::Result<()> {
        self.runtime.register(name, content)
    }

    fn complete(&mut self, input: &str, term: &completest::Term) -> std::io::Result<String> {
        let output = self.runtime.complete(input, term)?;
        // HACK: elvish prints and clears this message when a completer takes too long which is
        // dependent on a lot of factors, making this show up or no sometimes (especially if we
        // aren't clearing the screen properly for fish)
        let output = output.replace("\nCOMPLETING argument\n", "\n");
        Ok(output)
    }
}

#[cfg(feature = "unstable-shell-tests")]
pub(crate) fn has_command(command: &str) -> bool {
    let output = match std::process::Command::new(command)
        .arg("--version")
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            // CI is expected to support all of the commands
            if is_ci() && cfg!(target_os = "linux") {
                panic!("expected command `{command}` to be somewhere in PATH: {e}");
            }
            return false;
        }
    };
    if !output.status.success() {
        panic!(
            "expected command `{}` to be runnable, got error {}:\n\
            stderr:{}\n\
            stdout:{}\n",
            command,
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!(
        "$ {command} --version
{stdout}"
    );
    if cfg!(target_os = "macos") && stdout.starts_with("GNU bash, version 3") {
        return false;
    }
    if cfg!(target_os = "macos") && command == "zsh" {
        // HACK: At least on CI, the prompt override is not working
        return false;
    }

    true
}

/// Whether or not this running in a Continuous Integration environment.
#[cfg(feature = "unstable-shell-tests")]
fn is_ci() -> bool {
    // Consider using `tracked_env` instead of option_env! when it is stabilized.
    // `tracked_env` will handle changes, but not require rebuilding the macro
    // itself like option_env does.
    option_env!("CI").is_some() || option_env!("TF_BUILD").is_some()
}
