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
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert!(!arg.is_multiple_values_set());
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -'b');
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert!(!arg.is_multiple_values_set());
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b ...);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Count));
        assert!(!arg.is_multiple_values_set());
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b "How to use it");
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert!(!arg.is_multiple_values_set());
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), Some("How to use it"));
    }

    #[test]
    fn short_and_long() {
        let arg = clap::arg!(foo: -b --hello);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert!(!arg.is_multiple_values_set());
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -'b' --hello);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert!(!arg.is_multiple_values_set());
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b --hello ...);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Count));
        assert!(!arg.is_multiple_values_set());
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b --hello "How to use it");
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert!(!arg.is_multiple_values_set());
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), Some("How to use it"));
    }

    #[test]
    fn short_with_value() {
        let arg = clap::arg!(foo: -b <NUM>);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert!(!arg.is_multiple_values_set());
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -'b' <NUM>);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert!(!arg.is_multiple_values_set());
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b  <NUM> ...);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Append));
        assert!(!arg.is_multiple_values_set());
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: -b  <NUM> "How to use it");
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert!(!arg.is_multiple_values_set());
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), Some("How to use it"));
    }

    #[test]
    fn positional() {
        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert!(!arg.is_multiple_values_set());
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!([NUM]);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert!(!arg.is_multiple_values_set());
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert!(!arg.is_multiple_values_set());
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(foo: <NUM>);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert!(!arg.is_multiple_values_set());
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(<NUM> ...);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Append));
        assert!(arg.is_multiple_values_set());
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help(), None);

        let arg = clap::arg!(<NUM> "How to use it");
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM"].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert!(!arg.is_multiple_values_set());
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
