use serde::Deserialize;
use std::{error::Error, fs, path::Path};

use super::conv_types::OutputConverterType;
use crate::constants::default_outconvs;

/// Config file options
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_outconvs")]
    pub default_outconvs: Vec<OutputConverterType>,
}

impl Config {
    /// Read config from file
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        // Read file to string
        let contents = fs::read_to_string(path).unwrap_or("".to_string());

        // Deserialize config
        let config = toml::from_str::<Config>(&contents)?;

        Ok(config)
    }
}
