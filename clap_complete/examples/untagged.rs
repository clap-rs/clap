fn main() {
    #[cfg(feature = "unstable-dynamic")]
    clap_complete::CompleteEnv::with_factory(cli)
        .completer("untagged")
        .complete();

    let _ = cli().get_matches();
}

fn cli() -> clap::Command {
    clap::Command::new("untagged").subcommand(ext_subcommand())
}

fn ext_subcommand() -> clap::Command {
    let cmd = clap::Command::new("ext").allow_external_subcommands(true);

    #[cfg(feature = "unstable-dynamic")]
    let cmd = {
        use clap_complete::engine::{CompletionCandidate, SubcommandCandidates};
        cmd.add(SubcommandCandidates::new(|| {
            vec![
                CompletionCandidate::new("external-cmd"),
                // Trailing `/` marks this as a directory candidate.
                CompletionCandidate::new("some-dir/"),
            ]
        }))
    };

    cmd
}

#[test]
fn verify_cli() {
    cli().debug_assert();
}
