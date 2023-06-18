use super::*;

/// # Buy and Hold
///
/// This is a simple strategy that allocates all capital to a given asset and holds
/// it for the duration of the backtest.
///
/// This test serves as a good baseline for comparison against other strategies. If the
/// results of another strategy are significantly better than the results of buy and
/// hold, then the strategy is likely a good one.
#[derive(Clone)]
pub struct BuyAndHold {
    bought: bool,
}

impl Default for BuyAndHold {
    fn default() -> Self {
        Self { bought: false }
    }
}

impl fmt::Display for BuyAndHold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buy and Hold")
    }
}

impl Strategy for BuyAndHold {
    fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError> {
        match self.bought {
            false => {
                self.bought = true;
                broker.submit_order(
                    0,
                    Order {
                        symbol: "AAPL".to_string(),
                        quantity: 100.0,
                        side: OrderSide::Buy,
                        order_type: OrderType::Market,
                        datetime: ticker.datetime.clone(),
                        execution: OrderExecutionStrategy::GTC,
                        on_execute: None,
                        on_cancel: None,
                    },
                )?;
            }
            _ => (),
        }
        Ok(())
    }
}
