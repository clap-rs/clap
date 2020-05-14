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
