use std::sync::atomic::{AtomicUsize, Ordering};

use clap::{Args, CommandFactory, Parser, Subcommand};

#[test]
fn named_fields() {
    #[derive(Parser)]
    #[command(defer = true)]
    enum Cli {
        /// Add a file
        #[command(visible_alias = "a", version = "1.0")]
        Add {
            #[arg(long)]
            file: String,
        },
    }

    for (mut cmd, required) in [(Cli::command(), true), (Cli::command_for_update(), false)] {
        let add = cmd.find_subcommand("add").unwrap();
        assert_eq!(add.get_about().unwrap().to_string(), "Add a file");
        assert_eq!(add.get_version(), Some("1.0"));
        assert_eq!(add.get_visible_aliases().collect::<Vec<_>>(), ["a"]);
        assert_eq!(add.get_arguments().count(), 0);
        cmd.build();
        let file = cmd
            .find_subcommand("add")
            .unwrap()
            .get_arguments()
            .find(|arg| arg.get_id() == "file")
            .unwrap();
        assert_eq!(file.is_required_set(), required);
    }
}

#[test]
fn newtype_with_flattened_args() {
    #[derive(Parser)]
    #[command(defer = true)]
    enum Cli {
        Account(Account),
    }

    #[derive(Args)]
    struct Account {
        account_id: String,
        #[command(flatten)]
        common: Common,
        #[command(subcommand)]
        action: Action,
    }

    #[derive(Args)]
    struct Common {
        #[arg(long)]
        verbose: bool,
    }

    #[derive(Subcommand)]
    enum Action {
        View,
    }

    for (mut cmd, required) in [(Cli::command(), true), (Cli::command_for_update(), false)] {
        let account = cmd.find_subcommand("account").unwrap();
        assert_eq!(account.get_arguments().count(), 0);
        assert_eq!(account.get_subcommands().count(), 0);
        cmd.build();
        let account = cmd.find_subcommand("account").unwrap();
        let account_id = account
            .get_arguments()
            .find(|arg| arg.get_id() == "account_id")
            .unwrap();
        assert_eq!(account_id.is_required_set(), required);
        assert!(account.get_arguments().any(|arg| arg.get_id() == "verbose"));
        assert!(account.find_subcommand("view").is_some());
    }
}

#[test]
fn initialization_runs_once() {
    static ARGS: AtomicUsize = AtomicUsize::new(0);
    static METADATA: AtomicUsize = AtomicUsize::new(0);

    fn value() -> &'static str {
        ARGS.fetch_add(1, Ordering::SeqCst);
        "value"
    }

    fn about() -> &'static str {
        METADATA.fetch_add(1, Ordering::SeqCst);
        "Selected command"
    }

    #[derive(Parser)]
    #[command(defer = true)]
    enum Cli {
        #[command(about = about())]
        Selected {
            #[arg(long, default_value = value())]
            value: String,
        },
        Unselected {
            #[arg(long, default_value = value())]
            value: String,
        },
    }

    let cmd = Cli::command();
    assert_eq!(ARGS.load(Ordering::SeqCst), 0);
    assert_eq!(METADATA.load(Ordering::SeqCst), 1);
    cmd.try_get_matches_from(["test", "selected"]).unwrap();
    assert_eq!(ARGS.load(Ordering::SeqCst), 1);
    assert_eq!(METADATA.load(Ordering::SeqCst), 1);
}

#[test]
fn flattened_and_nested_subcommands() {
    #[derive(Parser)]
    #[command(defer = true)]
    enum Cli {
        #[command(flatten)]
        Flat(Commands),
        #[command(subcommand)]
        Nested(Commands),
    }

    #[derive(Subcommand)]
    #[command(defer = true)]
    enum Commands {
        Run {
            #[arg(long)]
            flag: bool,
        },
    }

    let cmd = Cli::command();
    let flat = cmd.find_subcommand("run").unwrap();
    let nested = cmd
        .find_subcommand("nested")
        .unwrap()
        .find_subcommand("run")
        .unwrap();
    assert_eq!(flat.get_arguments().count(), 0);
    assert_eq!(nested.get_arguments().count(), 0);
}

#[test]
fn args_metadata() {
    #[derive(Parser)]
    #[command(defer = true)]
    enum Cli {
        Run(Options),
    }

    #[derive(Args)]
    #[command(about = "From Args")]
    struct Options {
        #[arg(long)]
        flag: bool,
    }

    let mut cmd = Cli::command();
    assert_eq!(
        cmd.find_subcommand("run")
            .unwrap()
            .get_about()
            .map(ToString::to_string),
        None
    );
    cmd.build();
    assert_eq!(
        cmd.find_subcommand("run")
            .unwrap()
            .get_about()
            .map(ToString::to_string),
        Some("From Args".into())
    );
}

#[test]
fn eager_subcommands() {
    #[derive(Parser)]
    #[command(defer = false)]
    enum Cli {
        Run {
            #[arg(long)]
            flag: bool,
        },
    }

    let cmd = Cli::command();
    assert_eq!(
        cmd.find_subcommand("run").unwrap().get_arguments().count(),
        1
    );
}

#[test]
fn default_initialization() {
    #[derive(Parser)]
    enum Cli {
        Run {
            #[arg(long)]
            flag: bool,
        },
    }

    let cmd = Cli::command();
    assert_eq!(
        cmd.find_subcommand("run").unwrap().get_arguments().count(),
        usize::from(!cfg!(feature = "unstable-v5"))
    );
}

#[test]
fn raw_defer() {
    #[derive(Parser)]
    #[command(defer(|cmd| cmd.about("From callback")))]
    struct Cli {}

    let mut cmd = Cli::command();
    assert_eq!(cmd.get_about(), None);
    cmd.build();
    assert_eq!(cmd.get_about().unwrap().to_string(), "From callback");
}
