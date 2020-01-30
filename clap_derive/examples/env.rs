//! How to use environment variable fallback an how it
//! interacts with `default_value`.

use clap::Clap;

/// Example for allowing to specify options via environment variables.
#[derive(Clap, Debug)]
#[clap(name = "env")]
struct Opt {
    // Use `env` to enable specifying the option with an environment
    // variable. Command line arguments take precedence over env.
    /// URL for the API server
    #[clap(long, env = "API_URL")]
    api_url: String,

    // The default value is used if neither argument nor environment
    // variable is specified.
    /// Number of retries
    #[clap(long, env = "RETRIES", default_value = "5")]
    retries: u32,
}

fn main() {
    let opt = Opt::parse();
    println!("{:#?}", opt);
}
