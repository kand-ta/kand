use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::mfi::mfi;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_mfi(c: &mut Criterion) {
    let mut group = c.benchmark_group("mfi");

    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let input_close = generate_test_data(size);
        let input_volume = generate_test_data(size);
        let mut output_mfi = vec![0.0; size];
        let mut output_typ_prices = vec![0.0; size];
        let mut output_money_flows = vec![0.0; size];
        let mut output_pos_flows = vec![0.0; size];
        let mut output_neg_flows = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = mfi(
                            black_box(&input_high),
                            black_box(&input_low),
                            black_box(&input_close),
                            black_box(&input_volume),
                            black_box(period),
                            black_box(&mut output_mfi),
                            black_box(&mut output_typ_prices),
                            black_box(&mut output_money_flows),
                            black_box(&mut output_pos_flows),
                            black_box(&mut output_neg_flows),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_mfi);
