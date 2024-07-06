mod args;
mod config;
mod conv_types;

use args::{ArgParseError, ArgVals};
use std::{error::Error, fmt::Display, path::PathBuf};

use crate::{constants::CONFIG_FILE_PATH, strings::usage};
use config::Config;
pub use conv_types::{InputConverterType, OutputConverterType};

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

        // Parse args
        let argvals = ArgVals::from_args(args)?;

        dbg!(argvals);

        Ok(Self {
            input: "TEST".to_string(),
            inconv: None,
            outconvs: config.default_outconvs,
        })
    }
}

/// Parse

/// Error encountered while building of Opts struct
pub enum OptsBuildError<'a> {
    Args(ArgParseError<'a>),
    Config(Box<dyn Error>),
}

impl<'a> OptsBuildError<'a> {
    /// Perform graceful exit?
    pub fn graceful_exit(&self) -> bool {
        if let Self::Args(ArgParseError::GracefulExit) = self {
            true
        } else {
            false
        }
    }
}

impl<'a> Display for OptsBuildError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Args(err) => {
                write!(f, "{}\n", err)?;
                write!(f, "{}", usage())
            }
            Self::Config(err) => write!(
                f,
                "invalid configuration in config file:\n{}",
                err.to_string()
            ),
        }
    }
}

impl<'a> From<ArgParseError<'a>> for OptsBuildError<'a> {
    fn from(value: ArgParseError<'a>) -> Self {
        Self::Args(value)
    }
}
