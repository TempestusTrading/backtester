use super::*;

/// # [Simple Moving Average](https://www.investopedia.com/terms/s/sma.asp)
#[derive(Clone)]
pub struct SMA {
    /// Use the last `period` tickers to use in the calculation.
    period: u32,
    /// The last `period` closing values.
    ticks: Vec<f32>,
    values: Vec<f32>,
}

impl Default for SMA {
    fn default() -> Self {
        Self::new(10)
    }
}

impl SMA {
    /// Default uses a `10` ticker period.
    pub fn new(period: u32) -> Self {
        Self {
            period,
            ticks: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl fmt::Display for SMA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMA(Period: {})", self.period)
    }
}

impl Indicator for SMA {
    type Result = f32;

    fn update(&mut self, ticker: &Ticker) -> IndicatorResult<()> {
        self.ticks.push(ticker.close);

        // Return early if we don't have enough data
        if self.ticks.len() < self.period as usize {
            return Ok(());
        }

        // If we have not calculated the SMA yet, we must add all the prices
        if self.values.is_empty() {
            let value = self.ticks.iter().sum::<f32>() / self.period as f32;
            self.values.push(value);
        } else {
            // Otherwise, we can simply add the latest price and remove the oldest
            let oldest = self.ticks.remove(0);
            let latest_value = self.get_value().unwrap();
            let value =
                (latest_value * self.period as f32 - oldest + ticker.close) / self.period as f32;
            self.values.push(value);
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn get_ticks(n: usize) -> Vec<Ticker> {
        let mut ticks = Vec::new();
        for i in 0..n {
            ticks.push(Ticker {
                open: 1.0,
                high: 2.0,
                low: 0.5,
                close: 1.5,
                volume: 100,
                datetime: Utc::now(),
            });
        }
        ticks
    }

    #[test]
    fn no_tickers_period_5() {
        let sma = SMA::new(5);

        assert!(sma.get_value().is_err());
    }

    #[test]
    fn invalid_index() {
        let sma = SMA::new(5);

        assert!(sma.at(2).is_err());
    }

    #[test]
    fn period_5_ticks_10() {
        let mut sma = SMA::new(5);

        let ticks = get_ticks(10);
        let mut sum = 0.0;
        for (i, tick) in ticks.iter().enumerate() {
            if i < 5 {
                assert!(sma.get_value().is_err());
            } else {
                sum += tick.close;
            }
            sma.update(&tick).unwrap();
        }

        assert_eq!(sma.get_value().unwrap(), sum / 5.0);
    }
}
