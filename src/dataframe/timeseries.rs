use std::{path::Path, io::Read};

use csv;
use std::fs::File;

use crate::dataframe::ticker::Ticker;

/// This struct provides a stream of 'Tickers' from a CSV file.
/// # Note
/// This struct is lazily evaluated. Rather than loading the whole
/// file into memory upon initialization, it will only create a 
/// reader that will be loaded the next time the iterator is called.
/// CONSIDER: WE MIGHT WANT TO BUFFER READING FROM THE FILE
pub struct TimeSeries {
   reader: csv::DeserializeRecordsIntoIter<File, Ticker>,
}

impl TimeSeries {
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Self {
        let reader: csv::DeserializeRecordsIntoIter<File, Ticker> = csv::Reader::from_path(path)
            .unwrap()
            .into_deserialize::<Ticker>();

            // .expect(format!("Cannot find file {}", path.as_ref().display().to_str()));
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