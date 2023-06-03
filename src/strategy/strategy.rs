use crate::dataframe::ticker::Ticker;

pub trait Strategy {
	fn on_ticker(&mut self, ticker: Ticker);
	fn on_order_filled();
	fn on_order_changed();
	fn update_indicators(&mut self, ticker: &Ticker);
}

pub struct SimpleSMA {
	close20Tickers: Vec<f32>,
	sma20Tickers: f32
} 

impl SimpleSMA {
	pub fn new() -> SimpleSMA {
		SimpleSMA { close20Tickers: vec![], sma20Tickers: 0.0 }
	}
}

impl Strategy for SimpleSMA {
	fn on_ticker(&mut self, ticker: Ticker) {
		self.update_indicators(&ticker);

		if (self.sma20Tickers > ticker.close) {
			println!("BUY!");
		}
	}

	fn on_order_filled() {
	}

	fn on_order_changed() {
	}

	fn update_indicators(&mut self, ticker: &Ticker) {
		self.close20Tickers.push(ticker.close);
		if self.close20Tickers.len() == 21 {
			let first_ticker = self.close20Tickers.remove(0);
			self.sma20Tickers = (20.0 * self.sma20Tickers - first_ticker + self.close20Tickers[19]) / 20.0;
		}
	}
}