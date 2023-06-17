//! The main entity that a strategy interacts with throughout the core event loop.
use crate::types::*;

use serde_derive::{Deserialize, Serialize};

use log::info;
use std::collections::HashMap;
use std::fmt;

type Symbol = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrokerError {
    InsufficientFundsForPurchase,
    OutOfMoneyError,
    InsufficientMargin,
    OrderIdNotFound,
}

pub type BrokerResult<T> = Result<T, BrokerError>;

/// The Broker is responsible for maintaining bookkeeping of all `active_orders` placed,
/// providing the strategy with information about the current state of the market,
/// and managing the strategy's portfolio.
///
/// If `trade_on_close` is `True`, allow trades to be executed on the close of a bar,
/// rather than the next day.
///
/// If `hedging` is true, allow trades in both directions simultaneously.
/// Otherwise, opposite-facing orders first close existing trades in a [FIFO] manner.
#[derive(Clone)]
pub struct Broker {
    name: String,
    initial_cash: f32,
    commission: f32,
    leverage: f32,
    trade_on_close: bool,
    hedging: bool,
    exclusive_orders: bool,
    logging: bool,
    datetime: DatetimeField,

    /// Internal bookkeeping
    active_orders: HashMap<OrderId, Order>,
    canceled_orders: HashMap<OrderId, Order>, // Keeps track of all the orders that were cancelled.
    trades: HashMap<OrderId, Order>, // Keeps track of all the trades that were executed (orders that were filled)
    current_cash: f32,
    positions: HashMap<Symbol, Position>, // Keeps track of all the active positions
}

impl fmt::Display for Broker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Broker: {}\nCurrent Cash:{}\nPositions:{:?}\n",
            self.name, self.current_cash, self.positions
        )
    }
}

impl Broker {
    /// Creates a new Broker instance.
    /// - `name` - Useful for identifying the broker within a Backtest.
    /// - `initial_cash` - The amount of cash to start with.
    /// - `commission` - The percentage of cash to be paid as commission for each trade. This value should be in [-0.1, 0.1].
    /// - [`margin`](https://www.interactivebrokers.com/en/trading/margin.php) - The percentage of cash to be used as margin for each trade. This value should be in [0, 1].
    /// - `trade_on_close` - If `True`, allow trades to be executed on the close of a bar, rather than the next day.
    /// - [`hedging`](https://www.investopedia.com/terms/h/hedge.asp) - If `true`, allow trades in both directions simultaneously. If `false`, opposite-facing orders first close existing trades in a [FIFO] manner.
    /// - `exclusive_orders` - If `true`, only one order can be active at a time. Thus, if other orders are active, the older orders will be canceled in place of the new one.
    /// - `logging` - If `true`, log all the broker's activity. Useful for debugging.
    pub fn new(
        name: &str,
        initial_cash: f32,
        commission: f32,
        margin: f32,
        trade_on_close: bool,
        hedging: bool,
        exclusive_orders: bool,
        logging: bool,
    ) -> Self {
        if initial_cash < 0.0 {
            panic!("Broker: {} initial_cash should be positive.", name);
        }

        if commission > 0.1 || commission < -0.1 {
            panic!("Broker: {} commission should between -10% (market-maker's rebates) and 10% (fees).", name);
        }

        if margin < 0.0 || margin > 1.0 {
            panic!("Broker: {} margin should be between 0 and 1.", name);
        }

        Self {
            name: name.to_string(),
            initial_cash,
            commission,
            leverage: 1.0 / margin,
            trade_on_close,
            hedging,
            exclusive_orders,
            logging,
            datetime: DatetimeField::Number(0),
            active_orders: HashMap::new(),
            canceled_orders: HashMap::new(),
            trades: HashMap::new(),
            current_cash: initial_cash,
            positions: HashMap::new(),
        }
    }

    pub fn next(&mut self, ticker: &Ticker) -> Result<(), BrokerError> {
        self.datetime = ticker.datetime.clone();
        self.process_active_orders(ticker)?;

        if self.logging {
            info!("{}", self);
        }

        Ok(())
    }

    pub fn submit_order(&mut self, id: OrderId, order: Order) -> Result<(), BrokerError> {
        self.active_orders.insert(id, order);

        Ok(())
    }

    pub fn cancel_order(&mut self, id: OrderId) -> Result<(), BrokerError> {
        if let Some(order) = self.active_orders.remove(&id) {
            if let Some(callback) = order.on_cancel {
                callback(self)?;
            }
        } else {
            return Err(BrokerError::OrderIdNotFound);
        }

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

        // Handle the `on_execute` callback
        if let Some(callback) = order.on_execute {
            callback(self)?;
        }

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
                OrderType::Market => {
                    self.execute_order(order, ticker)?;
                    continue;
                }
                OrderType::Limit(price) => match order.side {
                    OrderSide::Buy => {
                        if ticker.close <= price {
                            self.execute_order(order, ticker)?;
                            continue;
                        }
                    }
                    OrderSide::Sell => {
                        if ticker.close >= price {
                            self.execute_order(order, ticker)?;
                            continue;
                        }
                    }
                },
                OrderType::Stop(price) => match order.side {
                    OrderSide::Buy => {
                        if ticker.close >= price {
                            self.execute_order(order, ticker)?;
                            continue;
                        }
                    }
                    OrderSide::Sell => {
                        if ticker.close <= price {
                            self.execute_order(order, ticker)?;
                            continue;
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

            // This code will be executed if no order was executed.
            // Otherwise, we skip over this block with the use of `continue`.
            non_executed_active_orders.insert(id, order);
        }

        self.active_orders = non_executed_active_orders;

        Ok(())
    }

    pub fn get_datetime(&self) -> DatetimeField {
        self.datetime.clone()
    }
}
