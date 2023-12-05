// use std::fs::File;
// use std::io::BufReader;
use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day3::process_input;

pub fn full_benchmark(c: &mut Criterion) {
    // let f = File::open("test.txt").unwrap();
    // let mut rdr = BufReader::new(f);
    let cnt = fs::read("test.txt").unwrap();
    c.bench_function("day3_bench", |b| b.iter(|| process_input(black_box(&cnt))));
}

criterion_group!(benches, full_benchmark);
// criterion_group!(benches, bench_correct_digits);
criterion_main!(benches);
