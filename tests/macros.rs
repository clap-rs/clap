mod arg {
    #[test]
    fn name_explicit() {
        let arg = clap::arg!(foo: --bar <NUM>);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("bar"));
        assert_eq!(arg.get_value_names(), Some(vec!["NUM".into()].as_slice()));
        assert!(!arg.is_required_set());
    }

    #[test]
    fn name_from_long() {
        let arg = clap::arg!(--bar <NUM>);
        assert_eq!(arg.get_id(), "bar");
        assert_eq!(arg.get_long(), Some("bar"));
        assert_eq!(arg.get_value_names(), Some(vec!["NUM".into()].as_slice()));
        assert!(!arg.is_required_set());
    }

    #[test]
    fn name_from_value() {
        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_long(), None);
        assert_eq!(arg.get_value_names(), Some(vec!["NUM".into()].as_slice()));
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
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(foo: -'b');
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(foo: -b ...);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Count));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(foo: -b "How to use it");
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(
            arg.get_help().map(|s| s.to_string()),
            Some("How to use it".to_owned())
        );
    }

    #[test]
    fn short_and_long() {
        let arg = clap::arg!(foo: -b --hello);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(foo: -'b' --hello);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(foo: -b --hello ...);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Count));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(foo: -b --hello "How to use it");
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_long(), Some("hello"));
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(
            arg.get_help().map(|s| s.to_string()),
            Some("How to use it".to_owned())
        );
    }

    #[test]
    fn short_help() {
        let arg = clap::arg!(help: -b);
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));

        let mut cmd = clap::Command::new("cmd")
            .disable_help_flag(true)
            .arg(clap::arg!(help: -b).action(clap::ArgAction::Help));
        cmd.build();
        let arg = cmd
            .get_arguments()
            .find(|arg| arg.get_id() == "help")
            .unwrap();
        assert!(matches!(arg.get_action(), clap::ArgAction::Help));
    }

    #[test]
    fn long_help() {
        let arg = clap::arg!(-'?' - -help);
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));

        let mut cmd = clap::Command::new("cmd")
            .disable_help_flag(true)
            .arg(clap::arg!(-'?' - -help).action(clap::ArgAction::Help));
        cmd.build();
        let arg = cmd
            .get_arguments()
            .find(|arg| arg.get_id() == "help")
            .unwrap();
        assert!(matches!(arg.get_action(), clap::ArgAction::Help));
    }

    #[test]
    fn short_version() {
        let arg = clap::arg!(version: -b);
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));

        let mut cmd = clap::Command::new("cmd")
            .disable_version_flag(true)
            .version("1.0.0")
            .arg(clap::arg!(version: -b).action(clap::ArgAction::Version));
        cmd.build();
        let arg = cmd
            .get_arguments()
            .find(|arg| arg.get_id() == "version")
            .unwrap();
        assert!(matches!(arg.get_action(), clap::ArgAction::Version));
    }

    #[test]
    fn long_version() {
        let arg = clap::arg!(-'?' - -version);
        assert!(matches!(arg.get_action(), clap::ArgAction::SetTrue));

        let mut cmd = clap::Command::new("cmd")
            .disable_version_flag(true)
            .version("1.0.0")
            .arg(clap::arg!(-'?' - -version).action(clap::ArgAction::Version));
        cmd.build();
        let arg = cmd
            .get_arguments()
            .find(|arg| arg.get_id() == "version")
            .unwrap();
        assert!(matches!(arg.get_action(), clap::ArgAction::Version));
    }

    #[test]
    fn short_with_value() {
        let arg = clap::arg!(foo: -b <NUM>);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(foo: -'b' <NUM>);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(foo: -b  <NUM> ...);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Append));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(foo: -b  <NUM> "How to use it");
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_short(), Some('b'));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(
            arg.get_help().map(|s| s.to_string()),
            Some("How to use it".to_owned())
        );
    }

    #[test]
    fn positional() {
        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM".into()].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert_eq!(arg.get_num_args(), None);
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!([NUM]);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM".into()].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert_eq!(arg.get_num_args(), None);
        assert!(!arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(<NUM>);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM".into()].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert_eq!(arg.get_num_args(), None);
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(foo: <NUM>);
        assert_eq!(arg.get_id(), "foo");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM".into()].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert_eq!(arg.get_num_args(), None);
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(<NUM> ...);
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM".into()].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Append));
        assert_eq!(arg.get_num_args(), Some((1..).into()));
        assert!(arg.is_required_set());
        assert_eq!(arg.get_help().map(|s| s.to_string()), None);

        let arg = clap::arg!(<NUM> "How to use it");
        assert_eq!(arg.get_id(), "NUM");
        assert_eq!(arg.get_value_names(), Some(vec!["NUM".into()].as_slice()));
        assert!(matches!(arg.get_action(), clap::ArgAction::Set));
        assert_eq!(arg.get_num_args(), None);
        assert!(arg.is_required_set());
        assert_eq!(
            arg.get_help().map(|s| s.to_string()),
            Some("How to use it".to_owned())
        );
    }

    #[test]
    #[cfg(all(feature = "help", feature = "usage"))]
    fn optional_value() {
        let mut cmd = clap::Command::new("test")
            .args_override_self(true)
            .arg(clap::arg!(port: -p [NUM]));

        let r = cmd.try_get_matches_from_mut(["test", "-p42"]);
        assert!(r.is_ok(), "{}", r.unwrap_err());
        let m = r.unwrap();
        assert!(m.contains_id("port"));
        assert_eq!(m.get_one::<String>("port").unwrap(), "42");

        let r = cmd.try_get_matches_from_mut(["test", "-p"]);
        assert!(r.is_ok(), "{}", r.unwrap_err());
        let m = r.unwrap();
        assert!(m.contains_id("port"));
        assert!(m.get_one::<String>("port").is_none());

        let r = cmd.try_get_matches_from_mut(["test", "-p", "24", "-p", "42"]);
        assert!(r.is_ok(), "{}", r.unwrap_err());
        let m = r.unwrap();
        assert!(m.contains_id("port"));
        assert_eq!(m.get_one::<String>("port").unwrap(), "42");

        let help = cmd.render_help().to_string();
        snapbox::assert_data_eq!(
            help,
            snapbox::str![[r#"
Usage: test [OPTIONS]

Options:
  -p [<NUM>]      
  -h, --help      Print help

"#]]
        );
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
        assert_eq!(m.get_one::<String>("arg").unwrap(), "some-val");
    }
}
