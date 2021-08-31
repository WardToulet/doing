use std::{fs, path::{Path, PathBuf}};

use crate::record::Record;

use super::Store;

/// A record store that keeps all the records in memory and can dump them to a file
pub struct TomlStore {
    records: Vec<Record>,
    path: PathBuf,
}

impl Store for TomlStore {
    fn push(&mut self, record: &Record) {
        self.records.push(record.clone());
        self.save();
    }

    fn records(&self) -> &Vec<Record> {
        &self.records
    }
}

impl TomlStore {
    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        let records: Vec<Record> = if let Ok(bytes) = fs::read(&path) {
            toml::from_slice(&bytes).unwrap()
        } else {
            Vec::new()
        };

        Self {
            records,
            path: path.as_ref().to_path_buf(),
        }
    }

    pub fn save(&self) {
        fs::write(&self.path, toml::to_string(&self.records).unwrap()).unwrap();
    }
}
