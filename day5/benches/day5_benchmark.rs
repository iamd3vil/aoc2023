use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day5::process_input;

pub fn full_benchmark(c: &mut Criterion) {
    let cnt = fs::read("test.txt").unwrap();
    c.bench_function("day3_bench", |b| b.iter(|| process_input(black_box(&cnt))));
}

// pub fn get_seeds_bench(c: &mut Criterion) {
//     let mut grp = c.benchmark_group("get_seeds");

//     let line = "seeds: 79 14 55 13";

//     grp.bench_function("get_seeds", |b| b.iter(|| get_seeds(black_box(line))));
// }

criterion_group!(benches, full_benchmark);
criterion_main!(benches);
