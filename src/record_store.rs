use crate::record::Record;

/// A record store can store and retrieve records
pub trait RecordStore {
    /// Add a record to the store
    fn push(&mut self, record: Record);

    // NOTE: think about returning an iterator instead of a Vec
    fn records(&self) -> &Vec<Record>;
}

/// A record store that keeps all the records in memory and can dump them to a file
pub struct CsvRecordStore {
    records: Vec<Record>,
}

impl RecordStore for CsvRecordStore {
    fn push(&mut self, record: Record) {
        self.records.push(record);
    }

    fn records(&self) -> &Vec<Record> {
        &self.records
    }
}

impl CsvRecordStore {
    pub fn open() -> Self {
        let file = std::fs::File::open("doing.csv").unwrap();
        let mut reader = csv::Reader::from_reader(file);

        let records: Vec<Record> = reader.deserialize().map(|x| x.unwrap()).collect();

        Self {
            records
        }
    }

    pub fn save(self) {
        let file = std::fs::File::create("doing.csv").unwrap();
        let mut writer = csv::Writer::from_writer(file);

        for rec in self.records {
            writer.serialize(rec).unwrap();
        }

        writer.flush().unwrap();
    }
} 
