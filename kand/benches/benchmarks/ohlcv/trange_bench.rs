use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::trange::trange;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_trange(c: &mut Criterion) {
    let mut group = c.benchmark_group("trange");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let high = generate_test_data(size);
        let low = generate_test_data(size);
        let close = generate_test_data(size);
        let mut output = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = trange(
                    black_box(&high),
                    black_box(&low),
                    black_box(&close),
                    black_box(&mut output),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_trange);
