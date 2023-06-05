use crate::dataframe::timeseries::TimeSeries;
use crate::strategy::strategy::Strategy;

use chrono::{offset::Utc, DateTime, NaiveDateTime};
use std::time::{Duration, Instant};

use super::broker::Broker;

pub struct BacktestBuilder {
    feed: TimeSeries,
    broker: Broker,
    strategy: Box<dyn Strategy>,
}

impl BacktestBuilder {
    pub fn new(feed: TimeSeries, broker: Broker, strategy: Box<dyn Strategy>) -> Self {
        Self {
            feed: feed,
            broker,
            strategy,
        }
    }

    pub fn add_feed(mut self, feed: TimeSeries) -> Self {
        self.feed = feed;
        self
    }

    pub fn set_broker(mut self, broker: Broker) -> Self {
        self.broker = broker;
        self
    }

    pub fn add_strategy(mut self, strategy: Box<dyn Strategy>) -> Self {
        self.strategy = strategy;
        self
    }

    /// Perform a cartesian product of the brokers and strategies. This will
    /// result in a vector of runs that will be executed in parallel.
    pub fn build(mut self) -> Backtest {
        Backtest::new(self.feed, self.broker, self.strategy)
    }
}

pub struct Backtest {
    feed: TimeSeries,
    broker: Broker,

    // Strategy makes a series of decisions based on indicators. Thus, we do not need
    // ownership of the strategy. But we do need to make sure that the indicator
    // are synced for each instance of the broker.
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
            if let Err(error) = self.broker.next(&ticker) {
                panic!("Broker error: {:?}", error);
            }
            // TODO: Implement communication channel between broker and strategy
            self.strategy.on_ticker(&ticker);
        }

        BacktestResults {
            runtime: start.elapsed(),
            start_date: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
                Utc,
            ),
            end_date: DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_opt(61, 0).unwrap(),
                Utc,
            ),
            starting_amount: self.broker.initial_cash,
            ending_amount: self.broker.current_cash,
        }
    }
}

#[derive(Debug)]
pub struct BacktestResults {
    runtime: Duration,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    starting_amount: f32,
    ending_amount: f32,
}