mod args;
mod config;

use args::{ArgParseError, ArgVals};
use indexmap::{indexset, IndexSet};
use std::{error::Error, path::PathBuf};

use crate::{
    constants::CONFIG_FILE_PATH,
    convert::{InputConverterType, OutputConverterType},
};
use config::Config;

/// Conversion options
///
/// Contains input data and configuration
#[derive(Debug, PartialEq)]
pub struct Opts {
    // Input string to be converted
    pub input: String,
    // Vector of input converters that should be applied
    pub inconvs: IndexSet<InputConverterType>,
    // Vector of output converters that should be applied
    pub outconvs: IndexSet<OutputConverterType>,
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
        config.default_outconvs.into_iter().collect()
    };

    // Decide whether or not to use default input converters
    let inconvs = if let Some(inconv) = args.inconv {
        indexset![inconv]
    } else {
        config.default_inconvs.into_iter().collect()
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
                },
                Opts {
                    input: "test123".to_string(),
                    inconvs: indexset![
                        InputConverterType::HEX,
                        InputConverterType::BIN,
                        InputConverterType::DEC,
                    ],
                    outconvs: indexset![
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
                    outconvs: Some(indexset![
                        OutputConverterType::HEX,
                        OutputConverterType::BIN
                    ]),
                },
                Opts {
                    input: "test123".to_string(),
                    inconvs: indexset![InputConverterType::BIN],
                    outconvs: indexset![OutputConverterType::HEX, OutputConverterType::BIN],
                },
            ),
        ];

        for (config, args, exp) in tests {
            assert_eq!(opts_build_internal(config, args), exp);
        }
    }
}
