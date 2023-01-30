use super::utils;

use std::io::Write;
use std::str;

use clap::{Arg, Command};

static SCF2OP: &str = "flag present 2 times
option NOT present
positional NOT present
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option NOT present
positional NOT present
subcmd present
flag present 2 times
scoption present with value: some
An scoption: some
scpositional present with value: value
";

static SCFOP: &str = "flag present 1 times
option NOT present
positional NOT present
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option NOT present
positional NOT present
subcmd present
flag present 1 times
scoption present with value: some
An scoption: some
scpositional present with value: value
";

static O2P: &str = "flag NOT present
option present with value: some
An option: some
An option: other
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present with value: some
An option: some
An option: other
positional present with value: value
subcmd NOT present
";

static F2OP: &str = "flag present 2 times
option present with value: some
An option: some
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present with value: some
An option: some
positional present with value: value
subcmd NOT present
";

static FOP: &str = "flag present 1 times
option present with value: some
An option: some
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present with value: some
An option: some
positional present with value: value
subcmd NOT present
";

pub fn check_complex_output(args: &str, out: &str) {
    let mut w = vec![];
    let matches = utils::complex_app()
        .try_get_matches_from(args.split(' ').collect::<Vec<_>>())
        .unwrap();
    match matches.get_one::<u8>("flag").unwrap() {
        0 => {
            writeln!(w, "flag NOT present").unwrap();
        }
        n => {
            writeln!(w, "flag present {} times", n).unwrap();
        }
    }

    if matches.contains_id("option") {
        if let Some(v) = matches.get_one::<String>("option").map(|v| v.as_str()) {
            writeln!(w, "option present with value: {}", v).unwrap();
        }
        if let Some(ov) = matches.get_many::<String>("option") {
            for o in ov {
                writeln!(w, "An option: {}", o).unwrap();
            }
        }
    } else {
        writeln!(w, "option NOT present").unwrap();
    }

    if let Some(p) = matches.get_one::<String>("positional").map(|v| v.as_str()) {
        writeln!(w, "positional present with value: {}", p).unwrap();
    } else {
        writeln!(w, "positional NOT present").unwrap();
    }

    if *matches.get_one::<bool>("flag2").expect("defaulted by clap") {
        writeln!(w, "flag2 present").unwrap();
        writeln!(
            w,
            "option2 present with value of: {}",
            matches
                .get_one::<String>("long-option-2")
                .map(|v| v.as_str())
                .unwrap()
        )
        .unwrap();
        writeln!(
            w,
            "positional2 present with value of: {}",
            matches
                .get_one::<String>("positional2")
                .map(|v| v.as_str())
                .unwrap()
        )
        .unwrap();
    } else {
        writeln!(w, "flag2 NOT present").unwrap();
        writeln!(
            w,
            "option2 maybe present with value of: {}",
            matches
                .get_one::<String>("long-option-2")
                .map(|v| v.as_str())
                .unwrap_or("Nothing")
        )
        .unwrap();
        writeln!(
            w,
            "positional2 maybe present with value of: {}",
            matches
                .get_one::<String>("positional2")
                .map(|v| v.as_str())
                .unwrap_or("Nothing")
        )
        .unwrap();
    }

    let _ = match matches
        .get_one::<String>("option3")
        .map(|v| v.as_str())
        .unwrap_or("")
    {
        "fast" => writeln!(w, "option3 present quickly"),
        "slow" => writeln!(w, "option3 present slowly"),
        _ => writeln!(w, "option3 NOT present"),
    };

    let _ = match matches
        .get_one::<String>("positional3")
        .map(|v| v.as_str())
        .unwrap_or("")
    {
        "vi" => writeln!(w, "positional3 present in vi mode"),
        "emacs" => writeln!(w, "positional3 present in emacs mode"),
        _ => writeln!(w, "positional3 NOT present"),
    };

    if matches.contains_id("option") {
        if let Some(v) = matches.get_one::<String>("option").map(|v| v.as_str()) {
            writeln!(w, "option present with value: {}", v).unwrap();
        }
        if let Some(ov) = matches.get_many::<String>("option") {
            for o in ov {
                writeln!(w, "An option: {}", o).unwrap();
            }
        }
    } else {
        writeln!(w, "option NOT present").unwrap();
    }

    if let Some(p) = matches.get_one::<String>("positional").map(|v| v.as_str()) {
        writeln!(w, "positional present with value: {p}").unwrap();
    } else {
        writeln!(w, "positional NOT present").unwrap();
    }
    if let Some("subcmd") = matches.subcommand_name() {
        writeln!(w, "subcmd present").unwrap();
        if let Some(matches) = matches.subcommand_matches("subcmd") {
            match matches.get_one::<u8>("flag").unwrap() {
                0 => {
                    writeln!(w, "flag NOT present").unwrap();
                }
                n => {
                    writeln!(w, "flag present {n} times").unwrap();
                }
            }

            if matches.contains_id("option") {
                if let Some(v) = matches.get_one::<String>("option").map(|v| v.as_str()) {
                    writeln!(w, "scoption present with value: {v}").unwrap();
                }
                if let Some(ov) = matches.get_many::<String>("option") {
                    for o in ov {
                        writeln!(w, "An scoption: {o}").unwrap();
                    }
                }
            } else {
                writeln!(w, "scoption NOT present").unwrap();
            }

            if let Some(p) = matches
                .get_one::<String>("scpositional")
                .map(|v| v.as_str())
            {
                writeln!(w, "scpositional present with value: {p}").unwrap();
            }
        }
    } else {
        writeln!(w, "subcmd NOT present").unwrap();
    }

    let res = str::from_utf8(&w).unwrap();
    snapbox::assert_eq(out, res);
}

