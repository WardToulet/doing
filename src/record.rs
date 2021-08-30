use std::fs;

use serde::{Serialize, Deserialize};

/// A recordig of a task, it includes basic information about the task, the stargin and ending
/// time.
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    /// The timestamp of when the tracking started
    starting_timestamp: i64,

    /// The timestamp of when the tracking ended
    ending_timestamp: i64,

    /// What is being tracked
    item: String,

}

impl Record {
    /// Start tracking time
    ///
    /// To get a record from this we first need to stop tracking time useing the [Current::stop()]
    /// function.
    ///
    /// ## Example
    ///
    /// ```rust
    /// // Start tracking time
    /// let record = Record::start("testing the start method");
    ///
    /// // To end tracking time we just call the `.stop` method
    /// let record = record.stop();
    /// ```
    pub fn start(item: &str) -> Current {
        Current {
            starting_timestamp: chrono::Local::now().timestamp(),
            item: item.into(),
        } 
    }
}

/// The currently tracked item
#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    /// The timestamp of when the tracking started
    starting_timestamp: i64,

    /// What is being tracked
    item: String,
}

impl Current {
    /// Stop tracking time
    ///
    /// This stops tracking the time and returns a [Record].
    pub fn stop(self) -> Record {
       Record {
            starting_timestamp: self.starting_timestamp,
            ending_timestamp: chrono::Local::now().timestamp(),
            item: self.item,
       }  
    }

    /// Look at the file on disk holding the current tracking info
    pub fn open() -> Option<Current> {
        let raw = fs::read_to_string(".workingon").ok()?;
        let mut iter = raw.split('|');

        let starting_timestamp = iter.next()?.parse::<i64>().ok()?;
        let item = iter.next()?.into();

        Some(
            Current {
                starting_timestamp,
                item,
            }
        )
    }

    pub fn save(self) -> std::io::Result<()> {
        fs::write(".workingon", format!("{}|{}", self.starting_timestamp, self.item))
    }
}
