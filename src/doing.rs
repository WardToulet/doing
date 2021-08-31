use crate::{
    config::{self, Config},
    current::Current,
    record::Record,
    store::Store,
};

pub struct Doing {
    config: Config,
}

pub enum Error {
    AlreadyTracking(Current),
    NotTrackingAnything,
    IoError(std::io::Error),
}

impl Doing {
    pub fn new() -> Doing {
        Doing {
            // FIXME: propagate error
            config: config::get_config().expect("Cannot get config"),
        }
    }

    /// Start tracking an item, returns an error when anlready tracking something
    pub fn now(&self, item: &str) -> Result<Current, Error> {
        match Current::get_current(self.config.current_path()) {
            None => {
                let current = Current::start(item.into());
                current
                    .write(self.config.current_path())
                    .map_err(Error::IoError)?;
                Ok(current)
            }
            Some(current) => Err(Error::AlreadyTracking(current)),
        }
    }

    /// Stop tracking the current item, add it to the store and return it's record
    pub fn done(&self) -> Result<Record, Error> {
        match Current::get_current(self.config.current_path()) {
            Some(current) => {
                let record = current.stop();
                self.config.get_store().push(&record);

                // Clear the current file
                Current::clear(self.config.current_path()).map_err(Error::IoError)?;

                Ok(record)
            }
            None => Err(Error::NotTrackingAnything),
        }
    }

    /// Return what currently is being tracked
    pub fn what(&self) -> Option<Current> {
        Current::get_current(self.config.current_path())
    }
}
