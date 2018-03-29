#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use std::error::Error;

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s.find('=').ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "D", parse(try_from_str = "parse_key_val"))]
    defines: Vec<(String, i32)>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
