use crate::broker::{Broker, BrokerError};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use chrono::{DateTime, Utc};


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
    #[serde(with = "backtester_date_format")]
    pub datetime: DateTime<Utc>,
}

/// Represents an update in the market state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: u32,
    #[serde(with = "backtester_date_format")]
    pub datetime: DateTime<Utc>,
}

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ticker(Open: {}, High: {}, Low: {}, Close: {}, Volume: {}, Datetime: {})",
            self.open, self.high, self.low, self.close, self.volume, self.datetime
        )
    }
}

pub type OrderId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "Buy"),
            OrderSide::Sell => write!(f, "Sell"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    /// [Market](https://www.investopedia.com/terms/m/marketorder.asp)
    Market,
    /// [Limit](https://www.investopedia.com/terms/l/limitorder.asp)
    Limit(f32),
    /// [Stop](https://www.investopedia.com/terms/s/stoporder.asp)
    Stop(f32),
    /// [Stop Limit](https://www.investopedia.com/terms/s/stop-limitorder.asp)
    StopLimit(f32, f32),
    /// [Market On Close](https://www.investopedia.com/terms/m/marketonclose.asp)
    MOC,
    /// [Market On Open](https://www.investopedia.com/terms/m/marketonopen-order-moo.asp)
    MOO,
    /// [Limit On Close](https://www.investopedia.com/terms/l/limitoncloseorder.asp)
    LOC(f32),
    /// [Limit On Open](https://www.investopedia.com/terms/l/limitonopenorder.asp)
    LOO(f32),
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderType::Market => write!(f, "Market"),
            OrderType::Limit(limit) => write!(f, "Limit({})", limit),
            OrderType::Stop(stop) => write!(f, "Stop({})", stop),
            OrderType::StopLimit(stop, limit) => write!(f, "StopLimit(Stop: {}, Limit: {})", stop, limit),
            OrderType::MOC => write!(f, "MOC"),
            OrderType::MOO => write!(f, "MOO"),
            OrderType::LOC(limit) => write!(f, "LOC({})", limit),
            OrderType::LOO(limit) => write!(f, "LOO({})", limit),
        }
    }
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

impl fmt::Display for OrderExecutionStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderExecutionStrategy::GTC => write!(f, "GTC"),
            OrderExecutionStrategy::GTD => write!(f, "GTD"),
            OrderExecutionStrategy::GFD => write!(f, "GFD"),
            OrderExecutionStrategy::FOK => write!(f, "FOK"),
            OrderExecutionStrategy::IOC => write!(f, "IOC"),
        }
    }
}

/// Represents an order
///
/// One can place orders within a strategy by calling `Broker::submit_order`.
/// A log of all the current active orders can be found in `Broker::active_orders`.
/// Orders that are executed result in a `Trade`.
///
/// If you seek to update an order, cancel the existing order, and place a new one.
/// 
/// ## Dynamic Orders
/// 
/// Notice, that `on_execute` and `on_cancel` callbacks are provided.
/// These are useful for setting contingency orders.
///
/// ### Stop Loss Example
/// 
/// If you want to place a [stop loss](https://www.investopedia.com/articles/active-trading/091813/which-order-use-stoploss-or-stoplimit-orders.asp)
/// order when the original order is executed, an `on_execute` callback can be provided that places a stop order.
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
///                 datetime: ticker.datetime.clone(),
///                 execution: OrderExecutionStrategy::GTC,
///                 on_execute: Some(|broker| {
///                     broker.submit_order(
///                         1,
///                         Order {  
///                             symbol: "AAPL".to_string(),
///                             quantity: 100.0, 
///                             side: OrderSide::Sell,
///                             order_type: OrderType::Stop(90.0), // -$10 Profit at 100 Shares = -$1000
///                             datetime: broker.get_datetime(),
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
/// ```
/// 
/// ### Take Profit Example
/// 
/// Similar, we can create a strategy that places a [take profit](https://www.investopedia.com/terms/t/take-profitorder.asp)
/// limit order when the original order is executed.
/// 
/// ```
/// use backtester::prelude::*;
/// # use std::fmt;
/// #
/// # #[derive(Clone)]
/// # pub struct TakeProfit;
/// #
/// # impl fmt::Display for TakeProfit {
/// #     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/// #         write!(f, "Stop Loss")
/// #     }
/// # }
/// 
/// impl Strategy for TakeProfit {
///     fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError> {
///         broker.submit_order(
///             0,
///             Order {
///                 symbol: "AAPL".to_string(),
///                 quantity: 100.0,
///                 side: OrderSide::Buy,
///                 order_type: OrderType::Market,
///                 datetime: ticker.datetime.clone(),
///                 execution: OrderExecutionStrategy::GTC,
///                 on_execute: Some(|broker| {
///                     broker.submit_order(
///                         1,
///                         Order {  
///                             symbol: "AAPL".to_string(),
///                             quantity: 100.0, 
///                             side: OrderSide::Sell,
///                             order_type: OrderType::Stop(110.0), // $10 Profit * 100 Shares = $1000
///                             datetime: broker.get_datetime(),
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
/// ```
/// 

#[derive(Clone)]
pub struct Order {
    pub symbol: String,
    pub quantity: f32,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub datetime: DateTime<Utc>,
    pub execution: OrderExecutionStrategy,
    /// If provided, this function is executed when the order is executed.
    pub on_execute: Option<fn(&mut Broker) -> Result<(), BrokerError>>,
    /// If provided, this function is executed when the order is cancelled.
    pub on_cancel: Option<fn(&mut Broker) -> Result<(), BrokerError>>,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Order: {} {} {} @ {} {}",
            self.side, self.quantity, self.symbol, self.order_type, self.datetime
        )
    }
}

mod backtester_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp: i64 = Deserialize::deserialize(deserializer)?;
        let naive_datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
        Ok(naive_datetime)
    }
}