//! The main entity that a strategy interacts with throughout the core event loop.
use crate::types::*;

use serde_derive::{Deserialize, Serialize};

use log::info;
use std::collections::HashMap;
use std::fmt;
use chrono::{DateTime, Duration, Utc, Date};

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
    exclusive_orders: bool,
    hedging: bool,
    logging: bool,
    datetime: DateTime<Utc>,

    /// Internal bookkeeping
    active_orders: HashMap<OrderId, Order>,
    canceled_orders: HashMap<OrderId, Order>, // Keeps track of all the orders that were cancelled.
    trades: HashMap<OrderId, Order>, // Keeps track of all the trades that were executed (orders that were filled)
    current_cash: f32,
    positions: HashMap<Symbol, Position>, // Keeps track of all the active positions
    previous_ticker: Option<Ticker>
}

impl fmt::Display for Broker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{}(\n", self.name));
        result.push_str(&format!("Initial Cash: {}\n", self.initial_cash));
        result.push_str(&format!("Commission: {}\n", self.commission));
        result.push_str(&format!("Leverage: {}\n", self.leverage));
        result.push_str(&format!("Exclusive Orders: {}\n", self.exclusive_orders));
        result.push_str(&format!("Hedging: {}\n", self.hedging));
        // result.push_str(&format!("Trades: {}\n", self.trades));
        result.push_str(&format!("Current Cash: {}\n", self.current_cash));
        result.push_str(&format!("Positions: {:?}\n)", self.positions));
        write!(f, "{}", result)
    }
}

