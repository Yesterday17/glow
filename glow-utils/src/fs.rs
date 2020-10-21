use glow_common::Result;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

const OPEN_FILE_FAILED: &str = "Failed to open file.";
const READ_FILE_FAILED: &str = "Failed to read file.";
const WRITE_FILE_FAILED: &str = "Failed to write to file.";
const REMOVE_FILE_FAILED: &str = "Failed to remove file.";
const FILE_NOT_EXIST: &str = "File not exist.";

pub fn file_exist(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.exists()
}

pub fn read_file(file_path: &str) -> Result<String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(FILE_NOT_EXIST.to_owned());
    }

    let mut file = fs::File::open(path).expect(OPEN_FILE_FAILED);
    let mut content = String::new();
    file.read_to_string(&mut content).expect(READ_FILE_FAILED);
    Ok(content)
}

pub fn write_file(file_path: &str, content: &str) -> Result<()> {
    let path = Path::new(file_path);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect(OPEN_FILE_FAILED);
    file.write_all(content.as_bytes()).expect(WRITE_FILE_FAILED);
    Ok(())
}

pub fn remove_file(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);
    if path.exists() {
        fs::remove_file(path).expect(REMOVE_FILE_FAILED);
    }
    Ok(())
}