#[test]
fn create_app() {
    let _ = Command::new("test")
        .version("1.0")
        .author("kevin")
        .about("does awesome things")
        .try_get_matches_from(vec![""])
        .unwrap();
}

#[test]
fn add_multiple_arg() {
    let _ = Command::new("test")
        .args([Arg::new("test").short('s'), Arg::new("test2").short('l')])
        .try_get_matches_from(vec![""])
        .unwrap();
}
#[test]
fn flag_x2_opt() {
    check_complex_output(
        "clap-test value -f -f -o some",
        "flag present 2 times
option present with value: some
An option: some
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present with value: some
An option: some
positional present with value: value
subcmd NOT present
",
    );
}

#[test]
fn long_opt_x2_pos() {
    check_complex_output("clap-test value --option some --option other", O2P);
}

#[test]
fn long_opt_eq_x2_pos() {
    check_complex_output("clap-test value --option=some --option=other", O2P);
}

#[test]
fn short_opt_x2_pos() {
    check_complex_output("clap-test value -o some -o other", O2P);
}

#[test]
fn short_opt_eq_x2_pos() {
    check_complex_output("clap-test value -o=some -o=other", O2P);
}

#[test]
fn short_flag_x2_comb_short_opt_pos() {
    check_complex_output("clap-test value -ff -o some", F2OP);
}

#[test]
fn short_flag_short_opt_pos() {
    check_complex_output("clap-test value -f -o some", FOP);
}

#[test]
fn long_flag_long_opt_pos() {
    check_complex_output("clap-test value --flag --option some", FOP);
}

#[test]
fn long_flag_long_opt_eq_pos() {
    check_complex_output("clap-test value --flag --option=some", FOP);
}

#[test]
fn sc_long_flag_long_opt() {
    check_complex_output("clap-test subcmd value --flag --option some", SCFOP);
}

#[test]
fn sc_long_flag_short_opt_pos() {
    check_complex_output("clap-test subcmd value --flag -o some", SCFOP);
}

#[test]
fn sc_long_flag_long_opt_eq_pos() {
    check_complex_output("clap-test subcmd value --flag --option=some", SCFOP);
}

#[test]
fn sc_short_flag_long_opt_pos() {
    check_complex_output("clap-test subcmd value -f --option some", SCFOP);
}

#[test]
fn sc_short_flag_short_opt_pos() {
    check_complex_output("clap-test subcmd value -f -o some", SCFOP);
}

#[test]
fn sc_short_flag_short_opt_eq_pos() {
    check_complex_output("clap-test subcmd value -f -o=some", SCFOP);
}

