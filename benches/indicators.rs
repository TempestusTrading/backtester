use backtester::{indicators::*, timeseries::TimeSeries};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use std::fs;

const DATASETS_DIR: &str = "./benches/datasets";

pub fn sma(c: &mut Criterion) {
    let mut group = c.benchmark_group("indicators");

    let entries = fs::read_dir(DATASETS_DIR).expect("datasets directory not found");
    for entry in entries {
        let entry = entry.expect("Failed to read entry").path();
        let filename = entry
            .file_name()
            .expect("Failed to get filename")
            .to_string_lossy();
        group.bench_with_input(BenchmarkId::from_parameter(filename), &entry, |b, entry| {
            b.iter(|| {
                let timeseries = TimeSeries::from_csv(entry);
                let mut sma = SMA::new(20);
                for ticker in timeseries {
                    sma.update(&ticker.unwrap()).expect("Failed to update sma");
                }
            })
        });
    }
    group.finish();
}

criterion_group!(benches, sma);
criterion_main!(benches);
