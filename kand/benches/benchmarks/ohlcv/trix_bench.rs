use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::trix::trix;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_trix(c: &mut Criterion) {
    let mut group = c.benchmark_group("trix");

    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input = generate_test_data(size);
        let mut output = vec![0.0; size];
        let mut ema1_output = vec![0.0; size];
        let mut ema2_output = vec![0.0; size];
        let mut ema3_output = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = trix(
                            black_box(&input),
                            black_box(period),
                            black_box(&mut output),
                            black_box(&mut ema1_output),
                            black_box(&mut ema2_output),
                            black_box(&mut ema3_output),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_trix);
