use serde::{self, Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::store;

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    DeserialzationError(toml::de::Error),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Store {
    Csv {
        #[serde(default = "default_csv_path")]
        path: PathBuf,
    },
    Toml {
        #[serde(default = "default_toml_path")]
        path: PathBuf,
    },
}

impl Default for Store {
    fn default() -> Self {
        Store::Csv {
            path: default_csv_path(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_current_path")]
    current_path: PathBuf,
    store: Store,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            current_path: default_current_path(),
            store: Store::default(),
        }
    }
}

impl Config {
    /// Get a reference to the pahth of the current.doing file
    pub fn current_path(&self) -> &PathBuf {
        &self.current_path
    }

    /// Get the store
    pub fn get_store<'a>(&'a self) -> Box<dyn store::Store + 'a> {
        match &self.store {
            Store::Csv { path } => Box::new(store::CsvStore::open(path)),
            Store::Toml { path } => Box::new(store::TomlStore::open(path)),
        }
    }
}

/// Get the current config from the users config directory,
/// if no config file is found the default config is returened
///
/// ## Errors
/// - [ConfigError::DeserilzaztionError] If the file cannot be parsed
/// - [ConfigError::IoError] If the file cannot be read, not if it doesn't exist this is
///   checked seperately
pub fn get_config() -> Result<Config, ConfigError> {
    let mut path = dirs::config_dir().expect("Cannot find the systems config directory.");
    path.push("doing.toml");

    if path.is_file() {
        toml::from_slice(&fs::read(path).map_err(ConfigError::IoError)?)
            .map_err(ConfigError::DeserialzationError)
    } else {
        Ok(Config::default())
    }
}

/// Return the default path for the csv file
fn default_csv_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Cannot find the users home directory.");
    path.push("doing.csv");

    path
}

/// Return the default path for the csv file
fn default_toml_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Cannot find the users home directory.");
    path.push("doing.toml");

    path
}

/// Return the default path for the .current.doing file
fn default_current_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Cannot find the users home directory.");
    path.push(".currently.doing");

    path
}
