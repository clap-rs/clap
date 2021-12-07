use clap::{app_from_crate, arg};

fn main() {
    let matches = app_from_crate!()
        .arg(
            arg!(<MODE>)
                .help("What mode to run the program in")
                .possible_values(["fast", "slow"]),
        )
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    match matches
        .value_of("MODE")
        .expect("'MODE' is required and parsing will fail if its missing")
    {
        "fast" => {
            println!("Hare");
        }
        "slow" => {
            println!("Tortoise");
        }
        _ => unreachable!(),
    }
}
