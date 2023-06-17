use crate::broker::{Broker, BrokerError};
use serde_derive::{Deserialize, Serialize};

/// Since most of the data feeds are working with contain
/// integer timestamps as well as string representations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatetimeField {
    Number(u64),
    String(String),
}

/// Represents a position that a strategy has opened.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub amount: f32,
    pub price: f32,
}

/// When an order is filled a `Trade` is results.
///
/// This struct is mostly used for bookkeeping purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub symbol: String,
    pub quantity: f32,
    pub price: f32,
    pub commission: f32,
    pub time: DatetimeField,
}

/// Represents an update in the market state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: u32,
    pub datetime: DatetimeField,
}

pub type OrderId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    /// [Market](https://www.investopedia.com/terms/m/marketorder.asp)
    Limit(f32),
    /// [Limit](https://www.investopedia.com/terms/l/limitorder.asp)
    Stop(f32),
    /// [Stop]()
    StopLimit,
    /// [Stop Limit]()
    MarketOnClose,
    MarketOnOpen,
    LimitOnClose,
    LimitOnOpen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderExecutionStrategy {
    /// [Good-Till-Cancelled](https://www.investopedia.com/terms/g/gtc.asp)
    GTC,
    /// TODO: [Good-Till-Date](https://www.interactivebrokers.com/en/trading/orders/gtd.php)
    GTD,
    /// TODO: Good For Day
    GFD,
    /// TODO: [Fill-Or-Kill](https://www.investopedia.com/terms/f/fok.asp)
    FOK,
    /// TODO: [Immediate-Or-Cancel](https://www.investopedia.com/terms/i/immediateorcancel.asp)
    IOC,
}

/// Represents an order
///
/// One can place orders within a strategy by calling `Broker::submit_order`.
/// A log of all the current active orders can be found in `Broker::active_orders`.
/// Orders that are executed result in a `Trade`.
///
/// If you seek to update an order, cancel the existing order, and place a new one.
///
/// Notice, that `on_execute` and `on_cancel` callbacks are provided.
/// This is useful for setting contingeny orders.
///
/// For example, if you want to place a stop loss order when the original order is executed,
/// an `on_execute` callback can be provided that places a stop loss order.
///
/// ```
/// use backtester::prelude::*;
/// # use std::fmt;
/// #
/// # #[derive(Clone)]
/// # pub struct StopLoss;
/// #
/// # impl fmt::Display for StopLoss {
/// #     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/// #         write!(f, "Stop Loss")
/// #     }
/// # }
/// 
/// impl Strategy for StopLoss {
///     fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError> {
///         broker.submit_order(
///             0,
///             Order {
///                 symbol: "AAPL".to_string(),
///                 quantity: 100.0,
///                 side: OrderSide::Buy,
///                 order_type: OrderType::Market,
///                 time: ticker.datetime.clone(),
///                 execution: OrderExecutionStrategy::GTC,
///                 on_execute: Some(|broker| {
///                     broker.submit_order(
///                         1,
///                         Order {  
///                             symbol: "AAPL".to_string(),
///                             quantity: 100.0, 
///                             side: OrderSide::Sell,
///                             order_type: OrderType::Stop(100.0),
///                             time: broker.get_datetime(),
///                             execution: OrderExecutionStrategy::GTC,
///                             on_execute: None,
///                             on_cancel: None,
///                         }
///                     )?;
///                     Ok(())
///                 }),
///                 on_cancel: None,
///             },
///         )?;
///         Ok(())
///     }
/// }

#[derive(Clone)]
pub struct Order {
    pub symbol: String,
    pub quantity: f32,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub time: DatetimeField,
    pub execution: OrderExecutionStrategy,
    /// The following functions are meant to interact with the broker
    /// Therefore, they return a `BrokerError` if something goes wrong.
    /// If provided, this function is executed when the order is executed.
    pub on_execute: Option<fn(&mut Broker) -> Result<(), BrokerError>>,
    /// If provided, this function is executed when the order is cancelled.
    pub on_cancel: Option<fn(&mut Broker) -> Result<(), BrokerError>>,
}
