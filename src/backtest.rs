use crate::{
    broker::Broker,
    prelude::BrokerError,
    strategy::{Strategy, StrategyError},
    timeseries::TimeSeries,
};
use std::ffi::OsString;
use std::fmt;
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
        let feed_path = self.feed.get_path().as_os_str().into();

        for ticker in self.feed {
            let ticker = ticker.expect("Failed to parse ticker.");
            self.broker.next(&ticker)?;
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
        // result.push_str(&format!("Broker: {}\n", self.broker));
        result.push_str(&format!("Strategy: {}\n", self.strategy));
        result.push_str(&format!("Runtime: {:?}\n", self.runtime));
        write!(f, "{}", result)
    }
}

