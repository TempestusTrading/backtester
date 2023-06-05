use crate::dataframe::ticker::Ticker;
use crate::indicators::indicator::Indicator;

pub struct StrategyBuilder {
    indicators: Vec<Box<dyn Indicator>>,
}

impl StrategyBuilder {
    pub fn new() -> StrategyBuilder {
        StrategyBuilder {
            indicators: Vec::new(),
        }
    }

    pub fn add_indicator(mut self, indicator: Box<dyn Indicator>) -> StrategyBuilder {
        self.indicators.push(indicator);
        self
    }

    pub fn build(self) -> Strategy {
        Strategy { 
            indicators: self.indicators 
        }
    }
}

/// Any strategy that is to be used within the core event loop must implement
/// this trait.
///
/// # Example
///
/// ```
pub struct Strategy {
    indicators: Vec<Box<dyn Indicator>>,
}

impl Strategy {
    pub fn on_ticker(&mut self, ticker: &Ticker) {
        for indicator in self.indicators.iter_mut() {
            indicator.update(ticker);
        }
    }
}