#[test]
fn sc_short_flag_long_opt_eq_pos() {
    check_complex_output("clap-test subcmd value -f --option=some", SCFOP);
}

#[test]
fn sc_short_flag_x2_comb_long_opt_pos() {
    check_complex_output("clap-test subcmd value -ff --option some", SCF2OP);
}

#[test]
fn sc_short_flag_x2_comb_short_opt_pos() {
    check_complex_output("clap-test subcmd value -ff -o some", SCF2OP);
}

#[test]
fn sc_short_flag_x2_comb_long_opt_eq_pos() {
    check_complex_output("clap-test subcmd value -ff --option=some", SCF2OP);
}

#[test]
fn sc_short_flag_x2_comb_short_opt_eq_pos() {
    check_complex_output("clap-test subcmd value -ff -o=some", SCF2OP);
}

#[test]
fn sc_long_flag_x2_long_opt_pos() {
    check_complex_output("clap-test subcmd value --flag --flag --option some", SCF2OP);
}

#[test]
fn sc_long_flag_x2_short_opt_pos() {
    check_complex_output("clap-test subcmd value --flag --flag -o some", SCF2OP);
}

#[test]
fn sc_long_flag_x2_short_opt_eq_pos() {
    check_complex_output("clap-test subcmd value --flag --flag -o=some", SCF2OP);
}

#[test]
fn sc_long_flag_x2_long_opt_eq_pos() {
    check_complex_output("clap-test subcmd value --flag --flag --option=some", SCF2OP);
}

#[test]
fn sc_short_flag_x2_long_opt_pos() {
    check_complex_output("clap-test subcmd value -f -f --option some", SCF2OP);
}

#[test]
fn sc_short_flag_x2_short_opt_pos() {
    check_complex_output("clap-test subcmd value -f -f -o some", SCF2OP);
}

#[test]
fn sc_short_flag_x2_short_opt_eq_pos() {
    check_complex_output("clap-test subcmd value -f -f -o=some", SCF2OP);
}

#[test]
fn sc_short_flag_x2_long_opt_eq_pos() {
    check_complex_output("clap-test subcmd value -f -f --option=some", SCF2OP);
}

#[test]
fn mut_arg_all() {
    let mut cmd = utils::complex_app();
    let arg_names = cmd
        .get_arguments()
        .map(|a| a.get_id().clone())
        .filter(|a| a != "version" && a != "help")
        .collect::<Vec<_>>();

    for arg_name in arg_names {
        cmd = cmd.mut_arg(arg_name, |arg| arg.hide_possible_values(true));
    }
}

#[test]
fn mut_subcommand_all() {
    let cmd = utils::complex_app();

    assert_eq!(
        cmd.find_subcommand("subcmd")
            .unwrap()
            .is_disable_version_flag_set(),
        false
    );
    let cmd = cmd.mut_subcommand("subcmd", |subcmd| subcmd.disable_version_flag(true));
    assert_eq!(
        cmd.find_subcommand("subcmd")
            .unwrap()
            .is_disable_version_flag_set(),
        true
    );
}

#[test]
fn mut_subcommand_with_alias_resolve() {
    let mut cmd =
        Command::new("foo").subcommand(Command::new("bar").alias("baz").about("test subcmd"));
    assert_eq!(
        cmd.find_subcommand("baz")
            .unwrap()
            .get_about()
            .unwrap()
            .to_string(),
        "test subcmd"
    );

    let true_name = cmd.find_subcommand("baz").unwrap().get_name().to_string();
    assert_eq!(true_name, "bar");

    cmd = cmd.mut_subcommand(&*true_name, |subcmd| subcmd.about("modified about"));
    assert_eq!(
        cmd.find_subcommand("baz")
            .unwrap()
            .get_about()
            .unwrap()
            .to_string(),
        "modified about"
    );
}

#[test]
fn issue_3669_command_build_recurses() {
    let mut cmd = Command::new("ctest").subcommand(
        Command::new("subcmd").subcommand(
            Command::new("multi")
                .about("tests subcommands")
                .author("Kevin K. <kbknapp@gmail.com>")
                .version("0.1")
                .arg(clap::arg!(
                    <FLAG>                    "tests flags"
                )),
        ),
    );
    cmd.build();
}
