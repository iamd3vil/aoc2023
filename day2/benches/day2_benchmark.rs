use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day2::process_input;

pub fn full_benchmark(c: &mut Criterion) {
    let lines = fs::read("test.txt").unwrap();
    c.bench_function("day2_bench", |b| {
        b.iter(|| process_input(black_box(&lines)))
    });
}

criterion_group!(benches, full_benchmark);
// criterion_group!(benches, bench_correct_digits);
criterion_main!(benches);
