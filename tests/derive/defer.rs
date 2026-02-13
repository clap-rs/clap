// Tests for subcommand initialization behavior.
//
// These tests establish the current (eager) behavior as a baseline.
// A subsequent commit will add `#[command(defer = true)]` to enable lazy
// initialization, and the test diffs will show how behavior changes.
//
// See: https://github.com/clap-rs/clap/issues/4959

use clap::{Args, CommandFactory, Parser, Subcommand};

#[test]
fn subcommand_with_named_fields() {
    #[derive(Parser, Debug)]
    struct Cli {
        #[command(subcommand)]
        cmd: Commands,
    }

    #[derive(Subcommand, Debug)]
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

    let cmd = Cli::command();
    let add_cmd = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "add")
        .expect("add subcommand should exist");
    assert_eq!(
        add_cmd.get_about().map(|s| s.to_string()),
        Some("Add a file".to_string()),
    );

    let remove_cmd = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "remove")
        .expect("remove should exist");
    assert_eq!(
        remove_cmd.get_about().map(|s| s.to_string()),
        Some("Remove a file".to_string()),
    );

    let user_args: Vec<_> = add_cmd
        .get_arguments()
        .filter(|a| a.get_id() != "help" && a.get_id() != "version")
        .map(|a| a.get_id().as_str())
        .collect();
    assert!(
        user_args.contains(&"file"),
        "args should be immediately visible, got: {user_args:?}"
    );
}

#[test]
fn nested_subcommands() {
    #[derive(Parser, Debug, PartialEq)]
    struct Cli {
        #[command(subcommand)]
        cmd: Option<TopLevel>,
    }

    #[derive(Subcommand, Debug, PartialEq)]
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
    enum AccountAction {
        /// View account
        View {
            #[arg(long)]
            verbose: bool,
        },
    }

    let cmd = Cli::command();

    let account = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "account")
        .expect("account should exist");
    let args: Vec<_> = account
        .get_arguments()
        .filter(|a| a.get_id() != "help" && a.get_id() != "version")
        .map(|a| a.get_id().as_str())
        .collect();
    assert!(
        args.contains(&"account_id"),
        "account_id should be visible, got: {args:?}"
    );

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

#[test]
fn flatten_args_in_subcommand_are_eager() {
    #[derive(Parser, Debug, PartialEq)]
    struct Cli {
        #[command(subcommand)]
        cmd: Commands,
    }

    #[derive(Subcommand, Debug, PartialEq)]
    enum Commands {
        Add(FlattenedArgs),
    }

    #[derive(Args, Debug, PartialEq)]
    struct FlattenedArgs {
        #[arg(long)]
        file: String,
        #[command(flatten)]
        common: CommonArgs,
    }

    #[derive(Args, Debug, PartialEq)]
    struct CommonArgs {
        #[arg(long)]
        verbose: bool,
    }

    let cmd = Cli::command();
    let add = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "add")
        .expect("add should exist");

    let user_args: Vec<_> = add
        .get_arguments()
        .filter(|a| a.get_id() != "help" && a.get_id() != "version")
        .map(|a| a.get_id().as_str())
        .collect();
    assert!(
        user_args.contains(&"file"),
        "file should be visible, got: {user_args:?}"
    );
    assert!(
        user_args.contains(&"verbose"),
        "flattened args should be visible, got: {user_args:?}"
    );

    let parsed =
        Cli::try_parse_from(["test", "add", "--file", "config.toml", "--verbose"]).unwrap();
    assert_eq!(
        parsed,
        Cli {
            cmd: Commands::Add(FlattenedArgs {
                file: "config.toml".to_string(),
                common: CommonArgs { verbose: true },
            }),
        }
    );
}

#[test]
fn enum_variant_with_inline_args() {
    #[derive(Parser, Debug, PartialEq)]
    struct Cli {
        #[command(subcommand)]
        cmd: Cmd,
    }

    #[derive(Subcommand, Debug, PartialEq)]
    enum Cmd {
        Sub {
            #[arg(long)]
            flag: bool,
        },
    }

    let parsed = Cli::try_parse_from(["test", "sub", "--flag"]).unwrap();
    assert_eq!(
        parsed,
        Cli {
            cmd: Cmd::Sub { flag: true }
        }
    );

    let parsed = Cli::try_parse_from(["test", "sub"]).unwrap();
    assert_eq!(
        parsed,
        Cli {
            cmd: Cmd::Sub { flag: false }
        }
    );
}

#[test]
fn default_behavior_is_eager() {
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
    assert!(
        user_args.contains(&"file"),
        "default: args should be immediately visible"
    );
}
