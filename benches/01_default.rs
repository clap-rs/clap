use clap::App;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn build_empty_app(c: &mut Criterion) {
    c.bench_function("build_empty_app", |b| b.iter(|| App::new("claptests")));
}

pub fn parse_empty_app(c: &mut Criterion) {
    c.bench_function("parse_empty_app", |b| {
        b.iter(|| App::new("claptests").get_matches_from(vec![""]))
    });
}

criterion_group!(benches, build_empty_app, parse_empty_app);
criterion_main!(benches);
