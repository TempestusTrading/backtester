//! Contains the `Strategy` trait and example implementations for pedagogical purposes.
//!
//! To create your own strategy, implement the `Strategy` trait. The `Broker` will
//! call the `on_ticker` method of your strategy with the latest ticker data. Your
//! strategy can then use the ticker data to make trading decisions and send orders
//! to the broker.
//!
//! ```
//! use trading::{
//!    broker::Broker;
//!    strategy::Strategy;
//!    types::{Order, OrderSide, OrderType, Ticker};
//! }
//!
//! pub struct DumbStrategy;
//!
//! impl Strategy for DumbStrategy {
//!    fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError> {
//!       if ticker.close > 69.0 {
//!         broker.submit_order(Order {
//!                symbol: "AAPL".to_string()
//!                quantity: 100.0,
//!                side: OrderSide::Buy,
//!                order_type: OrderType::Market,
//!                time: ticker.datetime.clone(),
//!         })
//!       }   
//!    }  
//! }
//! ```

use crate::{
    broker::{Broker, BrokerError},
    indicators::Indicator,
    types::{Ticker, Order, OrderType, OrderSide},
};

pub enum StrategyError {
    // The broker experienced an error while processing the strategy's action.
    BrokerError,
}

/// Sends orders to a broker based on decisions made from the ticker data.
/// Contains indicators that are updated with the ticker data and used to make
/// trading decisions.
pub trait Strategy {
    fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError>;
}

mod sma_crossover;
pub use sma_crossover::SMACrossoverStrategy;
