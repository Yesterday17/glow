use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use crate::error::Error;

pub fn file_exist<P: AsRef<Path>>(file: P) -> bool {
    file.as_ref().exists()
}

pub fn read_file<P: AsRef<Path>>(file_path: P) -> Result<String, Error> {
    if !file_exist(file_path.as_ref()) {
        return Err(Error::FileNotExist);
    }

    let mut file = fs::File::open(file_path.as_ref()).map_err(|e| Error::FileOpen(e))?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|e| Error::FileRead(e))?;
    Ok(content)
}

pub fn write_file<P: AsRef<Path>>(file_path: P, content: &str) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path.as_ref())
        .map_err(|e| Error::FileOpen(e))?;
    file.write_all(content.as_bytes()).map_err(|e| Error::FileWrite(e))?;
    Ok(())
}

pub fn remove_file<P: AsRef<Path>>(file_path: P) -> Result<(), Error> {
    if file_exist(file_path.as_ref()) {
        fs::remove_file(file_path.as_ref()).map_err(|e| Error::FileRemove(e))?;
    }
    Ok(())
}
