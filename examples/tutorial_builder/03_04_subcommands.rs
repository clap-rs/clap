// Note: this requires the `cargo` feature

use clap::{arg, command, Command};

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds files to myapp")
                .arg(arg!([NAME])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "'myapp add' was used, name is: {:?}",
            sub_matches.value_of("NAME")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
