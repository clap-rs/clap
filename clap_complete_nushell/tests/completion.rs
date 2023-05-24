use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use nu_cli::NuCompleter;
use nu_command::create_default_context;
use nu_parser::parse;
use nu_protocol::{
    engine::{EngineState, Stack, StateWorkingSet},
    Value,
};

use reedline::{Completer as _, Suggestion};

// creates a new engine with the current path into the completions fixtures folder
fn new_engine() -> (PathBuf, EngineState, Stack) {
    // Target folder inside assets
    let mut dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    dir.push("snapshots");

    let mut dir_str = dir
        .clone()
        .into_os_string()
        .into_string()
        .unwrap_or_default();
    dir_str.push(std::path::MAIN_SEPARATOR);

    // Create a new engine with default context
    let mut engine_state = create_default_context();

    // New stack
    let mut stack = Stack::new();

    // Add pwd as env var
    stack.add_env_var(
        "PWD".to_string(),
        Value::String {
            val: dir_str.clone(),
            span: nu_protocol::Span::new(0, dir_str.len()),
        },
    );

    #[cfg(windows)]
    stack.add_env_var(
        "Path".to_string(),
        Value::String {
            val: "c:\\some\\path;c:\\some\\other\\path".to_string(),
            span: nu_protocol::Span::new(0, dir_str.len()),
        },
    );

    #[cfg(not(windows))]
    stack.add_env_var(
        "PATH".to_string(),
        Value::String {
            val: "/some/path:/some/other/path".to_string(),
            span: nu_protocol::Span::new(0, dir_str.len()),
        },
    );

    // Merge environment into the permanent state
    let merge_result = engine_state.merge_env(&mut stack, &dir);
    assert!(merge_result.is_ok(), "{}", merge_result.unwrap_err());

    (dir, engine_state, stack)
}

fn external_completion(file_name: &str) -> NuCompleter {
    // Create a new engine
    let (dir, mut engine_state, mut stack) = new_engine();

    let path = dir.join(file_name);
    let mut buf = Vec::new();
    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Failed to open {}", path.display()));
    file.read_to_end(&mut buf)
        .unwrap_or_else(|_| panic!("Failed to open {}", path.display()));

    let (_, delta) = {
        let mut working_set = StateWorkingSet::new(&engine_state);
        let (block, err) = parse(&mut working_set, None, &buf, false, &[]);
        assert!(err.is_none(), "{:?}", err.unwrap());

        (block, working_set.render())
    };

    assert!(engine_state.merge_delta(delta).is_ok());

    // Merge environment into the permanent state
    assert!(engine_state.merge_env(&mut stack, &dir).is_ok());

    let latest_block_id = engine_state.num_blocks() - 1;

    // Change config adding the external completer
    let mut config = engine_state.get_config().clone();
    config.external_completer = Some(latest_block_id);
    engine_state.set_config(&config);

    // Instantiate a new completer
    NuCompleter::new(Arc::new(engine_state), stack)
}

// match a list of suggestions with the expected values
#[track_caller]
fn assert_suggestions(expected: &[&str], suggestions: Vec<Suggestion>) {
    let expected = expected
        .iter()
        .map(|s| (*s).to_owned())
        .collect::<Vec<String>>();
    let suggestions = suggestions
        .into_iter()
        .map(|s| s.value)
        .collect::<Vec<String>>();
    assert_eq!(expected, suggestions);
}

#[test]
fn completion_basic() {
    let mut completer = external_completion("basic.nu");

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = &["--help", "-c", "-h", "-v"];
    assert_suggestions(expected, suggestions);

    let input = "my-app test -";
    let suggestions = completer.complete(input, input.len());
    let expected = &["--help", "-c", "-d", "-h"];
    assert_suggestions(expected, suggestions);
}

#[test]
fn completion_feature_sample() {
    let mut completer = external_completion("feature_sample.nu");

    let input = "my-app test --";
    let suggestions = completer.complete(input, input.len());
    let expected = &["--case", "--help", "--version"];
    assert_suggestions(expected, suggestions);

    let input = "my-app choice ";
    let suggestions = completer.complete(input, input.len());
    let expected = &["first", "second"];
    assert_suggestions(expected, suggestions);

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = &[
        "--conf",
        "--config",
        "--help",
        "--version",
        "-C",
        "-V",
        "-c",
        "-h",
    ];
    assert_suggestions(expected, suggestions);

    let input = "my-app --";
    let suggestions = completer.complete(input, input.len());
    let expected = &["--conf", "--config", "--help", "--version"];
    assert_suggestions(expected, suggestions);
}

