use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::macd::macd;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_macd(c: &mut Criterion) {
    let mut group = c.benchmark_group("macd");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let fast_periods = [12, 26, 9]; // Standard MACD periods
    let slow_periods = vec![26, 52, 18];
    let signal_periods = vec![9, 18, 6];

    for size in sizes {
        let input = generate_test_data(size);
        let mut macd_line = vec![0.0; size];
        let mut signal_line = vec![0.0; size];
        let mut histogram = vec![0.0; size];
        let mut fast_ema = vec![0.0; size];
        let mut slow_ema = vec![0.0; size];

        for ((fast_period, slow_period), signal_period) in
            fast_periods.iter().zip(&slow_periods).zip(&signal_periods)
        {
            group.bench_with_input(
                BenchmarkId::new(
                    format!(
                        "size_{size}_fast_{fast_period}_slow_{slow_period}_signal_{signal_period}"
                    ),
                    format!("{fast_period}-{slow_period}-{signal_period}"),
                ),
                &(*fast_period, *slow_period, *signal_period),
                |b, &(fast_period, slow_period, signal_period)| {
                    b.iter(|| {
                        let _ = macd(
                            black_box(&input),
                            black_box(fast_period),
                            black_box(slow_period),
                            black_box(signal_period),
                            black_box(&mut macd_line),
                            black_box(&mut signal_line),
                            black_box(&mut histogram),
                            black_box(&mut fast_ema),
                            black_box(&mut slow_ema),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_macd);
