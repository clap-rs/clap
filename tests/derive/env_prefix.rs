#![cfg(feature = "env")]
#![cfg(feature = "string")]

use clap::{Args, CommandFactory, Parser};
use std::env;

#[test]
fn command_next_env_prefix_applied() {
    #[derive(Debug, Clone, Parser)]
    #[command(next_env_prefix = "MYAPP")]
    struct CliOptions {
        #[arg(long, env = "CONFIG")]
        config: Option<String>,

        #[arg(long)]
        verbose: bool,
    }

    let cmd = CliOptions::command();

    let config_arg = cmd
        .get_arguments()
        .find(|a| a.get_id() == "config")
        .unwrap();
    assert_eq!(
        config_arg.get_env_prefix(),
        Some(std::ffi::OsStr::new("MYAPP"))
    );
}

#[test]
fn command_next_env_prefix_value_resolved() {
    env::set_var("DERIVE_APP_HOST", "localhost");

    #[derive(Debug, Clone, Parser)]
    #[command(next_env_prefix = "DERIVE_APP")]
    struct CliOptions {
        #[arg(long, env = "HOST")]
        host: Option<String>,
    }

    let m = CliOptions::try_parse_from(vec![""]).unwrap();
    assert_eq!(m.host.as_deref(), Some("localhost"));
}

#[test]
fn command_next_env_prefix_with_flatten() {
    env::set_var("FLAT_APP_DB", "mydb");

    #[derive(Debug, Clone, Args)]
    #[command(next_env_prefix = "FLAT_APP")]
    struct DbArgs {
        #[arg(long, env = "DB")]
        db: Option<String>,
    }

    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[command(flatten)]
        db_args: DbArgs,
    }

    let m = CliOptions::try_parse_from(vec![""]).unwrap();
    assert_eq!(m.db_args.db.as_deref(), Some("mydb"));
}

#[test]
fn flatten_field_with_env_prefix() {
    env::set_var("FIELD_PFX_PORT", "9090");

    #[derive(Debug, Clone, Args)]
    struct ServerArgs {
        #[arg(long, env = "PORT")]
        port: Option<String>,
    }

    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[command(flatten)]
        #[command(next_env_prefix = "FIELD_PFX")]
        server: ServerArgs,
    }

    let m = CliOptions::try_parse_from(vec![""]).unwrap();
    assert_eq!(m.server.port.as_deref(), Some("9090"));
}
