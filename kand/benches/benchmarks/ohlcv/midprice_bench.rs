use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::midprice::midprice;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_midprice(c: &mut Criterion) {
    let mut group = c.benchmark_group("midprice");

    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let mut output_midprice = vec![0.0; size];
        let mut output_highest_high = vec![0.0; size];
        let mut output_lowest_low = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = midprice(
                            black_box(&input_high),
                            black_box(&input_low),
                            black_box(period),
                            black_box(&mut output_midprice),
                            black_box(&mut output_highest_high),
                            black_box(&mut output_lowest_low),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_midprice);
