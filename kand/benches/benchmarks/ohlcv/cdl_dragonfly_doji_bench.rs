use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::cdl_dragonfly_doji::cdl_dragonfly_doji;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_cdl_dragonfly_doji(c: &mut Criterion) {
    let mut group = c.benchmark_group("cdl_dragonfly_doji");

    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let input_open = generate_test_data(size);
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let input_close = generate_test_data(size);
        let mut output_signals = vec![0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = cdl_dragonfly_doji(
                    black_box(&input_open),
                    black_box(&input_high),
                    black_box(&input_low),
                    black_box(&input_close),
                    black_box(0.1),
                    black_box(&mut output_signals),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_cdl_dragonfly_doji);
