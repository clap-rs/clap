// We intentionally don't import `clap_app!` here; not having it in scope protects against the
// class of errors where the macro refers to itself as `clap_app!` instead of `$crate::clap_app!`
use clap::ErrorKind;

#[test]
fn basic() {
    #![allow(deprecated)]
    clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (@global_setting AllowNegativeNumbers)
        (@arg opt: -o --option +takes_value ... "tests options")
        (@arg positional: index(1) "tests positionals")
        (@arg flag: -f --flag ... +global "tests flags")
        (@arg flag2: -F conflicts_with[flag] requires[option2]
            "tests flags with exclusions")
        (@arg option2: --long_option_2 conflicts_with[option] requires[positional2]
            "tests long options with exclusions")
        (@arg positional2: index(2) "tests positionals with exclusions")
        (@arg option3: -O --Option +takes_value possible_value[fast slow]
            "tests options with specific value sets")
        (@arg positional3: index(3) ... possible_value[vi emacs]
            "tests positionals with specific values")
        (@arg multvals: --multvals +takes_value value_name[one two]
            "Tests multiple values, not mult occs")
        (@arg multvalsmo: --multvalsmo ... +takes_value value_name[one two]
            "Tests multiple values, not mult occs")
        (@arg minvals: --minvals2 min_values(1) ... +takes_value "Tests 2 min vals")
        (@arg maxvals: --maxvals3 ... +takes_value max_values(3) "Tests 3 max vals")
        (@subcommand subcmd =>
            (about: "tests subcommands")
            (version: "0.1")
            (author: "Kevin K. <kbknapp@gmail.com>")
            (@arg scoption: -o --option ... +takes_value "tests options")
            (@arg scpositional: index(1) "tests positionals"))
    );
}

#[test]
fn quoted_app_name() {
    #![allow(deprecated)]
    let mut app = clap::clap_app!(("app name with spaces-and-hyphens") =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (@arg option: -o --option +takes_value ... "tests options")
        (@arg positional: index(1) "tests positionals")
        (@arg flag: -f --flag ... +global "tests flags")
        (@arg flag2: -F conflicts_with[flag] requires[option2]
            "tests flags with exclusions")
        (@arg option2: --long_option_2 conflicts_with[option] requires[positional2]
            "tests long options with exclusions")
        (@arg positional2: index(2) "tests positionals with exclusions")
        (@arg option3: -O --Option +takes_value possible_value[fast slow]
            "tests options with specific value sets")
        (@arg positional3: index(3) ... possible_value[vi emacs]
            "tests positionals with specific values")
        (@arg multvals: --multvals +takes_value value_name[one two]
            "Tests multiple values, not mult occs")
        (@arg multvalsmo: --multvalsmo ... +takes_value value_name[one two]
            "Tests multiple values, not mult occs")
        (@arg minvals: --minvals2 min_values(1) ... +takes_value "Tests 2 min vals")
        (@arg maxvals: --maxvals3 ... +takes_value max_values(3) "Tests 3 max vals")
        (@subcommand subcmd =>
            (about: "tests subcommands")
            (version: "0.1")
            (author: "Kevin K. <kbknapp@gmail.com>")
            (@arg scoption: -o --option ... +takes_value "tests options")
            (@arg scpositional: index(1) "tests positionals"))
    );

    assert_eq!(app.get_name(), "app name with spaces-and-hyphens");

    let mut help_text = vec![];
    app.write_help(&mut help_text)
        .expect("Could not write help text.");
    let help_text = String::from_utf8(help_text).expect("Help text is not valid utf-8");
    assert!(help_text.starts_with("app name with spaces-and-hyphens 0.1\n"));
}

