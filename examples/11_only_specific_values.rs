use clap::{App, Arg};

fn main() {
    let matches = App::new("myapp")
        .about("does awesome things")
        .arg(
            Arg::new("MODE")
                .help("What mode to run the program in")
                .index(1)
                .possible_values(["fast", "slow"])
                .required(true),
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
