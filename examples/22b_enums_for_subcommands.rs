// Next we'll look at how we can have sane subcommands, or ones with hyphens, and still make the
// compiler happy about camel_case_types

// First we import the clap macros
#[macro_use]
extern crate clap;
use clap::{App, SubCommand};

// Now we declare the variants, and their associated display/usage version
subcommands!{
    enum Git {
        Clone => "clone",
        Push = "push",
        Pull => "pull",
        DoStuff => "do-stuff"
    }
}

fn main() {
    use Git::*;

    let m = App::new("git")
        .subcommand(SubCommand::with_name(Clone))
        .subcommand(SubCommand::with_name(Push))
        .subcommand(SubCommand::with_name(Pull))
        .subcommand(SubCommand::with_name(DoStuff))
        .get_matches();

    // Now to access the subcommands we can pattern match against the variants
    //
    // Note, the tuple is (Variant, ArgMatches), but we're not using the ArgMatches here, so we
    // just throw them away with _
    match m.subcommand() {
        Some((Clone, _)) => println!("clone was used"),
        Some((Push, _)) => println!("push was used"),
        Some((Pull, _)) => println!("pull was used"), // <-- commment out this line to see the
                                                      // compiler complain about NonExaustive
                                                      // matches. Yay for no more strings! :)
        Some((DoStuff, _)) => println!("do-stuff was used"),
        None => println!("No subcommand was used... :("),
    }
}
