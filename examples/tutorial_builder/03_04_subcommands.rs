use clap::{app_from_crate, arg, App, AppSettings};

fn main() {
    let matches = app_from_crate!()
        .propagate_version(true)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("add")
                .about("Adds files to myapp")
                .arg(arg!([NAME])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "'myapp add' was used, name is: {:?}",
            sub_matches.value_of("NAME")
        ),
        _ => unreachable!(
            "Exhausted list of subcommands and SubcommandRequiredElseHelp prevents `None`"
        ),
    }
}
