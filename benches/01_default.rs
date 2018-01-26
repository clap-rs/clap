#![feature(test)]

extern crate clap;
extern crate test;

use clap::App;

use test::Bencher;

#[bench]
fn build_app(b: &mut Bencher) { b.iter(|| App::new("claptests")); }

#[bench]
fn parse_clean(b: &mut Bencher) { b.iter(|| App::new("claptests").get_matches_from(vec![""])); }
