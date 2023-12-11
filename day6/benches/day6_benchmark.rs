use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day6::process_input;

pub fn full_benchmark(c: &mut Criterion) {
    let cnt = fs::read("test.txt").unwrap();
    c.bench_function("day6_bench", |b| b.iter(|| process_input(black_box(&cnt))));
}

criterion_group!(benches, full_benchmark);
criterion_main!(benches);
