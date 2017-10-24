extern crate clap;
extern crate regex;

#[cfg(test)]
mod tests {
    include!("../clap-test.rs");
    use clap::{App, Arg, SubCommand, ArgMatches};

    fn get_app() -> App<'static, 'static> {
        App::new("myprog")
            .arg(Arg::with_name("GLOBAL_ARG")
                .long("global-arg")
                .help(
                    "Specifies something needed by the subcommands",
                )
                .global(true)
                .takes_value(true)
                .default_value("default_value"))
            .arg(Arg::with_name("GLOBAL_FLAG")
                .long("global-flag")
                .help(
                    "Specifies something needed by the subcommands",
                )
                .multiple(true)
                .global(true))
            .subcommand(SubCommand::with_name("outer")
                .subcommand(SubCommand::with_name("inner")))
    }

    #[test]
    fn issue_1076() {
        let mut app = get_app();
        app.get_matches_from_safe_borrow(vec!["myprog"]);
        app.get_matches_from_safe_borrow(vec!["myprog"]);
        app.get_matches_from_safe_borrow(vec!["myprog"]);
    }
}
