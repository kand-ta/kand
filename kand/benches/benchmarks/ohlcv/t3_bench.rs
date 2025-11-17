use criterion::{BenchmarkId, Criterion, criterion_group};
use kand::ohlcv::t3::t3;
use std::hint::black_box;

use crate::helper::generate_test_data;
#[allow(dead_code)]
fn bench_t3(c: &mut Criterion) {
    let mut group = c.benchmark_group("t3");

    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];
    let vfactors = vec![0.5, 0.7, 0.9];

    for size in sizes {
        let input = generate_test_data(size);
        let mut output = vec![0.0; size];
        let mut ema1 = vec![0.0; size];
        let mut ema2 = vec![0.0; size];
        let mut ema3 = vec![0.0; size];
        let mut ema4 = vec![0.0; size];
        let mut ema5 = vec![0.0; size];
        let mut ema6 = vec![0.0; size];

        for period in &periods {
            for vfactor in &vfactors {
                group.bench_with_input(
                    BenchmarkId::new(format!("size_{size}_vfactor_{vfactor:.1}"), period),
                    &(*period, *vfactor),
                    |b, &(period, vfactor)| {
                        b.iter(|| {
                            let _ = t3(
                                black_box(&input),
                                black_box(period),
                                black_box(vfactor),
                                black_box(&mut output),
                                black_box(&mut ema1),
                                black_box(&mut ema2),
                                black_box(&mut ema3),
                                black_box(&mut ema4),
                                black_box(&mut ema5),
                                black_box(&mut ema6),
                            );
                        });
                    },
                );
            }
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_t3);
