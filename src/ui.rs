use colored::{ColoredString, Colorize};
use core::fmt;
use std::{env, fmt::Display};

use crate::{
    convert::{ConversionError, ConversionOutput, ConversionResult},
    opts::{InputConverterType, OptsBuildError, OutputConverterType},
};

/// Custom colors used in printing
trait ColorPalette {
    /// Format text as a heading
    fn format_heading(&self) -> ColoredString;
    /// Format text as a subheading
    fn format_subheading(&self) -> ColoredString;
    /// Format text as binary name
    fn format_binary(&self) -> ColoredString;
    /// Format text as a subheading
    fn format_value(&self) -> ColoredString;
}

impl<'a> ColorPalette for &'a str {
    fn format_heading(&self) -> ColoredString {
        self.bold().bright_purple()
    }

    fn format_subheading(&self) -> ColoredString {
        self.normal()
    }

    fn format_binary(&self) -> ColoredString {
        self.bold().bright_white()
    }

    fn format_value(&self) -> ColoredString {
        self.bright_green()
    }
}

/// Get usage string
pub fn usage() -> String {
    let bin = env::args().next().unwrap();
    let mut res = String::new();
    res += &format!(
        "{} {} [-h] [<input converter>] <value> [<output convertesr>]",
        "Usage:".format_heading(),
        bin
    );
    res
}

/// Get help string
pub fn help() -> String {
    let bin = env::args().next().unwrap();
    let mut res = String::new();
    res += &format!(
        "{} v{}\n",
        "baseic".format_binary(),
        env!("CARGO_PKG_VERSION"),
    );
    res += &format!("\n{}\n", usage());
    res += &format!("\n{}\n", "Options:".format_heading());
    res += &format!("  -h: display this message\n");
    res += &format!("\n{}\n", "Input converters:".format_heading());
    res += &format!("  DEC: decimal\n");
    res += &format!("  BIN: binary\n");
    res += &format!("  HEX: hexadecimal\n");
    res += &format!("\n{}\n", "Output converters:".format_heading());
    res += &format!("  DEC: decimal\n");
    res += &format!("  BIN: binary\n");
    res += &format!("  HEX: hexadecimal\n");
    res += &format!("\n{} {} dec 1234 bin hex", "Example:".format_heading(), bin);
    res
}

/// Options build error
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

/// Conversion Error
impl Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ConversionError::Dummy => {
                write!(f, "Dummy error")
            }
        }
    }
}

/// Full Conversion Output
impl Display for ConversionOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (inconv, conv_res) in &self.inner {
            let inconv_str: &str = &inconv.to_string();
            write!(
                f,
                "{} {}: \n",
                "from".format_heading(),
                inconv_str.format_heading()
            )?;

            // Display conversion result
            conv_res.fmt(f)?;
        }
        Ok(())
    }
}

/// Conversion result from a single input converter
impl Display for ConversionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (outconv, val) in &self.inner {
            let outconv: &str = &outconv.to_string();
            let val: &str = &val;
            write!(
                f,
                "    {}: {}\n",
                outconv.format_subheading(),
                val.format_value()
            )?;
        }
        Ok(())
    }
}

impl ToString for InputConverterType {
    fn to_string(&self) -> String {
        match self {
            InputConverterType::DEC => {
                format!("Decimal")
            }
            InputConverterType::BIN => {
                format!("Binary")
            }
            InputConverterType::HEX => {
                format!("Hexadecimal")
            }
        }
    }
}

impl ToString for OutputConverterType {
    fn to_string(&self) -> String {
        match self {
            OutputConverterType::DEC => {
                format!("Decimal")
            }
            OutputConverterType::BIN => {
                format!("Binary")
            }
            OutputConverterType::HEX => {
                format!("Hexadecimal")
            }
        }
    }
}
