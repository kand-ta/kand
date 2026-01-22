use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::adosc::adosc;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_adosc(c: &mut Criterion) {
    let mut group = c.benchmark_group("adosc");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let fast_periods = [3, 5, 10];
    let slow_periods = [10, 20, 30];

    for size in sizes {
        let high = generate_test_data(size);
        let low = generate_test_data(size);
        let close = generate_test_data(size);
        let volume = generate_test_data(size);
        let mut output_adosc = vec![0.0; size];
        let mut output_ad = vec![0.0; size];
        let mut output_fast_ema = vec![0.0; size];
        let mut output_slow_ema = vec![0.0; size];

        for (fast_period, slow_period) in fast_periods.iter().zip(slow_periods.iter()) {
            group.bench_with_input(
                BenchmarkId::new(
                    format!("size_{size}_fast_{fast_period}_slow_{slow_period}"),
                    format!("{fast_period}-{slow_period}"),
                ),
                &(fast_period, slow_period),
                |b, &(fast_period, slow_period)| {
                    b.iter(|| {
                        let _ = adosc(
                            black_box(&high),
                            black_box(&low),
                            black_box(&close),
                            black_box(&volume),
                            black_box(*fast_period),
                            black_box(*slow_period),
                            black_box(&mut output_adosc),
                            black_box(&mut output_ad),
                            black_box(&mut output_fast_ema),
                            black_box(&mut output_slow_ema),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_adosc);
