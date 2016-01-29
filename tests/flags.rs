extern crate clap;

use clap::{App, Arg, ArgSettings};

#[test]
fn flag_using_short() {
    let m = App::new("flag")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::from_usage("-c, --color 'some other flag'")
            ])
        .get_matches_from(vec!["", "-f", "-c"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn flag_using_long() {
    let m = App::new("flag")
        .args(&[
            Arg::from_usage("--flag 'some flag'"),
            Arg::from_usage("--color 'some other flag'")
            ])
        .get_matches_from(vec!["", "--flag", "--color"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn flag_using_mixed() {
    let m = App::new("flag")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::from_usage("-c, --color 'some other flag'")
            ])
        .get_matches_from(vec!["", "-f", "--color"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));

    let m = App::new("flag")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::from_usage("-c, --color 'some other flag'")
            ])
        .get_matches_from(vec!["", "--flag", "-c"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
}

#[test]
fn multiple_flags_in_single() {
    let m = App::new("multe_flags")
        .args(&[
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::from_usage("-c, --color 'some other flag'"),
            Arg::from_usage("-d, --debug 'another other flag'")
            ])
        .get_matches_from(vec!["", "-fcd"]);
    assert!(m.is_present("flag"));
    assert!(m.is_present("color"));
    assert!(m.is_present("debug"));
}

#[test]
fn short_flag_misspel() {
    let a = Arg::from_usage("-f1, --flag 'some flag'");
    assert_eq!(a.name, "flag");
    assert_eq!(a.short.unwrap(), 'f');
    assert_eq!(a.long.unwrap(), "flag");
    assert_eq!(a.help.unwrap(), "some flag");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());
}

#[test]
fn short_flag_name_missing() {
    let a = Arg::from_usage("-f 'some flag'");
    assert_eq!(a.name, "f");
    assert_eq!(a.short.unwrap(), 'f');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some flag");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

}

#[test]
fn create_flag() {
    let _ = App::new("test")
                .arg(Arg::with_name("test")
                            .short("t")
                            .long("test")
                            .help("testing testing"))
                .get_matches();
}

#[test]
fn create_flag_usage() {
    let a = Arg::from_usage("[flag] -f 'some help info'");
    assert_eq!(a.name, "flag");
    assert_eq!(a.short.unwrap(), 'f');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("[flag] --flag 'some help info'");
    assert_eq!(b.name, "flag");
    assert_eq!(b.long.unwrap(), "flag");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.is_set(ArgSettings::Multiple));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("--flag 'some help info'");
    assert_eq!(b.name, "flag");
    assert_eq!(b.long.unwrap(), "flag");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.is_set(ArgSettings::Multiple));
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("[flag] -f --flag 'some help info'");
    assert_eq!(c.name, "flag");
    assert_eq!(c.short.unwrap(), 'f');
    assert_eq!(c.long.unwrap(), "flag");
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.is_set(ArgSettings::Multiple));
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("[flag] -f... 'some help info'");
    assert_eq!(d.name, "flag");
    assert_eq!(d.short.unwrap(), 'f');
    assert!(d.long.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.is_set(ArgSettings::Multiple));
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

    let e = Arg::from_usage("[flag] -f --flag... 'some help info'");
    assert_eq!(e.name, "flag");
    assert_eq!(e.long.unwrap(), "flag");
    assert_eq!(e.short.unwrap(), 'f');
    assert_eq!(e.help.unwrap(), "some help info");
    assert!(e.is_set(ArgSettings::Multiple));
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("-f --flag... 'some help info'");
    assert_eq!(e.name, "flag");
    assert_eq!(e.long.unwrap(), "flag");
    assert_eq!(e.short.unwrap(), 'f');
    assert_eq!(e.help.unwrap(), "some help info");
    assert!(e.is_set(ArgSettings::Multiple));
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("--flags");
    assert_eq!(e.name, "flags");
    assert_eq!(e.long.unwrap(), "flags");
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("--flags...");
    assert_eq!(e.name, "flags");
    assert_eq!(e.long.unwrap(), "flags");
    assert!(e.is_set(ArgSettings::Multiple));
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("[flags] -f");
    assert_eq!(e.name, "flags");
    assert_eq!(e.short.unwrap(), 'f');
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("[flags] -f...");
    assert_eq!(e.name, "flags");
    assert_eq!(e.short.unwrap(), 'f');
    assert!(e.is_set(ArgSettings::Multiple));
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let a = Arg::from_usage("-f 'some help info'");
    assert_eq!(a.name, "f");
    assert_eq!(a.short.unwrap(), 'f');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let e = Arg::from_usage("-f");
    assert_eq!(e.name, "f");
    assert_eq!(e.short.unwrap(), 'f');
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());

    let e = Arg::from_usage("-f...");
    assert_eq!(e.name, "f");
    assert_eq!(e.short.unwrap(), 'f');
    assert!(e.is_set(ArgSettings::Multiple));
    assert!(e.val_names.is_none());
    assert!(e.num_vals.is_none());
}
