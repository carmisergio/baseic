mod args;
mod config;
mod conv_types;

use args::{ArgParseError, ArgVals};
use std::{error::Error, path::PathBuf};

use crate::constants::CONFIG_FILE_PATH;
use config::Config;
pub use conv_types::{InputConverterType, OutputConverterType};

/// Conversion options
/// Contains input data and configuration
#[derive(Debug, PartialEq)]
pub struct Opts {
    pub input: String,
    pub inconvs: Vec<InputConverterType>,
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

        Ok(opts_build_internal(config, argvals))
    }
}

/// Internal build opts from parsed args and config
fn opts_build_internal(config: Config, args: ArgVals) -> Opts {
    // Decide whether or not to use default output converters
    let outconvs = if let Some(outconvs) = args.outconvs {
        outconvs
    } else {
        config.default_outconvs
    };

    // Decide whether or not to use default input converters
    let inconvs = if let Some(inconv) = args.inconv {
        vec![inconv]
    } else {
        config.default_inconvs
    };

    Opts {
        input: args.input,
        inconvs,
        outconvs,
    }
}

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

impl<'a> From<ArgParseError<'a>> for OptsBuildError<'a> {
    fn from(value: ArgParseError<'a>) -> Self {
        Self::Args(value)
    }
}

#[cfg(test)]
mod tests {
    use args::CliOptions;

    use super::*;

    #[test]
    fn opts_build_internal_ok() {
        let tests = [
            (
                Config {
                    default_outconvs: vec![
                        OutputConverterType::HEX,
                        OutputConverterType::BIN,
                        OutputConverterType::DEC,
                    ],
                    default_inconvs: vec![
                        InputConverterType::HEX,
                        InputConverterType::BIN,
                        InputConverterType::DEC,
                    ],
                },
                ArgVals {
                    input: "test123".to_string(),
                    inconv: None,
                    outconvs: None,
                    opts: CliOptions { help: false },
                },
                Opts {
                    input: "test123".to_string(),
                    inconvs: vec![
                        InputConverterType::HEX,
                        InputConverterType::BIN,
                        InputConverterType::DEC,
                    ],
                    outconvs: vec![
                        OutputConverterType::HEX,
                        OutputConverterType::BIN,
                        OutputConverterType::DEC,
                    ],
                },
            ),
            (
                Config {
                    default_outconvs: vec![
                        OutputConverterType::HEX,
                        OutputConverterType::BIN,
                        OutputConverterType::DEC,
                    ],
                    default_inconvs: vec![
                        InputConverterType::HEX,
                        InputConverterType::BIN,
                        InputConverterType::DEC,
                    ],
                },
                ArgVals {
                    input: "test123".to_string(),
                    inconv: Some(InputConverterType::BIN),
                    outconvs: Some(vec![OutputConverterType::HEX, OutputConverterType::BIN]),
                    opts: CliOptions { help: false },
                },
                Opts {
                    input: "test123".to_string(),
                    inconvs: vec![InputConverterType::BIN],
                    outconvs: vec![OutputConverterType::HEX, OutputConverterType::BIN],
                },
            ),
        ];

        for (config, args, exp) in tests {
            assert_eq!(opts_build_internal(config, args), exp);
        }
    }
}
