use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::medprice::medprice;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_medprice(c: &mut Criterion) {
    let mut group = c.benchmark_group("medprice");

    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let mut output_medprice = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = medprice(
                    black_box(&input_high),
                    black_box(&input_low),
                    black_box(&mut output_medprice),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_medprice);
