use super::*;

/// # EFFR Trading
/// 
/// This is a simple strategy that trades based on the Effective Federal Funds Rate.
/// In particular, as the EFFR increases, the strategy will linearly decrease its position size
/// in given asset. As the EFFR decreases, the strategy will linearly increase its position size.
/// 50% of capital is allocated from the start.
#[derive(CLone)]
pub struct EFFRTrading {
	bought: bool,
	effr: EFFR,
	scale_factor: f32 // The scale factor is used to determine the position size.
}

impl Default for EFFRTrading {
	fn default() -> Self {
		Self {
			bought: false,
			effr: EFFR::new(),
			scale_factor: 0.5
		}
	}
}