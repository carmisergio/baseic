use serde::{de, Deserialize};
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

use super::{
    ascii::{AsciiInputConverter, AsciiOutputConverter},
    bin::{BinInputConverter, BinOutputConverter},
    dec::{DecInputConverter, DecOutputConverter},
    hex::{HexInputConverter, HexOutputConverter},
    octal::{OctInputConverter, OctOutputConverter},
    InputConverter, OutputConverter,
};

/// Types of Output Converter
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
pub enum OutputConverterType {
    DEC,
    HEX,
    BIN,
    OCT,
    ASCII,
}

impl OutputConverterType {
    /// Get output converter of the specific type
    pub fn get_converter(&self) -> Box<dyn OutputConverter> {
        match self {
            OutputConverterType::DEC => Box::new(DecOutputConverter),
            OutputConverterType::BIN => Box::new(BinOutputConverter),
            OutputConverterType::HEX => Box::new(HexOutputConverter),
            OutputConverterType::OCT => Box::new(OctOutputConverter),
            OutputConverterType::ASCII => Box::new(AsciiOutputConverter),
        }
    }

    /// Compute maximum length in output converter type names
    /// (used for aligned display)
    pub fn max_str_len() -> usize {
        Self::iter()
            .map(|outconv| outconv.to_string().len())
            .max()
            .unwrap()
    }
}

impl FromStr for OutputConverterType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "DEC" => Ok(Self::DEC),
            "HEX" => Ok(Self::HEX),
            "BIN" => Ok(Self::BIN),
            "OCT" => Ok(Self::OCT),
            "ASCII" => Ok(Self::ASCII),
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
    OCT,
    ASCII,
}

impl InputConverterType {
    /// Get input converter of the specific type
    pub fn get_converter(&self) -> Box<dyn InputConverter> {
        match self {
            &InputConverterType::DEC => Box::new(DecInputConverter),
            &InputConverterType::BIN => Box::new(BinInputConverter),
            &InputConverterType::HEX => Box::new(HexInputConverter),
            &InputConverterType::OCT => Box::new(OctInputConverter),
            &InputConverterType::ASCII => Box::new(AsciiInputConverter),
        }
    }
}

impl InputConverterType {
    /// Check if an output converter should not be paired with this input converter
    pub fn is_outconv_excluded(&self, outconv: &OutputConverterType) -> bool {
        match self {
            Self::BIN => outconv == &OutputConverterType::BIN,
            Self::DEC => outconv == &OutputConverterType::DEC,
            Self::HEX => outconv == &OutputConverterType::HEX,
            Self::OCT => outconv == &OutputConverterType::OCT,
            Self::ASCII => outconv == &OutputConverterType::ASCII,
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
            "OCT" => Ok(Self::OCT),
            "ASCII" => Ok(Self::ASCII),
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
