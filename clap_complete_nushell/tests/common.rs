#![allow(dead_code)] // shared with other test modules

use clap::{builder::PossibleValue, Arg, ArgAction, Command, ValueHint};
use snapbox::prelude::*;

pub(crate) fn basic_command(name: &'static str) -> Command {
    Command::new(name)
        .arg(
            Arg::new("config")
                .short('c')
                .global(true)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .conflicts_with("config")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("test")
                .about("Subcommand\nwith a second line")
                .arg(Arg::new("debug").short('d').action(ArgAction::Count)),
        )
}

pub(crate) fn feature_sample_command(name: &'static str) -> Command {
    Command::new(name)
        .version("3.0")
        .propagate_version(true)
        .about("Tests completions")
        .arg(
            Arg::new("file")
                .value_hint(ValueHint::FilePath)
                .help("some input file"),
        )
        .arg(
            Arg::new("config")
                .action(ArgAction::Count)
                .help("some config file\nwith another line")
                .short('c')
                .visible_short_alias('C')
                .long("config")
                .visible_alias("conf"),
        )
        .arg(Arg::new("choice").value_parser(["first", "second"]))
        .subcommand(
            Command::new("test").about("tests things").arg(
                Arg::new("case")
                    .long("case")
                    .action(ArgAction::Set)
                    .help("the case to test"),
            ),
        )
}

pub(crate) fn special_commands_command(name: &'static str) -> Command {
    feature_sample_command(name)
        .subcommand(
            Command::new("some_cmd")
                .about("tests other things")
                .arg(
                    Arg::new("config")
                        .long("config")
                        .hide(true)
                        .action(ArgAction::Set)
                        .require_equals(true)
                        .help("the other case to test"),
                )
                .arg(Arg::new("path").num_args(1..)),
        )
        .subcommand(Command::new("some-cmd-with-hyphens").alias("hyphen"))
        .subcommand(Command::new("some-hidden-cmd").hide(true))
}

pub(crate) fn quoting_command(name: &'static str) -> Command {
    Command::new(name)
        .version("3.0")
        .arg(
            Arg::new("single-quotes")
                .long("single-quotes")
                .action(ArgAction::SetTrue)
                .help("Can be 'always', 'auto', or 'never'"),
        )
        .arg(
            Arg::new("double-quotes")
                .long("double-quotes")
                .action(ArgAction::SetTrue)
                .help("Can be \"always\", \"auto\", or \"never\""),
        )
        .arg(
            Arg::new("backticks")
                .long("backticks")
                .action(ArgAction::SetTrue)
                .help("For more information see `echo test`"),
        )
        .arg(
            Arg::new("backslash")
                .long("backslash")
                .action(ArgAction::SetTrue)
                .help("Avoid '\\n'"),
        )
        .arg(
            Arg::new("brackets")
                .long("brackets")
                .action(ArgAction::SetTrue)
                .help("List packages [filter]"),
        )
        .arg(
            Arg::new("expansions")
                .long("expansions")
                .action(ArgAction::SetTrue)
                .help("Execute the shell command with $SHELL"),
        )
        .subcommands([
            Command::new("cmd-single-quotes").about("Can be 'always', 'auto', or 'never'"),
            Command::new("cmd-double-quotes").about("Can be \"always\", \"auto\", or \"never\""),
            Command::new("cmd-backticks").about("For more information see `echo test`"),
            Command::new("cmd-backslash").about("Avoid '\\n'"),
            Command::new("cmd-brackets").about("List packages [filter]"),
            Command::new("cmd-expansions").about("Execute the shell command with $SHELL"),
        ])
}

pub(crate) fn aliases_command(name: &'static str) -> Command {
    Command::new(name)
        .version("3.0")
        .about("testing nushell completions")
        .arg(
            Arg::new("flag")
                .short('f')
                .visible_short_alias('F')
                .long("flag")
                .action(ArgAction::SetTrue)
                .visible_alias("flg")
                .help("cmd flag"),
        )
        .arg(
            Arg::new("option")
                .short('o')
                .visible_short_alias('O')
                .long("option")
                .visible_alias("opt")
                .help("cmd option")
                .action(ArgAction::Set),
        )
        .arg(Arg::new("positional"))
}

pub(crate) fn sub_subcommands_command(name: &'static str) -> Command {
    feature_sample_command(name).subcommand(
        Command::new("some_cmd")
            .about("top level subcommand")
            .visible_alias("some_cmd_alias")
            .subcommand(
                Command::new("sub_cmd").about("sub-subcommand").arg(
                    Arg::new("config")
                        .long("config")
                        .action(ArgAction::Set)
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

pub(crate) fn value_hint_command(name: &'static str) -> Command {
    Command::new(name)
        .arg(
            Arg::new("choice")
                .long("choice")
                .action(ArgAction::Set)
                .value_parser(["bash", "fish", "zsh"]),
        )
        .arg(
            Arg::new("unknown")
                .long("unknown")
                .value_hint(ValueHint::Unknown),
        )
        .arg(Arg::new("other").long("other").value_hint(ValueHint::Other))
        .arg(
            Arg::new("path")
                .long("path")
                .short('p')
                .value_hint(ValueHint::AnyPath),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .value_hint(ValueHint::DirPath),
        )
        .arg(
            Arg::new("exe")
                .long("exe")
                .short('e')
                .value_hint(ValueHint::ExecutablePath),
        )
        .arg(
            Arg::new("cmd_name")
                .long("cmd-name")
                .value_hint(ValueHint::CommandName),
        )
        .arg(
            Arg::new("cmd")
                .long("cmd")
                .short('c')
                .value_hint(ValueHint::CommandString),
        )
        .arg(
            Arg::new("command_with_args")
                .action(ArgAction::Set)
                .num_args(1..)
                .trailing_var_arg(true)
                .value_hint(ValueHint::CommandWithArguments),
        )
        .arg(
            Arg::new("user")
                .short('u')
                .long("user")
                .value_hint(ValueHint::Username),
        )
        .arg(
            Arg::new("host")
                .short('H')
                .long("host")
                .value_hint(ValueHint::Hostname),
        )
        .arg(Arg::new("url").long("url").value_hint(ValueHint::Url))
        .arg(
            Arg::new("email")
                .long("email")
                .value_hint(ValueHint::EmailAddress),
        )
}

pub(crate) fn assert_matches(
    expected: impl IntoData,
    gen: impl clap_complete::Generator,
    mut cmd: Command,
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
    let bin_path =
        snapbox::cmd::compile_example(name, ["--manifest-path", manifest_path.to_str().unwrap()])
            .unwrap();
    println!("Compiled");
    let bin_root = bin_path.parent().unwrap().to_owned();

    let mut registration = std::process::Command::new(&bin_path);
    match context {
        "static" => registration.args([format!("--generate={shell_name}")]),
        "dynamic" => registration.args([
            "complete".to_owned(),
            "--register=-".to_owned(),
            format!("--shell={shell_name}"),
        ]),
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
    let bin_path =
        snapbox::cmd::compile_example(name, ["--manifest-path", manifest_path.to_str().unwrap()])
            .unwrap();
    println!("Compiled");
    let bin_root = bin_path.parent().unwrap().to_owned();

    let runtime = R::with_home(bin_root, home).unwrap();

    Box::new(ScratchRuntime {
        _scratch: scratch,
        runtime: Box::new(runtime),
    })
}

#[derive(Debug)]
#[cfg(feature = "unstable-shell-tests")]
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
