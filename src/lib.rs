//! # High Performance Backtester
//! This library implements a high performance backtester for trading strategies.
//! In the future, I think this project needs a better name, but because of
//! my lack of creativity, I will leave it as is for now.
//!
//! ### Features
//! 1. SS-SD (Single Strategy, Single Dataset)
//! 2. MS-SD (Multi-Strategy, Single Dataset)
//! 3. MS-MD (Multi-Strategy, Multi-Dataset)
//! 4. Broker Tuning:
//! Rather than re-running your entire backtesting bench for each broker parameter,
//! because the results of your run are cached and computed cleverly, you can simply
//! adjust the parameter to see how differential changes effect the outcome of your strategy.
//!
//! ### Priorities
//! 1. Performance
//! Written in Rust with special detail to latency bottlenecks
//! 2. Parallelism
//! Designed to take advantage of
//! 3. Caching indicators
//! Provides an easy to use API for saving indicators that have already been calculated.
//! 4. Logging
//! Provides an easy to use API for logging trades and other events.
//! 5. Optimization
//! Determines the optimal parameters for a given strategy.
//! 6. Flexibility
//! Returns a set of results that can be easily analyzed and visualized.
//!
//! ## Overview
//!
//! ### Backtesting Strategies
//! Provides a simple interface for backtesting strategies.
//!
//! ```
//! use backtester::prelude::*;
//! use backtester::strategy::SMACrossover;
//!
//! fn main() -> Result<(), BacktestError> {
//! 	let aapl_timeseries = TimeSeries::from_csv("./benches/datasets/AAC.csv");
//! 	let broker = Broker::new("Simple Backtest", 100_000.0, 0.0, 0.0, false, false);
//! 	let strategy = Box::new(SMACrossover::default());
//! 	let backtest = BacktestBuilder::new()
//! 	               .add_feed(aapl_timeseries)
//! 	               .add_broker(broker)
//! 	               .add_strategy(strategy)
//! 	               .build();
//!
//! 	for test in backtest {
//! 		let results = test.run()?;
//! 		println!("{}", results);
//! 	}
//!
//!     Ok(())
//! }
//! 
//! 
//! ```
//!
//! ### Defining Custom Indicators
//!
//! One can easily define a custom indicator by deriving the `Indicator` trait.
//!
//! ```
//! use backtester::prelude::*;
//! use std::fmt;
//!
//! #[derive(Clone)]
//! pub struct MyIndicator {
//!    value: Option<f32>,
//! }
//!
//! impl fmt::Display for MyIndicator {
//!    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!       write!(f, "My Indicator")
//!    }
//! }
//!
//! impl Indicator for MyIndicator {
//!     type Result = f32;
//!
//!	    fn update(&mut self, ticker: &Ticker) -> IndicatorResult<()> {
//!	        self.value = Some(ticker.close);
//!			Ok(())
//!	    }
//!
//!	    fn get_value(&self) -> IndicatorResult<Self::Result> {
//!			if let Some(result) = self.value {
//!			    Ok(result)
//!			} else {
//!			    Err(IndicatorError::InsufficientData)
//!			}
//!	    }
//!
//!	    fn at(&self, index: usize) -> IndicatorResult<Self::Result> {
//!			self.get_value()
//!	    }
//! }
//! ```
//!
//! ### Creating Custom Strategies
//!
//! Creating custom strategies is just as simple. Simply derive the `Strategy` trait.
//! and add the logic for the `on_ticker` method, which will be executed by the
//! `Broker` for each step in the backtest.
//!
//!
//! ```
//! use backtester::prelude::*;
//! use std::fmt;
//!
//! #[derive(Clone)]
//! pub struct DumbStrategy;
//!
//! impl fmt::Display for DumbStrategy {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!        write!(f, "Dumb Strategy")
//!    }
//! }
//!
//! impl Strategy for DumbStrategy {
//!    fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError> {
//!       if ticker.close > 100.0 {
//!         broker.submit_order(1, Order {
//!                symbol: "AAPL".to_string(),
//!                quantity: 100.0,
//!                side: OrderSide::Buy,
//!                order_type: OrderType::Market,
//!                datetime: ticker.datetime.clone(),
//!                execution: OrderExecutionStrategy::GTC,
//!                on_execute: None,
//!                on_cancel: None,
//!         })?;
//!       }   
//! 	  Ok(())
//!    }  
//! }
//! ```
//!

mod backtest;
pub mod broker;
pub mod indicators;
pub mod strategy;
pub mod series;
pub mod timeseries;
mod types;

pub mod prelude {
    pub use crate::backtest::*;
    pub use crate::broker::*;
    pub use crate::indicators::*;
    pub use crate::strategy::*;
    pub use crate::series::*;
    pub use crate::timeseries::*;
    pub use crate::types::*;
}
