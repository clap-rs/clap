use std::process::exit;

use clap::{App, AppSettings, Arg};

fn applet_commands() -> [App<'static>; 2] {
    [
        App::new("true").about("does nothing successfully"),
        App::new("false").about("does nothing unsuccessfully"),
    ]
}

fn main() {
    let app = App::new(env!("CARGO_CRATE_NAME"))
        .setting(AppSettings::Multicall)
        .subcommand(
            App::new("busybox")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand_value_name("APPLET")
                .subcommand_help_heading("APPLETS")
                .arg(
                    Arg::new("install")
                        .long("install")
                        .help("Install hardlinks for all subcommands in path")
                        .exclusive(true)
                        .takes_value(true)
                        .default_missing_value("/usr/local/bin")
                        .use_delimiter(false),
                )
                .subcommands(applet_commands()),
        )
        .subcommands(applet_commands());

    let matches = app.get_matches();
    let mut subcommand = matches.subcommand();
    if let Some(("busybox", cmd)) = subcommand {
        if cmd.occurrences_of("install") > 0 {
            unimplemented!("Make hardlinks to the executable here");
        }
        subcommand = cmd.subcommand();
    }
    match subcommand {
        Some(("false", _)) => exit(1),
        Some(("true", _)) => exit(0),
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }
}
