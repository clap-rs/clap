use super::{settings::ArgSettings, Arg};

#[test]
fn short_flag_misspel() {
    let a = Arg::from("-f1, --flag 'some flag'");
    assert_eq!(a.name, "flag");
    assert_eq!(a.short.unwrap(), 'f');
    assert_eq!(a.long.unwrap(), "flag");
    assert_eq!(a.help.unwrap(), "some flag");
    assert!(!a.is_set(ArgSettings::MultipleOccurrences));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());
}

#[test]
fn short_flag_name_missing() {
    let a = Arg::from("-f 'some flag'");
    assert_eq!(a.name, "f");
    assert_eq!(a.short.unwrap(), 'f');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some flag");
    assert!(!a.is_set(ArgSettings::MultipleOccurrences));
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());
}
