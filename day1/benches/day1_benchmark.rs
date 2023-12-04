use std::fs::File;
use std::io::BufReader;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day1::{correct_digits_in_line, process_input};

pub fn full_benchmark(c: &mut Criterion) {
    let f = File::open("test.txt").unwrap();
    let mut rdr = BufReader::new(f);
    c.bench_function("fib 20", |b| b.iter(|| process_input(black_box(&mut rdr))));
}

pub fn bench_correct_digits(c: &mut Criterion) {
    let line = "xtqtwoneeightlvcjqfourckfour2nine";
    c.bench_function("bench_correct_digits", |b| {
        b.iter(|| correct_digits_in_line(black_box(line)))
    });
}

criterion_group!(benches, full_benchmark);
// criterion_group!(benches, bench_correct_digits);
criterion_main!(benches);
