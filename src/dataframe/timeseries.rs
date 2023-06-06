use crate::dataframe::ticker::Ticker;
use crate::util::io::read_lines;

use std::fs::File;
use std::io::{BufReader, Lines};
use std::path::Path;

#[derive(Debug)]
pub struct TimeSeries {
    pub symbol: String,
    pub buffer: Lines<BufReader<File>>,
}

impl TimeSeries {
    pub fn new<P: AsRef<Path>>(filename: P) -> TimeSeries {
        let file = filename.as_ref().to_str().unwrap().to_string();
        match read_lines(filename) {
            Ok(buffer) => TimeSeries {
                symbol: file,
                buffer: buffer,
            },
            Err(error) => panic!("The buffer could not be entered: {:?}", error),
        }
    }
}

impl IntoIterator for TimeSeries {
    type Item = Ticker;
    type IntoIter = TimeSeriesIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        TimeSeriesIntoIterator {
            buffer: self.buffer,
            index: 0,
            column_indices: [0, 0, 0, 0, 0, 0],
        }
    }
}

pub struct TimeSeriesIntoIterator {
    buffer: Lines<BufReader<File>>,
    index: usize,
    column_indices: [usize; 6], // 0 -> open idx, 1 -> high idx, 2 -> low idx, 3 -> close idx, 4 -> volume idx, 5 -> datetime idx
}

impl Iterator for TimeSeriesIntoIterator {
    type Item = Ticker;

    fn next(&mut self) -> Option<Ticker> {
        if self.index == 0 {
            if let Some(Ok(line)) = self.buffer.next() {
                for (i, entry) in line.split(',').enumerate() {
                    match entry.to_lowercase().as_str() {
                        "open" => self.column_indices[0] = i,
                        "high" => self.column_indices[1] = i,
                        "low" => self.column_indices[2] = i,
                        "close" => self.column_indices[3] = i,
                        "volume" => self.column_indices[4] = i,
                        "date" => self.column_indices[5] = i,
                        _ => (),
                    };
                }
            }
            self.index += 1;
        }

        if let Some(Ok(line)) = self.buffer.next() {
            let entries: Vec<&str> = line.split(',').collect();
            println!("{:?}", entries);
            return Some(Ticker::new(
                entries[self.column_indices[0]].parse::<f32>().unwrap(),
                entries[self.column_indices[1]].parse::<f32>().unwrap(),
                entries[self.column_indices[2]].parse::<f32>().unwrap(),
                entries[self.column_indices[3]].parse::<f32>().unwrap(),
                entries[self.column_indices[4]].parse::<u32>().unwrap(),
                0,
                // entries[self.column_indices[5]].parse::<u64>().unwrap(),
            ));
        }

        None
    }
}
