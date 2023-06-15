use super::{Indicator, IndicatorError, IndicatorResult, Ticker};

/// [Relative Strength Index](https://www.investopedia.com/terms/r/rsi.asp)
///
/// Measures the speed of a security's recent price changes.
/// An RSI reading of 30 or below indicates overbought market conditions,
/// while a reading of 30 or below indicates an oversold condition.
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
    /// Default period of `14` tickers, smoothing is `true`.
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
    type Result = f32;

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

        if self.values.len() < self.period as usize {
            self.gains.push(current_gain);
            self.losses.push(current_loss);
            return Ok(());
        }

        let average_gain = self.gains.iter().sum::<f32>() / self.period as f32;
        let average_loss = self.losses.iter().sum::<f32>() / self.period as f32;

        let step_one = 100.0 - (100.0 / (1.0 + (average_gain / average_loss)));

        if self.smooth {
            let step_two = 100.0
                - (100.0
                    / (1.0
                        + ((self.previous_average_gain * (self.period - 1) as f32
                            + current_gain)
                            / (self.previous_average_loss * (self.period - 1) as f32
                                + current_loss))));
            self.values.push(step_two);
        } else {
            self.values.push(step_one);
        }

        self.previous_average_gain = average_gain;
        self.previous_average_loss = average_loss;
        Ok(())
    }

    fn get_value(&self) -> IndicatorResult<Self::Result> {
        match !self.values.is_empty() {
            true => Ok(*self.values.last().unwrap()),
            false => Err(IndicatorError::InsufficientData),
        }
    }

    fn at(&self, index: usize) -> IndicatorResult<Self::Result> {
        match index < self.values.len() {
            true => Ok(*self.values.get(index).unwrap()),
            false => Err(IndicatorError::IndexOutOfRange),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_tickers_period_5() {
        let mut rsi = RSI::new(5, true);

        assert!(rsi.get_value().is_err());
    }

    #[test]
    fn invalid_index() {
        let sma = RSI::new(5, true);

        assert!(sma.at(2).is_err());
    }
}