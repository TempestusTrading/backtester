use super::{Indicator, IndicatorError, IndicatorResult, Ticker};

/// # Relative Strength Index
/// https://www.investopedia.com/terms/r/rsi.asp
/// Measures the sspeed of a security's recent price changes
/// An RSI reading of 30 or below indicators overbought
/// while a reading of 30 or below indicates an oversold
/// condition.
pub struct RSI {
    period: u32,
    smooth: bool,
    gains: Vec<f32>,
    losses: Vec<f32>,
    previous_average_gain: f32,
    previous_average_loss: f32,
    values: Vec<f32>,
}

impl Default for RSI {
    fn default() -> Self {
        Self::new(14, true)
    }
}

impl RSI {
    pub fn new(period: u32, smooth: bool) -> Self {
        Self {
            period,
            smooth,
            gains: Vec::new(),
            losses: Vec::new(),
            previous_average_gain: 0.0,
            previous_average_loss: 0.0,
            values: Vec::new(),
        }
    }
}

impl Indicator for RSI {
    fn update(&mut self, ticker: &Ticker) -> IndicatorResult<()> {
        let current_gain = if ticker.close > ticker.open {
            ticker.close - ticker.open
        } else {
            0.0
        };
        let current_loss = if ticker.close < ticker.open {
            ticker.open - ticker.close
        } else {
            0.0
        };

        if self.values.len() < self.period {
            self.gains.push(current_gain);
            self.losses.push(current_loss);
            return Ok(());
        }

        let average_gain = self.gains.iter().sum::<f32>() / self.period as f32;
        let average_loss = self.losses.iter().sum::<f32>() / self.period as f32;

        let step_one = 100.0 - (100.0 / (1.0 + (average_gain / average_loss)));

        if self.smooth {
            let step_two = 100
                - (100
                    / (1 + ((self.previous_average_gain * (self.period - 1) + current_gain)
                        / (self.previous_average_loss * (self.period - 1) + current_loss))));
            self.value = Some(step_two)
        } else {
            self.value = Some(step_one);
        }

        self.previous_average_gain = average_gain;
        self.previous_average_loss = average_loss;
        Ok(())
    }

    fn get_value(&self) -> IndicatorResult<f32> {
        if self.values.is_empty() {
            return Err(IndicatorError::InsufficientData);
        }
        Ok(self.values.last().copied())
    }

    fn at(&self, index: usize) -> IndicatorResult<f32> {
        if index < self.values.len() {
            Ok(self.values[index])
        }
        Err(IndicatorError::IndexOutOfRange)
    }
}