impl Broker {
    /// Creates a new Broker instance.
    /// - `name` - Useful for identifying the broker within a Backtest.
    /// - `initial_cash` - The amount of cash to start with.
    /// - `commission` - The percentage of cash to be paid as commission for each trade. This value should be in [-0.1, 0.1].
    /// - [`margin`](https://www.interactivebrokers.com/en/trading/margin.php) - The percentage of cash to be used as margin for each trade. This value should be in [0, 1].
    /// - `exclusive_orders` - If `true`, each new order auto-closes the previous trade/position, making at most a single trade (long or short) in effect at each time.
    /// - [`hedging`](https://www.investopedia.com/terms/h/hedge.asp) - If `true`, allow trades in both directions simultaneously. If `false`, opposite-facing orders first close existing trades in a [FIFO] manner.
    /// - `logging` - If `true`, log all the broker's activity. Useful for debugging.
    pub fn new(
        name: &str,
        initial_cash: f32,
        commission: f32,
        margin: f32,
        exclusive_orders: bool,
        hedging: bool,
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
            exclusive_orders,
            hedging,
            logging,
            datetime: Utc::now(),
            active_orders: HashMap::new(),
            canceled_orders: HashMap::new(),
            trades: HashMap::new(),
            current_cash: initial_cash,
            positions: HashMap::new(),
            previous_ticker: None,
        }
    }

    pub fn next(&mut self, ticker: &Ticker) -> Result<(), BrokerError> {
        if self.logging {
            info!("Ticker: {}\nBroker State: {}\n", ticker, self);
        }

        self.datetime = DateTime::from(ticker.datetime);
        self.process_active_orders(ticker)?;
        self.previous_ticker = Some(ticker.clone());

        Ok(())
    }

    pub fn submit_order(&mut self, id: OrderId, order: Order) -> Result<(), BrokerError> {
        if self.logging {
            info!("Order (submit): {}\n", order);
        }

        self.active_orders.insert(id, order);

        Ok(())
    }

    pub fn cancel_order(&mut self, id: OrderId) -> Result<(), BrokerError> {
        if self.logging {
            info!("Order (cancel): {}\n", id);
        }

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
                OrderType::Limit(limit) => match order.side {
                    OrderSide::Buy => {
                        if ticker.close <= limit {
                            self.execute_order(order, ticker)?;
                            continue;
                        }
                    }
                    OrderSide::Sell => {
                        if ticker.close >= limit {
                            self.execute_order(order, ticker)?;
                            continue;
                        }
                    }
                },
                OrderType::Stop(stop) => match order.side {
                    OrderSide::Buy => {
                        // Buy Stop Order turns into a Market Buy Order when the price is above the stop price
                        if ticker.close >= stop {
                            self.submit_order(id, Order {
                                symbol: order.symbol,
                                quantity: order.quantity,
                                side: OrderSide::Buy,
                                order_type: OrderType::Market,
                                execution: order.execution,
                                datetime: self.get_datetime(),
                                on_execute: order.on_execute,
                                on_cancel: order.on_cancel,
                            })?;
                            continue;
                        }
                    }
                    OrderSide::Sell => {
                        // Sell Stop Order turns into a Market Sell Order when the price is below the stop price
                        if ticker.close <= stop {
                            self.submit_order(id, Order {
                                symbol: order.symbol,
                                quantity: order.quantity,
                                side: OrderSide::Sell,
                                order_type: OrderType::Market,
                                execution: order.execution,
                                datetime: self.get_datetime(),
                                on_execute: order.on_execute,
                                on_cancel: order.on_cancel,
                            })?;
                            continue;
                        }
                    }
                },
                OrderType::StopLimit(stop, limit) => match order.side {
                    OrderSide::Buy => {
                        // Buy Stop Order turns into a Limit Buy Order when the price is above the stop price and below the limit price
                        if ticker.close >= stop && ticker.close < limit {
                            self.submit_order(id, Order {
                                symbol: order.symbol,
                                quantity: order.quantity,
                                side: OrderSide::Buy,
                                order_type: OrderType::Limit(limit),
                                execution: order.execution,
                                datetime: self.get_datetime(),
                                on_execute: order.on_execute,
                                on_cancel: order.on_cancel,
                            })?;
                            continue;
                        }
                    }
                    OrderSide::Sell => {
                        // Sell Stop Order turns into a Limit Sell Order when the price is below the stop price and above the limit price
                        if ticker.close <= stop && ticker.close > limit {
                            self.submit_order(id, Order {
                                symbol: order.symbol,
                                quantity: order.quantity,
                                side: OrderSide::Sell,
                                order_type: OrderType::Limit(limit),
                                execution: order.execution,
                                datetime: self.get_datetime(),
                                on_execute: order.on_execute,
                                on_cancel: order.on_cancel,
                            })?;
                            continue;
                        }
                    }
                },
                OrderType::MOC => {
                    if self.next_date() {
                        if let Some(previous) = &self.previous_ticker.clone() {
                            self.execute_order(order, previous)?;
                            continue;
                        }
                    }
                },
                OrderType::MOO => {
                    if self.next_date() {
                        self.execute_order(order, ticker)?;
                        continue;
                    }
                    todo!();
                },
                OrderType::LOC(limit) => {
                    if self.next_date() {
                        if let Some(previous) = &self.previous_ticker.clone() {
                            match order.side {
                                OrderSide::Buy => {
                                    if ticker.close <= limit {
                                        self.execute_order(order, previous)?;
                                        continue;
                                    }
                                }
                                OrderSide::Sell => {
                                    if ticker.close >= limit {
                                        self.execute_order(order, previous)?;
                                        continue;
                                    }
                                }
                            }
                        }
                    }   
                },
                OrderType::LOO(limit) => {
                    if self.next_date() {
                        match order.side {
                            OrderSide::Buy => {
                                if ticker.close <= limit {
                                    self.execute_order(order, ticker)?;
                                    continue;
                                }
                            }
                            OrderSide::Sell => {
                                if ticker.close >= limit {
                                    self.execute_order(order, ticker)?;
                                    continue;
                                }
                            }
                        }
                    }
                },
            }

            // This code will be executed if no order was executed.
            // Otherwise, we skip over this block with the use of `continue`.
            non_executed_active_orders.insert(id, order);
        }

        self.active_orders = non_executed_active_orders;

        Ok(())
    }

    pub fn get_datetime(&self) -> DateTime<Utc> {
        self.datetime.clone()
    }

    /// Returns `true` if the current `Ticker` being processed is the beginning of a new trading day.
    fn next_date(&self) -> bool {
        if let Some(previous) = &self.previous_ticker {
            return self.get_datetime() - DateTime::from(previous.datetime) > Duration::hours(8)
        }
        true
    }
}