use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::ecl::ecl;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
#[allow(clippy::similar_names)]
fn bench_ecl(c: &mut Criterion) {
    let mut group = c.benchmark_group("ecl");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let input_close = generate_test_data(size);
        let mut output_h5 = vec![0.0; size];
        let mut output_h4 = vec![0.0; size];
        let mut output_h3 = vec![0.0; size];
        let mut output_h2 = vec![0.0; size];
        let mut output_h1 = vec![0.0; size];
        let mut output_l1 = vec![0.0; size];
        let mut output_l2 = vec![0.0; size];
        let mut output_l3 = vec![0.0; size];
        let mut output_l4 = vec![0.0; size];
        let mut output_l5 = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = ecl(
                    black_box(&input_high),
                    black_box(&input_low),
                    black_box(&input_close),
                    black_box(&mut output_h5),
                    black_box(&mut output_h4),
                    black_box(&mut output_h3),
                    black_box(&mut output_h2),
                    black_box(&mut output_h1),
                    black_box(&mut output_l1),
                    black_box(&mut output_l2),
                    black_box(&mut output_l3),
                    black_box(&mut output_l4),
                    black_box(&mut output_l5),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_ecl);
