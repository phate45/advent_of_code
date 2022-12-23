use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_of_code_2022 as aoc;

pub fn bench_part2_imp(c: &mut Criterion) {
    c.bench_function("solve imp", |b| b.iter(|| aoc::part2_bench(black_box(true))));
}

pub fn bench_part2_rec(c: &mut Criterion) {
    c.bench_function("solve rec", |b| b.iter(|| aoc::part2_bench(black_box(false))));
}

criterion_group!(benches, bench_part2_imp, bench_part2_rec);
criterion_main!(benches);
