// Now we'll see how we can support external subcommands. These should be used with caution, as
// they silence certain circumstances that would otherwise be and error. These conditions must be
// checked manually. The specific cases are when someone uses a "potential" external subcommand,
// i.e. something that wasn't defined at compile time, but it turns out no external subcommand for
// that exists. One must then check for, and inform the user of the error manually.

// First we import the clap macros
#[macro_use]
extern crate clap;
use clap::{App, SubCommand};

// The only thing we need to do differently from the previous two examples is make the **FIRST**
// variant "External", and clap will generate all the necessary code.
//
// Note: All the previous version are still supported, such as pub/#[meta] items, and the basic or
// this alternate version
subcommands!{
    enum Git {
        External,
        Clone => "clone",
        Push => "push",
        Pull => "pull"
    }
}

fn main() {
    use Git::*;

    let m = App::new("git")
        .subcommand(SubCommand::with_name(Clone))
        .subcommand(SubCommand::with_name(Push))
        .subcommand(SubCommand::with_name(Pull))
        .get_matches();

    // Now to access the subcommands we can pattern match against the variants like normal. the
    // difference is now clap has generated an `External(Vec<OsString>)` variant that contains the
    // args along with the subcommand that was used.
    match m.subcommand() {
        Some((Clone, _)) => println!("Clone was used"),
        Some((Push, _)) => println!("Push was used"),
        Some((Pull, _)) => println!("Pull was used"),
        Some((External(ref args), _)) => println!("An external subcommand was used: {:?}", args),
        None => println!("No subcommand was used... :("),
    }
}
