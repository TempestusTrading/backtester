use super::*;
use crate::{
    indicators::SMA,
};

/// # SMA Crossover Strategy
///
/// ## States
/// - `Waiting` - Waiting for the SMA value to be calculated.
/// - `No Position` - No position is established because either (1) the SMA was just calculated
/// and the value has yet to cross the ticker close, executing a market buy order, or (2) the SMA
/// crossed below the ticker price, executing a market sell order.
/// - `Long` - The SMA has crossed about the ticker price, so we execute a market buy order
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
        self.sma_indicator.update(ticker).err();

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
