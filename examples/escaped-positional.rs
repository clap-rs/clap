// Note: this requires the `cargo` feature

use clap::{arg, command, value_parser, ArgAction};

fn main() {
    let matches = command!()
        .arg(arg!(eff: -f).action(ArgAction::SetTrue))
        .arg(
            arg!(pea: -p <PEAR>)
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .arg(
            // Indicates that `slop` is only accessible after `--`.
            arg!(slop: [SLOP])
                .multiple_values(true)
                .last(true)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    // This is what will happen with `myprog -f -p=bob -- sloppy slop slop`...

    // -f used: true
    println!(
        "-f used: {:?}",
        *matches.get_one::<bool>("eff").expect("defaulted by clap")
    );
    // -p's value: Some("bob")
    println!("-p's value: {:?}", matches.get_one::<String>("pea"));
    // 'slops' values: Some(["sloppy", "slop", "slop"])
    println!(
        "'slops' values: {:?}",
        matches
            .get_many::<String>("slop")
            .map(|vals| vals.collect::<Vec<_>>())
            .unwrap_or_default()
    );

    // Continued program logic goes here...
}
