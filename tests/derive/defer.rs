// Tests for #[command(defer = true/false)] attribute
//
// This feature enables lazy initialization of subcommand arguments,
// dramatically improving CLI startup time for applications with many subcommands.
// See: https://github.com/clap-rs/clap/issues/4959

use clap::{Args, CommandFactory, Parser, Subcommand};

/// Test that #[command(defer = true)] on a Subcommand enum defers argument creation.
#[test]
fn defer_on_subcommand_defers_arguments() {
    #[derive(Parser, Debug)]
    struct Cli {
        #[command(subcommand)]
        cmd: Commands,
    }

    #[derive(Subcommand, Debug)]
    #[command(defer = true)]
    enum Commands {
        /// Add a file
        Add {
            #[arg(long)]
            file: String,
        },
        /// Remove a file
        Remove {
            #[arg(long)]
            force: bool,
        },
    }

    // Before build: subcommand names should be visible, but args should be deferred
    let cmd = Cli::command();
    let add_cmd = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "add")
        .expect("add subcommand should exist");
    assert_eq!(
        add_cmd.get_about().map(|s| s.to_string()),
        Some("Add a file".to_string()),
        "add subcommand should have its description"
    );

    let remove_cmd = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "remove")
        .expect("remove should exist");
    assert_eq!(
        remove_cmd.get_about().map(|s| s.to_string()),
        Some("Remove a file".to_string()),
        "remove subcommand should have its description"
    );

    let user_args: Vec<_> = add_cmd
        .get_arguments()
        .filter(|a| a.get_id() != "help" && a.get_id() != "version")
        .collect();
    assert!(
        user_args.is_empty(),
        "args should be deferred before build, got: {:?}",
        user_args.iter().map(|a| a.get_id()).collect::<Vec<_>>()
    );

    // After build: args should be visible
    let mut cmd = cmd;
    cmd.build();
    let add_cmd = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "add")
        .expect("add subcommand should exist");

    let user_args: Vec<_> = add_cmd
        .get_arguments()
        .filter(|a| a.get_id() != "help" && a.get_id() != "version")
        .map(|a| a.get_id().as_str())
        .collect();
    assert!(
        user_args.contains(&"file"),
        "args should be visible after build, got: {user_args:?}"
    );
}

/// This tests the pattern where:
/// - Top-level has subcommands
/// - Subcommand variants contain Args structs
/// - Args structs have nested subcommands
#[test]
fn defer_nested_subcommands() {
    #[derive(Parser, Debug, PartialEq)]
    struct Cli {
        #[command(subcommand)]
        cmd: Option<TopLevel>,
    }

    #[derive(Subcommand, Debug, PartialEq)]
    #[command(defer = true)]
    enum TopLevel {
        /// Account operations
        Account(AccountArgs),
    }

    #[derive(Args, Debug, PartialEq)]
    struct AccountArgs {
        /// Account ID
        account_id: Option<String>,
        #[command(subcommand)]
        action: Option<AccountAction>,
    }

    #[derive(Subcommand, Debug, PartialEq)]
    #[command(defer = true)]
    enum AccountAction {
        /// View account
        View {
            #[arg(long)]
            verbose: bool,
        },
    }

    // Build command and verify structure
    let mut cmd = Cli::command();

    // Account subcommand should exist, but args defered
    let account = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "account")
        .expect("account should exist");
    assert!(
        account.get_arguments().next().is_none(),
        "account arguments should be deferred"
    );

    cmd.build();

    // Account subcommand should exist
    let account = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "account")
        .expect("account should exist");

    // Account should have the positional account_id argument
    let args: Vec<_> = account
        .get_arguments()
        .filter(|a| a.get_id() != "help" && a.get_id() != "version")
        .map(|a| a.get_id().as_str())
        .collect();
    assert!(
        args.contains(&"account_id"),
        "account_id should be visible, got: {args:?}"
    );

    // Account should have view subcommand
    let subcmds: Vec<_> = account
        .get_subcommands()
        .filter(|s| s.get_name() != "help")
        .map(|s| s.get_name())
        .collect();
    assert!(
        subcmds.contains(&"view"),
        "view subcommand should exist, got: {subcmds:?}"
    );

    // Parsing should work
    let parsed = Cli::try_parse_from(["test", "account", "alice", "view"]).unwrap();
    assert_eq!(
        parsed,
        Cli {
            cmd: Some(TopLevel::Account(AccountArgs {
                account_id: Some("alice".to_string()),
                action: Some(AccountAction::View { verbose: false })
            }))
        }
    );

    // Parsing with flags should work
    let parsed = Cli::try_parse_from(["test", "account", "alice", "view", "--verbose"]).unwrap();
    assert_eq!(
        parsed,
        Cli {
            cmd: Some(TopLevel::Account(AccountArgs {
                account_id: Some("alice".to_string()),
                action: Some(AccountAction::View { verbose: true })
            }))
        }
    );
}

/// Test that without defer attribute, arguments are immediately visible on v4 and defered on v5.
#[test]
fn no_explicit_defer_default_is_applied_properly_for_v4_and_v5() {
    #[derive(Parser, Debug)]
    struct Cli {
        #[command(subcommand)]
        cmd: Commands,
    }

    #[derive(Subcommand, Debug)]
    enum Commands {
        Add {
            #[arg(long)]
            file: String,
        },
    }

    let cmd = Cli::command();
    let add_cmd = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "add")
        .expect("add should exist");

    let user_args: Vec<_> = add_cmd
        .get_arguments()
        .filter(|a| a.get_id() != "help" && a.get_id() != "version")
        .map(|a| a.get_id().as_str())
        .collect();

    #[cfg(not(feature = "unstable-v5"))]
    assert!(
        user_args.contains(&"file"),
        "args should be immediately visible without defer = true"
    );

    #[cfg(feature = "unstable-v5")]
    assert!(
        !user_args.contains(&"file"),
        "args should be not immediately visible without defer = false"
    );
}
