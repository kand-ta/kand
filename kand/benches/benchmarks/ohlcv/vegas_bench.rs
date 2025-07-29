use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::vegas::vegas;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_vegas(c: &mut Criterion) {
    let mut group = c.benchmark_group("vegas");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let input = generate_test_data(size);
        let mut output_channel_upper = vec![0.0; size];
        let mut output_channel_lower = vec![0.0; size];
        let mut output_boundary_upper = vec![0.0; size];
        let mut output_boundary_lower = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = vegas(
                    black_box(&input),
                    black_box(&mut output_channel_upper),
                    black_box(&mut output_channel_lower),
                    black_box(&mut output_boundary_upper),
                    black_box(&mut output_boundary_lower),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_vegas);
