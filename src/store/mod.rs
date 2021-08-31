mod csv_store;
mod toml_store;

pub use csv_store::CsvStore;
pub use toml_store::TomlStore;

use crate::record::Record;

/// A record store can store and retrieve records
pub trait Store {
    /// Add a record to the store
    fn push(&mut self, record: &Record);

    /// Retrievea all the records
    fn records(&self) -> &Vec<Record>;
}
