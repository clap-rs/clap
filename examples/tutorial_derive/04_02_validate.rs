use clap::Parser;
use std::ops::RangeInclusive;
use std::str::FromStr;

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Network port to use
    #[clap(parse(try_from_str), validator = port_in_range)]
    port: usize,
}

fn main() {
    let cli = Cli::parse();

    println!("PORT = {}", cli.port);
}

fn port_in_range(v: &str) -> Result<(), String> {
    usize::from_str(v)
        .map(|d| PORT_RANGE.contains(&d))
        .map_err(|e| e.to_string())
        .and_then(|d| match d {
            true => Ok(()),
            false => Err(format!(
                "Port not in range {}-{}",
                PORT_RANGE.start(),
                PORT_RANGE.end()
            )),
        })
}
