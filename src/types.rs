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
#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub amount: f32,
    pub price: f32,
}

/// When an order is filled a `Trade` is executed.
/// This struct is mostly used for bookkeeping purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub symbol: String,
    pub quantity: f32,
    pub price: f32,
    pub commission: f32,
    pub time: DatetimeField,
}

/// Represnts an update in the market state
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
    Market,     // https://www.investopedia.com/terms/m/marketorder.asp
    Limit(f32), // https://www.investopedia.com/terms/l/limitorder.asp
    Stop(f32),
    StopLimit,
    TrailingStop,
    TrailingStopLimit,
    MarketOnClose,
    MarketOnOpen,
    LimitOnClose,
    LimitOnOpen,
}

/// Represents an order
///
/// TODO: This is a stub. We need to figure out how to represent orders.
/// For example, there are different types of orders, such as market orders,
/// limit orders, stop orders, etc. How should we represent these effectively?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub symbol: String,
    pub quantity: f32,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub time: DatetimeField,
}
