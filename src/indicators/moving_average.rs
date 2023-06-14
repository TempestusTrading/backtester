use crate::dataframe::ticker::Ticker;
use crate::indicators::indicator::Indicator;

pub struct MovingAverage {
    period: u32,
    values: Vec<f32>,
    pub value: Option<f32>,
}

impl MovingAverage {
    pub fn new(period: u32) -> MovingAverage {
        MovingAverage {
            period,
            values: Vec::new(),
            value: None,
        }
    }
}

impl Indicator for MovingAverage {
    fn update(&mut self, ticker: &Ticker) {
        self.values.push(ticker.close);

        if self.values.len() < self.period as usize {
            return;
        } else {
            self.values.remove(0);
        }

        let mut sum = 0.0;
        for value in &self.values {
            sum += value;
        }

        self.value = Some(sum / self.values.len() as f32);
    }

    fn get_value(&self) -> Option<f32> {
        self.value
    }
}
