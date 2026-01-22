use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::cdl_marubozu::cdl_marubozu;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_cdl_marubozu(c: &mut Criterion) {
    let mut group = c.benchmark_group("cdl_marubozu");

    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input_open = generate_test_data(size);
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let input_close = generate_test_data(size);
        let mut output_signals = vec![0; size];
        let mut output_body_avg = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = cdl_marubozu(
                            black_box(&input_open),
                            black_box(&input_high),
                            black_box(&input_low),
                            black_box(&input_close),
                            black_box(period),
                            black_box(0.1),
                            black_box(&mut output_signals),
                            black_box(&mut output_body_avg),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_cdl_marubozu);
