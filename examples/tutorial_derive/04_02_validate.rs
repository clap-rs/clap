use clap::Parser;
use std::ops::RangeInclusive;
use std::str::FromStr;

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Network port to use
    #[clap(validator = port_in_range)]
    port: usize,
}

fn main() {
    let cli = Cli::parse();

    println!("PORT = {}", cli.port);
}

fn port_in_range(s: &str) -> Result<(), String> {
    usize::from_str(s)
        .map(|port| PORT_RANGE.contains(&port))
        .map_err(|e| e.to_string())
        .and_then(|result| match result {
            true => Ok(()),
            false => Err(format!(
                "Port not in range {}-{}",
                PORT_RANGE.start(),
                PORT_RANGE.end()
            )),
        })
}
