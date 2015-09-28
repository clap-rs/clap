extern crate clap;

use clap::{App, ArgGroup, ClapErrorType};

#[test]
fn required_group_missing_arg() {
    let result = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color 'some other flag'")
        .arg_group(ArgGroup::with_name("req")
            .add_all(&["flag", "color"])
            .required(true))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.error_type, ClapErrorType::MissingRequiredArgument);
}

#[test]
fn group_single_value() {
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color] 'some option'")
        .arg_group(ArgGroup::with_name("grp")
            .add_all(&["flag", "color"]))
        .get_matches_from(vec!["", "-c", "blue"]);
    assert!(m.is_present("grp"));
    assert_eq!(m.value_of("grp").unwrap(), "blue");
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color] 'some option'")
        .arg_group(ArgGroup::with_name("grp")
            .add_all(&["flag", "color"]))
        .get_matches_from(vec!["", "-f"]);
    assert!(m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color] 'some option'")
        .arg_group(ArgGroup::with_name("grp")
            .add_all(&["flag", "color"]))
        .get_matches_from(vec![""]);
    assert!(!m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_multi_value_single_arg() {
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color]... 'some option'")
        .arg_group(ArgGroup::with_name("grp")
            .add_all(&["flag", "color"]))
        .get_matches_from(vec!["", "-c", "blue", "red", "green"]);
    assert!(m.is_present("grp"));
    assert_eq!(m.values_of("grp").unwrap(), &["blue", "red", "green"]);
}
