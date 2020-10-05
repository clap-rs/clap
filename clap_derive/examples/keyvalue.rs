//! How to parse "key=value" pairs with #[derive(Clap)].

use clap::Clap;
use std::error::Error;

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Clap, Debug)]
struct Opt {
    // number_of_values = 1 forces the user to repeat the -D option for each key-value pair:
    // my_program -D a=1 -D b=2
    // Without number_of_values = 1 you can do:
    // my_program -D a=1 b=2
    // but this makes adding an argument after the values impossible:
    // my_program -D a=1 -D b=2 my_input_file
    // becomes invalid.
    #[clap(short = 'D', parse(try_from_str = parse_key_val), number_of_values = 1)]
    defines: Vec<(String, i32)>,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
