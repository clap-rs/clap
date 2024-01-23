use std::io::Write;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Echo(EchoArgs),
    Ping,
    Exit,
}

#[derive(Args, Debug)]
pub struct EchoArgs {
    #[arg(
        short = 't',
        long = "text",
        visible_alias = "text",
        help = "The text to be echoed",
        help_heading = "Echo",
    )]
	text: String,
}

fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let cli = Cli::try_parse_from(args).map_err(|e| e.to_string())?;
    match cli.command {
        Commands::Echo(args) => {
            println!("{}", args.text);
        }
        Commands::Ping => {
            println!("Pong");
        }
        Commands::Exit => {
            println!("Exiting ...");
            return Ok(true);
        }
    }
    Ok(false)
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}