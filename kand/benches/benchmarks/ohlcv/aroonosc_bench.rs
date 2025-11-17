use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::aroonosc::aroonosc;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_aroonosc(c: &mut Criterion) {
    let mut group = c.benchmark_group("aroonosc");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let mut output_aroonosc = vec![0.0; size];
        let mut output_prev_high = vec![0.0; size];
        let mut output_prev_low = vec![0.0; size];
        let mut output_days_since_high = vec![0; size];
        let mut output_days_since_low = vec![0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = aroonosc(
                            black_box(&input_high),
                            black_box(&input_low),
                            black_box(period),
                            black_box(&mut output_aroonosc),
                            black_box(&mut output_prev_high),
                            black_box(&mut output_prev_low),
                            black_box(&mut output_days_since_high),
                            black_box(&mut output_days_since_low),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_aroonosc);
