use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::vwap::vwap;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_vwap(c: &mut Criterion) {
    let mut group = c.benchmark_group("vwap");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let high = generate_test_data(size);
        let low = generate_test_data(size);
        let close = generate_test_data(size);
        let volume = generate_test_data(size);
        let mut output_vwap = vec![0.0; size];
        let mut output_cum_pv = vec![0.0; size];
        let mut output_cum_vol = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = vwap(
                    black_box(&high),
                    black_box(&low),
                    black_box(&close),
                    black_box(&volume),
                    black_box(&mut output_vwap),
                    black_box(&mut output_cum_pv),
                    black_box(&mut output_cum_vol),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_vwap);
