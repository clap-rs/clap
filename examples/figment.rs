use clap::{error::ErrorKind, CommandFactory, Parser};
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use serde_default_utils::*;
use std::path::PathBuf;

/// A demo showing the use of figment with clap
#[serde_inline_default]
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// Name of the person to greet
    #[arg(short, long)]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    name: Option<String>,

    /// Number of times to greet - default value of 1
    #[arg(short, long)]
    #[serde_inline_default(Some(1))]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    count: Option<u8>,
}

fn main() {
    let cfgpath = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("figment_config.toml");

    let config: AppConfig = Figment::new()
        .merge(Toml::file(cfgpath))
        .merge(Serialized::defaults(AppConfig::parse()))
        .extract()
        .unwrap();

    // Custom Validation
    if config.name.is_none() {
        let mut cmd = AppConfig::command();
        cmd.error(ErrorKind::MissingRequiredArgument, "name option not found")
            .exit();
    }

    println!("Name: {}", config.name.unwrap());
    println!("Count: {}", config.count.unwrap());
}
