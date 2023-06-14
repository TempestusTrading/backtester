pub mod rsi;
pub mod sma;

pub use crate::types::Ticker;

/// Represents the possible errors that can occur when using an indicator.
pub enum IndicatorError {
    IndexOutOfRange,
    InsufficientData,
    FailedToUpdate,
}

pub type IndicatorResult<T> = Result<T, IndicatorError>;

pub trait Indicator: Default {
    /// Update the indicator with the latest ticker data.
    fn update(&mut self, ticker: &Ticker) -> IndicatorResult<()>;
    /// Get the current value of the indicator.
    fn get_value(&self) -> IndicatorResult<f32>;
    /// Randomly access a past value in the indicator's internal data structure.
    fn at(&self, index: usize) -> IndicatorResult<f32>;
}
