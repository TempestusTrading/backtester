use backtester::{
    indicators::*, 
    timeseries::TimeSeries,
    series::Series
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use std::fs;

const TIMESERIES_DIR: &str = "./benches/datasets/timeseries";
// const EFFR_PATH: &str = "./benches/datasets/indicators/DFF.csv";

// Testing the performance of reading an indicator from disc.
// pub fn effr(c: &mut Criterion) {
//     let mut group = c.benchmark_group("indicators");

//     let effr_feed = Series<DDF>::from_csv(EFFR_PATH);
//     let entries = fs::read_dir(TIMESERIES_DIR).expect("datasets directory not found");
//     for entry in entries {
//         let entry = entry.expect("Failed to read entry").path();
//         let filename = entry
//             .file_name()
//             .expect("Failed to get filename")
//             .to_string_lossy();
//         group.bench_with_input(BenchmarkId::from_parameter(filename), &entry, |b, entry| {
//             b.iter(|| {
//                 let timeseries = TimeSeries::from_csv(entry);
//                 let mut effr = EFFR::new(effr_feed.clone());
//                 for ticker in timeseries {
//                     effr.update(&ticker.unwrap()).expect("Failed to update sma");
//                 }
//             })
//         });
//     }
   
//     group.finish();
// }

// Testing the performance of a live indicator that is computed from a dataset
pub fn sma(c: &mut Criterion) {
    let mut group = c.benchmark_group("indicators");

    let entries = fs::read_dir(TIMESERIES_DIR).expect("datasets directory not found");
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

// criterion_group!(benches, effr);
criterion_group!(benches, sma);
criterion_main!(benches);