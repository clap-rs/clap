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
                        .multiple(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .about("view package information (-ii for backup files)")
                        .multiple(true),
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
                        .about("search remote repositories for matching strings")
                        .multiple(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .about("view package information (-ii for extended information)")
                        .multiple(true),
                )
                .arg(
                    Arg::new("package")
                        .about("package")
                        .multiple(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("sync", Some(sync_matches)) => {
            if sync_matches.is_present("info") {
                // Values required here, so it's safe to unwrap
                let packages: Vec<_> = sync_matches.values_of("info").unwrap().collect();
                let comma_sep = packages.join(", ");
                println!("Retrieving info for {}...", comma_sep);
            } else if sync_matches.is_present("search") {
                // Values required here, so it's safe to unwrap
                let queries: Vec<_> = sync_matches.values_of("search").unwrap().collect();
                let comma_sep = queries.join(", ");
                println!("Searching for {}...", comma_sep);
            } else {
                // Sync was called without any arguments
                match sync_matches.values_of("package") {
                    Some(packages) => {
                        let pkgs: Vec<_> = packages.collect();
                        let comma_sep = pkgs.join(", ");
                        println!("Installing {}...", comma_sep);
                    }
                    None => panic!("No targets specified (use -h for help)"),
                }
            }
        }
        ("query", Some(query_matches)) => {
            if query_matches.is_present("info") {
                // Values required here, so it's safe to unwrap
                let packages: Vec<_> = query_matches.values_of("info").unwrap().collect();
                let comma_sep = packages.join(", ");
                println!("Retrieving info for {}...", comma_sep);
            } else if query_matches.is_present("search") {
                // Values required here, so it's safe to unwrap
                let queries: Vec<_> = query_matches.values_of("search").unwrap().collect();
                let comma_sep = queries.join(", ");
                println!("Searching Locally for {}...", comma_sep);
            } else {
                // Query was called without any arguments
                println!("Displaying all locally installed packages...");
            }
        }
        ("", None) => panic!("error: no operation specified (use -h for help)"), // If no subcommand was used
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
