use super::*;
use crate::{
    indicators::SMA,
    types::{Order, OrderSide, OrderType, Ticker},
};

pub struct SMACrossoverStrategy {
    order_id: usize,
    previous_sma: f32,
    previous_ticker: Option<Ticker>,
    sma_indicator: SMA,
}

impl SMACrossoverStrategy {
    pub fn new(period: u32) -> SMACrossoverStrategy {
        SMACrossoverStrategy {
            order_id: 0,
            previous_sma: 0.0,
            previous_ticker: None,
            sma_indicator: SMA::new(period),
        }
    }
}

impl Strategy for SMACrossoverStrategy {
    fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError> {
        self.sma_indicator.update(ticker);

        if let Ok(sma) = self.sma_indicator.get_value() {
            if sma > ticker.close
                && self.previous_sma < self.previous_ticker.as_ref().unwrap().close
            {
                broker
                    .submit_order(
                        self.order_id,
                        Order {
                            symbol: "AAPL".to_string(),
                            quantity: 100.0,
                            side: OrderSide::Buy,
                            order_type: OrderType::Market,
                            time: ticker.datetime.clone(),
                        },
                    )
                    .err();
                self.order_id += 1;
            } else if sma < ticker.close
                && self.previous_sma > self.previous_ticker.as_ref().unwrap().close
            {
                broker
                    .submit_order(
                        self.order_id,
                        Order {
                            symbol: "AAPL".to_string(),
                            quantity: 100.0,
                            side: OrderSide::Sell,
                            order_type: OrderType::Market,
                            time: ticker.datetime.clone(),
                        },
                    )
                    .err();
                self.order_id += 1;
            }

            self.previous_sma = sma;
        }
        self.previous_ticker = Some(ticker.clone());
        Ok(())
    }
}
