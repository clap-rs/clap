// Note: this requires the `derive` feature

use clap::Parser;
use std::ops::RangeInclusive;
use std::str::FromStr;

const COUNT_RANGE: RangeInclusive<u8> = 1..=5;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 2, validator = in_range)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    println!("count is {}", args.count);
}

fn in_range(v: &str) -> Result<(), String> {
    u8::from_str(v)
        .map(|d| COUNT_RANGE.contains(&d))
        .map_err(|e| e.to_string())
        .and_then(|d| match d {
            true => Ok(()),
            false => Err(String::from("value isn't in range")),
        })
}
