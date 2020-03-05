//! How to use `#[clap(skip)]`

use clap::Clap;

#[derive(Clap, Debug, PartialEq)]
pub struct Opt {
    #[clap(long, short)]
    number: u32,
    #[clap(skip)]
    k: Kind,
    #[clap(skip)]
    v: Vec<u32>,

    #[clap(skip = Kind::A)]
    k2: Kind,
    #[clap(skip = vec![1, 2, 3])]
    v2: Vec<u32>,
    #[clap(skip = "cake")] // &str implements Into<String>
    s: String,
}

#[derive(Debug, PartialEq)]
enum Kind {
    A,
    B,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::B
    }
}

fn main() {
    assert_eq!(
        Opt::parse_from(&["test", "-n", "10"]),
        Opt {
            number: 10,
            k: Kind::B,
            v: vec![],

            k2: Kind::A,
            v2: vec![1, 2, 3],
            s: String::from("cake")
        }
    );
}
