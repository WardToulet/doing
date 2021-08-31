use std::{fmt::Display, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::record::Record;

/// The currently tracked item
#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    /// The timestamp of when the tracking started
    pub starting_timestamp: i64,

    /// What is being tracked
    pub item: String,

    /// A tag to be able to sort the project
    pub tag: Option<String>,
}

impl Current {
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
    pub fn start(item: &str, tag: Option<String>) -> Current {
        Current {
            starting_timestamp: chrono::Local::now().timestamp(),
            item: item.into(),
            tag
        }
    }

    /// Stop tracking time
    ///
    /// This stops tracking the time and returns a [Record].
    pub fn stop(self) -> Record {
        Record {
            starting_timestamp: self.starting_timestamp,
            ending_timestamp: chrono::Local::now().timestamp(),
            item: self.item,
            tag: self.tag,
        }
    }

    /// Look at the file on disk holding the current tracking info
    pub fn get_current<P: AsRef<Path>>(path: P) -> Option<Current> {
        toml::from_slice(&fs::read(path).ok()?).ok()
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        fs::write(path, &toml::to_vec(&self).expect("Serialization failed"))
    }

    pub fn clear<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        fs::write(path, "")
    }
}

impl Display for Current {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: add the elased time to the display
        write!(f, "{}", self.item)?;
        if let Some(tag) = &self.tag {
            write!(f, " with tag {}", tag)?;
        }

        Ok(())
    }
}
