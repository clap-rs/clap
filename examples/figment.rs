use clap::{CommandFactory, Parser, error::ErrorKind};
use figment::{Figment, providers::{Serialized, Toml, Format}};
use serde::{Serialize, Deserialize};
use serde_default_utils::*;

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
    let config: AppConfig = Figment::new()
        .merge(Toml::file("figment_config.toml"))
        .merge(Serialized::defaults(AppConfig::parse()))
        .extract().unwrap();

    // Custom Validation
    if config.name.is_none() {
        let mut cmd = AppConfig::command();
        cmd.error(
            ErrorKind::MissingRequiredArgument,
            "name option not found",
        )
        .exit();
    }

    println!("Name: {}", config.name.unwrap());
    println!("Count: {}", config.count.unwrap());
}
