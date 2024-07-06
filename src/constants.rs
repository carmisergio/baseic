use crate::opts::OutputConverterType;

pub const CONFIG_FILE_PATH: &str = ".config/baseic/config.toml";

// Configuration deafults
pub fn default_outconvs() -> Vec<OutputConverterType> {
    vec![
        OutputConverterType::DEC,
        OutputConverterType::HEX,
        OutputConverterType::BIN,
    ]
}
