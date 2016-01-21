#![feature(test)]

extern crate clap;
extern crate test;

use clap::App;

use test::Bencher;

macro_rules! create_app {
    () => ({
        App::new("claptests")
                .version("0.1")
                .about("tests clap library")
                .author("Kevin K. <kbknapp@gmail.com>")
                .args_from_usage("-f --flag         'tests flags'
                                  -o --option=[opt] 'tests options'
                                  [positional]      'tests positional'")
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

#[bench]
fn parse_flag(b: &mut Bencher) {
    b.iter(|| create_app!().get_matches_from(vec!["myprog", "-f"]));
}

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
