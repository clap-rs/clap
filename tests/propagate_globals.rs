extern crate clap;
extern crate regex;

#[cfg(test)]
mod tests {
    include!("../clap-test.rs");
    use clap::{App, Arg, SubCommand, AppSettings, ArgMatches};

    fn get_global_arg() -> Arg<'static, 'static> {
        Arg::with_name("GLOBAL_ARG")
            .long("global-arg")
            .help(
                "Specifies something needed by the subcommands",
            )
            .global(true)
            .takes_value(true)
    }

    fn get_global_flag() -> Arg<'static, 'static> {
        Arg::with_name("GLOBAL_FLAG")
            .long("global-flag")
            .help(
                "Specifies something needed by the subcommands",
            )
            .global(true)
    }

    fn get_app() -> App<'static, 'static> {
        App::new("myprog")
            .global_setting(AppSettings::PropagateGlobalValuesDown)
    }

    fn get_subcommands() -> App<'static, 'static> {
        SubCommand::with_name("outer")
            .subcommand(SubCommand::with_name("inner"))
    }

    fn get_matches(app: App<'static, 'static>, argv: &'static str) -> ArgMatches<'static> {
        app.get_matches_from(argv.split(" ").collect::<Vec<_>>())
    }

    fn get_outer_matches<'a>(m: &'a ArgMatches<'static>) -> &'a ArgMatches<'static> {
        m.subcommand_matches("outer").expect("could not access outer subcommand")
    }

    fn get_inner_matches<'a>(m: &'a ArgMatches<'static>) -> &'a ArgMatches<'static> {
        get_outer_matches(m).subcommand_matches("inner").expect("could not access inner subcommand")
    }

    fn top_can_access_arg<T: Into<Option<&'static str>>>(m: &ArgMatches<'static>, val: T) -> bool {
        m.value_of("GLOBAL_ARG") == val.into()
    }

    fn inner_can_access_arg<T: Into<Option<&'static str>>>(m: &ArgMatches<'static>, val: T) -> bool {
        get_inner_matches(m).value_of("GLOBAL_ARG") == val.into()
    }

    fn outer_can_access_arg<T: Into<Option<&'static str>>>(m: &ArgMatches<'static>, val: T) -> bool {
        get_outer_matches(m).value_of("GLOBAL_ARG") == val.into()
    }

    fn inner_can_access_flag(m: &ArgMatches<'static>, present: bool, occurrences: u64) -> bool {
        let m = get_inner_matches(m);
        (m.is_present("GLOBAL_FLAG") == present) && (m.occurrences_of("GLOBAL_FLAG") == occurrences)
    }

    fn outer_can_access_flag(m: &ArgMatches<'static>, present: bool, occurrences: u64) -> bool {
        let m = get_outer_matches(m);
        (m.is_present("GLOBAL_FLAG") == present) && (m.occurrences_of("GLOBAL_FLAG") == occurrences)
    }

    #[test]
    fn global_arg_defined_top_level_used_top_level() {
        let app = get_app()
            .arg(get_global_arg())
            .subcommand(get_subcommands());

        let m = get_matches(app, "myprog --global-arg=some_value outer inner");

        assert!(top_can_access_arg(&m, "some_value"));
        assert!(inner_can_access_arg(&m, "some_value"));
        assert!(outer_can_access_arg(&m, "some_value"));
    }

    #[test]
    fn global_arg_defined_top_level_used_outer() {
        let app = get_app()
            .arg(get_global_arg())
            .subcommand(get_subcommands());

        let m = get_matches(app, "myprog outer --global-arg=some_value inner");

        assert!(top_can_access_arg(&m, "some_value"));
        assert!(inner_can_access_arg(&m, "some_value"));
        assert!(outer_can_access_arg(&m, "some_value"));
    }

    #[test]
    fn global_arg_defined_top_level_used_inner() {
        let app = get_app()
            .arg(get_global_arg())
            .subcommand(get_subcommands());

        let m = get_matches(app, "myprog outer inner --global-arg=some_value");

        assert!(top_can_access_arg(&m, "some_value"), "{:?}", m);
        assert!(inner_can_access_arg(&m, "some_value"));
        assert!(outer_can_access_arg(&m, "some_value"));
    }

    // #[test]
    // fn global_arg_defined_nested_used_top_level() {
    //     let app = get_app()
    //         .subcommand(get_subcommands()
    //             .arg(get_global_arg()));

    //     let m = get_matches(app, "myprog --global-arg=some_value outer inner");

    //     assert!(top_can_access_arg(&m, "some_value"));
    //     assert!(inner_can_access_arg(&m, "some_value"));
    //     assert!(outer_can_access_arg(&m, "some_value"));
    // }

    // #[test]
    // fn global_arg_defined_nested_used_outer() {
    //     let app = get_app()
    //         .subcommand(get_subcommands()
    //             .arg(get_global_arg()));

    //     let m = get_matches(app, "myprog outer --global-arg=some_value inner");

    //     assert!(top_can_access_arg(&m, "some_value"));
    //     assert!(inner_can_access_arg(&m, "some_value"));
    //     assert!(outer_can_access_arg(&m, "some_value"));
    // }

    // #[test]
    // fn global_arg_defined_nested_used_inner() {
    //     let app = get_app()
    //         .subcommand(get_subcommands()
    //             .arg(get_global_arg()));

    //     let m = get_matches(app, "myprog outer inner --global-arg=some_value");

    //     assert!(top_can_access_arg(&m, "some_value"));
    //     assert!(inner_can_access_arg(&m, "some_value"));
    //     assert!(outer_can_access_arg(&m, "some_value"));
    // }
}
