#[test]
fn insert() {
    let mut raw = clap_lex::RawArgs::new(["bin", "a", "b", "c"]);
    let mut cursor = raw.cursor();

    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("bin")));
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("a")));
    raw.insert(&cursor, ["1", "2", "3"]);
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("1")));
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("2")));
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("3")));
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("b")));
    assert_eq!(raw.next_os(&mut cursor), Some(std::ffi::OsStr::new("c")));

    let mut cursor = raw.cursor();
    let rest = raw
        .remaining(&mut cursor)
        .map(|s| s.to_string_lossy())
        .collect::<Vec<_>>();
    assert_eq!(rest, vec!["bin", "a", "1", "2", "3", "b", "c"]);
}
