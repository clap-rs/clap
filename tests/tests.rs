#[macro_use]
extern crate clap;
extern crate regex;

use std::io::Write;
use std::str;

include!("../clap-test.rs");

use clap::{App, Arg};

static SCF2OP: &'static str = "flag present 2 times
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

static SCFOP: &'static str = "flag present 1 times
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

static O2P: &'static str = "flag NOT present
option present 2 times with value: some
An option: some
An option: other
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present 2 times with value: some
An option: some
An option: other
positional present with value: value
subcmd NOT present
";

static F2OP: &'static str = "flag present 2 times
option present 1 times with value: some
An option: some
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present 1 times with value: some
An option: some
positional present with value: value
subcmd NOT present
";

static FOP: &'static str = "flag present 1 times
option present 1 times with value: some
An option: some
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present 1 times with value: some
An option: some
positional present with value: value
subcmd NOT present
";

pub fn check_complex_output(args: &str, out: &str) {
    let mut w = vec![];
    let matches = test::complex_app().get_matches_from(args.split(' ').collect::<Vec<_>>());
    if matches.is_present("flag") {
        writeln!(w, "flag present {} times", matches.occurrences_of("flag")).unwrap();
    } else {
        writeln!(w, "flag NOT present").unwrap();
    }

    if matches.is_present("option") {
        if let Some(v) = matches.value_of("option") {
            writeln!(
                w,
                "option present {} times with value: {}",
                matches.occurrences_of("option"),
                v
            )
            .unwrap();
        }
        if let Some(ov) = matches.values_of("option") {
            for o in ov {
                writeln!(w, "An option: {}", o).unwrap();
            }
        }
    } else {
        writeln!(w, "option NOT present").unwrap();
    }

    if let Some(p) = matches.value_of("positional") {
        writeln!(w, "positional present with value: {}", p).unwrap();
    } else {
        writeln!(w, "positional NOT present").unwrap();
    }

    if matches.is_present("flag2") {
        writeln!(w, "flag2 present").unwrap();
        writeln!(
            w,
            "option2 present with value of: {}",
            matches.value_of("long-option-2").unwrap()
        )
        .unwrap();
        writeln!(
            w,
            "positional2 present with value of: {}",
            matches.value_of("positional2").unwrap()
        )
        .unwrap();
    } else {
        writeln!(w, "flag2 NOT present").unwrap();
        writeln!(
            w,
            "option2 maybe present with value of: {}",
            matches.value_of("long-option-2").unwrap_or("Nothing")
        )
        .unwrap();
        writeln!(
            w,
            "positional2 maybe present with value of: {}",
            matches.value_of("positional2").unwrap_or("Nothing")
        )
        .unwrap();
    }

    let _ = match matches.value_of("Option3").unwrap_or("") {
        "fast" => writeln!(w, "option3 present quickly"),
        "slow" => writeln!(w, "option3 present slowly"),
        _ => writeln!(w, "option3 NOT present"),
    };

    let _ = match matches.value_of("positional3").unwrap_or("") {
        "vi" => writeln!(w, "positional3 present in vi mode"),
        "emacs" => writeln!(w, "positional3 present in emacs mode"),
        _ => writeln!(w, "positional3 NOT present"),
    };

    if matches.is_present("option") {
        if let Some(v) = matches.value_of("option") {
            writeln!(
                w,
                "option present {} times with value: {}",
                matches.occurrences_of("option"),
                v
            )
            .unwrap();
        }
        if let Some(ov) = matches.values_of("option") {
            for o in ov {
                writeln!(w, "An option: {}", o).unwrap();
            }
        }
    } else {
        writeln!(w, "option NOT present").unwrap();
    }

    if let Some(p) = matches.value_of("positional") {
        writeln!(w, "positional present with value: {}", p).unwrap();
    } else {
        writeln!(w, "positional NOT present").unwrap();
    }
    if matches.is_present("subcmd") {
        writeln!(w, "subcmd present").unwrap();
        if let Some(matches) = matches.subcommand_matches("subcmd") {
            if matches.is_present("flag") {
                writeln!(w, "flag present {} times", matches.occurrences_of("flag")).unwrap();
            } else {
                writeln!(w, "flag NOT present").unwrap();
            }

            if matches.is_present("option") {
                if let Some(v) = matches.value_of("option") {
                    writeln!(w, "scoption present with value: {}", v).unwrap();
                }
                if let Some(ov) = matches.values_of("option") {
                    for o in ov {
                        writeln!(w, "An scoption: {}", o).unwrap();
                    }
                }
            } else {
                writeln!(w, "scoption NOT present").unwrap();
            }

            if let Some(p) = matches.value_of("scpositional") {
                writeln!(w, "scpositional present with value: {}", p).unwrap();
            }
        }
    } else {
        writeln!(w, "subcmd NOT present").unwrap();
    }

    let res = str::from_utf8(&w).unwrap();
    assert_eq!(res, out);
}

