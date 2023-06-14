use crate::types::*;

use serde_derive::{Deserialize, Serialize};

use log::{info, warn};
use std::collections::HashMap;
use std::fmt;

type Symbol = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrokerError {
    InsufficientFunds,
    InsufficientMargin,
    InvalidOrder,
}

pub type BrokerResult<T> = Result<T, BrokerError>;

/// The main entity that a strategy interacts with throughout the core event loop.
/// The Broker is responsible for maintaining bookkeeping of all `active_orders` placed,
/// providing the strategy with information about the current state of the market,
/// and managing the strategy's portfolio.
///
/// If `trade_on_close` is `True`, allow trades to be executed on the close of a bar,
/// rather than the next day.
///
/// If `hedging` is true, allow trades in both directions simultaneously.
/// Otherwise, opposite-facing orders first close existing trades in a [FIFO] manner.
///
/// Consider: exclusive_orders
pub struct Broker {
    pub name: String,
    pub initial_cash: f32,
    pub commission: f32,
    pub margin: f32,
    pub trade_on_close: bool,
    pub hedging: bool,

    /// Internal bookkeeping of all active_orders placed.
    pub active_orders: HashMap<OrderId, Order>,
    pub canceled_orders: HashMap<OrderId, Order>, // Keeps track of all the orders that were cancelled.
    pub trades: HashMap<OrderId, Order>, // Keeps track of all the trades that were executed (orders that were filled)
    pub current_cash: f32,
    pub positions: HashMap<Symbol, Position>, // Keeps track of all the active positions
}

impl Broker {
    pub fn new(
        name: &str,
        initial_cash: f32,
        commission: f32,
        margin: f32,
        trade_on_close: bool,
        hedging: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            initial_cash,
            commission,
            margin,
            trade_on_close,
            hedging,
            active_orders: HashMap::new(),
            canceled_orders: HashMap::new(),
            trades: HashMap::new(),
            current_cash: initial_cash,
            positions: HashMap::new(),
        }
    }

    pub fn next(&mut self, ticker: &Ticker) -> Result<(), BrokerError> {
        self.process_active_orders(ticker)?;

        Ok(())
    }

    pub fn submit_order(&mut self, id: OrderId, order: Order) -> Result<(), BrokerError> {
        self.active_orders.insert(id, order);

        Ok(())
    }

    pub fn cancel_order(&mut self, id: OrderId) -> Result<(), BrokerError> {
        self.active_orders.remove(&id);

        Ok(())
    }

    /// Processes a single order.
    fn execute_order(&mut self, order: Order, ticker: &Ticker) -> Result<(), BrokerError> {
        match order.side {
            OrderSide::Buy => {
                if let Some(position) = self.positions.remove(&order.symbol) {
                    // We already have a position in this symbol. We need to update the position.
                    self.positions.insert(
                        order.symbol.clone(),
                        Position {
                            symbol: order.symbol,
                            amount: position.amount + order.quantity,
                            price: (position.amount * position.price
                                + order.quantity * ticker.close)
                                / (position.amount + order.quantity),
                        },
                    );
                } else {
                    self.positions.insert(
                        order.symbol.clone(),
                        Position {
                            symbol: order.symbol,
                            amount: order.quantity,
                            price: ticker.close,
                        },
                    );
                }
                // TODO: The ticker.close is not explicitly used
                // For example, in a limit order, the price is set.
                // Also, we must factor in the commission.
                info!("Bought {} shares @ {}", order.quantity, ticker.close);
                self.current_cash -= order.quantity * ticker.close;
            }
            OrderSide::Sell => {
                if let Some(position) = self.positions.remove(&order.symbol) {
                    // We already have a position in this symbol. We need to update the position.
                    let new_amount = position.amount - order.quantity;
                    if new_amount.abs() > std::f32::EPSILON {
                        self.positions.insert(
                            order.symbol.clone(),
                            Position {
                                symbol: order.symbol,
                                amount: new_amount,
                                price: (position.amount * position.price
                                    - order.quantity * ticker.close)
                                    / (position.amount - order.quantity),
                            },
                        );
                    }
                } else {
                    self.positions.insert(
                        order.symbol.clone(),
                        Position {
                            symbol: order.symbol,
                            amount: -order.quantity,
                            price: ticker.close,
                        },
                    );
                }
                info!("Sold {} shares @ {}", order.quantity, ticker.close);
                self.current_cash += order.quantity * ticker.close;
            }
        };

        info!("Positions: {:?}", self.positions);

        Ok(())
    }

    /// Processes all the withstanding active_orders in the order book.
    /// This function mainly handles the order processing logic, but the
    /// actual order execution is performed in 'execute_order'.
    ///
    /// # TODO: There needs to be some sense of time delay
    fn process_active_orders(&mut self, ticker: &Ticker) -> Result<(), BrokerError> {
        let mut non_executed_active_orders = HashMap::new();
        for (id, order) in self.active_orders.clone() {
            match order.order_type {
                OrderType::Market => self.execute_order(order, ticker)?,
                OrderType::Limit(price) => match order.side {
                    OrderSide::Buy => {
                        if ticker.close <= price {
                            self.execute_order(order, ticker)?;
                        } else {
                            non_executed_active_orders.insert(id, order);
                        }
                    }
                    OrderSide::Sell => {
                        if ticker.close >= price {
                            self.execute_order(order, ticker)?;
                        } else {
                            non_executed_active_orders.insert(id, order);
                        }
                    }
                },
                OrderType::Stop(price) => match order.side {
                    OrderSide::Buy => {
                        if ticker.close >= price {
                            self.execute_order(order, ticker)?;
                        } else {
                            non_executed_active_orders.insert(id, order);
                        }
                    }
                    OrderSide::Sell => {
                        if ticker.close <= price {
                            self.execute_order(order, ticker)?;
                        } else {
                            non_executed_active_orders.insert(id, order);
                        }
                    }
                },
                OrderType::MarketOnClose => {
                    todo!("Implement logic to determine if the market is closing");
                }
                OrderType::MarketOnOpen => {
                    todo!("Implement logic to determine if the market is opening");
                }
                _ => {
                    todo!("Implement the rest of the order types");
                }
            }
        }

        self.active_orders = non_executed_active_orders;

        Ok(())
    }
}
