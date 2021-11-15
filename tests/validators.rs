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

#[test]
fn validator_all() {
    App::new("test")
        .arg(
            Arg::new("test")
                .short('d')
                .takes_value(true)
                .multiple_values(true)
                .validator_all(|vals| {
                    if vals.len() % 3 == 0 {
                        Ok(())
                    } else {
                        Err("not % 3!")
                    }
                }),
        )
        .try_get_matches_from(&["app", "-d", "f", "f", "f"])
        .unwrap();
}

#[test]
fn validator_all_and_validator() {
    let app = App::new("test").arg(
        Arg::new("test")
            .short('d')
            .takes_value(true)
            .multiple_values(true)
            .validator_all(|vals| {
                if vals.len() % 3 == 0 {
                    Ok(())
                } else {
                    Err("not % 3!")
                }
            })
            .validator(|val| val.parse::<u32>().map_err(|e| e.to_string())),
    );

    app.clone()
        .try_get_matches_from(&["app", "-d", "10", "0", "10"])
        .unwrap();
    assert!(app
        .try_get_matches_from(&["app", "-d", "a", "0", "10"])
        .is_err());
}

#[test]
fn validator_all_os_error() {
    let res = App::new("test")
        .arg(
            Arg::new("test")
                .short('d')
                .takes_value(true)
                .multiple_values(true)
                .validator_all_os(|vals| {
                    if vals.len() % 3 == 0 {
                        Ok(())
                    } else {
                        Err(format!("not % 3 == 0, == {}!", vals.len() % 3))
                    }
                }),
        )
        .try_get_matches_from(&["app", "-d", "f", "f", "f", "f"]);

    assert!(res.is_err());
    let err = res.unwrap_err();

    eprintln!("{}", err.to_string());

    assert!(err
        .to_string()
        .contains("Invalid values for '-d <test>...': not % 3 == 0, == 1!"));
}
