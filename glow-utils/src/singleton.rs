use crate::fs::{remove_file, write_file};
use crate::error::Error;
use std::path::Path;

pub struct Singleton {
    pid_file: String,
}

impl Singleton {
    pub fn init(file: &str) -> Result<Singleton, Error> {
        let file_path = Path::new(file);
        if file_path.exists() {
            return Err(Error::PidFileExist);
        }

        match write_file(file, &std::process::id().to_string()) {
            Ok(_) => Ok(Singleton {
                pid_file: String::from(file),
            }),
            Err(e) => Err(e),
        }
    }

    /// Singleton.exit() will always try to remove pid file
    pub fn exit(&self) {
        let _ = remove_file(&self.pid_file);
    }
}
