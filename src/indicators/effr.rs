use super::*;
use crate::{
	util::serde_ext::*,
	series::SeriesIntoIterator
};
use chrono::{DateTime, Utc};
use std::path::Path;	

#[derive(Clone, Deserialize, Debug)]
struct DFF {
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
	series: Series<DFF>,
	stream: SeriesIntoIterator<DFF>
}

impl fmt::Display for EFFR {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "EFFR({:?})", self.current)
	}
}

impl EFFR {
	pub fn from_csv<P: AsRef<Path>>(path: P) -> Self {
		Self {
			previous: None,
			current: None,
			date: DateTime::from_timestamp(0, 0).unwrap(),
			series: Series::<DFF>::from_csv(&path),
			stream: Series::<DFF>::from_csv(&path).into_iter()
		}
	}
}

impl Clone for EFFR {
	fn clone(&self) -> Self {
		Self {
			previous: self.previous.clone(),
			current: self.current.clone(),
			date: self.date.clone(),
			series: self.series.clone(),
			stream: self.series.clone().into_iter()
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

	fn get_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
		let datetime = NaiveDate::from_ymd_opt(year, month, day)
			.unwrap()
			.and_hms_opt(0, 0, 0)
			.unwrap()
			.into();
		DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc)
	}
	
	/// We haven't even called update yet, so there is no value.
	#[test]
	fn no_update() {
		let effr = EFFR::from_csv("./benches/datasets/indicators/DFF.csv");
		assert!(effr.get_value().is_err());
	}

	// We call the update function many times. All calls are for sequential dates and should be sometime
	// in the middle of the data feed
	#[test]
	fn middle() {
		let mut effr = EFFR::from_csv("./benches/datasets/indicators/DFF.csv");
		let datetimes = vec![
			(get_date(2014, 02, 20), 0.07),
			(get_date(2014, 03, 03), 0.07),
			(get_date(2015, 02, 12), 0.12),
			(get_date(2015, 09, 29), 0.13),
			(get_date(2021, 03, 02), 0.07),
			(get_date(2023, 09, 27), 5.33),
		];
		for (datetime, expected) in datetimes {
			assert!(effr.update(&Ticker {
				datetime,
				open: 0.0,
				high: 0.0,
				low: 0.0,
				close: 0.0,
				volume: 0,
			}).is_ok());
			assert_eq!(effr.get_value().unwrap(), expected);
		}
	}

	// We call the update function once with a date that is past the end of the data feed.
	// The expected value should be an error since we do not want to extrapolate the data.
	#[test]
	fn end() {
		let mut effr = EFFR::from_csv("./benches/datasets/indicators/DFF.csv");
		let datetime = get_date(2025, 10, 20); // Feed only goes to 2023-11-06
		assert!(effr.update(&Ticker {
			datetime,
			open: 0.0,
			high: 0.0,
			low: 0.0,
			close: 0.0,
			volume: 0,
		}).is_err());
		assert!(effr.get_value().is_err());
	}

	// Corner Case: We call the update function once with a date that is in the data feed.
	// Then again with a date that is before the previous date.
	// The expected value for the second update should be the exact same as the first
	// since we have not advanced the global clock forward at all.
	#[test]
	fn second_before_first() {
		let mut effr = EFFR::from_csv("./benches/datasets/indicators/DFF.csv");
		let expected_on_first = 0.08; 
		let first = get_date(2021, 10, 20); // Feed only goes to 2023-11-06
		let second = get_date(2019, 10, 20); // Feed only goes to 2023-11-06
		assert!(effr.update(&Ticker {
			datetime: first,
			open: 0.0,
			high: 0.0,
			low: 0.0,
			close: 0.0,
			volume: 0,
		}).is_ok());
		assert_eq!(effr.get_value().unwrap(), expected_on_first);
		assert!(effr.update(&Ticker {
			datetime: second,
			open: 0.0,
			high: 0.0,
			low: 0.0,
			close: 0.0,
			volume: 0,
		}).is_ok());
		assert_eq!(effr.get_value().unwrap(), expected_on_first);
	}
}