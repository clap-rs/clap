//! Somewhat complex example of usage of structopt.

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "example")]
/// An example of clap_derive usage.
struct Opt {
    // A flag, true if used in the command line.
    #[clap(short, long)]
    /// Activate debug mode
    debug: bool,

    // An argument of type float, with a default value.
    #[clap(short, long, default_value = "42")]
    /// Set speed
    speed: f64,

    // Needed parameter, the first on the command line.
    /// Input file
    input: String,

    // An optional parameter, will be `None` if not present on the
    // command line.
    /// Output file, stdout if not present
    output: Option<String>,

    // An optional parameter with optional value, will be `None` if
    // not present on the command line, will be `Some(None)` if no
    // argument is provided (i.e. `--log`) and will be
    // `Some(Some(String))` if argument is provided (e.g. `--log
    // log.txt`).
    #[clap(long)]
    #[allow(clippy::option_option)]
    /// Log file, stdout if no file, no logging if not present
    log: Option<Option<String>>,

    // An optional list of values, will be `None` if not present on
    // the command line, will be `Some(vec![])` if no argument is
    // provided (i.e. `--optv`) and will be `Some(Some(String))` if
    // argument list is provided (e.g. `--optv a b c`).
    #[clap(long)]
    optv: Option<Vec<String>>,

    // Skipped option: it won't be parsed and will be filled with the
    // default value for its type (in this case it'll be an empty string).
    #[clap(skip)]
    skipped: String,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
