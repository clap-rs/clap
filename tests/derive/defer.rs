use clap::{Args, CommandFactory, Parser, Subcommand};

#[test]
fn defer_on_args_struct() {
    #[derive(Parser, Debug, PartialEq)]
    #[command(defer = true)]
    struct Opt {
        #[arg(long)]
        flag: bool,
    }

    let opt = Opt::try_parse_from(["test", "--flag"]).unwrap();
    assert!(opt.flag);

    // Verify that the command has no args before building (deferred)
    let cmd = Opt::command();
    assert!(
        cmd.get_arguments().next().is_none(),
        "args should be deferred"
    );
}

#[test]
fn defer_false_on_args_struct() {
    #[derive(Parser, Debug, PartialEq)]
    #[command(defer = false)]
    struct Opt {
        #[arg(long)]
        value: String,
    }

    let opt = Opt::try_parse_from(["test", "--value", "hello"]).unwrap();
    assert_eq!(opt.value, "hello");
}

#[test]
fn defer_on_subcommand_enum() {
    #[derive(Parser, Debug, PartialEq)]
    struct Cli {
        #[command(subcommand)]
        cmd: Cmd,
    }

    #[derive(Subcommand, Debug, PartialEq)]
    #[command(defer = true)]
    enum Cmd {
        Add { name: String },
        Remove { name: String },
    }

    let cli = Cli::try_parse_from(["test", "add", "foo"]).unwrap();
    assert_eq!(
        cli.cmd,
        Cmd::Add {
            name: "foo".to_string()
        }
    );

    // Verify that subcommand args are deferred
    let cmd = Cli::command();
    let add_subcmd = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "add")
        .unwrap();
    assert!(
        add_subcmd.get_arguments().next().is_none(),
        "subcommand args should be deferred"
    );
}

#[test]
fn defer_with_nested_subcommands() {
    // Note: Only the inner Cmd enum has defer = true, not the outer Cli.
    // This tests that subcommand variant args are deferred.
    #[derive(Parser, Debug, PartialEq)]
    struct Cli {
        #[command(subcommand)]
        cmd: Cmd,
    }

    #[derive(Subcommand, Debug, PartialEq)]
    #[command(defer = true)]
    enum Cmd {
        Sub {
            #[arg(long)]
            flag: bool,
        },
    }

    let cli = Cli::try_parse_from(["test", "sub", "--flag"]).unwrap();
    assert_eq!(cli.cmd, Cmd::Sub { flag: true });

    // Verify that args in the nested subcommand are deferred
    let cmd = Cli::command();
    let sub_subcmd = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "sub")
        .unwrap();
    assert!(
        sub_subcmd.get_arguments().next().is_none(),
        "nested subcommand args should be deferred"
    );
}

#[test]
fn defer_with_flatten() {
    #[derive(Args, Debug, PartialEq)]
    struct Common {
        #[arg(long)]
        verbose: bool,
    }

    #[derive(Parser, Debug, PartialEq)]
    #[command(defer = true)]
    struct Opt {
        #[command(flatten)]
        common: Common,
        #[arg(long)]
        name: String,
    }

    let opt = Opt::try_parse_from(["test", "--verbose", "--name", "foo"]).unwrap();
    assert!(opt.common.verbose);
    assert_eq!(opt.name, "foo");
}

#[test]
fn defer_bare_means_true() {
    #[derive(Parser, Debug, PartialEq)]
    #[command(defer)]
    struct Opt {
        #[arg(long)]
        flag: bool,
    }

    let opt = Opt::try_parse_from(["test", "--flag"]).unwrap();
    assert!(opt.flag);
}

/// Test for defer on enums with nested #[command(subcommand)] variants.
/// This tests the Kind::Subcommand code path in subcommand.rs.
/// Note: defer only affects .arg() calls, not subcommand registration.
/// Nested subcommands must always be registered for parsing to work.
#[test]
fn defer_on_deeply_nested_subcommands() {
    #[derive(Parser, Debug, PartialEq)]
    struct Cli {
        #[command(subcommand)]
        cmd: Level1,
    }

    #[derive(Subcommand, Debug, PartialEq)]
    #[command(defer = true)]
    enum Level1 {
        /// First level has a nested subcommand
        #[command(subcommand)]
        First(Level2),
    }

    #[derive(Subcommand, Debug, PartialEq)]
    enum Level2 {
        Second {
            #[arg(long)]
            value: String,
        },
    }

    // Parsing must work - nested subcommands should be registered
    let cli = Cli::try_parse_from(["test", "first", "second", "--value", "hello"]).unwrap();
    assert_eq!(
        cli.cmd,
        Level1::First(Level2::Second {
            value: "hello".to_string()
        })
    );

    // Nested subcommands must be present for parsing to work
    let cmd = Cli::command();
    let first_subcmd = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "first")
        .unwrap();
    // Subcommands are always registered (not deferred), only args are deferred
    assert!(
        first_subcmd.get_subcommands().next().is_some(),
        "nested subcommands must be registered for parsing"
    );
}

/// Test that defer defaults based on the unstable-v5 feature flag.
/// Without unstable-v5, defer defaults to false (args are eager).
/// With unstable-v5, defer defaults to true (args are deferred).
#[test]
fn defer_default_based_on_feature() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(long)]
        flag: bool,
    }

    let cmd = Opt::command();
    let has_args = cmd.get_arguments().next().is_some();

    if cfg!(feature = "unstable-v5") {
        assert!(
            !has_args,
            "with unstable-v5, defer should default to true (no args yet)"
        );
    } else {
        assert!(
            has_args,
            "without unstable-v5, defer should default to false (args present)"
        );
    }
}
