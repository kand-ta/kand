use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::tema::tema;
use std::hint::black_box;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_tema(c: &mut Criterion) {
    let mut group = c.benchmark_group("tema");
    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input = generate_test_data(size);
        let mut output = vec![0.0; size];
        let mut output_ema1 = vec![0.0; size];
        let mut output_ema2 = vec![0.0; size];
        let mut output_ema3 = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = tema(
                            black_box(&input),
                            black_box(period),
                            black_box(&mut output),
                            black_box(&mut output_ema1),
                            black_box(&mut output_ema2),
                            black_box(&mut output_ema3),
                        );
                    });
                },
            );
        }
    }
    group.finish();
}

criterion_group!(ohlcv, bench_tema);
