use std::collections::HashMap;

use crate::dataframe::ticker::Ticker;
use crate::indicators::indicator::Indicator;

pub struct StrategyBuilder {
    indicators: HashMap<String, Box<dyn Indicator>>,
}

impl StrategyBuilder {
    pub fn new() -> StrategyBuilder {
        StrategyBuilder {
            indicators: HashMap::new(),
        }
    }

    pub fn add_indicator(mut self, id: &str, indicator: Box<dyn Indicator>) -> StrategyBuilder {
        self.indicators.insert(id.to_string(), indicator);
        self
    }

    pub fn build<T>(self) -> T
    where
        T: Strategy,
    {
        T::with_indicators(self.indicators)
    }
}

/// Any strategy that is to be used within the core event loop must implement
/// this trait.
///
/// # Example
///
/// ```
pub trait Strategy {
    fn with_indicators(indicators: HashMap<String, Box<dyn Indicator>>) -> Self
    where
        Self: Sized;
    fn on_ticker(&mut self, ticker: &Ticker);
}
