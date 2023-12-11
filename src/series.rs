///! Trait Stream for Backtesting 
///! 
///! This trait is used to create custom streams of data for backtesting. For example,
///! if you wanted to load a stream of macroeconomic data, you could implement this trait
///! for your custom data type. 
///! If you are looking to create a stream of ticker data, use the `TimeSeries` struct.
use std::path::Path;

/// Provides a flexible stream of data for backtesting.
/// 
/// # Example 
/// ```
/// use backtester::prelude::*;
/// 
/// ```
pub trait Series: IntoIterator {
    fn from_csv<P: AsRef<Path>>(path: P) -> Self;
}