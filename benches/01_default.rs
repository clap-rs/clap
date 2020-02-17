use clap::App;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn empty_app(c: &mut Criterion) {
    c.bench_function("build_app", |b| b.iter(|| App::new("claptests")));
}

pub fn parse_clean(c: &mut Criterion) {
    c.bench_function("parse_clean", |b| b.iter(|| App::new("claptests").get_matches_from(vec![""])));
}

criterion_group!(benches, empty_app, parse_clean);
criterion_main!(benches);
