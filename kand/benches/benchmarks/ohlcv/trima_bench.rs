use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::trima::trima;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_trima(c: &mut Criterion) {
    let mut group = c.benchmark_group("trima");

    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input = generate_test_data(size);
        let mut output_sma1 = vec![0.0; size];
        let mut output_sma2 = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = trima(
                            black_box(&input),
                            black_box(period),
                            black_box(&mut output_sma1),
                            black_box(&mut output_sma2),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_trima);
