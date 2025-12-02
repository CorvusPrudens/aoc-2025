use aoc_2025::days;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("1 p1", |b| {
        let input = include_str!("../inputs/one-p1.txt");
        b.iter(|| {
            let result = days::one::part_one(input);
            black_box(result);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
