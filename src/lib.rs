//! # High Performance Backtester
//! This library implements a high performance backtester for trading strategies. 
//! 
//! ## Prioritiies
//! 1. Performance
//! Written in Rust with special detail to potential bottlenecks
//! 2. Parallelism 
//! 3. Caching indicators
//! Provides an easy to use API for saving indicators that have already been calculated.
//! 4. 
//! - High performance backtesting
//! 5. Optimization
//! 
//! ## Features
//! 

mod broker;
mod bt;
mod indicators;
mod strategy;
mod timeseries;
mod types;