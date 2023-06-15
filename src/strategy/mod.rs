//! Contains the `Strategy` trait and example implementations for pedagogical purposes.
//!
//! To create your own strategy, implement the `Strategy` trait. The `Broker` will
//! call the `on_ticker` method of your strategy with the latest ticker data. Your
//! strategy can then use the ticker data to make trading decisions and send orders
//! to the broker.
//!

use crate::{
    broker::{Broker, BrokerError},
    indicators::Indicator,
    types::{Ticker, Order, OrderType, OrderSide},
};
use dyn_clone::DynClone;

#[derive(Debug)]
pub enum StrategyError {
    // The broker experienced an error while processing the strategy's action.
    BrokerError(BrokerError),
}

impl From<BrokerError> for StrategyError {
    fn from(err: BrokerError) -> Self {
        StrategyError::BrokerError(err)
    }
}

/// Sends orders to a broker based on decisions made from the ticker data.
/// Contains indicators that are updated with the ticker data and used to make
/// trading decisions.
pub trait Strategy: DynClone {
    /// Called by the broker for each step in the backtest. The strategy should
    /// use the ticker data to make trading decisions and send orders to the broker.
    fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError>;
}

mod sma_crossover;
pub use sma_crossover::SMACrossover;