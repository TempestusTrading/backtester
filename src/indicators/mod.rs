//! Contains the `Indicator` trait and a list of common market [indicators](https://www.investopedia.com/terms/i/indicator.asp).
//!
//! *Indicators are statistics used to measure current market conditions as well
//! forecast financial or economic trends.*
//!
//! # [Technical Indicators](https://www.investopedia.com/terms/t/technicalindicator.asp)
//!
//! # [Ecnonomic Indicators](https://www.investopedia.com/terms/e/economicindicator.asp)
//!
//! # [Sentiment Indicators](https://www.investopedia.com/terms/s/sentimentindicator.asp)
//!
//! # [Fundamental Indicators](https://www.investopedia.com/terms/f/fundamentalindicator.asp)
//!
use serde_derive::{Deserialize, Serialize};
use dyn_clone::DynClone;
use crate::{
    types::{Ticker}, 
};

pub(crate) use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum IndicatorError {
    IndexOutOfRange,
    InsufficientData,
}

pub type IndicatorResult<T> = Result<T, IndicatorError>;

pub trait Indicator: fmt::Display + DynClone {
    /// The type of value that the indicator returns.
    type Result;

    /// Update the indicator with the latest ticker data.
    fn update(&mut self, ticker: &Ticker) -> IndicatorResult<()>;
    /// Get the current value of the indicator.
    fn get_value(&self) -> IndicatorResult<Self::Result>;
    /// Randomly access the indicator's value at the `index`'th timestep.
    fn at(&self, index: usize) -> IndicatorResult<Self::Result>;
}

// Re-export all indicators
mod rsi;
mod sma;
pub use rsi::RSI;
pub use sma::SMA;