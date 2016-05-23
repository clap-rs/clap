// This next method we'll look at an instance where we'll want the arg to be displayed differently
// from the enum variant itself

// We import the clap macros
#[macro_use]
extern crate clap;
use clap::{App, Arg};

// Next we declare the "keys" we'll use to access the arguments associated a display version
// The downside to using this form is that it's all or othing, either all args use a "display"
// version or are used literally. The next example shows one way we can get around this...
args!{
    enum MyProg {
        Config => "FILE"
    }
}

fn main() {
    use MyProg::*;
    let m = App::new("myprog")
        .arg(Arg::with_name(Config)
            .takes_value(true)
            .required(true)
            .help("The custom config file to use"))
        .get_matches();

    // Run this program with --help to see that `Config` appears as "FILE"

    if let Some(file) = m.value_of(Config) {
        println!("Using config file: {}", file);
    }
}
