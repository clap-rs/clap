use clap::Clap;

fn non_negative(x: &str) -> Result<(), String> {
    if x.trim_start().starts_with('-') {
        Err(String::from("the value must be non-negative"))
    } else {
        Ok(())
    }
}

#[derive(Clap, Debug)]
#[clap(name = "basic")]
struct Opt {
    /// Set speed to a non-negative value.
    #[clap(short, long, validator = non_negative)]
    speed: f64,
}

#[test]
fn use_validator() {
    let opt = Opt::parse_from(&["test", "--speed=2.0"]);
    assert_eq!(opt.speed, 2.0);

    let err = Opt::try_parse_from(&["test", "--speed=-2.0"]).expect_err("validator should fail");
    assert!(err
        .to_string()
        .contains("error: Invalid value for \'--speed <speed>\': the value must be non-negative"));

    let err = Opt::try_parse_from(&["test", "--speed=bogus"]).expect_err("parsing should fail");
    assert!(err.to_string().contains("invalid float literal"));
}
