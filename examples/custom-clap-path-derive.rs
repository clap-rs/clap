use clap::Parser;
pub mod foo {
    pub use clap;
}

#[derive(Parser)] // requires `derive` feature
#[clap(crate = "foo::clap")]
enum Cargo {
    ExampleDerive(ExampleDerive),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
#[clap(crate = "foo::clap")]
struct ExampleDerive {
    #[arg(long)]
    manifest_path: Option<std::path::PathBuf>,
}

fn main() {
    let Cargo::ExampleDerive(args) = Cargo::parse();
    println!("{:?}", args.manifest_path);
}
