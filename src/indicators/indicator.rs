use crate::dataframe::ticker::Ticker;

pub trait Indicator {
    fn update(&mut self, ticker: &Ticker);
    fn get_value(&self) -> f32;
}