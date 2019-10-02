/// `ArgGroup`s are a family of related arguments and way for you to say, "Any of these arguments".
/// By placing arguments in a logical group, you can make easier requirement and exclusion rules
/// instead of having to list each individually, or when you want a rule to apply "any but not all"
/// arguments.
///
/// For instance, you can make an entire ArgGroup required, this means that one (and *only* one)
/// argument. from that group must be present. Using more than one argument from an ArgGroup causes
/// a failure (graceful exit).
///
/// You can also do things such as name an ArgGroup as a confliction or requirement, meaning any
/// of the arguments that belong to that group will cause a failure if present, or must present
/// respectively.
///
/// Perhaps the most common use of `ArgGroup`s is to require one and *only* one argument to be
/// present out of a given set. Imagine that you had multiple arguments, and you want one of them to
/// be required, but making all of them required isn't feasible because perhaps they conflict with
/// each other. For example, lets say that you were building an application where one could set a
/// given version number by supplying a string with an option argument, i.e. `--set-ver v1.2.3`, you
/// also wanted to support automatically using a previous version number and simply incrementing one
/// of the three numbers. So you create three flags `--major`, `--minor`, and `--patch`. All of
/// these arguments shouldn't be used at one time but you want to specify that *at least one* of
/// them is used. For this, you can create a group.
extern crate clap;

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
            ArgGroup::with_name("vers")
                .required(true)
                .args(&["ver", "major", "minor", "patch"]),
        )
        // Arguments can also be added to a group individually, these two arguments
        // are part of the "input" group which is not required
        .arg(Arg::from("[INPUT_FILE] 'some regular input'").group("input"))
        .arg(Arg::from("--spec-in [SPEC_IN] 'some special input argument'").group("input"))
        // Now let's assume we have a -c [config] argument which requires one of
        // (but **not** both) the "input" arguments
        .arg(
            Arg::with_name("config")
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
    let version = if let Some(ver) = matches.value_of("ver") {
        format!("{}", ver)
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
            .unwrap_or(matches.value_of("SPEC_IN").unwrap());
        println!(
            "Doing work using input {} and config {}",
            input,
            matches.value_of("config").unwrap()
        );
    }
}
