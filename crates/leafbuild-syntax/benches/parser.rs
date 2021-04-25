use criterion::{black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};

fn parse_speed(input: &str) {
    let _ = leafbuild_syntax::parser::parse(input);
}

fn simple_bench(b: &mut Bencher, i: &usize) {
    let s = "print('well ... yes, this is really \\' fast')\n".repeat(*i);
    b.iter(|| parse_speed(black_box(&s)));
}

fn simple_parse_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    for &input in [10, 100, 1000, 1000000].iter() {
        group.bench_with_input(BenchmarkId::new("parse", input), &input, simple_bench);
    }
}

criterion_group!(benches, simple_parse_group);
criterion_main!(benches);
