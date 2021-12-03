use clap::{App, Arg};

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

#[test]
fn test_validator_fromstr_trait() {
    use std::str::FromStr;

    let matches = App::new("test")
        .arg(Arg::new("from_str").validator(u32::from_str))
        .try_get_matches_from(&["app", "1234"])
        .expect("match failed");

    assert_eq!(matches.value_of_t::<u32>("from_str").ok(), Some(1234));
}

#[test]
fn test_validator_msg_newline() {
    let res = App::new("test")
        .arg(Arg::new("test").validator(|val| val.parse::<u32>().map_err(|e| e.to_string())))
        .try_get_matches_from(&["app", "f"]);

    assert!(res.is_err());
    let err = res.unwrap_err();

    assert!(
        err.to_string()
            .contains("Invalid value for '<test>': invalid digit found in string"),
        "{}",
        err
    );

    // This message is the only thing that gets printed -- make sure it ends with a newline
    let msg = format!("{}", err);
    assert!(msg.ends_with('\n'));
}

#[test]
fn stateful_validator() {
    let mut state = false;
    App::new("test")
        .arg(Arg::new("test").validator(|val| {
            state = true;
            val.parse::<u32>().map_err(|e| e.to_string())
        }))
        .try_get_matches_from(&["app", "10"])
        .unwrap();

    assert!(state);
}
