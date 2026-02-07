#![no_main]

use libfuzzer_sys::fuzz_target;
use clap::{Command, Arg, ArgAction};

fuzz_target!(|data: &[u8]| {
    // Convert bytes to strings for argument parsing
    if let Ok(input) = std::str::from_utf8(data) {
        // Split input into "arguments" by whitespace
        let args: Vec<&str> = input.split_whitespace().collect();

        if args.is_empty() {
            return;
        }

        // Create a simple command with various argument types
        let cmd = Command::new("fuzz")
            .arg(Arg::new("input")
                .short('i')
                .long("input")
                .action(ArgAction::Set))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .action(ArgAction::Set))
            .arg(Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("count")
                .short('c')
                .long("count")
                .value_parser(clap::value_parser!(u32)))
            .arg(Arg::new("files")
                .action(ArgAction::Append)
                .num_args(1..));

        // Try to parse the fuzzer-provided arguments
        let _ = cmd.try_get_matches_from(args);
    }
});
