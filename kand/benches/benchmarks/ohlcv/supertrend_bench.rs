use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::supertrend::supertrend;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_supertrend(c: &mut Criterion) {
    let mut group = c.benchmark_group("supertrend");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];
    let multiplier = 3.0;

    for size in sizes {
        let input = generate_test_data(size);
        let mut trend = vec![0; size];
        let mut supertrend_output = vec![0.0; size];
        let mut atr = vec![0.0; size];
        let mut upper = vec![0.0; size];
        let mut lower = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = supertrend(
                            black_box(&input), // high
                            black_box(&input), // low
                            black_box(&input), // close
                            black_box(period),
                            black_box(multiplier),
                            black_box(&mut trend),
                            black_box(&mut supertrend_output),
                            black_box(&mut atr),
                            black_box(&mut upper),
                            black_box(&mut lower),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_supertrend);
