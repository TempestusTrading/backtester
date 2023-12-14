use super::*;
use crate::{
	util::deserializers::*,
	series::SeriesIntoIterator
};
use chrono::DateTime;

#[derive(Deserialize, Debug)]
pub struct DFF {
	#[serde(rename = "DFF")]
	dff: f32,
	#[serde(rename = "DATE")]
	#[serde(with = "yyyy_mm_dd")]
	date: DateTime<Utc>
}

/// [Federal Funds Effective Rate](https://www.newyorkfed.org/markets/reference-rates/effr)
/// 
/// The actual rate at which commercial banks borrow and lend their excess reserves overnight.
/// Notice, that this is the actualized rate rather than the target federal funds rate.
pub struct EFFR {
	previous: Option<DFF>,
	current: Option<f32>,
	date: DateTime<Utc>,
	stream: SeriesIntoIterator<DFF>
}

impl fmt::Display for EFFR {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "EFFR({:?})", self.current)
	}
}

impl EFFR {
	pub fn new(stream: Series<DFF>) -> Self {
		Self {
			previous: None,
			current: None,
			date: DateTime::from_timestamp(0, 0).unwrap(),
			stream: stream.into_iter()
		}
	}
}

impl Indicator for EFFR {
	type Result = f32;

	fn update(&mut self, ticker: &Ticker) -> IndicatorResult<()> {
		// Iterate until we find the next update that is after the current ticker
		// Remember the previous ticker.
		while let Some(line) = self.stream.next() { 
			if let Ok(update) = line {
				println!("{:?}", update);
			  // If the current update is after the ticker, we use the previous update.
				if update.date >= ticker.datetime {
					self.current = Some(update.dff);
					self.date = update.date;
					return Ok(())
				} 
				self.previous = Some(update);
			}
		}
		Err(IndicatorError::InsufficientData)
	}

	fn get_value(&self) -> IndicatorResult<Self::Result> {
		if let Some(result) = self.current {
			Ok(result)
		} else {
			Err(IndicatorError::InsufficientData)
		}
	}

	fn at(&self, _: usize) -> IndicatorResult<Self::Result> {
		Err(IndicatorError::IndexOutOfRange)
	}
}

/// # Notice:
/// 
/// These test cases are tuned to the specific dataset used in the example: "./datasets/indicators/DFF.csv".
/// Therefore, these tests will fail if the dataset is changed.
#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{NaiveDate, Utc};

	#[test]
	fn no_update() {
		let feed = Series::<DFF>::from_csv("./benches/datasets/indicators/DFF.csv");
		let effr = EFFR::new(feed);
		assert!(effr.get_value().is_err());
	}

	#[test]
	fn update() {
		let feed = Series::<DFF>::from_csv("./benches/datasets/indicators/DFF.csv");
		let mut effr = EFFR::new(feed);
		let expected = 0.08;
    let datetime = NaiveDate::from_ymd_opt(2013, 11, 06)
			.unwrap()
			.and_hms_opt(0, 0, 0)
			.unwrap();
		let datetime = DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc);
		effr.update(&Ticker {
			datetime: datetime,
			open: 0.0,
			high: 0.0,
			low: 0.0,
			close: 0.0,
			volume: 0,
		}).unwrap();
		assert_eq!(effr.get_value().unwrap(), expected);
	}
}