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
//! let aapl_timeseries = TimeSeries::from_csv("./datasets/AAPL.csv");
//! let broker = Broker::new(100_000.0);
//! let strategy = Box::new(SMACrossover::new());
//! let backtest = BacktestBuilder::new()
//!                .add_feed(aapl_timeseries)
//!                .add_broker(broker)
//!                .add_strategy(strategy)
//!                .build();
//!
//! let results = backtest.run();
//! ```
//!
//! ### Defining Custom Indicators
//! One can easily define a custom indicator by deriving the `Indicator` trait.
//!
//! ```
//! use backtester::indicator::*;
//!
//! pub struct MyIndicator {
//!    value: f64,
//! }
//!
//! impl Indicator for MyIndicator {
//!    fn new() -> Self {
//!       Self { value: 0.0 }
//!   }
//! ```
//!

pub mod broker;
mod backtest;
pub mod indicators;
pub mod strategy;
pub mod timeseries;
mod types;

pub mod prelude {
    pub use crate::broker::*;
    pub use crate::indicators::*;
    pub use crate::strategy::*;
    pub use crate::timeseries::*;
    pub use crate::types::*;
}