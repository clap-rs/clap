use clap::{App, Arg};

fn main() {
    let matches = App::new("myprog")
        .arg(Arg::new("eff").short('f'))
        .arg(Arg::new("pea").short('p').takes_value(true))
        .arg(
            Arg::new("slop")
                .takes_value(true)
                .multiple_values(true)
                .last(true), // Indicates that `slop` is only accessible after `--`.
        )
        .get_matches();

    // This is what will happen with `myprog -f -p=bob -- sloppy slop slop`...
    println!("-f used: {:?}", matches.is_present("eff")); // -f used: true
    println!("-p's value: {:?}", matches.value_of("pea")); // -p's value: Some("bob")
    println!(
        "'slops' values: {:?}",
        matches
            .values_of("slop")
            .map(|vals| vals.collect::<Vec<_>>())
    ); // 'slops' values: Some(["sloppy", "slop", "slop"])

    // Continued program logic goes here...
}
