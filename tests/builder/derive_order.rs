use super::utils;

use std::str;

use clap::{Arg, ArgAction, Command};

#[test]
fn no_derive_order() {
    static NO_DERIVE_ORDER: &str = "\
Usage: test [OPTIONS]

Options:
      --flag_a               second flag
      --flag_b               first flag
  -h, --help                 Print help
      --option_a <option_a>  second option
      --option_b <option_b>  first option
  -V, --version              Print version
";

    let cmd = Command::new("test")
        .version("1.2")
        .next_display_order(None)
        .args([
            Arg::new("flag_b")
                .long("flag_b")
                .help("first flag")
                .action(ArgAction::SetTrue),
            Arg::new("option_b")
                .long("option_b")
                .action(ArgAction::Set)
                .help("first option"),
            Arg::new("flag_a")
                .long("flag_a")
                .help("second flag")
                .action(ArgAction::SetTrue),
            Arg::new("option_a")
                .long("option_a")
                .action(ArgAction::Set)
                .help("second option"),
        ]);

    utils::assert_output(cmd, "test --help", NO_DERIVE_ORDER, false);
}

#[test]
fn derive_order() {
    static UNIFIED_HELP_AND_DERIVE: &str = "\
Usage: test [OPTIONS]

Options:
      --flag_b               first flag
      --option_b <option_b>  first option
      --flag_a               second flag
      --option_a <option_a>  second option
  -h, --help                 Print help
  -V, --version              Print version
";

    let cmd = Command::new("test").version("1.2").args([
        Arg::new("flag_b")
            .long("flag_b")
            .help("first flag")
            .action(ArgAction::SetTrue),
        Arg::new("option_b")
            .long("option_b")
            .action(ArgAction::Set)
            .help("first option"),
        Arg::new("flag_a")
            .long("flag_a")
            .help("second flag")
            .action(ArgAction::SetTrue),
        Arg::new("option_a")
            .long("option_a")
            .action(ArgAction::Set)
            .help("second option"),
    ]);

    utils::assert_output(cmd, "test --help", UNIFIED_HELP_AND_DERIVE, false);
}

#[test]
fn derive_order_next_order() {
    static HELP: &str = "\
Usage: test [OPTIONS]

Options:
      --flag_b               first flag
      --option_b <option_b>  first option
  -h, --help                 Print help
  -V, --version              Print version
      --flag_a               second flag
      --option_a <option_a>  second option
";

    let cmd = Command::new("test")
        .version("1.2")
        .next_display_order(10000)
        .arg(
            Arg::new("flag_a")
                .long("flag_a")
                .help("second flag")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("option_a")
                .long("option_a")
                .action(ArgAction::Set)
                .help("second option"),
        )
        .next_display_order(10)
        .arg(
            Arg::new("flag_b")
                .long("flag_b")
                .help("first flag")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("option_b")
                .long("option_b")
                .action(ArgAction::Set)
                .help("first option"),
        );

    utils::assert_output(cmd, "test --help", HELP, false);
}

#[test]
fn derive_order_no_next_order() {
    static HELP: &str = "\
Usage: test [OPTIONS]

Options:
      --flag_a               first flag
      --flag_b               second flag
  -h, --help                 Print help
      --option_a <option_a>  first option
      --option_b <option_b>  second option
  -V, --version              Print version
";

    let cmd = Command::new("test")
        .version("1.2")
        .next_display_order(None)
        .arg(
            Arg::new("flag_a")
                .long("flag_a")
                .help("first flag")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("option_a")
                .long("option_a")
                .action(ArgAction::Set)
                .help("first option"),
        )
        .arg(
            Arg::new("flag_b")
                .long("flag_b")
                .help("second flag")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("option_b")
                .long("option_b")
                .action(ArgAction::Set)
                .help("second option"),
        );

    utils::assert_output(cmd, "test --help", HELP, false);
}

#[test]
fn derive_order_subcommand_propagate() {
    static UNIFIED_DERIVE_SC_PROP: &str = "\
Usage: test sub [OPTIONS]

Options:
      --flag_b               first flag
      --option_b <option_b>  first option
      --flag_a               second flag
      --option_a <option_a>  second option
  -h, --help                 Print help
  -V, --version              Print version
";

    let cmd = Command::new("test").subcommand(
        Command::new("sub").version("1.2").args([
            Arg::new("flag_b")
                .long("flag_b")
                .help("first flag")
                .action(ArgAction::SetTrue),
            Arg::new("option_b")
                .long("option_b")
                .action(ArgAction::Set)
                .help("first option"),
            Arg::new("flag_a")
                .long("flag_a")
                .help("second flag")
                .action(ArgAction::SetTrue),
            Arg::new("option_a")
                .long("option_a")
                .action(ArgAction::Set)
                .help("second option"),
        ]),
    );

    utils::assert_output(cmd, "test sub --help", UNIFIED_DERIVE_SC_PROP, false);
}

#[test]
fn derive_order_subcommand_propagate_with_explicit_display_order() {
    static UNIFIED_DERIVE_SC_PROP_EXPLICIT_ORDER: &str = "\
Usage: test sub [OPTIONS]

Options:
      --flag_a               second flag
      --flag_b               first flag
      --option_b <option_b>  first option
      --option_a <option_a>  second option
  -h, --help                 Print help
  -V, --version              Print version
";

    let cmd = Command::new("test").subcommand(
        Command::new("sub").version("1.2").args([
            Arg::new("flag_b")
                .long("flag_b")
                .help("first flag")
                .action(ArgAction::SetTrue),
            Arg::new("option_b")
                .long("option_b")
                .action(ArgAction::Set)
                .help("first option"),
            Arg::new("flag_a")
                .long("flag_a")
                .help("second flag")
                .display_order(0)
                .action(ArgAction::SetTrue),
            Arg::new("option_a")
                .long("option_a")
                .action(ArgAction::Set)
                .help("second option"),
        ]),
    );

    utils::assert_output(
        cmd,
        "test sub --help",
        UNIFIED_DERIVE_SC_PROP_EXPLICIT_ORDER,
        false,
    );
}

#[test]
fn subcommand_sorted_display_order() {
    static SUBCMD_ALPHA_ORDER: &str = "\
Usage: test [COMMAND]

Commands:
  a1    blah a1
  b1    blah b1
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let app_subcmd_alpha_order = Command::new("test")
        .version("1")
        .next_display_order(None)
        .subcommands(vec![
            Command::new("b1")
                .about("blah b1")
                .arg(Arg::new("test").short('t').action(ArgAction::SetTrue)),
            Command::new("a1")
                .about("blah a1")
                .arg(Arg::new("roster").short('r').action(ArgAction::SetTrue)),
        ]);

    utils::assert_output(
        app_subcmd_alpha_order,
        "test --help",
        SUBCMD_ALPHA_ORDER,
        false,
    );
}

#[test]
fn subcommand_derived_display_order() {
    static SUBCMD_DECL_ORDER: &str = "\
Usage: test [COMMAND]

Commands:
  b1    blah b1
  a1    blah a1
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
";

    let app_subcmd_decl_order = Command::new("test").version("1").subcommands(vec![
        Command::new("b1")
            .about("blah b1")
            .arg(Arg::new("test").short('t').action(ArgAction::SetTrue)),
        Command::new("a1")
            .about("blah a1")
            .arg(Arg::new("roster").short('r').action(ArgAction::SetTrue)),
    ]);

    utils::assert_output(
        app_subcmd_decl_order,
        "test --help",
        SUBCMD_DECL_ORDER,
        false,
    );
}
