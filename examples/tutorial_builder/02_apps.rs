use clap::{arg, Command};

fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(arg!(--two <VALUE>))
        .arg(arg!(--one <VALUE>))
        .get_matches();

    println!(
        "two: {:?}",
        matches
            .get_one::<String>("two")
            .expect("matches definition")
            .expect("required")
    );
    println!(
        "one: {:?}",
        matches
            .get_one::<String>("one")
            .expect("matches definition")
            .expect("required")
    );
}