#[test]
fn quoted_arg_long_name() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (@arg option: -o --option +takes_value ... "tests options")
        (@arg positional: index(1) "tests positionals")
        (@arg flag: -f --flag ... +global "tests flags")
        (@arg flag2: -F conflicts_with[flag] requires[option2]
            "tests flags with exclusions")
        (@arg option2: --("long-option-2") conflicts_with[option] requires[positional2]
            "tests long options with exclusions")
        (@arg positional2: index(2) "tests positionals with exclusions")
        (@arg option3: -O --Option +takes_value possible_value[fast slow]
            "tests options with specific value sets")
        (@arg positional3: index(3) ... possible_value[vi emacs]
            "tests positionals with specific values")
        (@arg multvals: --multvals +takes_value value_name[one two]
            "Tests multiple values, not mult occs")
        (@arg multvalsmo: --multvalsmo ... +takes_value value_name[one two]
            "Tests multiple values, not mult occs")
        (@arg minvals: --minvals2 min_values(1) ... +takes_value "Tests 2 min vals")
        (@arg maxvals: --maxvals3 ... +takes_value max_values(3) "Tests 3 max vals")
        (@subcommand subcmd =>
            (about: "tests subcommands")
            (version: "0.1")
            (author: "Kevin K. <kbknapp@gmail.com>")
            (@arg scoption: -o --option ... +takes_value "tests options")
            (@arg scpositional: index(1) "tests positionals"))
    );

    let matches = app
        .try_get_matches_from(vec!["bin_name", "value1", "value2", "--long-option-2"])
        .expect("Expected to successfully match the given args.");
    assert!(matches.is_present("option2"));
}

#[test]
fn quoted_arg_name() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (@arg option: -o --option +takes_value ... "tests options")
        (@arg ("positional-arg"): index(1) "tests positionals")
        (@arg flag: -f --flag ... +global "tests flags")
        (@arg flag2: -F conflicts_with[flag] requires[option2]
            "tests flags with exclusions")
        (@arg option2: --("long-option-2") conflicts_with[option] requires[positional2]
            "tests long options with exclusions")
        (@arg positional2: index(2) "tests positionals with exclusions")
        (@arg option3: -O --Option +takes_value possible_value[fast slow]
            "tests options with specific value sets")
        (@arg ("positional-3"): index(3) ... possible_value[vi emacs]
            "tests positionals with specific values")
        (@arg multvals: --multvals +takes_value value_name[one two]
            "Tests multiple values, not mult occs")
        (@arg multvalsmo: --multvalsmo ... +takes_value value_name[one two]
            "Tests multiple values, not mult occs")
        (@arg minvals: --minvals2 min_values(1) ... +takes_value "Tests 2 min vals")
        (@arg maxvals: --maxvals3 ... +takes_value max_values(3) "Tests 3 max vals")
        (@subcommand subcmd =>
            (about: "tests subcommands")
            (version: "0.1")
            (author: "Kevin K. <kbknapp@gmail.com>")
            (@arg scoption: -o --option ... +takes_value "tests options")
            (@arg scpositional: index(1) "tests positionals"))
    );

    let matches = app
        .try_get_matches_from(vec!["bin_name", "value1", "value2", "--long-option-2"])
        .expect("Expected to successfully match the given args.");
    assert!(matches.is_present("option2"));
}

#[test]
fn quoted_subcommand_name() {
    #![allow(deprecated)]
    clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (@arg opt: -o --option +takes_value ... "tests options")
        (@arg positional: index(1) "tests positionals")
        (@arg flag: -f --flag ... +global "tests flags")
        (@arg flag2: -F conflicts_with[flag] requires[option2]
            "tests flags with exclusions")
        (@arg option2: --long_option_2 conflicts_with[option] requires[positional2]
            "tests long options with exclusions")
        (@arg positional2: index(2) "tests positionals with exclusions")
        (@arg option3: -O --Option +takes_value possible_value[fast slow]
            "tests options with specific value sets")
        (@arg positional3: index(3) ... possible_value[vi emacs]
            "tests positionals with specific values")
        (@arg multvals: --multvals +takes_value value_name[one two]
            "Tests multiple values, not mult occs")
        (@arg multvalsmo: --multvalsmo ... +takes_value value_name[one two]
            "Tests multiple values, not mult occs")
        (@arg minvals: --minvals2 min_values(1) ... +takes_value "Tests 2 min vals")
        (@arg maxvals: --maxvals3 ... +takes_value max_values(3) "Tests 3 max vals")
        (@subcommand subcmd =>
            (about: "tests subcommands")
            (version: "0.1")
            (author: "Kevin K. <kbknapp@gmail.com>")
            (@arg scoption: -o --option ... +takes_value "tests options")
            (@arg scpositional: index(1) "tests positionals"))
        (@subcommand ("other-subcmd") =>
            (about: "some other subcommand"))
    );
}

