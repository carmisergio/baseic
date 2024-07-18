use serde::{de, Deserialize};
use std::str::FromStr;

use crate::convert::{DecInputConverter, DecOutputConverter, InputConverter, OutputConverter};

/// Types of Output Converter
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OutputConverterType {
    DEC,
    HEX,
    BIN,
}

impl OutputConverterType {
    /// Get output converter of the specific type
    pub fn get_converter(&self) -> Box<dyn OutputConverter> {
        match self {
            OutputConverterType::DEC => Box::new(DecOutputConverter),
            _ => panic!("Not implemented!"),
        }
    }
}

impl FromStr for OutputConverterType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "DEC" => Ok(Self::DEC),
            "HEX" => Ok(Self::HEX),
            "BIN" => Ok(Self::BIN),
            _ => Err(()),
        }
    }
}

impl<'de> Deserialize<'de> for OutputConverterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| de::Error::custom(format!("Invalid Ouptut Converter type: {}", s)))
    }
}

/// Types of Input Converter
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum InputConverterType {
    DEC,
    HEX,
    BIN,
}

impl InputConverterType {
    /// Get input converter of the specific type
    pub fn get_converter(&self) -> Box<dyn InputConverter> {
        match self {
            &InputConverterType::DEC => Box::new(DecInputConverter),
            _ => panic!("Not implemented!"),
        }
    }
}

impl FromStr for InputConverterType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "DEC" => Ok(Self::DEC),
            "HEX" => Ok(Self::HEX),
            "BIN" => Ok(Self::BIN),
            _ => Err(()),
        }
    }
}

impl<'de> Deserialize<'de> for InputConverterType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse()
            .map_err(|_| de::Error::custom(format!("Invalid Ouptut Converter type: {}", s)))
    }
}
