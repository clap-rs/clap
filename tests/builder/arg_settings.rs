use clap::{Arg, ArgSettings};

#[test]
fn setting() {
    let m = Arg::new("setting").setting(ArgSettings::Required);
    assert!(m.is_set(ArgSettings::Required));
}

#[test]
fn unset_setting() {
    let m = Arg::new("unset_setting").setting(ArgSettings::Required);
    assert!(m.is_set(ArgSettings::Required));

    let m = m.unset_setting(ArgSettings::Required);
    assert!(!m.is_set(ArgSettings::Required), "{:#?}", m);
}

#[test]
fn setting_bitor() {
    let m = Arg::new("setting_bitor")
        .setting(ArgSettings::Required | ArgSettings::Hidden | ArgSettings::Last);

    assert!(m.is_set(ArgSettings::Required));
    assert!(m.is_set(ArgSettings::Hidden));
    assert!(m.is_set(ArgSettings::Last));
}

#[test]
fn unset_setting_bitor() {
    let m = Arg::new("unset_setting_bitor")
        .setting(ArgSettings::Required)
        .setting(ArgSettings::Hidden)
        .setting(ArgSettings::Last);

    assert!(m.is_set(ArgSettings::Required));
    assert!(m.is_set(ArgSettings::Hidden));
    assert!(m.is_set(ArgSettings::Last));

    let m = m.unset_setting(ArgSettings::Required | ArgSettings::Hidden | ArgSettings::Last);
    assert!(!m.is_set(ArgSettings::Required), "{:#?}", m);
    assert!(!m.is_set(ArgSettings::Hidden), "{:#?}", m);
    assert!(!m.is_set(ArgSettings::Last), "{:#?}", m);
}
