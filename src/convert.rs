mod bin;
mod dec;
mod hex;

pub use dec::{DecInputConverter, DecOutputConverter};

use indexmap::IndexMap;

use crate::{
    opts::{InputConverterType, OutputConverterType},
    Opts,
};

/// Intermediate type used for conversions
type IntermediateValue = i128;

/// Represents a generic output converter
pub trait OutputConverter {
    /// Convert value to the intermediate type
    fn convert(&self, input: IntermediateValue) -> Result<String, ()>;
}

/// Represents a generic input converter
pub trait InputConverter {
    /// Convert value to the intermediate type
    fn convert(&self, input: &str) -> Result<IntermediateValue, ()>;
}

/// Represents the result of conversion
#[derive(Debug)]
pub struct ConversionOutput {
    pub inner: IndexMap<InputConverterType, ConversionResult>,
}

impl From<IndexMap<InputConverterType, ConversionResult>> for ConversionOutput {
    fn from(value: IndexMap<InputConverterType, ConversionResult>) -> Self {
        Self { inner: value }
    }
}

/// Represents the result of conversion starting from a single input converter type
#[derive(Debug)]
pub struct ConversionResult {
    pub inner: IndexMap<OutputConverterType, String>,
}

impl From<IndexMap<OutputConverterType, String>> for ConversionResult {
    fn from(value: IndexMap<OutputConverterType, String>) -> Self {
        Self { inner: value }
    }
}

/// Conversion error
#[derive(Debug)]
pub enum ConversionError {
    Dummy,
}

/// Performs conversion with given options
pub fn do_convert(opts: Opts) -> Result<ConversionOutput, ConversionError> {
    /// Iterate over enum variants
    Err(ConversionError::Dummy)
}
