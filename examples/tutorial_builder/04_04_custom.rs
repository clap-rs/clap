// Note: this requires the `cargo` feature

use clap::{arg, command, ErrorKind};

fn main() {
    // Create application like normal
    let mut cmd = command!()
        // Add the version arguments
        .arg(arg!(--"set-ver" <VER> "set version manually").required(false))
        .arg(arg!(--major         "auto inc major"))
        .arg(arg!(--minor         "auto inc minor"))
        .arg(arg!(--patch         "auto inc patch"))
        // Arguments can also be added to a group individually, these two arguments
        // are part of the "input" group which is not required
        .arg(arg!([INPUT_FILE] "some regular input"))
        .arg(arg!(--"spec-in" <SPEC_IN> "some special input argument").required(false))
        // Now let's assume we have a -c [config] argument which requires one of
        // (but **not** both) the "input" arguments
        .arg(arg!(config: -c <CONFIG>).required(false));
    let matches = cmd.get_matches_mut();

    // Let's assume the old version 1.2.3
    let mut major = 1;
    let mut minor = 2;
    let mut patch = 3;

    // See if --set-ver was used to set the version manually
    let version = if let Some(ver) = matches.value_of("set-ver") {
        if matches.is_present("major") || matches.is_present("minor") || matches.is_present("patch")
        {
            cmd.error(
                ErrorKind::ArgumentConflict,
                "Can't do relative and absolute version change",
            )
            .exit();
        }
        ver.to_string()
    } else {
        // Increment the one requested (in a real program, we'd reset the lower numbers)
        let (maj, min, pat) = (
            matches.is_present("major"),
            matches.is_present("minor"),
            matches.is_present("patch"),
        );
        match (maj, min, pat) {
            (true, false, false) => major += 1,
            (false, true, false) => minor += 1,
            (false, false, true) => patch += 1,
            _ => {
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    "Can only modify one version field",
                )
                .exit();
            }
        };
        format!("{}.{}.{}", major, minor, patch)
    };

    println!("Version: {}", version);

    // Check for usage of -c
    if matches.is_present("config") {
        let input = matches
            .value_of("INPUT_FILE")
            .or_else(|| matches.value_of("spec-in"))
            .unwrap_or_else(|| {
                cmd.error(
                    ErrorKind::MissingRequiredArgument,
                    "INPUT_FILE or --spec-in is required when using --config",
                )
                .exit()
            });
        println!(
            "Doing work using input {} and config {}",
            input,
            matches.value_of("config").unwrap()
        );
    }
}
