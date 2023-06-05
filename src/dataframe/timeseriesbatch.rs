use crate::dataframe::timeseries::TimeSeries;
use crate::util::io::read_lines;

use std::fs;
use std::io::{BufReader, Lines};
use std::path::Path;

#[derive(Debug)]
pub struct TimeSeriesBatch {
    pub data: Vec<TimeSeries>,
}

impl TimeSeriesBatch {
    pub fn new() -> TimeSeriesBatch {
        TimeSeriesBatch { data: Vec::new() }
    }

    pub fn load_from_folder(folder: &str) -> TimeSeriesBatch {
        let mut batch = Self::new();
        let paths = fs::read_dir(folder).unwrap();
        for path in paths {
            let series = TimeSeries::new(path.unwrap().path());
            batch.add(series);
        }
        batch
    }

    pub fn add(&mut self, series: TimeSeries) {
        self.data.push(series);
    }
}
