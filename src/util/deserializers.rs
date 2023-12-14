pub mod yyyy_mm_dd {
	use chrono::{NaiveDate, NaiveTime, DateTime, Utc, TimeZone, Date};
	use serde::{self, Deserialize, Serializer, Deserializer};

	const FORMAT: &'static str = "%Y-%m-%d";

	pub fn serialize<S>(
			date: &DateTime<Utc>,
			serializer: S,
	) -> Result<S::Ok, S::Error>
	where
			S: Serializer,
	{
			let s = format!("{}", date.format(FORMAT));
			serializer.serialize_str(&s)
	}

	pub fn deserialize<'de, D>(
			deserializer: D,
	) -> Result<DateTime<Utc>, D::Error>
	where
			D: Deserializer<'de>,
	{
		let s: &str = Deserialize::deserialize(deserializer)?;
		// There is probably a better way to do this...
    let parsed_date = NaiveDate::parse_from_str(s, FORMAT)
      .map_err(serde::de::Error::custom)?;
		let default_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
     Ok(DateTime::from_utc(parsed_date.and_time(default_time), Utc))
	}
}

pub mod yyyy_mm_dd_hh_mm_ss {
	use chrono::{DateTime, Utc, TimeZone};
	use serde::{self, Deserialize, Serializer, Deserializer};

  const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

  pub fn serialize<S>(
      date: &DateTime<Utc>,
      serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      let s = format!("{}", date.format(FORMAT));
      serializer.serialize_str(&s)
  }

  pub fn deserialize<'de, D>(
      deserializer: D,
  ) -> Result<DateTime<Utc>, D::Error>
  where
      D: Deserializer<'de>,
  {
      let timestamp: i64 = Deserialize::deserialize(deserializer)?;
      let naive_datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
      Ok(naive_datetime)
  }
}