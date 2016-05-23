// In this method we go back to the original, but find a way to get the desired affect without
// having to give a "display" version for all args

// We import the clap macros
#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

// Everything starts out normal, we declare the enums just like before.
//
// Notice we can also put multiple enums into a single `args!` macro. This is useful if we're using
// subcommands.
args!{
    enum MyProgArgs {
        Config
    }
    enum TestArgs {
        Verbose,
        SomeFlag
    }
}

fn main() {
    use MyProgArgs::*;
    use TestArgs::*;

    let m = App::new("myprog")
        .arg(Arg::with_name(Config)
            .usage("<FILE> 'The custom config file to use'")) // Here we use a usage method which
                                                              // allows setting all the normal
                                                              // usage style settings (Same as
                                                              // Arg::from_usage), but with the
                                                              // added benefit of "renaming" the
                                                              // display version of this arg. In
                                                              // this example, "FILE"
        .subcommand(SubCommand::with_name("test")
            .arg(Arg::with_name(Verbose)
                .usage("-v, --verbose 'print verbose output'")))
            .arg(Arg::with_name(SomeFlag)
                .usage("--some-flag 'some imaginary flag'")))
        .get_matches();

        // more logic here...
}
