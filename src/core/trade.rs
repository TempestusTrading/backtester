/// When an order is filled, it results in a 'Trade'
/// This struct is mostly used for bookkeeping purposes.
/// It is not used for any strategy logic.
pub struct Trade {
    pub symbol: String,
    pub quantity: f32,
    pub price: f32,
    pub commission: f32,
    // pub time: DateTime<Utc>,
}
