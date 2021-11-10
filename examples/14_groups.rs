use clap::{App, Arg, ArgGroup};

fn main() {
    // Create application like normal
    let matches = App::new("myapp")
        // Add the version arguments
        .arg("--set-ver [ver] 'set version manually'")
        .arg("--major         'auto inc major'")
        .arg("--minor         'auto inc minor'")
        .arg("--patch         'auto inc patch'")
        // Create a group, make it required, and add the above arguments
        .group(
            ArgGroup::new("vers")
                .required(true)
                .args(&["set-ver", "major", "minor", "patch"]),
        )
        // Arguments can also be added to a group individually, these two arguments
        // are part of the "input" group which is not required
        .arg(Arg::from("[INPUT_FILE] 'some regular input'").group("input"))
        .arg(Arg::from("--spec-in [SPEC_IN] 'some special input argument'").group("input"))
        // Now let's assume we have a -c [config] argument which requires one of
        // (but **not** both) the "input" arguments
        .arg(
            Arg::new("config")
                .short('c')
                .takes_value(true)
                .requires("input"),
        )
        .get_matches();

    // Let's assume the old version 1.2.3
    let mut major = 1;
    let mut minor = 2;
    let mut patch = 3;

    // See if --set-ver was used to set the version manually
    let version = if let Some(ver) = matches.value_of("set-ver") {
        ver.to_string()
    } else {
        // Increment the one requested (in a real program, we'd reset the lower numbers)
        let (maj, min, pat) = (
            matches.is_present("major"),
            matches.is_present("minor"),
            matches.is_present("patch"),
        );
        match (maj, min, pat) {
            (true, _, _) => major += 1,
            (_, true, _) => minor += 1,
            (_, _, true) => patch += 1,
            _ => unreachable!(),
        };
        format!("{}.{}.{}", major, minor, patch)
    };

    println!("Version: {}", version);

    // Check for usage of -c
    if matches.is_present("config") {
        let input = matches
            .value_of("INPUT_FILE")
            .unwrap_or_else(|| matches.value_of("spec-in").unwrap());
        println!(
            "Doing work using input {} and config {}",
            input,
            matches.value_of("config").unwrap()
        );
    }
}
