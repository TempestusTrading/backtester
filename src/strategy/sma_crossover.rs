use std::collections::HashMap;

use crate::dataframe::ticker::Ticker;
use crate::indicators::indicator::Indicator;
use crate::indicators::moving_average::MovingAverage;
use crate::strategy::strategy::{Strategy, StrategyBuilder};

pub struct SMACrossoverStrategy {
    indicators: HashMap<String, Box<dyn Indicator>>,
}

impl SMACrossoverStrategy {
    pub fn new(period: u32) -> SMACrossoverStrategy {
        StrategyBuilder::new()
            .add_indicator("sma", Box::new(MovingAverage::new(period)))
            .build()
    }
}

impl Strategy for SMACrossoverStrategy {
    fn with_indicators(indicators: HashMap<String, Box<dyn Indicator>>) -> Self
    where
        Self: Sized,
    {
        SMACrossoverStrategy { indicators }
    }

    /// This is the main entry point for the strategy. This function will be called
    /// each time a new ticker is processed by the exchange.
    ///
    /// # Example
    /// 1) Show an example first updating the indicator values. Then querying an
    /// indicator value and making a decision based on that value.
    /// TODO: ADD A CALLBACK FN HERE TO SEND ORDERS TO THE BROKER
    fn on_ticker(&mut self, ticker: &Ticker) {
        for (_, indicator) in &mut self.indicators {
            indicator.update(ticker);
        }

        let sma = self.indicators.get("sma").unwrap().get_value();
        if sma > ticker.close {
            todo!("We need to send a buy order to the broker");
        } else {
            todo!("We need to send a sell order to the broker");
        }
    }
}
