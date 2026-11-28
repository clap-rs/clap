use clap::Parser;

#[derive(Parser, Debug)]
struct Opt {
    #[arg(short, long, env = "PROGRAM_PASSWORD", hide_env_values)]
    password: Option<String>,
}

fn main() {
    let opt = Opt::parse();
    println!("{opt:?}");
}
