use clap::App;
use clap_generate::generate;
use clap_generate_fig::Fig;
use std::io;

fn main() {
    let mut app = App::new("myapp")
        .subcommand(App::new("test").subcommand(App::new("config")))
        .subcommand(App::new("hello"));

    generate::<Fig, _>(&mut app, "myapp", &mut io::stdout());
}
