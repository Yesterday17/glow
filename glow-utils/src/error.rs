use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to open file")]
    FileOpen(io::Error),
    #[error("failed to read file")]
    FileRead(io::Error),
    #[error("failed to write to file")]
    FileWrite(io::Error),
    #[error("failed to remove file")]
    FileRemove(io::Error),
    #[error("file not exist")]
    FileNotExist,

    #[error("pid file exists")]
    PidFileExist,
}