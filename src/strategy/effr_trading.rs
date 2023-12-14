use super::*;
use crate::{
	indicators::EFFR,
	series::Series,
};
use std::cmp::{max, min};

// The root cause of many Clone implementations is the dependency on the Series type, whose iterator is not clonable
// Either 
// 1) Make the seriese iterator cloneable
// 2) Builder Route
// - StrategyBuilder
// - IndicatorBuilder
// These will be clonable and allow a cartesian product to be performed in the intialization of the backtester.
// Then, we can move to non-clonable versions of the strategies and indicators.
// This might have performance gains since clone implementations no longer clone internal types which might be expensive
// 3) ...

/// # EFFR Trading
/// 
/// This is a simple strategy that trades based on the Effective Federal Funds Rate.
/// In particular, as the EFFR increases, the strategy will linearly decrease its position size
/// in given asset. As the EFFR decreases, the strategy will linearly increase its position size.
/// 50% of capital is allocated from the start.
/// 
/// The precise formula is:
/// l_t = long threshold (parameter)
/// s_t = short threshold (parameter)
/// percent_allocated(effr) = (-2*effr / (s_t - l_t)) + 1
/// percent_allocated_capped(effr) = max(min(percent_allocated(effr), 1), -1)
#[derive(Clone)]
pub struct EFFRTrading {
	effr: EFFR, // Effective Federal Funds Rate indicator
	long_threshold: f32, // The threshold to go 100% long all capital
	short_threshold: f32, // The threshold to go 100% short all capital
	starting_capital: f32, // Used to calculate the percent allocated
	order_id: usize
}

impl Default for EFFRTrading {
	fn default() -> Self {
		Self {
			effr: EFFR::from_csv("./datasets/indicators/DFF.csv"),
			long_threshold: 0.0,
			short_threshold: 2.0,
			starting_capital: 0.0,
			order_id: 0
		}
	}
}

impl EFFRTrading {
	pub fn new(effr: EFFR, long_threshold: f32, short_threshold: f32) -> Self {
		Self {
			effr,
			long_threshold,
			short_threshold,
			starting_capital: 0.0,
			order_id: 0
		}
	}
}

impl fmt::Display for EFFRTrading {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "EFFR Trading(long_threshold: {}, short_threshold: {})", self.long_threshold, self.short_threshold)
	}
}

impl Strategy for EFFRTrading {
	fn prepare(&mut self, broker: &mut Broker) -> Result<(), StrategyError> {
		self.starting_capital = broker.get_cash();
		Ok(())
	}

	fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError> {
		if self.effr.update(ticker).is_ok() { // EFFR was updated, scale position accordingly
			let effr = self.effr.get_value().unwrap();
			let available = broker.get_cash();
			let percent_allocated = self.get_percent_allocated(available);
			let target_allocated = self.get_target_position(effr);
			let percent_diff = percent_allocated - target_allocated;
			// Calculate buy / sell the correction amount
			let mut quantity = (percent_diff * self.starting_capital / ticker.close).floor();
			let side = if quantity > 0.0 { 
				OrderSide::Buy 
			} else { 
				quantity = -quantity;
				OrderSide::Sell 
			};
			broker.submit_order(self.order_id, Order { 
					symbol: "AAPL".to_string(),
					quantity, 
					side,
					order_type: OrderType::Market, 
					datetime: ticker.datetime.clone(), 
					execution: OrderExecutionStrategy::GTC,
					on_execute: None, 
					on_cancel: None 
				}
			).err();
			self.order_id += 1;
		}
		Ok(())
	}
}

impl EFFRTrading {
	/// Returns the percent of capital that is currently allocated
	fn get_percent_allocated(&self, available: f32) -> f32 {
			(self.starting_capital - available) / self.starting_capital
	}
	/// Returns the target percent of capital that should be allocated baseds on the EFFR
	fn get_target_position(&self, effr: f32) -> f32 {
		f32::max(f32::min(-2.0 * effr / (self.short_threshold - self.long_threshold) + 1.0, 1.0), -1.0)
	}
}