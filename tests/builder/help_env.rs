#![cfg(feature = "env")]

use std::env;

use clap::{Arg, ArgAction, Command};

use super::utils;

static HIDE_ENV: &str = "\
Usage: ctest [OPTIONS]

Options:
  -c, --cafe <FILE>  A coffeehouse, coffee shop, or café.
  -h, --help         Print help
  -V, --version      Print version
";

static SHOW_ENV: &str = "\
Usage: ctest [OPTIONS]

Options:
  -c, --cafe <FILE>  A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]
  -h, --help         Print help
  -V, --version      Print version
";

static HIDE_ENV_VALS: &str = "\
Usage: ctest [OPTIONS]

Options:
  -c, --cafe <FILE>  A coffeehouse, coffee shop, or café. [env: ENVVAR]
  -h, --help         Print help
  -V, --version      Print version
";

static SHOW_ENV_VALS: &str = "\
Usage: ctest [OPTIONS]

Options:
  -c, --cafe <FILE>  A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]
  -h, --help         Print help
  -V, --version      Print version
";

static HIDE_ENV_FLAG: &str = "\
Usage: ctest [OPTIONS]

Options:
  -c, --cafe     A coffeehouse, coffee shop, or café.
  -h, --help     Print help
  -V, --version  Print version
";

static SHOW_ENV_FLAG: &str = "\
Usage: ctest [OPTIONS]

Options:
  -c, --cafe     A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]
  -h, --help     Print help
  -V, --version  Print version
";

static HIDE_ENV_VALS_FLAG: &str = "\
Usage: ctest [OPTIONS]

Options:
  -c, --cafe     A coffeehouse, coffee shop, or café. [env: ENVVAR]
  -h, --help     Print help
  -V, --version  Print version
";

static SHOW_ENV_VALS_FLAG: &str = "\
Usage: ctest [OPTIONS]

Options:
  -c, --cafe     A coffeehouse, coffee shop, or café. [env: ENVVAR=MYVAL]
  -h, --help     Print help
  -V, --version  Print version
";

#[test]
fn hide_env() {
    env::set_var("ENVVAR", "MYVAL");

    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .hide_env(true)
            .env("ENVVAR")
            .help("A coffeehouse, coffee shop, or café.")
            .action(ArgAction::Set),
    );

    utils::assert_output(cmd, "ctest --help", HIDE_ENV, false);
}

#[test]
fn show_env() {
    env::set_var("ENVVAR", "MYVAL");

    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .env("ENVVAR")
            .help("A coffeehouse, coffee shop, or café.")
            .action(ArgAction::Set),
    );

    utils::assert_output(cmd, "ctest --help", SHOW_ENV, false);
}

#[test]
fn hide_env_vals() {
    env::set_var("ENVVAR", "MYVAL");

    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .hide_env_values(true)
            .env("ENVVAR")
            .help("A coffeehouse, coffee shop, or café.")
            .action(ArgAction::Set),
    );

    utils::assert_output(cmd, "ctest --help", HIDE_ENV_VALS, false);
}

#[test]
fn show_env_vals() {
    env::set_var("ENVVAR", "MYVAL");

    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .value_name("FILE")
            .env("ENVVAR")
            .help("A coffeehouse, coffee shop, or café.")
            .action(ArgAction::Set),
    );

    utils::assert_output(cmd, "ctest --help", SHOW_ENV_VALS, false);
}

#[test]
fn hide_env_flag() {
    env::set_var("ENVVAR", "MYVAL");

    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .action(ArgAction::SetTrue)
            .hide_env(true)
            .env("ENVVAR")
            .help("A coffeehouse, coffee shop, or café."),
    );

    utils::assert_output(cmd, "ctest --help", HIDE_ENV_FLAG, false);
}

#[test]
fn show_env_flag() {
    env::set_var("ENVVAR", "MYVAL");

    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .action(ArgAction::SetTrue)
            .env("ENVVAR")
            .help("A coffeehouse, coffee shop, or café."),
    );

    utils::assert_output(cmd, "ctest --help", SHOW_ENV_FLAG, false);
}

#[test]
fn hide_env_vals_flag() {
    env::set_var("ENVVAR", "MYVAL");

    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .action(ArgAction::SetTrue)
            .hide_env_values(true)
            .env("ENVVAR")
            .help("A coffeehouse, coffee shop, or café."),
    );

    utils::assert_output(cmd, "ctest --help", HIDE_ENV_VALS_FLAG, false);
}

#[test]
fn show_env_vals_flag() {
    env::set_var("ENVVAR", "MYVAL");

    let cmd = Command::new("ctest").version("0.1").arg(
        Arg::new("cafe")
            .short('c')
            .long("cafe")
            .action(ArgAction::SetTrue)
            .env("ENVVAR")
            .help("A coffeehouse, coffee shop, or café."),
    );

    utils::assert_output(cmd, "ctest --help", SHOW_ENV_VALS_FLAG, false);
}
