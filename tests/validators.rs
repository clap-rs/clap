use clap::{App, Arg};

#[cfg(feature = "validators")]
#[cfg(debug_assertions)]
#[test]
#[should_panic = "Argument 'test' has both `validator` and `validator_os` set which is not allowed"]
fn both_validator_and_validator_os() {
    let _ = App::new("test")
        .arg(
            Arg::new("test")
                .validator(|val| val.parse::<u32>().map_err(|e| e.to_string()))
                .validator_os(|val| {
                    val.to_str()
                        .unwrap()
                        .parse::<u32>()
                        .map_err(|e| e.to_string())
                }),
        )
        .try_get_matches_from(&["app", "1"]);
}

#[cfg(feature = "validators")]
#[test]
fn test_validator_msg_newline() {
    let res = App::new("test")
        .arg(Arg::new("test").validator(|val| val.parse::<u32>().map_err(|e| e.to_string())))
        .try_get_matches_from(&["app", "f"]);

    assert!(res.is_err());
    let err = res.unwrap_err();

    assert_eq!(
        err.cause,
        "Invalid value for \'<test>\': invalid digit found in string"
    );

    // This message is the only thing that gets printed -- make sure it ends with a newline
    let msg = format!("{}", err);
    assert!(msg.ends_with('\n'));
}
