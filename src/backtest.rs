use crate::{broker::Broker, strategy::{Strategy, StrategyError}, timeseries::TimeSeries, prelude::BrokerError};
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, Instant};

pub struct BacktestBuilder {
    feeds: Vec<TimeSeries>,
    brokers: Vec<Broker>,
    strategies: Vec<Box<dyn Strategy>>,
}

impl BacktestBuilder {
    pub fn new() -> Self {
        Self {
            feeds: Vec::new(),
            brokers: Vec::new(),
            strategies: Vec::new(),
        }
    }
    
    pub fn add_feed(mut self, feed: TimeSeries) -> Self {
        self.feeds.push(feed);
        self
    }

    pub fn add_broker(mut self, broker: Broker) -> Self {
        self.brokers.push(broker);
        self
    }

    pub fn add_strategy(mut self, strategy: Box<dyn Strategy>) -> Self {
        self.strategies.push(strategy);
        self
    }

    /// Perform a cartesian product of the brokers and strategies. This will
    /// result in a vector of runs that will be executed in parallel.
    pub fn build(mut self) -> Vec<Backtest> {
        let mut backtests = Vec::new();
        for strategy in self.strategies {
            for broker in &self.brokers {
                for feed in &self.feeds {
                    let backtest = Backtest::new(
                        feed.clone(),
                        broker.clone(),
                        dyn_clone::clone_box(&*strategy),
                    );
                    backtests.push(backtest);
                }
            }
        }
        backtests
    }
}

pub struct Backtest {
    feed: TimeSeries,
    broker: Broker,
    strategy: Box<dyn Strategy>,
}

#[derive(Debug)]
pub enum BacktestError {
    TickerParseError,
    BrokerError(BrokerError),
    StrategyError(StrategyError),
}

impl From<StrategyError> for BacktestError {
    fn from(err: StrategyError) -> Self {
        BacktestError::StrategyError(err)
    }
}

impl From<BrokerError> for BacktestError {
    fn from(err: BrokerError) -> Self {
        BacktestError::BrokerError(err)
    }
}

impl Backtest {
    pub fn new(feed: TimeSeries, broker: Broker, strategy: Box<dyn Strategy>) -> Self {
        Self {
            feed,
            broker,
            strategy,
        }
    }

    pub fn run(mut self) -> Result<BacktestResult, BacktestError> {
        let start = Instant::now();

        for ticker in self.feed {
            let ticker = ticker.expect("Failed to parse ticker.");
            self.broker.next(&ticker)?;
            self.strategy.on_ticker(&ticker, &mut self.broker)?;
        }

        Ok(BacktestResult {
            runtime: start.elapsed(),
            starting_amount: self.broker.initial_cash,
            ending_amount: self.broker.current_cash,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    runtime: Duration,
    starting_amount: f32,
    ending_amount: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_read() {
        let aapl_timeseries = TimeSeries::from_csv("./benches/datasets/AAPL_1Y.csv");

        for ticker in aapl_timeseries {
            assert!(ticker.is_ok());
        }
    }

    #[test]
    fn test_dir_read() {
        let datasets = TimeSeries::from_dir("./benches/datasets");

        for timeseries in datasets {
            for ticker in timeseries {
                assert!(ticker.is_ok());
            }
        }
    }
}