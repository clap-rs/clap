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
    // SAFETY: test-only, single-threaded
    unsafe { env::set_var("DERIVE_APP_HOST", "localhost") };

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
    // env prefix set on the flattened Args struct itself
    // SAFETY: test-only, single-threaded
    unsafe { env::set_var("FLAT_APP_DB", "mydb") };

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
    // env prefix set on the flatten field, not the Args struct
    // SAFETY: test-only, single-threaded
    unsafe { env::set_var("FIELD_PFX_PORT", "9090") };

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

#[test]
fn flatten_multiple_structs_different_prefixes() {
    // Multiple flattened structs, each with their own prefix
    // SAFETY: test-only, single-threaded
    unsafe {
        env::set_var("DB_PFX_HOST", "dbhost");
        env::set_var("CACHE_PFX_HOST", "cachehost");
    }

    #[derive(Debug, Clone, Args)]
    #[command(next_env_prefix = "DB_PFX")]
    struct DbArgs {
        #[arg(long, env = "HOST")]
        db_host: Option<String>,
    }

    #[derive(Debug, Clone, Args)]
    #[command(next_env_prefix = "CACHE_PFX")]
    struct CacheArgs {
        #[arg(long, env = "HOST")]
        cache_host: Option<String>,
    }

    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[command(flatten)]
        db: DbArgs,

        #[command(flatten)]
        cache: CacheArgs,
    }

    let m = CliOptions::try_parse_from(vec![""]).unwrap();
    assert_eq!(m.db.db_host.as_deref(), Some("dbhost"));
    assert_eq!(m.cache.cache_host.as_deref(), Some("cachehost"));
}

#[test]
fn parent_env_prefix_does_not_leak_into_flatten() {
    // A next_env_prefix on the parent should not affect args
    // inside a flattened struct that doesn't set its own prefix.
    // SAFETY: test-only, single-threaded
    unsafe { env::set_var("PARENT_PFX_NAME", "from_prefix") };

    #[derive(Debug, Clone, Args)]
    struct InnerArgs {
        #[arg(long, env = "INNER")]
        inner: Option<String>,
    }

    #[derive(Debug, Clone, Parser)]
    #[command(next_env_prefix = "PARENT_PFX")]
    struct CliOptions {
        #[arg(long, env = "NAME")]
        name: Option<String>,

        #[command(flatten)]
        inner: InnerArgs,
    }

    let m = CliOptions::try_parse_from(vec![""]).unwrap();
    assert_eq!(m.name.as_deref(), Some("from_prefix"));
    // inner doesn't have PARENT_PFX prefix because InnerArgs
    // doesn't set one and the parent's prefix only applies to
    // args added directly to the parent command
    assert_eq!(m.inner.inner.as_deref(), None);
}
