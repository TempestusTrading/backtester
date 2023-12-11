use super::*;
use crate::timeseries::*;

/// [Federal Funds Effective Rate](https://www.newyorkfed.org/markets/reference-rates/effr)
/// 
/// The actual rate at which commercial banks borrow and lend their excess reserves overnight.
/// Notice, that this is the actualized rate rather than the target federal funds rate.
#[derive(Clone)]
pub struct EFFR {
	current: f32,
	stream: TimeSeries
}

impl fmt::Display for EFFR {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "EFFR({})", self.current)
	}
}

impl EFFR {
	pub fn new(stream: TimeSeries) -> Self {
		Self {
			current: 0.0,
			stream
		}
	}
}

impl Indicator for EFFR {
	type Result = f32;

	fn update(&mut self, ticker: &Ticker) -> IndicatorResult<()> {
		// for l in self.stream {
		// 	if let Ok(t) = l {
		// 		self.current = t.close;
		// 	}
		// }
		Ok(())
	}

	fn get_value(&self) -> IndicatorResult<Self::Result> {
		Ok(self.current)
	}

	fn at(&self, index: usize) -> IndicatorResult<Self::Result> {
		Ok(self.current)
	}
}