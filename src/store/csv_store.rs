use std::{fs, path::{Path, PathBuf}};

use crate::record::Record;

use super::Store;

/// A record store that keeps all the records in memory and can dump them to a file
pub struct CsvStore {
    records: Vec<Record>,
    path: PathBuf,
}

impl Store for CsvStore {
    fn push(&mut self, record: &Record) {
        self.records.push(record.clone());
        self.save();
    }

    fn records(&self) -> &Vec<Record> {
        &self.records
    }
}

impl CsvStore {
    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        let file = if path.as_ref().is_file() {
            fs::File::open(&path).unwrap()
        } else {
            fs::File::create(&path).unwrap()
        };

        let mut reader = csv::Reader::from_reader(file);

        let records: Vec<Record> = reader.deserialize().map(|x| x.unwrap()).collect();

        Self {
            records,
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn save(&self) {
        let file = std::fs::File::create(&self.path).unwrap();
        let mut writer = csv::Writer::from_writer(file);

        for rec in &self.records {
            writer.serialize(rec).unwrap();
        }

        writer.flush().unwrap();
    }
}
