mod utils;

use clap::{clap_app, ErrorKind};

static LITERALS: &str = "clap-tests 0.1

USAGE:
    clap-tests [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -4, --4          Sets priority to 4
    -5, --5          Sets priority to 5
    -6, --6          Sets priority to 6
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --task-num <task-num>    Task number [possible values: all, 0, 1, 2]

SUBCOMMANDS:
    0             Set everything to zero priority
    help          Prints this message or the help of the given subcommand(s)
    view-tasks    View all tasks";

#[test]
fn basic() {
    clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (@global_setting ColorNever)
        (@setting VersionlessSubcommands)
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
    let mut app = clap_app!(("app name with spaces-and-hyphens") =>
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
    let app = clap_app!(claptests =>
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
    let app = clap_app!(claptests =>
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
    clap_app!(claptests =>
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
    let app = clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty =>
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
    let app = clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty +multiple =>
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
    let app = clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty !multiple =>
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
    let app = clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty +required =>
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
    let app = clap_app!(claptests =>
        (version: "0.1")
        (about: "tests clap library")
        (author: "Kevin K. <kbknapp@gmail.com>")
             (@group difficulty !required =>
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
fn literals() {
    let app = clap_app!("clap-tests" =>
        (version: "0.1")
        (@arg "task-num": -"t-n" --"task-num" +takes_value possible_value["all" 0 1 2]
            "Task number")
        (@group priority =>
            (@arg "4": -4 --4 "Sets priority to 4")
            (@arg ("5"): -('5') --5 "Sets priority to 5")
            (@arg 6: -6 --6 "Sets priority to 6")
        )
        (@subcommand "view-tasks" =>
            (about: "View all tasks"))
        (@subcommand 0 =>
            (about: "Set everything to zero priority"))
    );

    assert!(utils::compare_output(
        app,
        "clap-tests --help",
        LITERALS,
        false
    ));
}

#[test]
fn multiarg() {
    let app = clap_app!(claptests =>
        (@arg flag: --flag "value")
        (@arg multiarg: --multiarg
            default_value("flag-unset") default_value_if("flag", None, "flag-set")
            "multiarg")
        (@arg multiarg2: --multiarg2
            default_value("flag-unset") default_value_if("flag", None, "flag-set",)
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
