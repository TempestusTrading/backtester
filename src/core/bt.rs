use crate::dataframe::timeseries::TimeSeries;
use crate::dataframe::timeseriesbatch::TimeSeriesBatch;
use crate::strategy::strategy::Strategy;

use chrono::{offset::Utc, DateTime, NaiveDateTime};
use std::time::{Duration, Instant};

use super::broker::Broker;

#[derive(Debug)]
pub struct RunStatistics {
    runtime: Duration,
    symbol: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    starting_amount: f64,
    ending_amount: f64,
}

#[derive(Debug)]
pub struct BatchRunStatistics {
    runtime: Duration,
    individuals: Vec<RunStatistics>,
}

pub fn run(series: TimeSeries, strategy: Strategy, mut broker: Broker) -> RunStatistics {
    let start = Instant::now();

    for ticker in series {
        broker.next(&ticker);
    }

    RunStatistics {
        runtime: start.elapsed(),
        symbol: "ABC".to_string(),
        start_date: DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
            Utc,
        ),
        end_date: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(61, 0).unwrap(), Utc),
        starting_amount: 1000.0,
        ending_amount: 1000.0,
    }
}

pub fn run_batch(batch: &TimeSeriesBatch) -> BatchRunStatistics {
    let mut individuals: Vec<RunStatistics> = Vec::new();
    let start = Instant::now();

    BatchRunStatistics {
        runtime: start.elapsed(),
        individuals,
    }
}
