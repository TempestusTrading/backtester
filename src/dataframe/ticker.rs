use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatetimeField {
    Number(u64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: u32,
    pub datetime: DatetimeField,
}

impl Ticker {
    pub fn new(
        open: f32,
        high: f32,
        low: f32,
        close: f32,
        volume: u32,
        datetime: DatetimeField,
    ) -> Ticker {
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
