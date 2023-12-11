///! Trait Stream for Backtesting 
///! 
///! This trait is used to create custom streams of data for backtesting. For example,
///! if you wanted to load a stream of macroeconomic data, you could implement this trait
///! for your custom data type. 
///! If you are looking to create a stream of ticker data, use the `TimeSeries` struct.
use std::path::{Path, PathBuf};
use std::fs::File;
use csv::Error;

/// Provides a flexible stream of data for backtesting.
/// 
/// # Example 
/// ```
/// use backtester::prelude::*;
/// 
/// ```
pub struct TimeSeries<T> {
    path: PathBuf,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> TimeSeries<T> {
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

impl<T> IntoIterator for TimeSeries<T> {
    type Item = Result<T, Error>;
    type IntoIter = TimeSeriesIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        let reader: csv::DeserializeRecordsIntoIter<File, T> =
            csv::Reader::from_path(self.path.clone())
                .expect("Cannot not find file")
                .into_deserialize::<T>();
        TimeSeriesIntoIterator {
            deserialized_reader: reader,
        }
    }
}

pub struct TimeSeriesIntoIterator<T> {
    deserialized_reader: csv::DeserializeRecordsIntoIter<File, T>,
}

impl<T> Iterator for TimeSeriesIntoIterator<T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ticker) = self.deserialized_reader.next() {
            Some(ticker)
        } else {
            None
        }
    }
}