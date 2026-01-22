use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::sar::sar;
use std::hint::black_box;

use crate::helper::generate_test_data;
#[allow(dead_code)]
fn bench_sar(c: &mut Criterion) {
    let mut group = c.benchmark_group("sar");

    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let mut output_sar = vec![0.0; size];
        let mut output_is_long = vec![false; size];
        let mut output_af = vec![0.0; size];
        let mut output_ep = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = sar(
                    black_box(&input_high),
                    black_box(&input_low),
                    black_box(0.02),
                    black_box(0.2),
                    black_box(&mut output_sar),
                    black_box(&mut output_is_long),
                    black_box(&mut output_af),
                    black_box(&mut output_ep),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_sar);