#[test]
fn completion_special_commands() {
    let mut completer = external_completion("special_commands.nu");

    let input = "my-app some";
    let suggestions = completer.complete(input, input.len());
    let expected = &[
        "my-app some_cmd",
        "my-app some-hidden-cmd",
        "my-app some-cmd-with-hyphens",
    ];
    assert_suggestions(expected, suggestions);

    let input = "my-app choice ";
    let suggestions = completer.complete(input, input.len());
    let expected = &["first", "second"];
    assert_suggestions(expected, suggestions);

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = &[
        "--conf",
        "--config",
        "--help",
        "--version",
        "-C",
        "-V",
        "-c",
        "-h",
    ];
    assert_suggestions(expected, suggestions);

    let input = "my-app --";
    let suggestions = completer.complete(input, input.len());
    let expected = &["--conf", "--config", "--help", "--version"];
    assert_suggestions(expected, suggestions);
}

#[test]
fn completion_quoting() {
    let mut completer = external_completion("quoting.nu");

    let input = "my-app cmd-s";
    let suggestions = completer.complete(input, input.len());
    let expected = &["my-app cmd-single-quotes"];
    assert_suggestions(expected, suggestions);

    let input = "my-app --";
    let suggestions = completer.complete(input, input.len());
    let expected = &[
        "--backslash",
        "--backticks",
        "--brackets",
        "--double-quotes",
        "--expansions",
        "--help",
        "--single-quotes",
        "--version",
    ];
    assert_suggestions(expected, suggestions);
}

#[test]
fn completion_aliases() {
    let mut completer = external_completion("aliases.nu");

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = &[
        "--flag",
        "--flg",
        "--help",
        "--opt",
        "--option",
        "--version",
        "-F",
        "-O",
        "-V",
        "-f",
        "-h",
        "-o",
    ];
    assert_suggestions(expected, suggestions);
}

#[test]
fn completion_sub_subcommands() {
    let mut completer = external_completion("sub_subcommands.nu");

    let input = "my-app";
    let mut suggestions = completer.complete(input, input.len());
    suggestions.sort_by_key(|s| s.value.clone());
    let expected = &[
        "my-app",
        "my-app help",
        "my-app help help",
        "my-app help some_cmd",
        "my-app help some_cmd sub_cmd",
        "my-app help test",
        "my-app some_cmd",
        "my-app some_cmd help",
        "my-app some_cmd help help",
        "my-app some_cmd help sub_cmd",
        "my-app some_cmd sub_cmd",
        "my-app test",
    ];
    assert_suggestions(expected, suggestions);

    let input = "my-app some_cmd sub_cmd -";
    let suggestions = completer.complete(input, input.len());
    let expected = &["--config", "--help", "--version", "-V", "-h"];
    assert_suggestions(expected, suggestions);

    let input = "my-app some_cmd sub_cmd --config ";
    let suggestions = completer.complete(input, input.len());
    let expected = &[
        "\"Lest quotes, aren't escaped.\"",
        "\"Second to trigger display of options\"",
    ];
    assert_suggestions(expected, suggestions);
}

#[test]
fn completion_value_hint() {
    let mut completer = external_completion("value_hint.nu");

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = &[
        "--choice",
        "--cmd",
        "--cmd-name",
        "--dir",
        "--email",
        "--exe",
        "--file",
        "--help",
        "--host",
        "--other",
        "--path",
        "--unknown",
        "--url",
        "--user",
        "-H",
        "-c",
        "-d",
        "-e",
        "-f",
        "-h",
        "-p",
        "-u",
    ];
    assert_suggestions(expected, suggestions);

    let input = "my-app --choice ";
    let suggestions = completer.complete(input, input.len());
    let expected = &["bash", "fish", "zsh"];
    assert_suggestions(expected, suggestions);
}
