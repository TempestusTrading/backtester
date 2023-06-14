use super::{Indicator, IndicatorError, IndicatorResult, Ticker};

/// # [Simple Moving Average] (https://www.investopedia.com/terms/s/sma.asp)
pub struct SMA {
    period: u32,
    ticks: Vec<f32>,
    pub values: Vec<f32>,
}

impl Default for SMA {
    // Uses a 10 ticker period.
    fn default() -> Self {
        Self::new(10)
    }
}

impl SMA {
    pub fn new(period: u32) -> Self {
        Self {
            period,
            ticks: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl Indicator for SMA {
    type Result = f32;

    fn update(&mut self, ticker: &Ticker) -> IndicatorResult<()> {
        self.values.push(ticker.close);

        if self.values.len() < self.period as usize {
            return Ok(());
        } else {
            self.values.remove(0);
        }

        let mut sum = 0.0;
        for value in &self.values {
            sum += value;
        }

        self.values.push(sum / self.values.len() as f32);
        Ok(())
    }

    fn get_value(&self) -> IndicatorResult<Self::Result> {
        if self.values.is_empty() {
            return Err(IndicatorError::InsufficientData);
        }
        Ok(*self.values.last().unwrap())
    }

    fn at(&self, index: usize) -> IndicatorResult<Self::Result> {
        if index < self.values.len() {
            return Ok(*self.values.get(index).unwrap());
        }
        Err(IndicatorError::IndexOutOfRange)
    }
}
