use clap::{App, Arg};
use criterion::{criterion_group, criterion_main, Criterion};

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

pub fn build_app(c: &mut Criterion) {
    c.bench_function("build_app", |b| b.iter(|| create_app!()));
}

pub fn add_flag(c: &mut Criterion) {
    c.bench_function("add_flag", |b| b.iter(|| {
        App::new("claptests").arg(Arg::from("-s, --some 'something'"))
    }));
}

pub fn add_flag_ref(c: &mut Criterion) {
    c.bench_function("add_flag_ref", |b| b.iter(|| {
        let arg = Arg::from("-s, --some 'something'");
        App::new("claptests").arg(&arg)
    }));
}

pub fn add_opt(c: &mut Criterion) {
    c.bench_function("add_opt", |b| b.iter(|| {
        App::new("claptests").arg(Arg::from("-s, --some <FILE> 'something'"))
    }));
}

pub fn add_opt_ref(c: &mut Criterion) {
    c.bench_function("add_opt_ref", |b| b.iter(|| {
        let arg = Arg::from("-s, --some <FILE> 'something'");
        App::new("claptests").arg(&arg)
    }));
}

pub fn add_pos(c: &mut Criterion) {
    c.bench_function("add_pos", |b| b.iter(|| App::new("claptests").arg(Arg::with_name("some"))));
}

pub fn add_pos_ref(c: &mut Criterion) {
    c.bench_function("add_pos_ref", |b| b.iter(|| {
        let arg = Arg::with_name("some");
        App::new("claptests").arg(&arg)
    }));
}

pub fn parse_flag(c: &mut Criterion) {
    c.bench_function("parse_flag", |b| b.iter(|| {
        create_app!().get_matches_from(vec!["myprog", "-f"])
    }));
}

pub fn parse_option(c: &mut Criterion) {
    c.bench_function("parse_option", |b| b.iter(|| {
        create_app!().get_matches_from(vec!["myprog", "-o", "option1"])
    }));
}

pub fn parse_positional(c: &mut Criterion) {
    c.bench_function("parse_positional", |b| b.iter(|| {
        create_app!().get_matches_from(vec!["myprog", "arg1"])
    }));
}

pub fn parse_complex(c: &mut Criterion) {
    c.bench_function("parse_complex", |b| b.iter(|| {
        create_app!().get_matches_from(vec!["myprog", "-o", "option1", "-f", "arg1"])
    }));
}

criterion_group!(benches,
    parse_complex,
    parse_positional,
    parse_option,
    parse_flag,
    add_pos_ref,
    add_pos,
    add_opt_ref,
    add_opt,
    add_flag_ref,
    add_flag,
    build_app
);

criterion_main!(benches);
