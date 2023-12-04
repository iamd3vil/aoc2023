use std::fs::File;
use std::io::BufReader;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day2::process_input;

pub fn full_benchmark(c: &mut Criterion) {
    let f = File::open("test.txt").unwrap();
    let mut rdr = BufReader::new(f);
    c.bench_function("day2_bench", |b| {
        b.iter(|| process_input(black_box(&mut rdr)))
    });
}

criterion_group!(benches, full_benchmark);
// criterion_group!(benches, bench_correct_digits);
criterion_main!(benches);
