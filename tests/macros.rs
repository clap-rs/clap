// We intentionally don't import `clap_app!` here; not having it in scope protects against the
// class of errors where the macro refers to itself as `clap_app!` instead of `$crate::clap_app!`
use clap::error::ErrorKind;

#[test]
fn basic() {
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
    );
}

#[test]
fn quoted_app_name() {
    #![allow(deprecated)]
    let mut cmd = clap::clap_app!(("cmd name with spaces-and-hyphens") =>
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

    assert_eq!(cmd.get_name(), "cmd name with spaces-and-hyphens");

    let mut help_text = vec![];
    cmd.write_help(&mut help_text)
        .expect("Could not write help text.");
    let help_text = String::from_utf8(help_text).expect("Help text is not valid utf-8");
    assert!(help_text.starts_with("cmd name with spaces-and-hyphens 0.1\n"));
}

#[test]
fn quoted_arg_long_name() {
    #![allow(deprecated)]
    let cmd = clap::clap_app!(claptests =>
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

    let matches = cmd
        .try_get_matches_from(vec!["bin_name", "value1", "value2", "--long-option-2"])
        .expect("Expected to successfully match the given args.");
    #[allow(deprecated)]
    {
        assert!(matches.is_present("option2"));
    }
}

#[test]
fn quoted_arg_name() {
    #![allow(deprecated)]
    let cmd = clap::clap_app!(claptests =>
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

    let matches = cmd
        .try_get_matches_from(vec!["bin_name", "value1", "value2", "--long-option-2"])
        .expect("Expected to successfully match the given args.");
    assert!(matches.is_present("option2"));
}

#[test]
fn group_macro() {
    #![allow(deprecated)]
    let cmd = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty =>
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = cmd.try_get_matches_from(vec!["bin_name", "--hard"]);
    assert!(result.is_ok());
    let matches = result.expect("Expected to successfully match the given args.");
    assert!(matches.is_present("difficulty"));
    assert!(matches.is_present("hard"));
}

#[test]
fn group_macro_set_multiple() {
    #![allow(deprecated)]
    let cmd = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty +multiple =>
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = cmd.try_get_matches_from(vec!["bin_name", "--hard", "--easy"]);
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
    let cmd = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty !multiple =>
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = cmd.try_get_matches_from(vec!["bin_name", "--hard", "--easy"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
}

#[test]
fn group_macro_set_required() {
    #![allow(deprecated)]
    let cmd = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty +required =>
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = cmd.try_get_matches_from(vec!["bin_name"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn group_macro_set_not_required() {
    #![allow(deprecated)]
    let cmd = clap::clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty !required =>
                 (@arg hard: -h --hard "Sets hard mode")
                 (@arg normal: -n --normal "Sets normal mode")
                 (@arg easy: -e --easy "Sets easy mode")
             )
    );

    let result = cmd.try_get_matches_from(vec!["bin_name"]);
    assert!(result.is_ok());
    let matches = result.expect("Expected to successfully match the given args.");
    assert!(!matches.is_present("difficulty"));
}

#[test]
fn multiarg() {
    #![allow(deprecated)]
    let cmd = clap::clap_app!(claptests =>
        (@arg flag: --flag "value")
        (@arg multiarg: --multiarg
            default_value("flag-unset") default_value_if("flag", None, Some("flag-set"))
            "multiarg")
        (@arg multiarg2: --multiarg2
            default_value("flag-unset") default_value_if("flag", None, Some("flag-set"))
            "multiarg2")
    );

    let matches = cmd
        .clone()
        .try_get_matches_from(vec!["bin_name"])
        .expect("match failed");

    assert_eq!(matches.value_of("multiarg"), Some("flag-unset"));
    assert_eq!(matches.value_of("multiarg2"), Some("flag-unset"));

    let matches = cmd
        .try_get_matches_from(vec!["bin_name", "--flag"])
        .expect("match failed");

    assert_eq!(matches.value_of("multiarg"), Some("flag-set"));
    assert_eq!(matches.value_of("multiarg2"), Some("flag-set"));
}

mod arg {
    #[test]
    fn name_explicit() {
        let arg = clap::arg!(foo: --bar <NUM>);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("bar"));
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(arg.is_required_set());
    }

    #[test]
    fn name_from_long() {
        let arg = clap::arg!(--bar <NUM>);
        assert_eq!(arg.get_id(), "bar");
        assert_eq!(arg.get_long(), Some("bar"));
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(arg.is_required_set());
    }

    #[test]
    fn name_from_value() {
        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_long(), None);
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(arg.is_required_set());
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
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -'b');
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b ...);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        #[allow(deprecated)]
        {
            assert!(arg.is_multiple_occurrences_set());
        }
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b "How to use it");
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), Some("How to use it"));
    }

    #[test]
    fn short_and_long() {
        let arg = clap::arg!(foo: -b --hello);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -'b' --hello);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b --hello ...);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        #[allow(deprecated)]
        {
            assert!(arg.is_multiple_occurrences_set());
        }
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b --hello "How to use it");
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), Some("How to use it"));
    }

    #[test]
    fn positional() {
        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!([NUM]);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: <NUM>);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(<NUM> ...);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        #[allow(deprecated)]
        {
            assert!(arg.is_multiple_occurrences_set());
        }
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(<NUM> "How to use it");
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        #[allow(deprecated)]
        {
            assert!(!arg.is_multiple_occurrences_set());
        }
        assert!(arg.is_required_set());
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

    #[test]
    // allow double quoted dashed arg name in square brackets (e.g ["some-arg"])
    fn arg_name_dashed() {
        let arg = clap::arg!(["some-arg"] "some arg");
        assert_eq!(arg, clap::Arg::new("some-arg").help("some arg"));

        let m = clap::Command::new("flag")
            .arg(arg)
            .try_get_matches_from(vec!["", "some-val"])
            .unwrap();
        #[allow(deprecated)]
        {
            assert!(m.is_present("some-arg"));
        }
        assert_eq!(m.get_one::<String>("some-arg").unwrap(), "some-val");
    }

    #[test]
    // allow double quoted dashed arg value in triangle brackets (e.g <"some-val">)
    // test in combination with short argument name (e.g. -v)
    fn arg_value_dashed_with_short_arg() {
        let arg = clap::arg!(-a <"some-val"> "some arg");
        assert_eq!(
            arg,
            clap::Arg::new("some-val")
                .short('a')
                .long("arg")
                .value_name("some-val")
        );

        let m = clap::Command::new("cmd")
            .arg(arg)
            .try_get_matches_from(vec!["", "-a", "val"])
            .unwrap();
        #[allow(deprecated)]
        {
            assert!(m.is_present("some-val"));
        }
        assert_eq!(m.get_one::<String>("some-val").unwrap(), "val");
    }

    #[test]
    // allow double quoted dashed arg value in triangle brackets (e.g <"some-val">)
    // test in combination with long argument name (e.g. --value)
    fn arg_value_dashed_with_long_arg() {
        let arg = clap::arg!(-a --arg <"some-val"> "some arg");
        assert_eq!(
            arg,
            clap::Arg::new("arg")
                .short('a')
                .long("arg")
                .value_name("some-val")
        );

        let m = clap::Command::new("cmd")
            .arg(arg)
            .try_get_matches_from(vec!["", "--arg", "some-val"])
            .unwrap();
        #[allow(deprecated)]
        {
            assert!(m.is_present("arg"));
        }
        assert_eq!(m.get_one::<String>("arg").unwrap(), "some-val");
    }
}
