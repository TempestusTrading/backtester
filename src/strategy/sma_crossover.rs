use crate::indicators::moving_average::MovingAverage;
use crate::strategy::strategy::{Strategy, StrategyBuilder};

#[derive(Debug, Clone)]
pub struct SMACrossoverStrategy;

impl SMACrossoverStrategy {
    fn new(period: u32) -> Strategy {
        StrategyBuilder::new()
            .add_indicator(Box::new(MovingAverage::new(period)))
            .build()
    }
}