arg_enum! {
    #[derive(Debug)]
    enum Val1 {
        ValOne,
        ValTwo
    }
}
arg_enum! {
    #[derive(Debug)]
    pub enum Val2 {
        ValOne,
        ValTwo
    }
}
arg_enum! {
    enum Val3 {
        ValOne,
        ValTwo
    }
}
arg_enum! {
    pub enum Val4 {
        ValOne,
        ValTwo
    }
}

#[test]
fn test_enums() {
    let v1_lower = "valone";
    let v1_camel = "ValOne";

    let v1_lp = v1_lower.parse::<Val1>().unwrap();
    let v1_cp = v1_camel.parse::<Val1>().unwrap();
    match v1_lp {
        Val1::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val1::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val2>().unwrap();
    let v1_cp = v1_camel.parse::<Val2>().unwrap();
    match v1_lp {
        Val2::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val2::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val3>().unwrap();
    let v1_cp = v1_camel.parse::<Val3>().unwrap();
    match v1_lp {
        Val3::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val3::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val4>().unwrap();
    let v1_cp = v1_camel.parse::<Val4>().unwrap();
    match v1_lp {
        Val4::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val4::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
}

#[test]
fn create_app() {
    let _ = App::new("test")
        .version("1.0")
        .author("kevin")
        .about("does awesome things")
        .get_matches_from(vec![""]);
}

#[test]
fn add_multiple_arg() {
    let _ = App::new("test")
        .args(&[
            Arg::with_name("test").short('s'),
            Arg::with_name("test2").short('l'),
        ])
        .get_matches_from(vec![""]);
}
#[test]
fn flag_x2_opt() {
    check_complex_output(
        "clap-test value -f -f -o some",
        "flag present 2 times
option present 1 times with value: some
An option: some
positional present with value: value
flag2 NOT present
option2 maybe present with value of: Nothing
positional2 maybe present with value of: Nothing
option3 NOT present
positional3 NOT present
option present 1 times with value: some
An option: some
positional present with value: value
subcmd NOT present
",
    );
}

#[test]
fn long_opt_x2_pos() { check_complex_output("clap-test value --option some --option other", O2P); }

#[test]
fn long_opt_eq_x2_pos() {
    check_complex_output("clap-test value --option=some --option=other", O2P);
}

#[test]
fn short_opt_x2_pos() { check_complex_output("clap-test value -o some -o other", O2P); }

#[test]
fn short_opt_eq_x2_pos() { check_complex_output("clap-test value -o=some -o=other", O2P); }

#[test]
fn short_flag_x2_comb_short_opt_pos() { check_complex_output("clap-test value -ff -o some", F2OP); }

#[test]
fn short_flag_short_opt_pos() { check_complex_output("clap-test value -f -o some", FOP); }

#[test]
fn long_flag_long_opt_pos() { check_complex_output("clap-test value --flag --option some", FOP); }

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
