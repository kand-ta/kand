use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::ad::ad;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_ad(c: &mut Criterion) {
    let mut group = c.benchmark_group("AD");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let high = generate_test_data(size);
        let low = generate_test_data(size);
        let close = generate_test_data(size);
        let volume = generate_test_data(size);
        let mut output = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, _| {
            b.iter(|| {
                // Use black_box on the result to prevent LLVM from optimizing away the computation
                let _ = ad(
                    black_box(&high),
                    black_box(&low),
                    black_box(&close),
                    black_box(&volume),
                    black_box(&mut output),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_ad);
