extern crate clap;
extern crate regex;

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
            .subcommand(SubCommand::with_name("inner"));

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
        let sub_sub_match = sub_match.subcommand_matches("inner").expect("could not access inner sub");

        assert_eq!(sub_sub_match.value_of("GLOBAL_ARG").expect("inner subcommand could not access global arg"), 
                "some_value", "inner subcommand did not have expected value for global arg");
    }

    #[test]
    fn subcommand_can_access_global_arg_if_global_arg_is_first() {
        first_subcommand_can_access_global(vec!["myprog", "--global-arg", "some_value", "outer", "inner"]);
    }

    #[test]
    fn subcommand_can_access_global_arg_if_global_arg_is_in_the_middle() {
        first_subcommand_can_access_global(vec!["myprog", "outer",  "--global-arg", "some_value" ,"inner"]);
    }

    #[test]
    fn subcommand_can_access_global_arg_if_global_arg_is_last() {
        first_subcommand_can_access_global(vec!["myprog", "outer", "inner", "--global-arg", "some_value"]);
    }

    #[test]
    fn second_subcommand_can_access_global_arg_if_global_arg_is_first() {
        second_subcommand_can_access_global(vec!["myprog", "--global-arg", "some_value", "outer", "inner"]);
    }

    #[test]
    fn second_subcommand_can_access_global_arg_if_global_arg_is_in_the_middle() {
        second_subcommand_can_access_global(vec!["myprog", "outer",  "--global-arg", "some_value" ,"inner"]);
    }

    #[test]
    fn second_subcommand_can_access_global_arg_if_global_arg_is_last() {
        second_subcommand_can_access_global(vec!["myprog", "outer", "inner", "--global-arg", "some_value"]);
    }
}


