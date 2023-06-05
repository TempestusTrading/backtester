pub type OrderId = usize;

#[derive(Debug, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub enum OrderType {
    Market, // https://www.investopedia.com/terms/m/marketorder.asp
    Limit(f32), // https://www.investopedia.com/terms/l/limitorder.asp
    Stop(f32),
    // StopLimit,
    // TrailingStop,
    // TrailingStopLimit,
    MarketOnClose,
    MarketOnOpen,
    // LimitOnClose,
    // LimitOnOpen,
}

/// Represents an order
///
/// TODO: This is a stub. We need to figure out how to represent orders.
/// For example, there are different types of orders, such as market orders,
/// limit orders, stop orders, etc. How should we represent these effectively?
#[derive(Debug, Clone)]
pub struct Order {
    pub symbol: String,
    pub quantity: f32,
    pub side: OrderSide,
    pub order_type: OrderType,
    // pub order_type: OrderType,
    // pub order_time: DateTime<Utc>,
}

impl Order {
    pub fn new(
        symbol: String,
        quantity: f32,
        side: OrderSide,
        order_type: OrderType,
    ) -> Order {
        Order {
            symbol,
            quantity,
            side,
            order_type,
        }
    }
}
