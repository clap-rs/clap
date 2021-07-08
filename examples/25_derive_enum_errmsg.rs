use clap::{crate_authors, crate_version, Clap, Error, ErrorKind};
use std::str::FromStr;

#[derive(Clap)]
#[clap(version =crate_version!() , author = crate_authors!())]
struct Options {
    #[clap(long, short, default_value = "127.0.0.1")]
    address: String,

    #[clap(long, short, default_value = "north")]
    direction: Direction,
}
#[derive(Debug)]
enum Direction {
    West,
    East,
    South,
    North,
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "north" => Ok(Direction::North),
            "south" => Ok(Direction::South),
            "east" => Ok(Direction::East),
            "west" => Ok(Direction::West),
            // customize error message
            _ => Err(Error::with_description(
                "direction should be west, east, south, or north".to_string(),
                ErrorKind::InvalidValue,
            )),
        }
    }
}
fn main() {
    let options = Options::parse();
    println!("{}", options.address);
    println!("{:?}", options.direction);
}
