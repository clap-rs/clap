#![feature(test)]

#[macro_use]
extern crate clap;
extern crate test;

use clap::{App, AppSettings, Arg, ArgSettings};

use test::Bencher;

static OPT3_VALS: [&'static str; 2] = ["fast", "slow"];
static POS3_VALS: [&'static str; 2] = ["vi", "emacs"];

macro_rules! create_app {
    () => {{
        App::new("claptests")
            .version("0.1")
            .about("tests clap library")
            .author("Kevin K. <kbknapp@gmail.com>")
            .arg("-o --option=[opt]... 'tests options'")
            .arg("[positional] 'tests positionals'")
            .arg(Arg::from("-f --flag... 'tests flags'").global(true))
            .args(&[
                Arg::from("[flag2] -F 'tests flags with exclusions'")
                    .conflicts_with("flag")
                    .requires("option2"),
                Arg::from("--long-option-2 [option2] 'tests long options with exclusions'")
                    .conflicts_with("option")
                    .requires("positional2"),
                Arg::from("[positional2] 'tests positionals with exclusions'"),
                Arg::from("-O --Option [option3] 'tests options with specific value sets'")
                    .possible_values(&OPT3_VALS),
                Arg::from("[positional3]... 'tests positionals with specific values'")
                    .possible_values(&POS3_VALS),
                Arg::from("--multvals [one] [two] 'Tests mutliple values, not mult occs'"),
                Arg::from("--multvalsmo... [one] [two] 'Tests mutliple values, not mult occs'"),
                Arg::from("--minvals2 [minvals]... 'Tests 2 min vals'").min_values(2),
                Arg::from("--maxvals3 [maxvals]... 'Tests 3 max vals'").max_values(3),
            ])
            .subcommand(
                App::new("subcmd")
                    .about("tests subcommands")
                    .version("0.1")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .arg("-o --option [scoption]... 'tests options'")
                    .arg("[scpositional] 'tests positionals'"),
            )
    }};
}

#[bench]
fn create_app_from_usage(b: &mut Bencher) { b.iter(|| create_app!()); }

#[bench]
fn create_app_builder(b: &mut Bencher) {
    b.iter(|| {
        App::new("claptests")
            .version("0.1")
            .about("tests clap library")
            .author("Kevin K. <kbknapp@gmail.com>")
            .arg(
                Arg::with_name("opt")
                    .help("tests options")
                    .short('o')
                    .long("option")
                    .setting(ArgSettings::MultipleValues)
                    .setting(ArgSettings::MultipleOccurrences),
            )
            .arg(
                Arg::with_name("positional")
                    .help("tests positionals")
                    .index(1),
            )
            .arg(
                Arg::with_name("flag")
                    .short('f')
                    .help("tests flags")
                    .long("flag")
                    .global(true)
                    .settings(&[ArgSettings::MultipleOccurrences]),
            )
            .arg(
                Arg::with_name("flag2")
                    .short('F')
                    .help("tests flags with exclusions")
                    .conflicts_with("flag")
                    .requires("option2"),
            )
            .arg(
                Arg::with_name("option2")
                    .help("tests long options with exclusions")
                    .conflicts_with("option")
                    .requires("positional2")
                    .setting(ArgSettings::TakesValue)
                    .long("long-option-2"),
            )
            .arg(
                Arg::with_name("positional2")
                    .index(3)
                    .help("tests positionals with exclusions"),
            )
            .arg(
                Arg::with_name("option3")
                    .short('O')
                    .long("Option")
                    .setting(ArgSettings::TakesValue)
                    .help("tests options with specific value sets")
                    .possible_values(&OPT3_VALS),
            )
            .arg(
                Arg::with_name("positional3")
                    .setting(ArgSettings::MultipleValues)
                    .setting(ArgSettings::MultipleOccurrences)
                    .help("tests positionals with specific values")
                    .index(4)
                    .possible_values(&POS3_VALS),
            )
            .arg(
                Arg::with_name("multvals")
                    .long("multvals")
                    .help("Tests mutliple values, not mult occs")
                    .value_names(&["one", "two"]),
            )
            .arg(
                Arg::with_name("multvalsmo")
                    .long("multvalsmo")
                    .setting(ArgSettings::MultipleValues)
                    .setting(ArgSettings::MultipleOccurrences)
                    .help("Tests mutliple values, not mult occs")
                    .value_names(&["one", "two"]),
            )
            .arg(
                Arg::with_name("minvals")
                    .long("minvals2")
                    .setting(ArgSettings::MultipleValues)
                    .setting(ArgSettings::MultipleOccurrences)
                    .help("Tests 2 min vals")
                    .min_values(2),
            )
            .arg(
                Arg::with_name("maxvals")
                    .long("maxvals3")
                    .setting(ArgSettings::MultipleValues)
                    .setting(ArgSettings::MultipleOccurrences)
                    .help("Tests 3 max vals")
                    .max_values(3),
            )
            .subcommand(
                App::new("subcmd")
                    .about("tests subcommands")
                    .version("0.1")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .arg(
                        Arg::with_name("scoption")
                            .short('o')
                            .long("option")
                            .setting(ArgSettings::MultipleValues)
                            .setting(ArgSettings::MultipleOccurrences)
                            .help("tests options"),
                    )
                    .arg(
                        Arg::with_name("scpositional")
                            .index(1)
                            .help("tests positionals"),
                    ),
            );
    });
}

#[bench]
fn create_app_macros(b: &mut Bencher) {
    b.iter(|| {
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
                    "Tests mutliple values, not mult occs")
                (@arg multvalsmo: --multvalsmo ... +takes_value value_name[one two]
                    "Tests mutliple values, not mult occs")
                (@arg minvals: --minvals2 min_values(1) ... +takes_value "Tests 2 min vals")
                (@arg maxvals: --maxvals3 ... +takes_value max_values(3) "Tests 3 max vals")
                (@subcommand subcmd =>
                    (about: "tests subcommands")
                    (version: "0.1")
                    (author: "Kevin K. <kbknapp@gmail.com>")
                    (@arg scoption: -o --option ... +takes_value "tests options")
                    (@arg scpositional: index(1) "tests positionals"))
        );
    });
}

