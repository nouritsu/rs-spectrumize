use crate::error::Error;
use directories::ProjectDirs;
use std::path::PathBuf;

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

        let config_file = dirs.config_dir().join("config.toml");
        if !config_file.exists() { /* Create File & Write Default */ }

        let data_file = dirs.data_dir().join("data.json");
        if !data_file.exists() { /* Create File & Write Starter */ }

        let log_file = dirs.data_dir().join("log.log");
        if !log_file.exists() { /* Create File & Write Init */ }

        Ok(PFM {
            config_file,
            data_file,
            log_file,
        })
    }
}
