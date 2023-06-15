use crate::{broker::Broker, strategy::Strategy, timeseries::TimeSeries};
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use dyn_clone::DynClone;

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

impl Backtest {
    pub fn new(feed: TimeSeries, broker: Broker, strategy: Box<dyn Strategy>) -> Self {
        Self {
            feed,
            broker,
            strategy,
        }
    }

    pub fn run(mut self) -> BacktestResults {
        let start = Instant::now();

        for ticker in self.feed {
            let ticker = ticker.expect("Failed to parse ticker.");
            self.broker
                .next(&ticker)
                .expect_err("Broker experienced an error.");
            self.strategy.on_ticker(&ticker, &mut self.broker);
        }

        BacktestResults {
            runtime: start.elapsed(),
            starting_amount: self.broker.initial_cash,
            ending_amount: self.broker.current_cash,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResults {
    runtime: Duration,
    starting_amount: f32,
    ending_amount: f32,
}
