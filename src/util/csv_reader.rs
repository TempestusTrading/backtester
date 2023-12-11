//! CSV Reader
//! 
//! Generic implementation for reading CSV files. In theory, this should work
//! for all CSV structures, assuming the data is consistent.
//! 
//! # Idea: 
//! Turn this into an indicator stream. In theory, you should only need 
//! TimeSeries and Indicator streams.
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

struct CsvReader {
    file: File,
    reader: csv::Reader<File>,
}

impl CsvReader {
    fn new(file_path: &str) -> Result<CsvReader, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let cloned_file = file.try_clone()?;
        let reader = csv::Reader::from_reader(cloned_file);
        Ok(CsvReader { file, reader })
    }

    fn read_records(&mut self) -> Result<Vec<HashMap<String, String>>, csv::Error> {
        let mut records = Vec::new();
        let headers = self.reader.headers()?.clone();

        for result in self.reader.records() {
            let record = result?;
            let mut csv_record = HashMap::new();

            for (i, field) in record.iter().enumerate() {
                let header = &headers[i];
                csv_record.insert(header.to_string(), field.to_string());
            }

            records.push(csv_record);
        }

        Ok(records)
    }
}