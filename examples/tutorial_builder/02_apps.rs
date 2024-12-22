use clap::{arg, text_provider::DEFAULT_TEXT_PROVIDER, Command};

fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .about("Does awesome things")
        .arg(arg!(--two <VALUE>).required(true))
        .arg(arg!(--one <VALUE>).required(true))
        .get_matches(&*DEFAULT_TEXT_PROVIDER);

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );
}
