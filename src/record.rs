use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// A recordig of a task, it includes basic information about the task, the stargin and ending
/// time.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    /// The timestamp of when the tracking started
    #[serde(rename = "from")]
    pub starting_timestamp: i64,

    /// The timestamp of when the tracking ended
    #[serde(rename = "to")]
    pub ending_timestamp: i64,

    /// What is being tracked
    pub item: String,

    pub tag: Option<String>,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.item)
    }
}
