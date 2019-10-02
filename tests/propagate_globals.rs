extern crate clap;
extern crate regex;

#[cfg(test)]
mod tests {
    include!("../clap-test.rs");
    use clap::{App, Arg, ArgMatches, ArgSettings};

    fn get_app() -> App<'static> {
        App::new("myprog")
            .arg(
                Arg::with_name("GLOBAL_ARG")
                    .long("global-arg")
                    .help("Specifies something needed by the subcommands")
                    .global(true)
                    .setting(ArgSettings::TakesValue)
                    .default_value("default_value"),
            )
            .arg(
                Arg::with_name("GLOBAL_FLAG")
                    .long("global-flag")
                    .help("Specifies something needed by the subcommands")
                    .global(true)
                    .setting(ArgSettings::MultipleOccurrences),
            )
            .subcommand(App::new("outer").subcommand(App::new("inner")))
    }

    fn get_matches(app: App<'static>, argv: &'static str) -> ArgMatches {
        app.get_matches_from(argv.split(" ").collect::<Vec<_>>())
    }

    fn get_outer_matches(m: &ArgMatches) -> &ArgMatches {
        m.subcommand_matches("outer")
            .expect("could not access outer subcommand")
    }

    fn get_inner_matches(m: &ArgMatches) -> &ArgMatches {
        get_outer_matches(m)
            .subcommand_matches("inner")
            .expect("could not access inner subcommand")
    }

    fn top_can_access_arg<T: Into<Option<&'static str>>>(m: &ArgMatches, val: T) -> bool {
        m.value_of("GLOBAL_ARG") == val.into()
    }

    fn inner_can_access_arg<T: Into<Option<&'static str>>>(m: &ArgMatches, val: T) -> bool {
        get_inner_matches(m).value_of("GLOBAL_ARG") == val.into()
    }

    fn outer_can_access_arg<T: Into<Option<&'static str>>>(m: &ArgMatches, val: T) -> bool {
        get_outer_matches(m).value_of("GLOBAL_ARG") == val.into()
    }

    fn top_can_access_flag(m: &ArgMatches, present: bool, occurrences: u64) -> bool {
        (m.is_present("GLOBAL_FLAG") == present) && (m.occurrences_of("GLOBAL_FLAG") == occurrences)
    }

    fn inner_can_access_flag(m: &ArgMatches, present: bool, occurrences: u64) -> bool {
        let m = get_inner_matches(m);
        (m.is_present("GLOBAL_FLAG") == present) && (m.occurrences_of("GLOBAL_FLAG") == occurrences)
    }

    fn outer_can_access_flag(m: &ArgMatches, present: bool, occurrences: u64) -> bool {
        let m = get_outer_matches(m);
        (m.is_present("GLOBAL_FLAG") == present) && (m.occurrences_of("GLOBAL_FLAG") == occurrences)
    }

    #[test]
    fn global_arg_used_top_level() {
        let m = get_matches(get_app(), "myprog --global-arg=some_value outer inner");

        assert!(top_can_access_arg(&m, "some_value"));
        assert!(inner_can_access_arg(&m, "some_value"));
        assert!(outer_can_access_arg(&m, "some_value"));
    }

    #[test]
    fn global_arg_used_outer() {
        let m = get_matches(get_app(), "myprog outer --global-arg=some_value inner");

        assert!(top_can_access_arg(&m, "some_value"));
        assert!(inner_can_access_arg(&m, "some_value"));
        assert!(outer_can_access_arg(&m, "some_value"));
    }

    #[test]
    fn global_arg_used_inner() {
        let m = get_matches(get_app(), "myprog outer inner --global-arg=some_value");

        assert!(top_can_access_arg(&m, "some_value"));
        assert!(inner_can_access_arg(&m, "some_value"));
        assert!(outer_can_access_arg(&m, "some_value"));
    }

    #[test]
    fn global_arg_default_value() {
        let m = get_matches(get_app(), "myprog outer inner");

        assert!(top_can_access_arg(&m, "default_value"));
        assert!(inner_can_access_arg(&m, "default_value"));
        assert!(outer_can_access_arg(&m, "default_value"));
    }

    #[test]
    fn global_flag_used_top_level() {
        let m = get_matches(get_app(), "myprog --global-flag outer inner");

        assert!(top_can_access_flag(&m, true, 1));
        assert!(inner_can_access_flag(&m, true, 1));
        assert!(outer_can_access_flag(&m, true, 1));
    }

    #[test]
    fn global_flag_used_outer() {
        let m = get_matches(get_app(), "myprog outer --global-flag inner");

        assert!(top_can_access_flag(&m, true, 1));
        assert!(inner_can_access_flag(&m, true, 1));
        assert!(outer_can_access_flag(&m, true, 1));
    }

    #[test]
    fn global_flag_used_inner() {
        let m = get_matches(get_app(), "myprog outer inner --global-flag");

        assert!(top_can_access_flag(&m, true, 1));
        assert!(inner_can_access_flag(&m, true, 1));
        assert!(outer_can_access_flag(&m, true, 1));
    }

    #[test]
    fn global_flag_2x_used_top_level() {
        let m = get_matches(get_app(), "myprog --global-flag --global-flag outer inner");

        assert!(top_can_access_flag(&m, true, 2));
        assert!(inner_can_access_flag(&m, true, 2));
        assert!(outer_can_access_flag(&m, true, 2));
    }

    #[test]
    fn global_flag_2x_used_inner() {
        let m = get_matches(get_app(), "myprog outer inner --global-flag --global-flag");

        assert!(top_can_access_flag(&m, true, 2));
        assert!(inner_can_access_flag(&m, true, 2));
        assert!(outer_can_access_flag(&m, true, 2));
    }
}
