///! Trait Stream for Backtesting 
///! 
///! This trait is used to create custom streams of data for backtesting. For example,
///! if you wanted to load a stream of macroeconomic data, you could implement this trait
///! for your custom data type. 
///! If you are looking to create a stream of ticker data, use the `TimeSeries` struct.
use std::path::Path;
use std::fs::read_dir;

use crate::{
	series::Series,
	types::Ticker,
};

/// Provides a flexible stream of data for backtesting.
/// 
/// # Example 
/// ```
/// use backtester::prelude::*;
/// 
/// ```
pub type TimeSeries = Series<Ticker>;

impl TimeSeries {
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
}