use crate::dataframe::timeseries::TimeSeries;
use crate::dataframe::timeseriesbatch::TimeSeriesBatch;
use crate::strategy::strategy::Strategy;

use std::time::{Instant, Duration};
use chrono::{DateTime, offset::Utc, NaiveDateTime};

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

pub fn run(
	series: TimeSeries,
	mut strategy: impl Strategy,
	starting_amount: f32,
	commission_rate: f32,
) -> RunStatistics {
	let start = Instant::now();
	
	for ticker in series {
		strategy.on_ticker(ticker);
	}

	RunStatistics { 
		runtime: start.elapsed(),
		symbol: "ABC".to_string(), 
		start_date: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(61, 0).unwrap(), Utc), 
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