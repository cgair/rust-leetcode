use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use rust_leetcode::fibonacci;
use rust_leetcode::test_reverse_list;

pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("reverse_list [1, 2, 3, 4]", |b| b.iter(|| test_reverse_list()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);