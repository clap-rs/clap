use super::utils;

use clap::{arg, Arg, ArgAction, Command};

static HIDDEN_ARGS: &str = "\
tests stuff

Usage: test [OPTIONS]

Options:
  -F, --flag2         some other flag
      --option <opt>  some option
  -h, --help          Print help
  -V, --version       Print version
";

#[test]
fn hide_args() {
    let cmd = Command::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.4")
        .args([
            arg!(-f --flag "some flag").hide(true),
            arg!(-F --flag2 "some other flag"),
            arg!(--option <opt> "some option"),
            Arg::new("DUMMY").hide(true),
        ]);
    utils::assert_output(cmd, "test --help", HIDDEN_ARGS, false);
}

static HIDDEN_SHORT_ARGS: &str = "\
hides short args

Usage: test [OPTIONS]

Options:
  -v, --visible  This text should be visible
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
";

/// Ensure hide with short option
#[test]
fn hide_short_args() {
    let cmd = Command::new("test")
        .about("hides short args")
        .author("Steve P.")
        .version("2.31.2")
        .args([
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hide_short_help(true)
                .action(ArgAction::SetTrue)
                .help("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .action(ArgAction::SetTrue)
                .help("This text should be visible"),
        ]);

    utils::assert_output(cmd, "test -h", HIDDEN_SHORT_ARGS, false);
}

/// Ensure visible with opposite option
#[test]
fn hide_short_args_long_help() {
    static HIDDEN_SHORT_ARGS_LONG_HELP: &str = "\
hides short args

Usage: test [OPTIONS]

Options:
  -c, --config
          Some help text describing the --config arg

  -v, --visible
          This text should be visible

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
";

    let cmd = Command::new("test")
        .about("hides short args")
        .author("Steve P.")
        .version("2.31.2")
        .args([
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hide_short_help(true)
                .action(ArgAction::SetTrue)
                .help("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .action(ArgAction::SetTrue)
                .help("This text should be visible"),
        ]);

    utils::assert_output(cmd, "test --help", HIDDEN_SHORT_ARGS_LONG_HELP, false);
}

static HIDDEN_LONG_ARGS: &str = "\
hides long args

Usage: test [OPTIONS]

Options:
  -v, --visible
          This text should be visible

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
";

#[test]
fn hide_long_args() {
    let cmd = Command::new("test")
        .about("hides long args")
        .author("Steve P.")
        .version("2.31.2")
        .args([
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hide_long_help(true)
                .action(ArgAction::SetTrue)
                .help("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .action(ArgAction::SetTrue)
                .help("This text should be visible"),
        ]);

    utils::assert_output(cmd, "test --help", HIDDEN_LONG_ARGS, false);
}

static HIDDEN_LONG_ARGS_SHORT_HELP: &str = "\
hides long args

Usage: test [OPTIONS]

Options:
  -c, --config   Some help text describing the --config arg
  -v, --visible  This text should be visible
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
";

#[test]
fn hide_long_args_short_help() {
    let cmd = Command::new("test")
        .about("hides long args")
        .author("Steve P.")
        .version("2.31.2")
        .args([
            Arg::new("cfg")
                .short('c')
                .long("config")
                .hide_long_help(true)
                .action(ArgAction::SetTrue)
                .help("Some help text describing the --config arg"),
            Arg::new("visible")
                .short('v')
                .long("visible")
                .action(ArgAction::SetTrue)
                .help("This text should be visible"),
        ]);

    utils::assert_output(cmd, "test -h", HIDDEN_LONG_ARGS_SHORT_HELP, false);
}

static HIDDEN_POS_ARGS: &str = "\
Usage: test [another]

Arguments:
  [another]  another pos

Options:
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn hide_pos_args() {
    let cmd = Command::new("test").version("1.4").args([
        Arg::new("pos").help("some pos").hide(true),
        Arg::new("another").help("another pos"),
    ]);

    utils::assert_output(cmd, "test --help", HIDDEN_POS_ARGS, false);
}

static HIDDEN_SUBCMDS: &str = "\
Usage: test

Options:
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn hide_subcmds() {
    let cmd = Command::new("test")
        .version("1.4")
        .subcommand(Command::new("sub").hide(true));

    utils::assert_output(cmd, "test --help", HIDDEN_SUBCMDS, false);
}

static HIDDEN_OPT_ARGS_ONLY: &str = "\
Usage: test

After help
";

#[test]
fn hide_opt_args_only() {
    let cmd = Command::new("test")
        .version("1.4")
        .after_help("After help")
        .disable_help_flag(true)
        .disable_version_flag(true)
        .arg(arg!(-h - -help).action(ArgAction::Help).hide(true))
        .arg(arg!(-v - -version).hide(true))
        .arg(arg!(--option <opt> "some option").hide(true));

    utils::assert_output(cmd, "test --help", HIDDEN_OPT_ARGS_ONLY, false);
}

static HIDDEN_POS_ARGS_ONLY: &str = "\
Usage: test

After help
";

#[test]
fn hide_pos_args_only() {
    let cmd = Command::new("test")
        .version("1.4")
        .after_help("After help")
        .disable_help_flag(true)
        .disable_version_flag(true)
        .arg(arg!(-h - -help).action(ArgAction::Help).hide(true))
        .arg(arg!(-v - -version).hide(true))
        .args([Arg::new("pos").help("some pos").hide(true)]);

    utils::assert_output(cmd, "test --help", HIDDEN_POS_ARGS_ONLY, false);
}

static HIDDEN_SUBCMDS_ONLY: &str = "\
Usage: test

After help
";

#[test]
fn hide_subcmds_only() {
    let cmd = Command::new("test")
        .version("1.4")
        .after_help("After help")
        .disable_help_flag(true)
        .disable_version_flag(true)
        .arg(arg!(-h - -help).action(ArgAction::Help).hide(true))
        .arg(arg!(-v - -version).hide(true))
        .subcommand(Command::new("sub").hide(true));

    utils::assert_output(cmd, "test --help", HIDDEN_SUBCMDS_ONLY, false);
}
