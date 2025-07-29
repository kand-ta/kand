use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::bbands::bbands;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_bbands(c: &mut Criterion) {
    let mut group = c.benchmark_group("bbands");

    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input_price = generate_test_data(size);
        let mut output_upper = vec![0.0; size];
        let mut output_middle = vec![0.0; size];
        let mut output_lower = vec![0.0; size];
        let mut output_sma = vec![0.0; size];
        let mut output_var = vec![0.0; size];
        let mut output_sum = vec![0.0; size];
        let mut output_sum_sq = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = bbands(
                            black_box(&input_price),
                            black_box(period),
                            black_box(2.0),
                            black_box(2.0),
                            black_box(&mut output_upper),
                            black_box(&mut output_middle),
                            black_box(&mut output_lower),
                            black_box(&mut output_sma),
                            black_box(&mut output_var),
                            black_box(&mut output_sum),
                            black_box(&mut output_sum_sq),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_bbands);
