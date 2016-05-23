// There are multiple ways in which to use enums as the "Keys" to access subcommands. This has the
// benefit of not being "stringly-typed" and is *far* less error prone. It also allows one to take
// full advantage of auto-completion with tools like Racer (https://github.com/phildawes/racer). It
// also lets rustc do the error checking for you when you decide to change arugment names and such.
// Finally, but certainly not least, it allows rustc to check against all possible variants when
// pattern matching, and complain about NonExaustive matches when one adds a new subcommand but
// forgets to check for it!
//
// This first method we'll look at is the most simple version, where enum variants are used
// literally as the subcommand keys and displayed to the user as such. This can be at odds
// with Rust best practices for style guides, so in other examples we'll look at other ways use
// enums with subcommands.
//
// Pro Tip: It's a good idea to name your enum after the parent command that the subcommands belong
// to. This avoids massive collisions and typos, and allows pattern matching against variants.

// On to the code...

// First we import the clap macros
#[macro_use]
extern crate clap;
use clap::{App, SubCommand};

// Next we declare the "keys"/"subcommands" we'll use to access the arguments. Notice, we use
// lowercase, non-camell-case variants so that users don't have to type "Clone" to access this
// subcommand. We'll need to use the allow(non_camel_case_types) to silence the warning. In another
// example we'll see how to get arounds this, and keep the compiler happy.
//
// NOTE: The enums support use of `pub` or `#[meta]` style attributes (like derive(), allow(), or
// cfg() attributes), or any combinations of those two elements
//
// NOTE 2: Just like the args! macro, one can declare multiple enums inside the same args! block
subcommands!{
    #[allow(non_camel_case_types)]
    enum Git {
        clone,
        push,
        pull
    }
}

fn main() {
    // We declare the App struct like normal. We could also use a use statement such as
    // `use MyProg::*;` to save a few key-strokes, but for this simple example we'll leave
    // that out.
    let m = App::new("git")
        .subcommand(SubCommand::with_name(Git::clone))
        .subcommand(SubCommand::with_name(Git::push))
        .subcommand(SubCommand::with_name(Git::pull))
        .get_matches();

    // Now to access the subcommands we can pattern match against the variants
    //
    // Note, the tuple is (Variant, ArgMatches), but we're not using the ArgMatches here, so we
    // just throw them away with _
    match m.subcommand() {
        Some((Git::clone, _)) => println!("Clone was used"),
        Some((Git::push, _)) => println!("Push was used"),
        Some((Git::pull, _)) => println!("Pull was used"), // <-- commment out this line to see the
                                                           // compiler complain about NonExaustive
                                                           // matches. Yay for no more strings! :)
        None => println!("No subcommand was used... :("),
    }
}
