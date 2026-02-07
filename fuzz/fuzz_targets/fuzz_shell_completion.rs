#![no_main]

use libfuzzer_sys::fuzz_target;
use clap::{Command, Arg, ArgAction};
use clap_complete::{generate, shells};
use std::io;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let cmd = Command::new("test")
            .arg(Arg::new("option")
                .long(input)
                .action(ArgAction::Set));

        // Try to generate completions for various shells
        let mut buf = Vec::new();
        let _ = generate(shells::Bash, &mut cmd.clone(), "test", &mut buf);
        let _ = generate(shells::Zsh, &mut cmd.clone(), "test", &mut buf);
        let _ = generate(shells::Fish, &mut cmd.clone(), "test", &mut buf);
    }
});
