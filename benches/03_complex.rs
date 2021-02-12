use clap::{clap_app, App, AppSettings, Arg, ArgSettings};
use criterion::{criterion_group, criterion_main, Criterion};

static OPT3_VALS: [&str; 2] = ["fast", "slow"];
static POS3_VALS: [&str; 2] = ["vi", "emacs"];

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
                Arg::from("--multvals [one] [two] 'Tests multiple values, not mult occs'"),
                Arg::from("--multvalsmo... [one] [two] 'Tests multiple values, not mult occs'"),
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

pub fn build_from_usage(c: &mut Criterion) {
    c.bench_function("build_from_usage", |b| b.iter(|| create_app!()));
}

pub fn build_from_builder(c: &mut Criterion) {
    c.bench_function("build_from_builder", |b| {
        b.iter(|| {
            App::new("claptests")
                .version("0.1")
                .about("tests clap library")
                .author("Kevin K. <kbknapp@gmail.com>")
                .arg(
                    Arg::new("opt")
                        .about("tests options")
                        .short('o')
                        .long("option")
                        .setting(ArgSettings::MultipleValues)
                        .setting(ArgSettings::MultipleOccurrences),
                )
                .arg(Arg::new("positional").about("tests positionals").index(1))
                .arg(
                    Arg::new("flag")
                        .short('f')
                        .about("tests flags")
                        .long("flag")
                        .global(true)
                        .setting(ArgSettings::MultipleOccurrences),
                )
                .arg(
                    Arg::new("flag2")
                        .short('F')
                        .about("tests flags with exclusions")
                        .conflicts_with("flag")
                        .requires("option2"),
                )
                .arg(
                    Arg::new("option2")
                        .about("tests long options with exclusions")
                        .conflicts_with("option")
                        .requires("positional2")
                        .setting(ArgSettings::TakesValue)
                        .long("long-option-2"),
                )
                .arg(
                    Arg::new("positional2")
                        .index(3)
                        .about("tests positionals with exclusions"),
                )
                .arg(
                    Arg::new("option3")
                        .short('O')
                        .long("Option")
                        .setting(ArgSettings::TakesValue)
                        .about("tests options with specific value sets")
                        .possible_values(&OPT3_VALS),
                )
                .arg(
                    Arg::new("positional3")
                        .setting(ArgSettings::MultipleValues)
                        .setting(ArgSettings::MultipleOccurrences)
                        .about("tests positionals with specific values")
                        .index(4)
                        .possible_values(&POS3_VALS),
                )
                .arg(
                    Arg::new("multvals")
                        .long("multvals")
                        .about("Tests multiple values, not mult occs")
                        .value_names(&["one", "two"]),
                )
                .arg(
                    Arg::new("multvalsmo")
                        .long("multvalsmo")
                        .setting(ArgSettings::MultipleValues)
                        .setting(ArgSettings::MultipleOccurrences)
                        .about("Tests multiple values, not mult occs")
                        .value_names(&["one", "two"]),
                )
                .arg(
                    Arg::new("minvals")
                        .long("minvals2")
                        .setting(ArgSettings::MultipleValues)
                        .setting(ArgSettings::MultipleOccurrences)
                        .about("Tests 2 min vals")
                        .min_values(2),
                )
                .arg(
                    Arg::new("maxvals")
                        .long("maxvals3")
                        .setting(ArgSettings::MultipleValues)
                        .setting(ArgSettings::MultipleOccurrences)
                        .about("Tests 3 max vals")
                        .max_values(3),
                )
                .subcommand(
                    App::new("subcmd")
                        .about("tests subcommands")
                        .version("0.1")
                        .author("Kevin K. <kbknapp@gmail.com>")
                        .arg(
                            Arg::new("scoption")
                                .short('o')
                                .long("option")
                                .setting(ArgSettings::MultipleValues)
                                .setting(ArgSettings::MultipleOccurrences)
                                .about("tests options"),
                        )
                        .arg(Arg::new("scpositional").index(1).about("tests positionals")),
                )
        })
    });
}

pub fn build_from_macros(c: &mut Criterion) {
    c.bench_function("build_from_macros", |b| {
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
        })
    });
}

pub fn parse_complex(c: &mut Criterion) {
    c.bench_function("parse_complex", |b| {
        b.iter(|| create_app!().get_matches_from(vec![""]))
    });
}

pub fn parse_complex_with_flag(c: &mut Criterion) {
    c.bench_function("parse_complex_with_flag", |b| {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "-f"]))
    });
}

pub fn parse_complex_with_opt(c: &mut Criterion) {
    c.bench_function("parse_complex_with_opt", |b| {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "-o", "option1"]))
    });
}

pub fn parse_complex_with_pos(c: &mut Criterion) {
    c.bench_function("parse_complex_with_pos", |b| {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "arg1"]))
    });
}

pub fn parse_complex_with_sc(c: &mut Criterion) {
    c.bench_function("parse_complex_with_sc", |b| {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "subcmd"]))
    });
}

pub fn parse_complex_with_sc_flag(c: &mut Criterion) {
    c.bench_function("parse_complex_with_sc_flag", |b| {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "subcmd", "-f"]))
    });
}

pub fn parse_complex_with_sc_opt(c: &mut Criterion) {
    c.bench_function("parse_complex_with_sc_opt", |b| {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "subcmd", "-o", "option1"]))
    });
}

pub fn parse_complex_with_sc_pos(c: &mut Criterion) {
    c.bench_function("parse_complex_with_sc_pos", |b| {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "subcmd", "arg1"]))
    });
}

pub fn parse_complex1(c: &mut Criterion) {
    c.bench_function("parse_complex1", |b| {
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
        })
    });
}

pub fn parse_complex2(c: &mut Criterion) {
    c.bench_function("parse_complex2", |b| {
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
        })
    });
}

pub fn parse_args_negate_scs(c: &mut Criterion) {
    c.bench_function("parse_args_negate_scs", |b| {
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
        })
    });
}

pub fn parse_complex_with_sc_complex(c: &mut Criterion) {
    c.bench_function("parse_complex_with_sc_complex", |b| {
        b.iter(|| {
            create_app!().get_matches_from(vec!["myprog", "subcmd", "-f", "-o", "option1", "arg1"])
        })
    });
}

criterion_group!(
    benches,
    build_from_usage,
    build_from_builder,
    build_from_macros,
    parse_complex,
    parse_complex_with_flag,
    parse_complex_with_opt,
    parse_complex_with_pos,
    parse_complex_with_sc,
    parse_complex_with_sc_flag,
    parse_complex_with_sc_opt,
    parse_complex_with_sc_pos,
    parse_complex1,
    parse_complex2,
    parse_args_negate_scs,
    parse_complex_with_sc_complex
);

criterion_main!(benches);
