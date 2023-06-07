use std::path::Path;
use std::fs::File;

use csv;

use super::ticker::Ticker;

/// Provides a stream of 'Tickers' from a CSV file.
/// # Note
/// This struct is lazily evaluated. Rather than loading the whole
/// file into memory upon initialization, it creates a deserialized
/// reader that can be turned into an iterator to load the data.
/// # Example
/// ```no_run
/// use backtester::dataframe::timeseries::*;
/// let timeseries = TimeSeries::from_csv("data/SPY.csv");
/// for ticker in timeseries {
///    println!("{:?}", ticker);
/// }
/// ```
pub struct TimeSeries {
   reader: csv::DeserializeRecordsIntoIter<File, Ticker>,
}

impl TimeSeries {
    /// Initializes a new TimeSeries from a CSV file.
    /// Ensure that the CSV file contains the following columns:
    /// open, high, low, close, volume, datetime.
    /// Otherwise, deserialization will fail.
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Self {
        let reader: csv::DeserializeRecordsIntoIter<File, Ticker> = csv::Reader::from_path(path.as_ref().clone())
            .expect(&format!("Cannot not find file {}", path.as_ref().display()))
            .into_deserialize::<Ticker>();

        Self {
            reader
        }
    }
}

impl IntoIterator for TimeSeries {
    type Item = Result<Ticker, csv::Error>;
    type IntoIter = TimeSeriesIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        TimeSeriesIntoIterator { 
            deserialized_reader: self.reader 
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