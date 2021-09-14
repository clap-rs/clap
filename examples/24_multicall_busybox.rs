//! Example of a `busybox-style` multicall program
//!
//! `busybox` is a single executable that contains a variety of applets
//! for a fully functional Linux userland.
//!
//! `busybox`-style differs from `hostname`-style in that there is a launcher program
//! which the applets are available as subcommands.
//! i.e. you can use the `cat` command either as a link named `cat`
//! or as `busybox cat`.
//!
//! This behaviour is opted-into by not naming an applet the same as the main program.
//!
//! This is desirable when the launcher program has additional options
//! or it is useful to run the applet without installing a symlink
//! e.g. for testing purposes, or there may already be a command of that name installed.
//!
//! This example omits every command except true and false,
//! which are the most trivial to implement,
//! but includes the `--install` option as an example of why it can be useful
//! for the main program to take arguments that aren't applet subcommands.

use std::{
    env::args_os,
    fs::{hard_link, read_link},
    path::{Path, PathBuf},
    process::exit,
};

use clap::{App, AppSettings, Arg};

fn main() {
    let mut app = App::new(env!("CARGO_CRATE_NAME"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::Multicall)
        .arg(
            Arg::new("install")
                .long("install")
                .about("Install hardlinks for all subcommands in path")
                .exclusive(true)
                .takes_value(true)
                .default_missing_value("/usr/local/bin")
                .use_delimiter(false),
        )
        .subcommand(App::new("true").about("does nothing successfully"))
        .subcommand(App::new("false").about("does nothing unsuccessfully"));
    let matches = app.get_matches_mut();
    if matches.occurrences_of("install") > 0 {
        let exec_path = read_link("/proc/self/exe")
            .ok()
            .or_else(|| {
                args_os().next().and_then(|s| {
                    let p: &Path = s.as_ref();
                    if p.is_absolute() {
                        Some(PathBuf::from(s))
                    } else {
                        None
                    }
                })
            })
            .expect(
                "Should be able to read /proc/self/exe or argv0 should be present and absolute",
            );
        let mut dest = PathBuf::from(matches.value_of("install").unwrap());
        for applet in app.get_subcommands().map(|c| c.get_name()) {
            dest.push(applet);
            hard_link(&exec_path, &dest).expect("Should be able to hardlink");
            dest.pop();
        }
        exit(0);
    }

    exit(match matches.subcommand_name() {
        Some("true") => 0,
        Some("false") => 1,
        _ => 127,
    })
}