#[bench]
fn parse_clean(b: &mut Bencher) { b.iter(|| create_app!().get_matches_from(vec![""])); }

#[bench]
fn parse_flag(b: &mut Bencher) { b.iter(|| create_app!().get_matches_from(vec!["myprog", "-f"])); }

#[bench]
fn parse_option(b: &mut Bencher) {
    b.iter(|| create_app!().get_matches_from(vec!["myprog", "-o", "option1"]));
}

#[bench]
fn parse_positional(b: &mut Bencher) {
    b.iter(|| create_app!().get_matches_from(vec!["myprog", "arg1"]));
}

#[bench]
fn parse_sc_clean(b: &mut Bencher) {
    b.iter(|| create_app!().get_matches_from(vec!["myprog", "subcmd"]));
}

#[bench]
fn parse_sc_flag(b: &mut Bencher) {
    b.iter(|| create_app!().get_matches_from(vec!["myprog", "subcmd", "-f"]));
}

#[bench]
fn parse_sc_option(b: &mut Bencher) {
    b.iter(|| create_app!().get_matches_from(vec!["myprog", "subcmd", "-o", "option1"]));
}

#[bench]
fn parse_sc_positional(b: &mut Bencher) {
    b.iter(|| create_app!().get_matches_from(vec!["myprog", "subcmd", "arg1"]));
}

#[bench]
fn parse_complex1(b: &mut Bencher) {
    b.iter(|| {
        create_app!().get_matches_from(vec![
            "myprog",
            "-ff",
            "-o",
            "option1",
            "arg1",
            "-O",
            "fast",
            "arg2",
            "--multvals",
            "one",
            "two",
            "emacs",
        ])
    });
}

#[bench]
fn parse_complex2(b: &mut Bencher) {
    b.iter(|| {
        create_app!().get_matches_from(vec![
            "myprog",
            "arg1",
            "-f",
            "arg2",
            "--long-option-2",
            "some",
            "-O",
            "slow",
            "--multvalsmo",
            "one",
            "two",
            "--minvals2",
            "3",
            "2",
            "1",
        ])
    });
}

#[bench]
fn parse_complex2_with_args_negate_scs(b: &mut Bencher) {
    b.iter(|| {
        create_app!()
            .setting(AppSettings::ArgsNegateSubcommands)
            .get_matches_from(vec![
                "myprog",
                "arg1",
                "-f",
                "arg2",
                "--long-option-2",
                "some",
                "-O",
                "slow",
                "--multvalsmo",
                "one",
                "two",
                "--minvals2",
                "3",
                "2",
                "1",
            ])
    });
}

#[bench]
fn parse_sc_complex(b: &mut Bencher) {
    b.iter(|| {
        create_app!().get_matches_from(vec!["myprog", "subcmd", "-f", "-o", "option1", "arg1"])
    });
}
