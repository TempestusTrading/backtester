use crate::{
    broker::Broker,
    prelude::BrokerError,
    strategy::{Strategy, StrategyError},
    timeseries::TimeSeries,
    indicators::Indicator,
};
use std::ffi::OsString;
use std::fmt;
use std::time::{Duration, Instant};
use std::collections::HashMap;

pub struct BacktestBuilder {
    indicators: Vec<Box<dyn Indicator<Result = i32>>>,
    feeds: Vec<TimeSeries>,
    brokers: Vec<Broker>,
    strategies: Vec<Box<dyn Strategy>>,
}

impl BacktestBuilder {
    pub fn new() -> Self {
        Self {
            indicators: Vec::new(),
            feeds: Vec::new(),
            brokers: Vec::new(),
            strategies: Vec::new(),
        }
    }

    pub fn add_indicator(mut self, indicator: Box<dyn Indicator<Result = i32>>) -> Self {
        self.indicators.push(indicator);
        self
    }

    pub fn add_feed(mut self, feed: TimeSeries) -> Self {
        self.feeds.push(feed);
        self
    }

    pub fn add_feeds(mut self, feeds: Vec<TimeSeries>) -> Self {
        for feed in feeds {
            self.feeds.push(feed);
        }
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
                        self.indicators.iter().map(|item| dyn_clone::clone_box(&**item)).collect(),
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
    indicators: Vec<Box<dyn Indicator<Result = i32>>>,
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
    pub fn new(indicators: Vec<Box<dyn Indicator<Result = i32>>>, feed: TimeSeries, broker: Broker, strategy: Box<dyn Strategy>) -> Self {
        Self {
            indicators,
            feed,
            broker,
            strategy,
        }
    }

    pub fn run(mut self) -> Result<BacktestResult, BacktestError> {
        let start = Instant::now();
        let feed_path = self.feed.get_path().as_os_str().into();

        for ticker in self.feed {
            let ticker = ticker.expect("Failed to parse ticker.");
            self.broker.next(&ticker)?;
            // for indicator in self.indicators {
                // if indicator.update(&ticker) {
                // self.strategy.on_indicator(indicator)?;
                // };
            // }
            self.strategy.on_ticker(&ticker, &mut self.broker)?;
        }

        Ok(BacktestResult {
            feed_path: feed_path,
            broker: self.broker,
            strategy: self.strategy,
            runtime: start.elapsed(),
        })
    }
}

pub struct BacktestResult {
    feed_path: OsString,
    broker: Broker,
    strategy: Box<dyn Strategy>,
    runtime: Duration,
}

impl fmt::Display for BacktestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("Feed: {}\n", self.feed_path.to_str().unwrap()));
        result.push_str(&format!("Broker: {}\n", self.broker));
        result.push_str(&format!("Strategy: {}\n", self.strategy));
        result.push_str(&format!("Runtime: {:?}\n", self.runtime));
        write!(f, "{}", result)
    }
}

