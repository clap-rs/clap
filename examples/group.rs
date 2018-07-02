// A functional translation of the example at
// https://docs.rs/clap/2.31.2/clap/struct.App.html#method.group

#[macro_use]
extern crate clap;

use clap::{ArgGroup, Clap};

// This function is not needed, we can insert everything in the group
// attribute, but, as it might be long, using a function is more
// lisible.
fn vers_arg_group() -> ArgGroup<'static> {
    // As the attributes of the struct are executed before the struct
    // fields, we can't use .args(...), but we can use the group
    // attribute on the fields.
    ArgGroup::with_name("vers").required(true)
}

#[derive(Clap, Debug)]
#[clap(raw(group = "vers_arg_group()"))]
struct Opt {
    /// set the version manually
    #[clap(long = "set-ver", group = "vers")]
    set_ver: Option<String>,
    /// auto increase major
    #[clap(long = "major", group = "vers")]
    major: bool,
    /// auto increase minor
    #[clap(long = "minor", group = "vers")]
    minor: bool,
    /// auto increase patch
    #[clap(long = "patch", group = "vers")]
    patch: bool,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
