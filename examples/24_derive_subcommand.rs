use clap::{crate_authors, crate_version, Clap};

#[derive(Clap)]
/// auto version and author
#[clap(version =crate_version!() , author = crate_authors!())]
struct Options {
    #[clap(subcommand)]
    sub_command: SubCommand,
}
#[derive(Clap)]
enum SubCommand {
    Get(Key),
    Set(KeyValue),
    RM(Key),
}
#[derive(Clap)]
struct Key {
    key: String,
}
#[derive(Clap)]
struct KeyValue {
    key: String,
    value: String,
}
fn main() {
    let opts = Options::parse();
    // match subcommand
    match opts.sub_command {
        SubCommand::Get(value) => {
            println!("subcommand get {}!", value.key);
        }
        SubCommand::RM(value) => {
            println!("subcommand remove {}", value.key)
        }
        SubCommand::Set(key_value) => {
            println!(
                "subcommand set ket {}, value {}",
                key_value.key, key_value.value
            );
        }
    }
}
