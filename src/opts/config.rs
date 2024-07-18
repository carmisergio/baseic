use serde::Deserialize;
use std::{error::Error, fs, path::Path};

use crate::{
    constants::{default_inconvs, default_outconvs},
    convert::{InputConverterType, OutputConverterType},
};

/// Config file options
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_outconvs")]
    pub default_outconvs: Vec<OutputConverterType>,

    #[serde(default = "default_inconvs")]
    pub default_inconvs: Vec<InputConverterType>,
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
