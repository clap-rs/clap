use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;

use nu_cli::NuCompleter;
use nu_command::create_default_context;
use nu_parser::parse;
use nu_protocol::{
    engine::{EngineState, Stack, StateWorkingSet},
    Value,
};
use nu_test_support::fs;

use reedline::{Completer, Suggestion};

const SEP: char = std::path::MAIN_SEPARATOR;

// creates a new engine with the current path into the completions fixtures folder
pub fn new_engine() -> (PathBuf, EngineState, Stack) {
    // Target folder inside assets
    let mut dir = fs::root().join("tests");
    dir.push("snapshots");

    let mut dir_str = dir
        .clone()
        .into_os_string()
        .into_string()
        .unwrap_or_default();
    dir_str.push(SEP);

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
    assert!(merge_result.is_ok());

    (dir, engine_state, stack)
}

// match a list of suggestions with the expected values
pub fn match_suggestions(expected: Vec<String>, suggestions: Vec<Suggestion>) {
    let expected_len = expected.len();
    let suggestions_len = suggestions.len();
    if expected_len != suggestions_len {
        panic!(
            "\nexpected {expected_len} suggestions but got {suggestions_len}: \n\
            Suggestions: {suggestions:#?} \n\
            Expected: {expected:#?}\n"
        )
    }
    expected.iter().zip(suggestions).for_each(|it| {
        assert_eq!(it.0, &it.1.value);
    });
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
        let block = parse(&mut working_set, None, &buf, false);
        assert!(working_set.parse_errors.is_empty());

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

#[test]
fn completion_basic() {
    let mut completer = external_completion("basic.nu");

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["-c".into(), "-v".into()];
    match_suggestions(expected, suggestions);

    let input = "my-app test -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["-c".into(), "-d".into()];
    match_suggestions(expected, suggestions);
}

#[test]
fn completion_feature_sample() {
    let mut completer = external_completion("feature_sample.nu");

    let input = "my-app test --";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["--case".into(), "--version".into()];
    match_suggestions(expected, suggestions);

    let input = "my-app choice ";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["first".into(), "second".into()];
    match_suggestions(expected, suggestions);

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "--conf".into(),
        "--config".into(),
        "--version".into(),
        "-C".into(),
        "-V".into(),
        "-c".into(),
    ];
    match_suggestions(expected, suggestions);

    let input = "my-app --";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["--conf".into(), "--config".into(), "--version".into()];
    match_suggestions(expected, suggestions);
}

#[test]
fn completion_special_commands() {
    let mut completer = external_completion("special_commands.nu");

    let input = "my-app some";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "my-app some_cmd".into(),
        "my-app some-hidden-cmd".into(),
        "my-app some-cmd-with-hyphens".into(),
    ];
    match_suggestions(expected, suggestions);

    let input = "my-app choice ";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["first".into(), "second".into()];
    match_suggestions(expected, suggestions);

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "--conf".into(),
        "--config".into(),
        "--version".into(),
        "-C".into(),
        "-V".into(),
        "-c".into(),
    ];
    match_suggestions(expected, suggestions);

    let input = "my-app --";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["--conf".into(), "--config".into(), "--version".into()];
    match_suggestions(expected, suggestions);
}

#[test]
fn completion_quoting() {
    let mut completer = external_completion("quoting.nu");

    let input = "my-app cmd-s";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["my-app cmd-single-quotes".into()];
    match_suggestions(expected, suggestions);

    let input = "my-app --";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "--backslash".into(),
        "--backticks".into(),
        "--brackets".into(),
        "--double-quotes".into(),
        "--expansions".into(),
        "--single-quotes".into(),
        "--version".into(),
    ];
    match_suggestions(expected, suggestions);
}

#[test]
fn completion_aliases() {
    let mut completer = external_completion("aliases.nu");

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "--flag".into(),
        "--flg".into(),
        "--opt".into(),
        "--option".into(),
        "--version".into(),
        "-F".into(),
        "-O".into(),
        "-V".into(),
        "-f".into(),
        "-o".into(),
    ];
    match_suggestions(expected, suggestions);
}

#[test]
fn completion_sub_subcommands() {
    let mut completer = external_completion("sub_subcommands.nu");

    let input = "my-app";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "my-app".into(),
        "my-app test".into(),
        "my-app some_cmd".into(),
        "my-app some_cmd sub_cmd".into(),
    ];
    match_suggestions(expected, suggestions);

    let input = "my-app some_cmd sub_cmd -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["--config".into(), "--version".into(), "-V".into()];
    match_suggestions(expected, suggestions);

    let input = "my-app some_cmd sub_cmd --config ";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "\"Lest quotes, aren't escaped.\"".into(),
        "\"Second to trigger display of options\"".into(),
    ];
    match_suggestions(expected, suggestions);
}

#[test]
fn completion_value_hint() {
    let mut completer = external_completion("value_hint.nu");

    let input = "my-app -";
    let suggestions = completer.complete(input, input.len());
    let expected = vec![
        "--choice".into(),
        "--cmd".into(),
        "--cmd-name".into(),
        "--dir".into(),
        "--email".into(),
        "--exe".into(),
        "--file".into(),
        "--host".into(),
        "--other".into(),
        "--path".into(),
        "--unknown".into(),
        "--url".into(),
        "--user".into(),
        "-H".into(),
        "-c".into(),
        "-d".into(),
        "-e".into(),
        "-f".into(),
        "-p".into(),
        "-u".into(),
    ];
    match_suggestions(expected, suggestions);

    let input = "my-app --choice ";
    let suggestions = completer.complete(input, input.len());
    let expected = vec!["bash".into(), "fish".into(), "zsh".into()];
    match_suggestions(expected, suggestions);
}
