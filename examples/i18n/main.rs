use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use clap::text_provider::TextProvider;
use rust_i18n::set_locale;
use rust_i18n::t;
use rust_i18n::Backend;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

pub struct RemoteI18n {
    pub trs: HashMap<String, HashMap<String, String>>,
}

impl RemoteI18n {
    fn new() -> Self {
        let en = read_to_string("./examples/i18n/locales/en.yml").unwrap();
        let jp = read_to_string("./examples/i18n/locales/jp.yml").unwrap();
        let en_trs = serde_yml::from_str::<HashMap<String, String>>(&en).unwrap();
        let jp_trs = serde_yml::from_str::<HashMap<String, String>>(&jp).unwrap();
        let trs = HashMap::from([("en".to_owned(), en_trs), ("jp".to_owned(), jp_trs)]);

        Self {
            trs
        }
    }
}

impl Backend for RemoteI18n {
    fn available_locales(&self) -> Vec<&str> {
        self.trs.keys().map(|k| k.as_str()).collect()
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        self.trs.get(locale)?.get(key).map(|k| k.as_str())
    }
}

rust_i18n::i18n!("locales", backend = RemoteI18n::new());


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// {name-help-text}
    #[arg(value_name = "{name}")]
    name: Option<String>,

    /// {config-help-text}
    #[arg(short, long, value_name = "{file}")]
    config: Option<PathBuf>,

    /// {debug-help-text}
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// {test-help-text}
    Test {
        /// {list.help-text}
        #[arg(short, long)]
        list: bool,
    },
}

struct I18n;

impl TextProvider for I18n {
    fn get(&self, key: &str) -> impl AsRef<str> {
        t!(key)
    }
}

fn main() {
    let locale = env::var("LOCALE").unwrap_or("en".into());
    set_locale(&locale);
    let cli = Cli::parse_with_texts(&I18n);
    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}


