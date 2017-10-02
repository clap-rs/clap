extern crate clap;
extern crate regex;



static VISIBLE_ALIAS_HELP: &'static str = "clap-test 2.6

USAGE:
    clap-test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    Some help [aliases: dongle, done]";

static INVISIBLE_ALIAS_HELP: &'static str = "clap-test 2.6

USAGE:
    clap-test [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    Some help";

#[cfg(feature = "suggestions")]
static DYM: &'static str = "error: The subcommand 'subcm' wasn't recognized
\tDid you mean 'subcmd'?

If you believe you received this message in error, try re-running with 'clap-test -- subcm'

USAGE:
    clap-test [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

For more information try --help";

#[cfg(feature = "suggestions")]
static DYM2: &'static str = "error: Found argument '--subcm' which wasn't expected, or isn't valid in this context
\tDid you mean to put '--subcmdarg' after the subcommand 'subcmd'?

USAGE:
    clap-test [FLAGS] [OPTIONS] [ARGS] [SUBCOMMAND]

For more information try --help";

#[cfg(test)]
mod tests {
    include!("../clap-test.rs");
    use clap;
    use clap::{App, Arg, SubCommand, ErrorKind, AppSettings};

    fn setup_app_with_globals_and_subcommands<'a, 'b>() -> clap::App<'a, 'b> {
        let global_arg = Arg::with_name("GLOBAL_ARG")
            .long("global-arg")
            .help(
                "Specifies something needed by the subcommands",
            )
            .global(true)
            .takes_value(true);
        
        let double_sub_command = SubCommand::with_name("outer")
            .subcommand(SubCommand::with_name("run"));

        App::new("myprog")
            .setting(AppSettings::PropagateGlobalValuesDown)
            .arg(global_arg)
            .subcommand(double_sub_command)
    }

    fn first_subcommand_can_access_global(arg_vector : Vec<&str>) {
        let matches = setup_app_with_globals_and_subcommands().get_matches_from(
            arg_vector
        );

        let sub_match = matches.subcommand_matches("outer").expect("could not access subcommand");

        assert_eq!(sub_match.value_of("GLOBAL_ARG").expect("subcommand could not access global arg"), 
                    "some_value", "subcommand did not have expected value for global arg");

    }

    fn second_subcommand_can_access_global(arg_vector : Vec<&str>) {
        let matches = setup_app_with_globals_and_subcommands().get_matches_from(
            arg_vector
        );

        let sub_match = matches.subcommand_matches("outer").expect("could not access subcommand");
        let sub_sub_match = sub_match.subcommand_matches("run").expect("could not access inner sub");

        assert_eq!(sub_sub_match.value_of("GLOBAL_ARG").expect("inner subcommand could not access global arg"), 
                "some_value", "inner subcommand did not have expected value for global arg");
    }

    #[test]
    fn subcommand_can_access_global_arg_if_global_arg_is_first() {
        // calls propagate zero times, works
        first_subcommand_can_access_global(vec!["myprog", "--global-arg", "some_value", "outer", "run"]);
    }

    #[test]
    fn subcommand_can_access_global_arg_if_global_arg_is_in_the_middle() {
        // calls propagate twice, doesn't work
        first_subcommand_can_access_global(vec!["myprog", "outer",  "--global-arg", "some_value" ,"run"]);
    }

    #[test]
    fn subcommand_can_access_global_arg_if_global_arg_is_last() {
        // calls propagate twice, doesn't work
        first_subcommand_can_access_global(vec!["myprog", "outer", "run", "--global-arg", "some_value"]);
    }

    #[test]
    fn second_subcommand_can_access_global_arg_if_global_arg_is_first() {
        second_subcommand_can_access_global(vec!["myprog", "--global-arg", "some_value", "outer", "run"]);
    }

    #[test]
    fn second_subcommand_can_access_global_arg_if_global_arg_is_in_the_middle() {
        second_subcommand_can_access_global(vec!["myprog", "outer",  "--global-arg", "some_value" ,"run"]);
    }

    #[test]
    fn second_subcommand_can_access_global_arg_if_global_arg_is_last() {
        second_subcommand_can_access_global(vec!["myprog", "outer", "run", "--global-arg", "some_value"]);
    }
}


