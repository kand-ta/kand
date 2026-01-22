use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::midpoint::midpoint;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_midpoint(c: &mut Criterion) {
    let mut group = c.benchmark_group("midpoint");

    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input_price = generate_test_data(size);
        let mut output_midpoint = vec![0.0; size];
        let mut output_highest = vec![0.0; size];
        let mut output_lowest = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = midpoint(
                            black_box(&input_price),
                            black_box(period),
                            black_box(&mut output_midpoint),
                            black_box(&mut output_highest),
                            black_box(&mut output_lowest),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_midpoint);
