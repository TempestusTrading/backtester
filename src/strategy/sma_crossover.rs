use std::collections::HashMap;

use crate::core::broker::Broker;
use crate::core::order::{Order, OrderSide, OrderType};
use crate::dataframe::ticker::Ticker;
use crate::indicators::indicator::Indicator;
use crate::indicators::moving_average::MovingAverage;
use crate::strategy::strategy::{Strategy, StrategyBuilder};

pub struct SMACrossoverStrategy {
    order_id: usize,
    previous_sma: f32,
    previous_ticker: Option<Ticker>,
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
        SMACrossoverStrategy {
            order_id: 0,
            previous_sma: 0.0,
            previous_ticker: None,
            indicators,
        }
    }

    /// This is the main entry point for the strategy. This function will be called
    /// each time a new ticker is processed by the exchange.
    ///
    /// # Example
    /// 1) Show an example first updating the indicator values. Then querying an
    /// indicator value and making a decision based on that value.
    /// TODO: ADD A CALLBACK FN HERE TO SEND ORDERS TO THE BROKER
    fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) {
        for (_, indicator) in &mut self.indicators {
            indicator.update(ticker);
        }

        let sma = self.indicators.get("sma").unwrap().get_value();
        if let Some(previous_ticker) = &self.previous_ticker {
            if sma > ticker.close && self.previous_sma < previous_ticker.close {
                broker.submit_order(
                    self.order_id,
                    Order::new("AAPL".to_string(), 100.0, OrderSide::Buy, OrderType::Market),
                );
                self.order_id += 1;
            } else if sma < ticker.close && self.previous_sma > previous_ticker.close {
                broker.submit_order(
                    self.order_id,
                    Order::new(
                        "AAPL".to_string(),
                        100.0,
                        OrderSide::Sell,
                        OrderType::Market,
                    ),
                );
                self.order_id += 1;
            }
        }

        self.previous_sma = sma;
        self.previous_ticker = Some(ticker.clone());
    }
}
