#![feature(test)]

extern crate clap;
extern crate test;

use clap::{App, Arg};

use test::Bencher;

macro_rules! create_app {
    () => {{
        App::new("claptests")
            .version("0.1")
            .about("tests clap library")
            .author("Kevin K. <kbknapp@gmail.com>")
            .arg("-f --flag         'tests flags'")
            .arg("-o --option=[opt] 'tests options'")
            .arg("[positional]      'tests positional'")
    }};
}

#[bench]
fn build_app(b: &mut Bencher) { b.iter(|| create_app!()); }

#[bench]
fn add_flag(b: &mut Bencher) {
    fn build_app() -> App<'static> { App::new("claptests") }

    b.iter(|| build_app().arg(Arg::from("-s, --some 'something'")));
}

#[bench]
fn add_flag_ref(b: &mut Bencher) {
    fn build_app() -> App<'static> { App::new("claptests") }

    b.iter(|| {
        let arg = Arg::from("-s, --some 'something'");
        build_app().arg(&arg)
    });
}

#[bench]
fn add_opt(b: &mut Bencher) {
    fn build_app() -> App<'static> { App::new("claptests") }

    b.iter(|| build_app().arg(Arg::from("-s, --some <FILE> 'something'")));
}

#[bench]
fn add_opt_ref(b: &mut Bencher) {
    fn build_app() -> App<'static> { App::new("claptests") }

    b.iter(|| {
        let arg = Arg::from("-s, --some <FILE> 'something'");
        build_app().arg(&arg)
    });
}

#[bench]
fn add_pos(b: &mut Bencher) {
    fn build_app() -> App<'static> { App::new("claptests") }

    b.iter(|| build_app().arg(Arg::with_name("some")));
}

#[bench]
fn add_pos_ref(b: &mut Bencher) {
    fn build_app() -> App<'static> { App::new("claptests") }

    b.iter(|| {
        let arg = Arg::with_name("some");
        build_app().arg(&arg)
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
fn parse_complex(b: &mut Bencher) {
    b.iter(|| create_app!().get_matches_from(vec!["myprog", "-o", "option1", "-f", "arg1"]));
}
