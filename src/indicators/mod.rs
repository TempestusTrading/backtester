use serde_derive::{Deserialize, Serialize};

pub use crate::types::Ticker;

/// Represents the possible errors that can occur when computing an indicator.
#[derive(Debug, Serialize, Deserialize)]
pub enum IndicatorError {
    IndexOutOfRange,
    InsufficientData,
}

pub type IndicatorResult<T> = Result<T, IndicatorError>;

pub trait Indicator: Default {
    type Result;

    /// Update the indicator with the latest ticker data.
    fn update(&mut self, ticker: &Ticker) -> IndicatorResult<()>;
    /// Get the current value of the indicator.
    fn get_value(&self) -> IndicatorResult<Self::Result>;
    /// Randomly access a past value in the indicator's internal data structure.
    fn at(&self, index: usize) -> IndicatorResult<Self::Result>;
}

// Re-export all indicators.
pub mod rsi;
pub mod sma;