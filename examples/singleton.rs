use clap::Parser;

use once_cell::sync::Lazy;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[arg(short, long, default_value_t = 42)]
    the_answer: u8,
}

static ARGS: Lazy<Args> = Lazy::new(Args::parse);

fn main() {
    println!("The answer is {}!", ARGS.the_answer)
}
