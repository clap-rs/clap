#![cfg(not(tarpaulin))]

use std::ffi::OsStr;
use std::fs;
use std::process::{Command, Output};

fn run_example<S: AsRef<str>>(name: S, args: &[&str]) -> Output {
    let mut all_args = vec![
        "run",
        "--example",
        name.as_ref(),
        "--features",
        "yaml",
        "--",
    ];
    all_args.extend_from_slice(args);

    Command::new(env!("CARGO"))
        .args(all_args)
        .output()
        .expect("failed to run example")
}

#[test]
fn examples_are_functional() {
    let example_paths = fs::read_dir("examples")
        .expect("couldn't read examples directory")
        .map(|result| result.expect("couldn't get directory entry").path())
        .filter(|path| path.is_file() && path.extension().and_then(OsStr::to_str) == Some("rs"));

    let mut example_count = 0;
    for path in example_paths {
        example_count += 1;

        let example_name = path
            .file_stem()
            .and_then(OsStr::to_str)
            .expect("unable to determine example name");

        let help_output = run_example(example_name, &["--help"]);
        assert!(
            help_output.status.success(),
            "{} --help exited with nonzero",
            example_name,
        );
        assert!(
            !help_output.stdout.is_empty(),
            "{} --help had no output",
            example_name,
        );
    }
    assert!(example_count > 0);
}
