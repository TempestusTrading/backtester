#[derive(Debug)]
pub struct Ticker {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: u32,
    pub datetime: u64,
}

impl Ticker {
    pub fn new(open: f32, high: f32, low: f32, close: f32, volume: u32, datetime: u64) -> Ticker {
        Ticker {
            open,
            high,
            low,
            close,
            volume,
            datetime,
        }
    }
}
