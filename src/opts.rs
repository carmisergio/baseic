use serde::{de, Deserialize};
use std::{
    error::Error,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use crate::constants::{default_outconvs, CONFIG_FILE_PATH};

/// Conversion options
/// Contains input data and configuration
#[derive(Debug)]
pub struct Opts {
    pub input: String,
    pub inconv: Option<InputConverterType>,
    pub outconvs: Vec<OutputConverterType>,
}

impl Opts {
    /// Build Opts from args and config file
    pub fn build(args: &[String]) -> Result<Self, OptsBuildError> {
        // Read config from config file
        let file_path = dirs::home_dir().unwrap().join(CONFIG_FILE_PATH);
        let config = Config::from_file(&PathBuf::from(file_path))
            .map_err(|err| OptsBuildError::Config(err))?;

        Ok(Self {
            input: "TEST".to_string(),
            inconv: None,
            outconvs: config.default_outconvs,
        })
    }
}

/// Error encountered while building of Opts struct
pub enum OptsBuildError {
    Args,
    Config(Box<dyn Error>),
}

impl Display for OptsBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Args => write!(f, "Argument error"), // TODO: Implement proper display
            Self::Config(err) => write!(
                f,
                "invalid configuration in config file:\n{}",
                err.to_string()
            ),
        }
    }
}

/// Config file options
#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_outconvs")]
    default_outconvs: Vec<OutputConverterType>,
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

/// Types of Output Converter
#[derive(Debug)]
pub enum OutputConverterType {
    DEC,
    HEX,
    BIN,
}

impl<'de> Deserialize<'de> for OutputConverterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_ascii_uppercase().as_str() {
            "DEC" => Ok(Self::DEC),
            "HEX" => Ok(Self::HEX),
            "BIN" => Ok(Self::BIN),
            e => Err(de::Error::custom(format!(
                "Invalid Ouptut Converter type: {}",
                e
            ))),
        }
    }
}

/// Types of Input Converter
#[derive(Debug)]
enum InputConverterType {
    DEC,
    HEX,
    BIN,
}

impl<'de> Deserialize<'de> for InputConverterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_ascii_uppercase().as_str() {
            "DEC" => Ok(Self::DEC),
            "HEX" => Ok(Self::HEX),
            "BIN" => Ok(Self::BIN),
            _ => Err(de::Error::custom(format!("invalid Ouptut Converter type"))),
        }
    }
}
