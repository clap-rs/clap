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
                        .about("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .about("view package information")
                        .takes_value(true)
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
                        .required_unless_present("search")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
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
        Some(("query", query_matches)) => {
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
