mod sma_crossover;

pub use crate::{
    broker::{Broker, BrokerError},
    indicators::Indicator,
    types::Ticker,
};

pub enum StrategyError {
    // Placeholder
    Invalid,
    // The broker experienced an error while processing the strategy's action.
    BrokerError,
}

/// Any strategy that is to be used within the core event loop must implement
/// this trait.
///
/// ```
pub trait Strategy {
    fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError>;
}
