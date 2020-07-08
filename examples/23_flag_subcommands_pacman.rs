// Working with flag subcommands allows behavior similar to the popular Archlinux package manager Pacman.
// Man page: https://jlk.fjfi.cvut.cz/arch/manpages/man/pacman.8
//
// It's suggested that you read examples/20_subcommands.rs prior to learning about `FlagSubCommand`s
//
// This differs from normal subcommands because it allows passing subcommands in the same fashion as `clap::Arg` in short or long args.
//
//            Top Level App (pacman)                              TOP
//                           |
//    ---------------------------------------------------
//   /     |        |        |         |        \        \
// sync  database remove    files      query   deptest   upgrade  LEVEL 1
//
// Given the above hierachy, valid runtime uses would be (not an all inclusive list):
//
// $ pacman -Ss
//           ^--- subcommand followed by an arg in its scope.
//
// $ pacman -Qs
//
// $ pacman -Rns
//
// NOTE: Subcommands short flags can be uppercase or lowercase.
//
// $ pacman --sync --search
//            ^--- subcommand
//
// $ pacman sync -s
//          ^--- subcommand
//
// NOTE: this isn't valid for pacman, but is done implicitly by Clap which
// adds support for both flags and standard subcommands out of the box.
// Allowing your users to make the choice of what feels more intuitive for them.
//
// Notice only one command per "level" may be used. You could not, for example, do:
//
// $ pacman -SQR
//
// It's also important to know that subcommands each have their own set of matches and may have args
// with the same name as other subcommands in a different part of the tree heirachy (i.e. the arg
// names aren't in a flat namespace).
//
// In order to use subcommands in clap, you only need to know which subcommand you're at in your
// tree, and which args are defined on that subcommand.
//
// Let's make a quick program to illustrate. We'll be using the same example as above but for
// brevity sake we won't implement all of the subcommands, only a few.
use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("pacman")
        .about("package manager utility")
        .version("5.2.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Pacman Development Team")
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            App::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .about("search locally-installed packages for matching strings")
                        .conflicts_with("info")
                        .multiple_values(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .about("view package information")
                        .multiple_values(true),
                ),
        )
        // Sync subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            App::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Synchronize packages.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true)
                        .about("search remote repositories for matching strings"),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .conflicts_with("search")
                        .short('i')
                        .about("view package information"),
                )
                .arg(
                    Arg::new("package")
                        .about("packages")
                        .multiple(true)
                        .required_unless_one(&["search"])
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("sync", Some(sync_matches)) => {
            if sync_matches.is_present("search") {
                let packages: Vec<_> = sync_matches.values_of("search").unwrap().collect();
                let values = packages.join(", ");
                println!("Searching for {}...", values);
                return;
            }

            let packages: Vec<_> = sync_matches.values_of("package").unwrap().collect();
            let values = packages.join(", ");

            if sync_matches.is_present("info") {
                println!("Retrieving info for {}...", values);
            } else {
                println!("Installing {}...", values);
            }
        }
        ("query", Some(query_matches)) => {
            if let Some(packages) = query_matches.values_of("info") {
                let comma_sep = packages.collect::<Vec<_>>().join(", ");
                println!("Retrieving info for {}...", comma_sep);
            } else if let Some(queries) = query_matches.values_of("search") {
                let comma_sep = queries.collect::<Vec<_>>().join(", ");
                println!("Searching Locally for {}...", comma_sep);
            } else {
                println!("Displaying all locally installed packages...");
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
