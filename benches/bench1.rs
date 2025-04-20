// use examplerust::source1;
use examplerust::source1;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("examplerust20", |b| b.iter(||
        source1::example_bench1(black_box(20))
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
