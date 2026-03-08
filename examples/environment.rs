use clap::Parser;

/// A CLI that reads configuration from arguments, environment variables, or
/// defaults (in that priority order).
///
/// Try running:
///
///     cargo run --example environment -- --host 0.0.0.0
///     APP_HOST=0.0.0.0 cargo run --example environment
///     APP_HOST=0.0.0.0 cargo run --example environment -- --host 127.0.0.1
///
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Address to bind to
    #[arg(long, env = "APP_HOST", default_value = "127.0.0.1")]
    host: String,

    /// Port to listen on
    #[arg(short, long, env = "APP_PORT", default_value_t = 3000)]
    port: u16,

    /// Enable verbose output
    #[arg(short, long, env = "APP_VERBOSE")]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("Configuration:");
        println!("  host:    {}", args.host);
        println!("  port:    {}", args.port);
        println!("  verbose: {}", args.verbose);
    }

    println!("Listening on {}:{}", args.host, args.port);
}
