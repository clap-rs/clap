use super::{settings::ArgSettings, Arg};

#[test]
fn short_flag_misspell() {
    let a = Arg::from_usage("-f1, --flag 'some flag'");
    assert_eq!(a.name, "flag");
    assert_eq!(a.short.unwrap(), 'f');
    assert_eq!(a.long.unwrap(), "flag");
    assert_eq!(a.help.unwrap(), "some flag");
    assert!(!a.is_set(ArgSettings::MultipleOccurrences));
    assert!(a.val_names.is_empty());
    assert!(a.num_vals.is_none());
}

#[test]
fn short_flag_name_missing() {
    let a = Arg::from_usage("-f 'some flag'");
    assert_eq!(a.name, "f");
    assert_eq!(a.short.unwrap(), 'f');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some flag");
    assert!(!a.is_set(ArgSettings::MultipleOccurrences));
    assert!(a.val_names.is_empty());
    assert!(a.num_vals.is_none());
}

// This test will *fail to compile* if Arg is not Send + Sync
#[test]
fn arg_send_sync() {
    fn foo<T: Send + Sync>(_: T) {}
    foo(Arg::new("test"))
}