#[test]
fn group_macro() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty:
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = app.try_get_matches_from(vec!["bin_name", "--hard"]);
    assert!(result.is_ok());
    let matches = result.expect("Expected to successfully match the given args.");
    assert!(matches.is_present("difficulty"));
    assert!(matches.is_present("hard"));
}

#[test]
fn group_macro_set_multiple() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty: +multiple
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = app.try_get_matches_from(vec!["bin_name", "--hard", "--easy"]);
    assert!(result.is_ok());
    let matches = result.expect("Expected to successfully match the given args.");
    assert!(matches.is_present("difficulty"));
    assert!(matches.is_present("hard"));
    assert!(matches.is_present("easy"));
    assert!(!matches.is_present("normal"));
}

#[test]
fn group_macro_set_not_multiple() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty: !multiple
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = app.try_get_matches_from(vec!["bin_name", "--hard", "--easy"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn group_macro_set_required() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty: +required
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = app.try_get_matches_from(vec!["bin_name"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_macro_set_not_required() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty: !required
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = app.try_get_matches_from(vec!["bin_name"]);
    assert!(result.is_ok());
    let matches = result.expect("Expected to successfully match the given args.");
    assert!(!matches.is_present("difficulty"));
}

#[test]
fn group_macro_attributes_alternative() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty:
                 (@attributes +multiple +required)
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = app
        .clone()
        .try_get_matches_from(vec!["bin_name", "--hard", "--easy"]);
    assert!(result.is_ok());
    let matches = result.expect("Expected to successfully match the given args.");
    assert!(matches.is_present("difficulty"));
    assert!(matches.is_present("hard"));
    assert!(matches.is_present("easy"));
    assert!(!matches.is_present("normal"));

    let result = app.try_get_matches_from(vec!["bin_name"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_macro_multiple_methods() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty: +multiple +required
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = app
        .clone()
        .try_get_matches_from(vec!["bin_name", "--hard", "--easy"]);
    assert!(result.is_ok());
    let matches = result.expect("Expected to successfully match the given args.");
    assert!(matches.is_present("difficulty"));
    assert!(matches.is_present("hard"));
    assert!(matches.is_present("easy"));
    assert!(!matches.is_present("normal"));

    let result = app.try_get_matches_from(vec!["bin_name"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_macro_multiple_methods_alternative() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty: * ...
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = app
        .clone()
        .try_get_matches_from(vec!["bin_name", "--hard", "--easy"]);
    assert!(result.is_ok());
    let matches = result.expect("Expected to successfully match the given args.");
    assert!(matches.is_present("difficulty"));
    assert!(matches.is_present("hard"));
    assert!(matches.is_present("easy"));
    assert!(!matches.is_present("normal"));

    let result = app.try_get_matches_from(vec!["bin_name"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_macro_multiple_invocations() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (@arg foo: --foo)
        (@arg bar: --bar)
             (@group difficulty: conflicts_with[foo bar]
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = app
        .clone()
        .try_get_matches_from(vec!["bin_name", "--hard", "--foo"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);

    let result = app.try_get_matches_from(vec!["bin_name", "--hard", "--bar"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind, ErrorKind::ArgumentConflict);
}

#[test]
fn literals() {
    #![allow(deprecated)]
    clap::clap_app!("clap-tests" =>
        (version: "0.1")
        (@arg "task-num": -"t-n" --"task-num" +takes_value possible_value["all" 0 1 2]
            "Task number")
        (@group priority:
            (@arg "4": -4 --4 "Sets priority to 4")
            (@arg ("5"): -('5') --5 "Sets priority to 5")
            (@arg 6: -6 --6 "Sets priority to 6")
        )
        (@subcommand "view-tasks" =>
            (about: "View all tasks"))
        (@subcommand 0 =>
            (about: "Set everything to zero priority"))
    );
}

#[test]
fn multiarg() {
    #![allow(deprecated)]
    let app = clap::clap_app!(claptests =>
        (@arg flag: --flag "value")
        (@arg multiarg: --multiarg
            default_value("flag-unset") default_value_if("flag", None, Some("flag-set"))
            "multiarg")
        (@arg multiarg2: --multiarg2
            default_value("flag-unset") default_value_if("flag", None, Some("flag-set"))
            "multiarg2")
    );

    let matches = app
        .clone()
        .try_get_matches_from(vec!["bin_name"])
        .expect("match failed");

    assert_eq!(matches.value_of("multiarg"), Some("flag-unset"));
    assert_eq!(matches.value_of("multiarg2"), Some("flag-unset"));

    let matches = app
        .try_get_matches_from(vec!["bin_name", "--flag"])
        .expect("match failed");

    assert_eq!(matches.value_of("multiarg"), Some("flag-set"));
    assert_eq!(matches.value_of("multiarg2"), Some("flag-set"));
}

#[test]
fn validator() {
    #![allow(deprecated)]

    use std::str::FromStr;

    fn validate(val: &str) -> Result<u32, String> {
        val.parse::<u32>().map_err(|e| e.to_string())
    }

    let app = clap::clap_app!(claptests =>
        (@arg inline: { |val| val.parse::<u16>() })
        (@arg func1: { validate })
        (@arg func2: { u64::from_str })
    );

    let matches = app
        .try_get_matches_from(&["bin", "12", "34", "56"])
        .expect("match failed");

    assert_eq!(matches.value_of_t::<u16>("inline").ok(), Some(12));
    assert_eq!(matches.value_of_t::<u16>("func1").ok(), Some(34));
    assert_eq!(matches.value_of_t::<u16>("func2").ok(), Some(56));
}

mod arg {
    #[test]
    fn name_explicit() {
        let arg = clap::arg!(foo: --bar <NUM>);
        assert_eq!(arg.get_name(), "foo");
        assert_eq!(arg.get_long(), Some("bar"));
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(arg.is_set(clap::ArgSettings::Required));
    }

    #[test]
    fn name_from_long() {
        let arg = clap::arg!(--bar <NUM>);
        assert_eq!(arg.get_name(), "bar");
        assert_eq!(arg.get_long(), Some("bar"));
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(arg.is_set(clap::ArgSettings::Required));
    }

    #[test]
    fn name_from_value() {
        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_name(), "NUM");
        assert_eq!(arg.get_long(), None);
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(arg.is_set(clap::ArgSettings::Required));
    }

    #[test]
    #[should_panic]
    fn name_none_fails() {
        clap::arg!("Help");
    }

    #[test]
    #[should_panic]
    fn short_only_fails() {
        clap::arg!(-b);
    }

    #[test]
    fn short() {
        let arg = clap::arg!(foo: -b);
        assert_eq!(arg.get_name(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(!arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -'b');
        assert_eq!(arg.get_name(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(!arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b ...);
        assert_eq!(arg.get_name(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(!arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b "How to use it");
        assert_eq!(arg.get_name(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(!arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), Some("How to use it"));
    }

    #[test]
    fn short_and_long() {
        let arg = clap::arg!(foo: -b --hello);
        assert_eq!(arg.get_name(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(!arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -'b' --hello);
        assert_eq!(arg.get_name(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(!arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b --hello ...);
        assert_eq!(arg.get_name(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(!arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b --hello "How to use it");
        assert_eq!(arg.get_name(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(!arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), Some("How to use it"));
    }

    #[test]
    fn positional() {
        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_name(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!([NUM]);
        assert_eq!(arg.get_name(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(!arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_name(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: <NUM>);
        assert_eq!(arg.get_name(), "foo");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(<NUM> ...);
        assert_eq!(arg.get_name(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(<NUM> "How to use it");
        assert_eq!(arg.get_name(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(!arg.is_set(clap::ArgSettings::MultipleOccurrences));
        assert!(arg.is_set(clap::ArgSettings::Required));
        assert_eq!(arg.get_help(), Some("How to use it"));
    }
}

mod arg_impl {
    #[test]
    fn string_ident() {
        let expected = "one";
        let actual = clap::arg_impl! { @string one };
        assert_eq!(actual, expected);
    }

    #[test]
    fn string_literal() {
        let expected = "one";
        let actual = clap::arg_impl! { @string "one" };
        assert_eq!(actual, expected);
    }

    #[test]
    fn char_ident() {
        let expected = 'o';
        let actual = clap::arg_impl! { @char o };
        assert_eq!(actual, expected);
    }

    #[test]
    fn char_literal() {
        let expected = 'o';
        let actual = clap::arg_impl! { @char 'o' };
        assert_eq!(actual, expected);
    }
}
