extern crate clap;

use clap::{App, Arg};

fn main() {
    // If you have arguments of specific values you want to test for, you can use the
    // .possible_values() method of Arg
    //
    // This allows you specify the valid values for that argument. If the user does not use one of
    // those specific values, they will receive a graceful exit with error message informing them
    // of the mistake, and what the possible valid values are
    //
    // For this example, assume you want one positional argument of either "fast" or "slow"
    // i.e. the only possible ways to run the program are "myprog fast" or "myprog slow"
    let matches = App::new("myapp")
        .about("does awesome things")
        .arg(
            Arg::with_name("MODE")
                .help("What mode to run the program in")
                .index(1)
                .possible_values(&["fast", "slow"])
                .required(true),
        )
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    match matches.value_of("MODE").unwrap() {
        "fast" => {
            // Do fast things...
        }
        "slow" => {
            // Do slow things...
        }
        _ => unreachable!(),
    }
}
