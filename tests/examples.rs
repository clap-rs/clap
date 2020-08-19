use insta;
use std::ffi::OsStr;
use std::fs;
use std::process::{Command, Output};

fn run_example<S: AsRef<str>>(name: S, args: &[&str]) -> Output {
    let mut all_args = vec![
        "run",
        "--example",
        name.as_ref(),
        "--features",
        "yaml unstable",
        "--",
    ];
    all_args.extend_from_slice(args);

    Command::new(env!("CARGO"))
        .args(all_args)
        .output()
        .expect("failed to run example")
}

#[test]
fn examples_match_snapshots() {
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

        let help = {
            let help_output = run_example(example_name, &["--help"]);
            assert!(
                help_output.status.success(),
                "{} example failed",
                example_name
            );
            String::from_utf8(help_output.stdout).expect("help is not valid utf-8")
        };

        insta::assert_snapshot!(format!("{} --help", example_name), help);
    }
    assert!(example_count > 0);
}
