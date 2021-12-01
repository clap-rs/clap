use std::path::PathBuf;

use clap::{arg, App, AppSettings};

fn main() {
    let matches = App::new("git")
        .about("A fictional versioning CLI")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(
            App::new("clone")
                .about("Clones repos")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(
            App::new("push")
                .about("pushes things")
                .arg(arg!(<REMOTE> "The remote to target"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(
            App::new("add")
                .about("adds things")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(arg!(<PATH> ... "Stuff to add").allow_invalid_utf8(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("clone", sub_matches)) => {
            println!(
                "Cloning {}",
                sub_matches.value_of("REMOTE").expect("required")
            );
        }
        Some(("push", sub_matches)) => {
            println!(
                "Pushing to {}",
                sub_matches.value_of("REMOTE").expect("required")
            );
        }
        Some(("add", sub_matches)) => {
            let paths = sub_matches
                .values_of_os("PATH")
                .unwrap_or_default()
                .map(PathBuf::from)
                .collect::<Vec<_>>();
            println!("Adding {:?}", paths);
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .values_of_os("")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    // Continued program logic goes here...
}
