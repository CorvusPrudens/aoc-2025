use aoc_2025::days;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("1 p1", |b| {
        let input = include_str!("../inputs/one.txt");
        b.iter(|| {
            let result = days::one::part_one(input);
            black_box(result);
        })
    });

    c.bench_function("1 p2", |b| {
        let input = include_str!("../inputs/one.txt");
        b.iter(|| {
            let result = days::one::part_two(input);
            black_box(result);
        })
    });

    c.bench_function("2 p1", |b| {
        let input = include_str!("../inputs/two.txt");
        b.iter(|| {
            let result = days::two::part_one(input);
            black_box(result);
        })
    });

    c.bench_function("2 p2", |b| {
        let input = include_str!("../inputs/two.txt");
        b.iter(|| {
            let result = days::two::part_two(input);
            black_box(result);
        })
    });

    c.bench_function("3 p1", |b| {
        let input = include_str!("../inputs/three.txt");
        b.iter(|| {
            let result = days::three::part_one(input);
            black_box(result);
        })
    });

    c.bench_function("3 p2", |b| {
        let input = include_str!("../inputs/three.txt");
        b.iter(|| {
            let result = days::three::part_two(input);
            black_box(result);
        })
    });

    c.bench_function("4 p1", |b| {
        let input = include_str!("../inputs/four.txt");
        b.iter(|| {
            let result = days::four::part_one(input);
            black_box(result);
        })
    });

    c.bench_function("4 p2", |b| {
        let input = include_str!("../inputs/four.txt");
        b.iter(|| {
            let result = days::four::part_two(input);
            black_box(result);
        })
    });

    c.bench_function("5 p1", |b| {
        let input = include_str!("../inputs/five.txt");
        b.iter(|| {
            let result = days::five::part_one(input);
            black_box(result);
        })
    });

    c.bench_function("5 p2", |b| {
        let input = include_str!("../inputs/five.txt");
        b.iter(|| {
            let result = days::five::part_two(input);
            black_box(result);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
