use crate::convert::{InputConverterType, OutputConverterType};

pub const CONFIG_FILE_PATH: &str = ".config/baseic/config.toml";

// Configuration deafults
pub fn default_outconvs() -> Vec<OutputConverterType> {
    vec![
        OutputConverterType::DEC,
        OutputConverterType::HEX,
        OutputConverterType::BIN,
        OutputConverterType::OCT,
        OutputConverterType::ASCII,
    ]
}

pub fn default_inconvs() -> Vec<InputConverterType> {
    vec![
        InputConverterType::DEC,
        InputConverterType::HEX,
        InputConverterType::BIN,
        InputConverterType::OCT,
        InputConverterType::ASCII,
    ]
}
