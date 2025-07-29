use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::natr::natr;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_natr(c: &mut Criterion) {
    let mut group = c.benchmark_group("natr");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let high = generate_test_data(size);
        let low = generate_test_data(size);
        let close = generate_test_data(size);
        let mut output = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = natr(
                            black_box(&high),
                            black_box(&low),
                            black_box(&close),
                            black_box(period),
                            black_box(&mut output),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_natr);
