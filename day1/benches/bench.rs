use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = compress::aoc_input!("../input.compressed");
    c.bench_function("part1", |b| b.iter(|| day1::part1(black_box(&input))));
    c.bench_function("part2", |b| b.iter(|| day1::part2(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
