use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::stoch::stoch;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_stoch(c: &mut Criterion) {
    let mut group = c.benchmark_group("stoch");

    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let k_periods = [5, 14, 30];
    let k_slow_periods = vec![3, 5, 9];
    let d_periods = vec![3, 5, 9];

    for size in sizes {
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let input_close = generate_test_data(size);
        let mut output_fast_k = vec![0.0; size];
        let mut output_k = vec![0.0; size];
        let mut output_d = vec![0.0; size];

        for (&k_period, &k_slow_period, &d_period) in k_periods
            .iter()
            .zip(&k_slow_periods)
            .zip(&d_periods)
            .map(|((a, b), c)| (a, b, c))
        {
            group.bench_with_input(
                BenchmarkId::new(
                    format!("size_{size}_k{k_period}_ks{k_slow_period}_d{d_period}"),
                    format!("{k_period}-{k_slow_period}-{d_period}"),
                ),
                &(k_period, k_slow_period, d_period),
                |b, &(k_period, k_slow_period, d_period)| {
                    b.iter(|| {
                        let _ = stoch(
                            black_box(&input_high),
                            black_box(&input_low),
                            black_box(&input_close),
                            black_box(k_period),
                            black_box(k_slow_period),
                            black_box(d_period),
                            black_box(&mut output_fast_k),
                            black_box(&mut output_k),
                            black_box(&mut output_d),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_stoch);
