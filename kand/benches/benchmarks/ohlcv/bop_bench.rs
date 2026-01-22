use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::bop::bop;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_bop(c: &mut Criterion) {
    let mut group = c.benchmark_group("bop");

    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let input_open = generate_test_data(size);
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let input_close = generate_test_data(size);
        let mut output_bop = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = bop(
                    black_box(&input_open),
                    black_box(&input_high),
                    black_box(&input_low),
                    black_box(&input_close),
                    black_box(&mut output_bop),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_bop);
