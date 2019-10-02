extern crate clap;
extern crate regex;

#[cfg(test)]
mod tests {
    include!("../clap-test.rs");
    use clap::{App, Arg};

    fn get_app() -> App<'static> {
        App::new("myprog")
            .arg(
                Arg::with_name("GLOBAL_ARG")
                    .long("global-arg")
                    .help("Specifies something needed by the subcommands")
                    .global(true)
                    .takes_value(true)
                    .default_value("default_value"),
            )
            .arg(
                Arg::with_name("GLOBAL_FLAG")
                    .long("global-flag")
                    .help("Specifies something needed by the subcommands")
                    .multiple(true)
                    .global(true),
            )
            .subcommand(App::new("outer").subcommand(App::new("inner")))
    }

    #[test]
    fn issue_1076() {
        let mut app = get_app();
        let _ = app.try_get_matches_from_mut(vec!["myprog"]);
        let _ = app.try_get_matches_from_mut(vec!["myprog"]);
        let _ = app.try_get_matches_from_mut(vec!["myprog"]);
    }
}
