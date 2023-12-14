use super::*;
use crate::{
	indicators::EFFR,
	series::Series,
};

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
#[derive(Clone)]
pub struct EFFRTradingBuilder {
	bought: bool,
	// effr: Series::<DFF>,
	scale_factor: f32 // The scale factor is used to determine the position size.
}

// impl Default for EFFRTrading {
// 	fn default() -> Self {
// 		Self {
// 			bought: false,
// 			effr: EFFR::from_csv("./datasets/indicators/DFF.csv"),
// 			scale_factor: 0.5
// 		}
// 	}
// }

// impl EFFRTrading {
// 	pub fn new(effr: EFFR, scale_factor: f32) -> Self {
// 		Self {
// 			bought: false,
// 			effr,
// 			scale_factor
// 		}
// 	}
// }

// impl fmt::Display for EFFRTrading {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		write!(f, "EFFR Trading")
// 	}
// }

// impl Strategy for EFFRTrading {
// 	fn on_ticker(&mut self, ticker: &Ticker, broker: &mut Broker) -> Result<(), StrategyError> {
// 		if self.effr.update(ticker).is_ok() { // EFFR was updated, scale position accordingly
// 			if let Ok(effr) = self.effr.get_value() {
// 				if effr
// 			}
// 		};
// 	}
// }