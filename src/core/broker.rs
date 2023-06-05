use std::collections::HashMap;
use std::fmt;

use crate::core::order::*;
use crate::dataframe::ticker::Ticker;
use crate::strategy::strategy::Strategy;

#[derive(Debug, Clone)]
struct Position {
    symbol: String,
    amount: f32,
    price: f32,
}

#[derive(Debug, Clone)]
pub enum BrokerError {
    InsufficientFunds,
    InsufficientMargin,
    InvalidOrder,
}

pub struct Broker {
    name: String,
    strategy: Vec<Box<dyn Strategy>>,
    pub initial_cash: f32,
    commission: f32,
    margin: f32,
    trade_on_close: bool,
    hedging: bool,

    /// Internal bookkeeping of all orders placed.
    orders: HashMap<OrderId, Order>,
    pub current_cash: f32,
    positions: HashMap<String, Position>,
}

/// The main entity that a strategy interacts with throughout the core event loop.
/// The Broker is responsible for maintaining bookkeeping of all orders placed,
/// providing the strategy with information about the current state of the market,
/// and managing the strategy's portfolio.
///
impl Broker {
    pub fn new(
        name: &str,
        strategy: Vec<Box<dyn Strategy>>,
        initial_cash: f32,
        commission: f32,
        margin: f32,
        trade_on_close: bool,
        hedging: bool,
    ) -> Broker {
        Broker {
            name: name.to_string(),
            strategy,
            initial_cash,
            commission,
            margin,
            trade_on_close,
            hedging,
            orders: HashMap::new(),
            current_cash: initial_cash,
            positions: HashMap::new(),
        }
    }

    pub fn next(&mut self, ticker: &Ticker) -> Result<(), BrokerError> {
        self.process_orders(ticker)?;

        for strategy in &mut self.strategy {
            strategy.on_ticker(ticker);
        }

        Ok(())
    }

    pub fn submit_order(&mut self, id: OrderId, order: Order) -> Result<(), BrokerError> {
        self.orders.insert(id, order);

        Ok(())
    }

    pub fn cancel_order(&mut self, id: OrderId) -> Result<(), BrokerError> {
        self.orders.remove(&id);

        Ok(())
    }

    /// Processes a single order.
    fn execute_order(&mut self, order: Order, ticker: &Ticker) -> Result<(), BrokerError> {
        match order.side {
            OrderSide::Buy => {
                if let Some(position) = self.positions.remove(&order.symbol) {
                    todo!("Update the position");
                } else {
                    // We have not yet established a position. We need to create a position.
                    self.positions.insert(
                        order.symbol.clone(),
                        Position {
                            symbol: order.symbol.clone(),
                            amount: order.quantity,
                            price: ticker.close,
                        },
                    );
                    // And update the current balance
                    self.current_cash -= ticker.close;
                }
            }
            OrderSide::Sell => {
                if let Some(Position) = self.positions.remove(&order.symbol) {
                    todo!("Update the position");
                } else {
                    todo!("Add the ability to short");
                }
            }
        };

        Ok(())
    }

    /// Processes all the withstanding orders in the order book.
    /// This function mainly handles the order processing logic, but the
    /// actual order execution is performed in 'execute_order'.
    ///
    /// # TODO
    /// There needs to be some sense of time delay
    fn process_orders(&mut self, ticker: &Ticker) -> Result<(), BrokerError> {
        let mut non_executed_orders = HashMap::new();
        for (id, order) in self.orders.clone() {
            match order.order_type {
                OrderType::Market => self.execute_order(order, ticker)?,
                OrderType::Limit(price) => match order.side {
                    OrderSide::Buy => {
                        if ticker.close <= price {
                            self.execute_order(order, ticker)?;
                        } else {
                            non_executed_orders.insert(id, order);
                        }
                    }
                    OrderSide::Sell => {
                        if ticker.close >= price {
                            self.execute_order(order, ticker)?;
                        } else {
                            non_executed_orders.insert(id, order);
                        }
                    }
                },
                OrderType::Stop(price) => match order.side {
                    OrderSide::Buy => {
                        if ticker.close >= price {
                            self.execute_order(order, ticker)?;
                        } else {
                            non_executed_orders.insert(id, order);
                        }
                    }
                    OrderSide::Sell => {
                        if ticker.close <= price {
                            self.execute_order(order, ticker)?;
                        } else {
                            non_executed_orders.insert(id, order);
                        }
                    }
                },
                OrderType::MarketOnClose => {
                    todo!("Implement logic to determine if the market is closing");
                }
                OrderType::MarketOnOpen => {
                    todo!("Implement logic to determine if the market is opening");
                }
            }
        }

        self.orders = non_executed_orders;

        Ok(())
    }
    // pub fn place_order(&mut self, order: Order) -> Result<(), BrokerError>;
}
