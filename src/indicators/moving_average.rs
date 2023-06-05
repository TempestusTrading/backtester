use crate::dataframe::ticker::Ticker;
use crate::indicators::indicator::Indicator;

pub struct MovingAverage {
    period: u32,
    values: Vec<f32>,
    pub value: f32,
}

impl MovingAverage {
    pub fn new(period: u32) -> MovingAverage {
        MovingAverage {
            period,
            values: Vec::new(),
            value: 0.0,
        }
    }
}

impl Indicator for MovingAverage {
    fn update(&mut self, ticker: &Ticker) {
        self.values.push(ticker.close);
        if self.values.len() > self.period as usize {
            self.values.remove(0);
        }

        let mut sum = 0.0;
        for value in &self.values {
            sum += value;
        }
        self.value = sum / self.values.len() as f32;
    }

    fn get_value(&self) -> f32 {
        self.value
    }
}
