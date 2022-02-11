#![allow(deprecated)]
use clap::{Arg, ArgSettings};

#[test]
fn setting() {
    let m = Arg::new("setting").setting(ArgSettings::Required);
    assert!(m.is_required_set());
}

#[test]
fn unset_setting() {
    let m = Arg::new("unset_setting").setting(ArgSettings::Required);
    assert!(m.is_required_set());

    let m = m.unset_setting(ArgSettings::Required);
    assert!(!m.is_required_set(), "{:#?}", m);
}

#[test]
fn setting_bitor() {
    let m = Arg::new("setting_bitor")
        .setting(ArgSettings::Required | ArgSettings::Hidden | ArgSettings::Last);

    assert!(m.is_required_set());
    assert!(m.is_hide_set());
    assert!(m.is_last_set());
}

#[test]
fn unset_setting_bitor() {
    let m = Arg::new("unset_setting_bitor")
        .setting(ArgSettings::Required)
        .setting(ArgSettings::Hidden)
        .setting(ArgSettings::Last);

    assert!(m.is_required_set());
    assert!(m.is_hide_set());
    assert!(m.is_last_set());

    let m = m.unset_setting(ArgSettings::Required | ArgSettings::Hidden | ArgSettings::Last);
    assert!(!m.is_required_set(), "{:#?}", m);
    assert!(!m.is_hide_set(), "{:#?}", m);
    assert!(!m.is_last_set(), "{:#?}", m);
}
