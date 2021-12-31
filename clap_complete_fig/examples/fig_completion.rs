use clap::App;
use clap_complete::generate;
use clap_complete_fig::Fig;
use std::io;

fn main() {
    let mut app = App::new("myapp")
        .subcommand(App::new("test").subcommand(App::new("config")))
        .subcommand(App::new("hello"));

    generate(Fig, &mut app, "myapp", &mut io::stdout());
}
