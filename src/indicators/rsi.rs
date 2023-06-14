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
		pub value: Option<f32>,
}

impl RSI {
		pub fn new(period: u32, smooth: bool) -> RSI {
				RSI {
						period,
						smooth,
						values: Vec::new(),
						value: None,
				}
		}
}

impl Indicator for RSI {
	fn update(&mut self, ticker: &Ticker) {
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

			if self.values.len() < period {
				self.gains.push(current_gain);
				self.losses.push(current_loss);
				return;
			}

			let average_gain = self.gains.iter().sum::<f32>() / self.period as f32;
			let average_loss = self.losses.iter().sum::<f32>() / self.period as f32;

			let step_one = 100.0 - (100.0 / (1.0 + (average_gain / average_loss)));

			if smooth {
				let step_two = 100 - (100 / (1 + ((self.previous_average_gain * (self.period - 1 ) + current_gain) / (self.previous_average_loss * (self.period - 1) + current_loss))));
				self.value = Some(step_two)
			} else {
					self.value = Some(step_one);
			}

			self.previous_average_gain = average_gain;
			self.previous_average_loss = average_loss;
	}

	fn get_value(&self) -> Option<f32> {
			self.value
	}
}