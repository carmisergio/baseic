mod bin;
mod conv_types;
mod dec;
mod hex;
mod octal;

use indexmap::{IndexMap, IndexSet};

use crate::Opts;

pub use conv_types::{InputConverterType, OutputConverterType};

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

impl ConversionOutput {
    /// Checks if there are any results
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl From<IndexMap<InputConverterType, ConversionResult>> for ConversionOutput {
    fn from(value: IndexMap<InputConverterType, ConversionResult>) -> Self {
        Self { inner: value }
    }
}

impl FromIterator<(InputConverterType, ConversionResult)> for ConversionOutput {
    fn from_iter<T: IntoIterator<Item = (InputConverterType, ConversionResult)>>(iter: T) -> Self {
        let mut inner = IndexMap::new();

        for (outconv, val) in iter {
            inner.insert(outconv, val);
        }

        Self::from(inner)
    }
}

/// Represents the result of conversion starting from a single input converter type
#[derive(Debug)]
pub struct ConversionResult {
    pub inner: IndexMap<OutputConverterType, String>,
}

impl ConversionResult {
    /// Checks if there are any results
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl From<IndexMap<OutputConverterType, String>> for ConversionResult {
    fn from(value: IndexMap<OutputConverterType, String>) -> Self {
        Self { inner: value }
    }
}

impl FromIterator<(OutputConverterType, String)> for ConversionResult {
    fn from_iter<T: IntoIterator<Item = (OutputConverterType, String)>>(iter: T) -> Self {
        let mut inner = IndexMap::new();

        for (outconv, val) in iter {
            inner.insert(outconv, val);
        }

        Self::from(inner)
    }
}

/// Conversion error
#[derive(Debug)]
pub enum ConversionError {
    NoResults,
}

/// Performs conversion with given options
pub fn do_convert(opts: Opts) -> Result<ConversionOutput, ConversionError> {
    let res: ConversionOutput = opts
        .inconvs
        .into_iter()
        .filter_map(|inconv| {
            // Run input converter
            if let Ok(int) = inconv.get_converter().convert(&opts.input) {
                let res = proces_outconvs(&opts.outconvs, &inconv, int);
                if res.is_empty() {
                    None
                } else {
                    Some((inconv, res))
                }
            } else {
                None
            }
        })
        .collect();

    // Check if result is empty
    if res.is_empty() {
        Err(ConversionError::NoResults)
    } else {
        Ok(res)
    }
}

/// Process output converters from a given intermediate value
pub fn proces_outconvs(
    outconvs: &IndexSet<OutputConverterType>,
    inconv: &InputConverterType,
    input: IntermediateValue,
) -> ConversionResult {
    // Run selected output converters
    outconvs
        .into_iter()
        .filter_map(|outconv| {
            // Check if this output converter is excluded
            if inconv.is_outconv_excluded(outconv) {
                return None;
            }

            if let Ok(out) = outconv.get_converter().convert(input) {
                Some((outconv.clone(), out))
            } else {
                None
            }
        })
        .collect()
}
