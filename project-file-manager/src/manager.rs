use crate::error::Error;
use directories::ProjectDirs;
use std::{
    fs::{self, File},
    path::PathBuf,
};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "nouritsu";
const APPLICATION: &str = "spectrumize";

pub struct PFM {
    config_file: PathBuf,
    data_file: PathBuf,
    log_file: PathBuf,
}

impl PFM {
    pub fn new() -> Result<Self, Error> {
        let dirs =
            ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).ok_or(Error::GetDirectory)?;

        let config_dir = dirs.config_dir();
        fs::create_dir_all(&config_dir).map_err(|e| Error::CreateDir(e))?;
        let data_dir = dirs.data_dir();
        fs::create_dir_all(&data_dir).map_err(|e| Error::CreateDir(e))?;
        let log_dir = dirs.data_dir();
        fs::create_dir_all(&log_dir).map_err(|e| Error::CreateDir(e))?;

        let config_file = config_dir.join("config.toml");
        if !config_file.exists() {}

        let data_file = data_dir.join("data.json");
        if !data_file.exists() { /* Create File & Write Starter */ }

        let log_file = log_dir.join("log.log");
        if !log_file.exists() { /* Create File & Write Init */ }

        Ok(PFM {
            config_file,
            data_file,
            log_file,
        })
    }
}
