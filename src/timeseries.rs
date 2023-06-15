//! Data streams for backtesting.
//! 
//! ## Limitations
//! Currently, the only supported data source is CSV files.
//! Furthermore, the CSV file must contain the following columns:
//! 
//! - open
//! - high
//! - low
//! - close
//! - volume
//! - datetime
//! 
//! If any of these columns are omitted, deserialization will fail.
use crate::types::Ticker;
use std::fs::{File, read_dir, DirEntry};
use std::path::{Path, PathBuf};

/// Provides a stream of 'Tickers' from a CSV file.
/// ## Notice:
/// The timeseries is lazily evaluated. Rather than loading the whole
/// file into memory upon initialization, it creates a deserialized
/// reader that can be turned into an iterator to load the data.
/// 
/// # Example
///
/// ```no_run
/// use backtester::prelude::*;
/// 
/// let timeseries = TimeSeries::from_csv("data/SPY.csv");
/// for ticker in timeseries {
///    println!("{:?}", ticker);
/// }
/// ```
#[derive(Clone)]
pub struct TimeSeries {
    path: PathBuf
}

impl TimeSeries {
    /// Initializes a new TimeSeries from a CSV file.
    /// Ensure that the CSV file contains the following columns:
    /// `open, high, low, close, volume, datetime.`
    /// Otherwise, deserialization will fail.
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Self {
        Self { path: path.as_ref().to_path_buf() }
    }

    /// Initializes a set of TimeSeries from a directory.
    /// This function uses `from_csv` for each CSV file, so 
    /// ensure that the format of each CSV file is correct.
    pub fn from_dir<P: AsRef<Path>>(path: P) -> Vec<Self> {
        let mut result = Vec::new();
        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(extension) = entry.path().extension() {
                        if extension == "csv" {
                            result.push(Self::from_csv(entry.path()));
                        }
                    }
                }
            }
        } else {
            panic!("Cannot find directory");
        }
        result
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

impl IntoIterator for TimeSeries {
    type Item = Result<Ticker, csv::Error>;
    type IntoIter = TimeSeriesIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        let reader: csv::DeserializeRecordsIntoIter<File, Ticker> =
            csv::Reader::from_path(self.path.clone())
                .expect(&format!("Cannot not find file"))
                .into_deserialize::<Ticker>();
        TimeSeriesIntoIterator {
            deserialized_reader: reader,
        }
    }
}

pub struct TimeSeriesIntoIterator {
    deserialized_reader: csv::DeserializeRecordsIntoIter<File, Ticker>,
}

impl Iterator for TimeSeriesIntoIterator {
    type Item = Result<Ticker, csv::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ticker) = self.deserialized_reader.next() {
            Some(ticker)
        } else {
            None
        }
    }
}
