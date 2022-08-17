use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(global)]
    /// Converts any file names to lowercase.
    lowercase: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add { name: String },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { name } => {
            if cli.lowercase {
                println!("'myapp add' was used, name is: {:?}", name.to_lowercase())
            } else {
                println!("'myapp add' was used, name is: {:?}", name)
            }
        }
    }
}
