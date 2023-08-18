use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to get project directories")]
    GetDirectory,

    #[error("unable to create directory, reason: {0}")]
    CreateDir(io::Error),

    #[error("unable to create file, reason: {0}")]
    CreateFile(io::Error),

    #[error("unable to serialize struct, reason: {0}")]
    ConfigSerialize(toml::ser::Error),

    #[error("unable to write to file, reason: {0}")]
    WriteFile(io::Error),
}
