#![feature(test)]

extern crate clap;
extern crate test;

use clap::{App, Arg, SubCommand};

use test::Bencher;

static M_VAL_NAMES: [&'static str; 2] = ["one", "two"];
static ARGS: &'static str = "-o --option=[opt]... 'tests options'
                             [positional] 'tests positionals'";
static OPT3_VALS: [&'static str; 2] = ["fast", "slow"];
static POS3_VALS: [&'static str; 2] = ["vi", "emacs"];

macro_rules! create_app {
    () => ({
        App::new("claptests")
                .version("0.1")
                .about("tests clap library")
                .author("Kevin K. <kbknapp@gmail.com>")
                .args_from_usage(ARGS)
                .arg(Arg::from_usage("-f --flag... 'tests flags'")
                             .global(true))
                .args(vec![
                          Arg::from_usage("[flag2] -F 'tests flags with exclusions'").conflicts_with("flag").requires("option2"),
                          Arg::from_usage("--long-option-2 [option2] 'tests long options with exclusions'").conflicts_with("option").requires("positional2"),
                          Arg::from_usage("[positional2] 'tests positionals with exclusions'"),
                          Arg::from_usage("-O --Option [option3] 'tests options with specific value sets'").possible_values(&OPT3_VALS),
                          Arg::from_usage("[positional3]... 'tests positionals with specific values'").possible_values(&POS3_VALS),
                          Arg::from_usage("--multvals [multvals] 'Tests mutliple values, not mult occs'").value_names(&M_VAL_NAMES),
                          Arg::from_usage("--multvalsmo [multvalsmo]... 'Tests mutliple values, not mult occs'").value_names(&M_VAL_NAMES),
                          Arg::from_usage("--minvals2 [minvals]... 'Tests 2 min vals'").min_values(2),
                          Arg::from_usage("--maxvals3 [maxvals]... 'Tests 3 max vals'").max_values(3)
                    ])
                .subcommand(SubCommand::with_name("subcmd")
                                        .about("tests subcommands")
                                        .version("0.1")
                                        .author("Kevin K. <kbknapp@gmail.com>")
                                        .arg_from_usage("-o --option [scoption]... 'tests options'")
                                        .arg_from_usage("[scpositional] 'tests positionals'"))
    })
}

#[bench]
fn build_app(b: &mut Bencher) {

    b.iter(|| create_app!());
}

#[bench]
fn parse_clean(b: &mut Bencher) {
    b.iter(|| create_app!().get_matches_from(vec![""]));
}

