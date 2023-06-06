use std::collections::HashMap;

use std::error::Error;
use std::fmt;
use log::{warn, info};

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

/// Good reference for writing custom error types.
/// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html
impl fmt::Display for BrokerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BrokerError::InsufficientFunds => write!(f, "Insufficient funds"),
            BrokerError::InsufficientMargin => write!(f, "Insufficient margin"),
            BrokerError::InvalidOrder => write!(f, "Invalid order"),
        };
        Ok(())
    }
}

impl Error for BrokerError {}

pub struct Broker {
    name: String,
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
        initial_cash: f32,
        commission: f32,
        margin: f32,
        trade_on_close: bool,
        hedging: bool,
    ) -> Broker {
        Broker {
            name: name.to_string(),
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
                    // We already have a position in this symbol. We need to update the position.
                    self.positions.insert(
                        order.symbol.clone(),
                        Position {
                            symbol: order.symbol,
                            amount: position.amount + order.quantity,
                            price: (position.amount * position.price + order.quantity * ticker.close) / (position.amount + order.quantity),
                        },
                    );
                } else {
                    self.positions.insert(
                        order.symbol.clone(),
                        Position { 
                            symbol: order.symbol, 
                            amount: order.quantity, 
                            price: ticker.close, 
                        }
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
                                price: (position.amount * position.price - order.quantity * ticker.close) / (position.amount - order.quantity),
                            }
                        );
                    }
                } else {
                    self.positions.insert(
                        order.symbol.clone(),
                        Position { 
                            symbol: order.symbol, 
                            amount: -order.quantity, 
                            price: ticker.close, 
                        }
                    );
                }
                info!("Sold {} shares @ {}", order.quantity, ticker.close);
                self.current_cash += order.quantity * ticker.close;
            }
        };

        info!("Positions: {:?}", self.positions);

        Ok(())
    }

    /// Processes all the withstanding orders in the order book.
    /// This function mainly handles the order processing logic, but the
    /// actual order execution is performed in 'execute_order'.
    ///
    /// # TODO: There needs to be some sense of time delay
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
